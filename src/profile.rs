use serde::Deserialize;

#[derive(Deserialize)]
pub struct Profile {
    pub pk: u64,
    pub username: String,
    pub full_name: String,
    #[serde(default)]
    pub biography: String,
    pub is_private: bool,
    pub is_verified: bool,
}
