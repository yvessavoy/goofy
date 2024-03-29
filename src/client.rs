use crate::device::{get_device, Device};
use crate::get_base_url;
use crate::GoofyError;
use crate::INSTAGRAM_SIGN_KEY;
use hmac::{Hmac, Mac, NewMac};
use sha2::Sha256;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use uuid::Uuid;

type HmacSha256 = Hmac<Sha256>;

pub struct Client {
    pub http: reqwest::blocking::Client,
    pub base_url: String,
    cookies: String,
}

impl Client {
    // Create a new Client based on username and password
    pub fn new(username: &str, password: &str) -> Result<Self, GoofyError> {
        let phone_id = Uuid::new_v4();
        let guid = Uuid::new_v4();
        let device_id = generate_device_id(username);
        let data = format!(
            r#"
            {{
                "phone_id": "{}",
                "device_id": "{}",
                "guid": "{}",
                "username": "{}",
                "password": "{}"
            }}
        "#,
            phone_id, device_id, guid, username, password
        );
        let sig_data = generate_signature(&data);
        let base_url = get_base_url();
        let client = reqwest::blocking::Client::builder()
            .cookie_store(true)
            .default_headers(get_default_headers())
            .build()?;

        let login_url = format!("{}/accounts/login/", base_url);
        let resp = client.post(&login_url).body(sig_data).send()?;

        let mut cookies = HashMap::new();
        let mut cookie_string = String::new();
        for cookie in resp.cookies() {
            cookies.insert(cookie.name().to_string(), cookie.value().to_string());
            cookie_string.push_str(&format!("{}={}; ", cookie.name(), cookie.value()));
        }

        if resp.status() == 200 {
            Ok(Client {
                http: client,
                cookies: cookie_string,
                base_url,
            })
        } else {
            Err(GoofyError::LoginFailed(resp.status().as_u16()))
        }
    }

    pub fn import(path: &str) -> Result<Self, GoofyError> {
        let cookie_string = std::fs::read_to_string(path)?;

        let mut headers = get_default_headers();
        headers.insert(
            reqwest::header::COOKIE,
            reqwest::header::HeaderValue::from_str(&cookie_string).unwrap(),
        );
        let client = Client {
            http: reqwest::blocking::Client::builder()
                .cookie_store(true)
                .default_headers(headers)
                .build()?,
            cookies: cookie_string,
            base_url: get_base_url(),
        };

        Ok(client)
    }

    // Store the current session to disk for later usage
    pub fn export(&self, path: &str) -> Result<(), GoofyError> {
        let mut file = File::create(path)?;
        file.write_all(self.cookies.as_bytes())?;

        Ok(())
    }
}

fn get_default_headers() -> reqwest::header::HeaderMap {
    let device = get_device();
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        reqwest::header::CONTENT_TYPE,
        reqwest::header::HeaderValue::from_static(
            "application/x-www-form-urlencoded; charset=UTF-8",
        ),
    );
    headers.insert(
        reqwest::header::USER_AGENT,
        reqwest::header::HeaderValue::from_str(&get_user_agent(&device)).unwrap(),
    );

    headers
}

fn get_user_agent(device: &Device) -> String {
    format!(
        "Instagram {} Android ({}/{}; {}; {}; {}; {}; {}; {}; en_US)",
        device.instagram_version,
        device.android_version,
        device.android_release,
        device.dpi,
        device.resolution,
        device.manufacturer,
        device.device,
        device.model,
        device.cpu
    )
}

fn generate_device_id(username: &str) -> String {
    let volatile_seed = "12345";
    let seed = hex_digest(username);
    let digest = md5::compute(format!("{}{}", seed, volatile_seed));
    let digest_str = format!("{:x}", digest);
    format!("android-{}", digest_str[..16].to_string())
}

fn hex_digest(val: &str) -> String {
    let v = format!("{}{}", val, val);
    format!("{:x}", md5::compute(v.as_bytes()))
}

fn generate_signature(data: &str) -> String {
    let mut mac = HmacSha256::new_from_slice(INSTAGRAM_SIGN_KEY.as_bytes()).unwrap();
    mac.update(data.as_bytes());
    let body = format!("{:x}", mac.finalize().into_bytes());
    let url_body = format!("ig_sig_key_version=4&signed_body={}.{}", body, data);
    url_body
}
