use crate::Client;
use crate::GoofyError;
use crate::Profile;
use crate::API_BASE_URL;
use serde::Deserialize;

#[derive(Deserialize)]
struct FollowingResponse {
    next_max_id: Option<i32>,
    users: Vec<Profile>,
}

#[derive(Deserialize)]
struct FollowerResponse {
    next_max_id: Option<String>,
    users: Vec<Profile>,
}

impl Client {
    // Get users that follow a specific profile
    pub fn get_followers(&self, user_id: u64) -> Result<Vec<Profile>, GoofyError> {
        let base_url = format!("{}/friendships/{}/followers/", API_BASE_URL, user_id);
        let mut max_id = String::new();
        let mut profiles: Vec<Profile> = Vec::new();

        loop {
            let url = format!("{}?max_id={}&ig_sig_key_version=4", base_url, max_id);
            let mut response: FollowerResponse = self.http.get(&url).send()?.json()?;
            profiles.append(&mut response.users);

            match response.next_max_id {
                Some(id) => max_id = id.to_string(),
                None => break,
            };
        }

        Ok(profiles)
    }

    // Get users that a specific profile follows
    pub fn get_following(&self, user_id: u64) -> Result<Vec<Profile>, GoofyError> {
        let base_url = format!("{}/friendships/{}/following", API_BASE_URL, user_id);
        let mut max_id = String::new();
        let mut profiles: Vec<Profile> = Vec::new();

        loop {
            let url = format!("{}?max_id={}&ig_sig_key_version=4", base_url, max_id);
            let mut response: FollowingResponse = self.http.get(&url).send()?.json()?;
            profiles.append(&mut response.users);

            match response.next_max_id {
                Some(id) => max_id = id.to_string(),
                None => break,
            };
        }

        Ok(profiles)
    }
}