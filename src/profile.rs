use serde::Deserialize;

#[derive(Deserialize)]
pub struct Profile {
    pub username: String,
    pub full_name: String,
    pub biography: String,
    pub is_private: bool,
    pub is_verified: bool,
}
