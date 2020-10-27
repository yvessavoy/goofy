use crate::Client;
use crate::GoofyError;
use crate::Profile;
use crate::API_BASE_URL;

impl Client {
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
