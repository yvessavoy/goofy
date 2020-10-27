use crate::device::{get_device, Device};
use crate::GoofyError;
use crate::Profile;
use hmac::{Hmac, Mac};
use sha2::Sha256;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use uuid::Uuid;

type HmacSha256 = Hmac<Sha256>;

const API_BASE_URL: &str = "https://i.instagram.com/api/v1";

pub struct Client {
    http: reqwest::blocking::Client,
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
        let client = reqwest::blocking::Client::builder()
            .cookie_store(true)
            .default_headers(get_default_headers())
            .build()?;

        let login_url = format!("{}/accounts/login/", API_BASE_URL);
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
        };

        Ok(client)
    }

    // Store the current session to disk for later usage
    pub fn export(&self, path: &str) -> Result<(), GoofyError> {
        let mut file = File::create(path)?;
        file.write(self.cookies.as_bytes())?;

        Ok(())
    }

    pub fn get_profile_by_username(&self, username: &str) -> Result<Profile, GoofyError> {
        let url = format!("{}/users/{}/usernameinfo/", API_BASE_URL, username);
        let resp = self.http.get(&url).send()?;
        if resp.status() != 200 {
            return Err(GoofyError::ResponseNotSuccess(resp.status().as_u16()));
        }

        let resp_json = resp.json::<serde_json::Value>()?;
        let user = resp_json["user"].clone();
        let profile: Profile = serde_json::from_value(user)?;

        Ok(profile)
    }

    pub fn get_profile_by_id(&self, id: i64) -> Result<Profile, GoofyError> {
        let url = format!("{}/users/{}/info", API_BASE_URL, id);
        let resp = self.http.get(&url).send()?;
        if resp.status() != 200 {
            return Err(GoofyError::ResponseNotSuccess(resp.status().as_u16()));
        }

        let resp_json = resp.json::<serde_json::Value>()?;
        let user = resp_json["user"].clone();
        let profile: Profile = serde_json::from_value(user)?;

        Ok(profile)
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

fn generate_signature(data: &String) -> String {
    let ig_sig_key = "99e16edcca71d7c1f3fd74d447f6281bd5253a623000a55ed0b60014467a53b1";
    let mut mac = HmacSha256::new_varkey(ig_sig_key.as_bytes()).unwrap();
    mac.input(data.as_bytes());
    let body = format!("{:x}", mac.result().code());
    let url_body = format!("ig_sig_key_version=4&signed_body={}.{}", body, data);
    url_body
}
