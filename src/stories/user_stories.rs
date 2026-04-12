use crate::error::{InstagramError, Result};
use crate::transport::client::InstagramHttpClient;
use crate::types::story::{StoryReel, Story};

impl InstagramHttpClient {
    pub async fn get_stories_for_user(&self, user_id: &str) -> Result<Vec<Story>> {
        let url = format!("{}/api/v1/feed/user/{}/story/", crate::constants::HOST, user_id);
        
        let res = self.get(&url).send().await?;
        let text = res.text().await.map_err(InstagramError::NetworkError)?;
        
        let json_res: serde_json::Value = serde_json::from_str(&text).map_err(InstagramError::SerdeError)?;
        
        if let Some(reel_val) = json_res.get("reel") {
            if !reel_val.is_null() {
                if let Ok(reel) = serde_json::from_value::<StoryReel>(reel_val.clone()) {
                    return Ok(reel.stories);
                }
            }
        }
        
        Ok(Vec::new())
    }

    pub async fn mark_stories_as_seen(&self, stories: &[Story]) -> Result<()> {
        let url = format!("{}/api/v2/media/seen/?rt=", crate::constants::HOST);
        
        use serde_json::json;
        let mut items = Vec::new();
        for s in stories {
            items.push(json!({
                "id": s.id,
                "taken_at": s.taken_at,
                "user": s.user.pk
            }));
        }
        
        let mut current_user_id = "".to_string();
        {
            let guard = self.cookie_store.lock().map_err(|e| InstagramError::Unknown(e.to_string()))?;
            for cookie in guard.iter_any() {
                if cookie.name() == "ds_user_id" {
                    current_user_id = cookie.value().to_string();
                    break;
                }
            }
        }
        
        let payload = json!({
            "items": items,
            "_uuid": self.device.uuid,
            "_uid": current_user_id,
        });

        let signed_body = crate::transport::signing::sign_request(&payload)?;
        
        let res = self.post(&url)
            .header("content-type", "application/x-www-form-urlencoded; charset=UTF-8")
            .body(signed_body)
            .send()
            .await?;
            
        if res.status().is_success() {
            Ok(())
        } else {
            Err(InstagramError::ApiError("Failed to mark stories as seen".into()))
        }
    }
}
