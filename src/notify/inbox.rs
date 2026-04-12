use crate::error::{InstagramError, Result};
use crate::transport::client::InstagramHttpClient;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewsInbox {
    pub new_stories: Option<Vec<serde_json::Value>>,
    pub old_stories: Option<Vec<serde_json::Value>>,
}

impl InstagramHttpClient {
    pub async fn get_news_inbox(&self) -> Result<NewsInbox> {
        let url = format!("{}/api/v1/news/inbox/", crate::constants::HOST);
        
        let res = self.get(&url).send().await?;
        let text = res.text().await.map_err(InstagramError::NetworkError)?;
        
        let inbox: NewsInbox = serde_json::from_str(&text).map_err(InstagramError::SerdeError)?;
        
        Ok(inbox)
    }
}
