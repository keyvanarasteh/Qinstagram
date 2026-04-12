use crate::error::{InstagramError, Result};
use crate::transport::client::InstagramHttpClient;
use crate::types::profile::ProfileInfo;
use crate::types::thread::User;

impl InstagramHttpClient {
    pub async fn get_current_user(&self) -> Result<Option<User>> {
        let guard = self.cookie_store.lock().unwrap();
        let mut pk = None;
        for cookie in guard.iter_any() {
            if cookie.name() == "ds_user_id" {
                pk = Some(cookie.value().to_string());
                break;
            }
        }
        drop(guard); // avoid holding lock while calling async methods
        
        if let Some(user_pk) = pk {
            let info = self.get_user_info(&user_pk).await?;
            Ok(Some(User {
                pk: info.pk,
                username: info.username,
                full_name: info.full_name,
                profile_pic_url: info.profile_pic_url,
                is_verified: info.is_verified,
            }))
        } else {
            Ok(None)
        }
    }

    pub async fn get_user_info(&self, pk: &str) -> Result<ProfileInfo> {
        let url = format!("{}/api/v1/users/{}/info/", crate::constants::HOST, pk);
        
        let res = self.get(&url)
            .send()
            .await?;
            
        let text = res.text().await.map_err(InstagramError::NetworkError)?;
        let json_res: serde_json::Value = serde_json::from_str(&text).map_err(InstagramError::SerdeError)?;
        
        let user_obj = json_res.get("user").ok_or_else(|| InstagramError::Unknown("Missing user object".into()))?;
        let profile: ProfileInfo = serde_json::from_value(user_obj.clone()).map_err(InstagramError::SerdeError)?;
        
        Ok(profile)
    }

    pub async fn get_user_profile(&self, username: &str) -> Result<ProfileInfo> {
        let user_opt = self.search_user_exact(username).await?;
        if let Some(user) = user_opt {
            self.get_user_info(&user.pk).await
        } else {
            Err(InstagramError::ApiError(format!("User {} not found", username)))
        }
    }
}
