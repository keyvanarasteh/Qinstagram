use crate::error::{InstagramError, Result};
use crate::transport::client::InstagramHttpClient;
use crate::types::post::FeedInstance;
use serde::{Deserialize, Serialize};

impl InstagramHttpClient {
    pub async fn get_timeline_feed(&self) -> Result<FeedInstance> {
        let url = format!("{}/api/v1/feed/timeline/", crate::constants::HOST);
        
        // Instagram uses POST for timeline fetching
        let res = self.post(&url)
            .header("content-type", "application/x-www-form-urlencoded; charset=UTF-8")
            .send()
            .await?;
            
        let text = res.text().await.map_err(InstagramError::NetworkError)?;
        
        // As a minimal proxy mapping
        // We gracefully attempt to deserialize, ignoring unrecognized fields
        let feed: FeedInstance = serde_json::from_str(&text).map_err(InstagramError::SerdeError)?;
        
        Ok(feed)
    }
}
