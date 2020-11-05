use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Profile {
    pub pk: u64,
    pub username: String,
    #[serde(default)]
    pub full_name: String,
    #[serde(default)]
    pub biography: String,
    #[serde(default)]
    pub is_private: bool,
    #[serde(default)]
    pub is_verified: bool,
}
