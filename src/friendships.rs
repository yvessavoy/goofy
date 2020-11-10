use crate::Client;
use crate::GoofyError;
use crate::Profile;
use serde::Deserialize;

#[derive(Deserialize)]
struct FollowingResponse {
    next_max_id: Option<i32>,
    #[serde(default)]
    users: Vec<Profile>,
}

#[derive(Deserialize)]
struct FollowerResponse {
    next_max_id: Option<String>,
    #[serde(default)]
    users: Vec<Profile>,
}

impl Client {
    // Get users that follow a specific profile
    pub fn get_followers(&self, user_id: u64) -> Result<Vec<Profile>, GoofyError> {
        let base_url = format!("{}/friendships/{}/followers/", self.base_url, user_id);
        let mut max_id = String::new();
        let mut profiles: Vec<Profile> = Vec::new();

        loop {
            let url = format!("{}?max_id={}&ig_sig_key_version=4", base_url, max_id);
            let r = self.http.get(&url).send()?;
            if r.status() != 200 {
                return Err(GoofyError::ResponseNotSuccess(r.status().as_u16()));
            }

            let mut response: FollowerResponse = r.json()?;
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
        let base_url = format!("{}/friendships/{}/following/", self.base_url, user_id);
        let mut max_id: i32 = 0;
        let mut profiles: Vec<Profile> = Vec::new();

        loop {
            let url = format!("{}?max_id={}&ig_sig_key_version=4", base_url, max_id);
            let r = self.http.get(&url).send()?;
            if r.status() != 200 {
                return Err(GoofyError::ResponseNotSuccess(r.status().as_u16()));
            }

            let mut response: FollowingResponse = r.json()?;
            profiles.append(&mut response.users);

            match response.next_max_id {
                Some(id) => max_id = id,
                None => break,
            };
        }

        Ok(profiles)
    }
}
