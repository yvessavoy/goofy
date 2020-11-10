use crate::Client;
use crate::GoofyError;
use crate::Profile;
use serde::Deserialize;

#[derive(Deserialize)]
struct UsersResponse {
    user: Profile,
}

impl Client {
    pub fn get_profile_by_username(&self, username: &str) -> Result<Profile, GoofyError> {
        let url = format!("{}/users/{}/usernameinfo/", self.base_url, username);
        let r = self.http.get(&url).send()?;
        if r.status() != 200 {
            return Err(GoofyError::ResponseNotSuccess(r.status().as_u16()));
        }

        let response: UsersResponse = r.json()?;

        Ok(response.user)
    }

    pub fn get_profile_by_id(&self, id: i64) -> Result<Profile, GoofyError> {
        let url = format!("{}/users/{}/info/", self.base_url, id);
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
