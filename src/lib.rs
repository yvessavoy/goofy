use thiserror::Error;

mod client;
mod device;
mod profile;

pub use client::Client;
pub use profile::Profile;

#[cfg(test)]
mod tests;

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
