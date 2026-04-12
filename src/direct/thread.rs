use crate::error::{InstagramError, Result};
use crate::transport::client::InstagramHttpClient;
use crate::types::message::Message;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessagesResult {
    pub messages: Vec<Message>,
    pub has_more: bool,
    pub next_cursor: Option<String>,
}

impl InstagramHttpClient {
    pub async fn get_messages(&self, thread_id: &str, cursor: Option<&str>) -> Result<MessagesResult> {
        let mut url = format!("{}/api/v1/direct_v2/threads/{}/", crate::constants::HOST, thread_id);
        if let Some(c) = cursor {
            url.push_str(&format!("?cursor={}", url::form_urlencoded::byte_serialize(c.as_bytes()).collect::<String>()));
        }

        let res = self.get(&url)
            .send()
            .await?;
            
        let text = res.text().await.map_err(InstagramError::NetworkError)?;
        let json_res: serde_json::Value = serde_json::from_str(&text).map_err(InstagramError::SerdeError)?;
        
        let mut messages = Vec::new();
        if let Some(thread) = json_res.get("thread") {
            if let Some(items_arr) = thread.get("items").and_then(|i| i.as_array()) {
                for item_val in items_arr {
                    if let Ok(msg) = serde_json::from_value::<Message>(item_val.clone()) {
                        messages.push(msg);
                    }
                }
            }
        }
        
        let has_more = json_res.get("thread").and_then(|i| i.get("has_older")).and_then(|h| h.as_bool()).unwrap_or(false);
        let next_cursor = json_res.get("thread").and_then(|i| i.get("oldest_cursor")).and_then(|c| c.as_str()).map(|s| s.to_string());
        
        Ok(MessagesResult {
            messages,
            has_more,
            next_cursor,
        })
    }
}
