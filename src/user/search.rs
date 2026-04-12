use crate::error::{InstagramError, Result};
use crate::transport::client::InstagramHttpClient;
use crate::types::thread::User;
use serde::{Deserialize, Serialize};
use url::form_urlencoded;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSearchResult {
    pub user: User,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResponse {
    pub num_results: i32,
    pub users: Vec<UserSearchResult>,
}

impl InstagramHttpClient {
    pub async fn search_users(&self, query: &str) -> Result<Vec<User>> {
        let qs = form_urlencoded::Serializer::new(String::new())
            .append_pair("q", query)
            .append_pair("timezone_offset", "0")
            .finish();
            
        let url = format!("{}/api/v1/users/search/?{}", crate::constants::HOST, qs);
        
        let res = self.get(&url)
            .send()
            .await?;
            
        let text = res.text().await.map_err(InstagramError::NetworkError)?;
        let json_res: SearchResponse = serde_json::from_str(&text).map_err(InstagramError::SerdeError)?;
        
        Ok(json_res.users.into_iter().map(|u| u.user).collect())
    }

    pub async fn search_user_exact(&self, username: &str) -> Result<Option<User>> {
        let url = format!("{}/api/v1/users/{}/usernameinfo/", crate::constants::HOST, username);
        
        let res = self.get(&url)
            .send()
            .await?;
            
        if res.status() == reqwest::StatusCode::NOT_FOUND {
            return Ok(None);
        }
            
        let text = res.text().await.map_err(InstagramError::NetworkError)?;
        let json_res: serde_json::Value = serde_json::from_str(&text).map_err(InstagramError::SerdeError)?;
        
        if let Some(user_obj) = json_res.get("user") {
            let user: User = serde_json::from_value(user_obj.clone()).map_err(InstagramError::SerdeError)?;
            Ok(Some(user))
        } else {
            Ok(None)
        }
    }
}
