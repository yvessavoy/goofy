use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Profile {
    pub pk: u64,
    pub username: String,
    #[serde(default)]
    pub full_name: String,
    #[serde(default)]
    pub biography: String,
    pub is_private: bool,
    pub is_verified: bool,
}
