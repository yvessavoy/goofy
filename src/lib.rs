use thiserror::Error;

mod client;
mod device;
mod friendships;
mod profile;
mod users;

pub use client::Client;
pub use profile::Profile;

#[cfg(test)]
mod tests;

const INSTAGRAM_SIGN_KEY: &str = "99e16edcca71d7c1f3fd74d447f6281bd5253a623000a55ed0b60014467a53b1";

fn get_base_url() -> String {
    #[cfg(not(test))]
    return "https://i.instagram.com/api/v1".to_owned();

    #[cfg(test)]
    return mockito::server_url();
}

#[derive(Error, Debug)]
pub enum GoofyError {
    #[error("Generic HTTP error")]
    HttpError(#[from] reqwest::Error),
    #[error("Generic IO error")]
    IOError(#[from] std::io::Error),
    #[error("JSON error")]
    JSONError(#[from] serde_json::error::Error),
    #[error("API login failed with status code {0}")]
    LoginFailed(u16),
    #[error("API request failed with status code {0}")]
    ResponseNotSuccess(u16),
}
