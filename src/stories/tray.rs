use crate::error::{InstagramError, Result};
use crate::transport::client::InstagramHttpClient;
use crate::types::story::StoryReel;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReelsTrayResponse {
    pub tray: Vec<StoryReel>,
}

impl InstagramHttpClient {
    pub async fn get_reels_tray(&self) -> Result<Vec<StoryReel>> {
        let url = format!("{}/api/v1/feed/reels_tray/", crate::constants::HOST);
        
        let res = self.get(&url).send().await?;
        let text = res.text().await.map_err(InstagramError::NetworkError)?;
        
        let json_res: serde_json::Value = serde_json::from_str(&text).map_err(InstagramError::SerdeError)?;
        
        let mut reels = Vec::new();
        if let Some(tray) = json_res.get("tray").and_then(|t| t.as_array()) {
            for reel_val in tray {
                if let Ok(reel) = serde_json::from_value::<StoryReel>(reel_val.clone()) {
                    reels.push(reel);
                }
            }
        }
        
        Ok(reels)
    }
}
