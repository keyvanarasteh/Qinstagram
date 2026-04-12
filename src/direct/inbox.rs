use crate::error::{InstagramError, Result};
use crate::transport::client::InstagramHttpClient;
use crate::types::thread::Thread;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InboxResult {
    pub threads: Vec<Thread>,
    pub has_more: bool,
    pub next_cursor: Option<String>,
}

impl InstagramHttpClient {
    pub async fn get_threads(&self, cursor: Option<&str>) -> Result<InboxResult> {
        let mut url = format!("{}/api/v1/direct_v2/inbox/?visual_message_return_type=unseen", crate::constants::HOST);
        if let Some(c) = cursor {
            url.push_str(&format!("&cursor={}", url::form_urlencoded::byte_serialize(c.as_bytes()).collect::<String>()));
        }

        let res = self.get(&url)
            .send()
            .await?;
            
        let text = res.text().await.map_err(InstagramError::NetworkError)?;
        let json_res: serde_json::Value = serde_json::from_str(&text).map_err(InstagramError::SerdeError)?;
        
        let mut threads = Vec::new();
        if let Some(inbox) = json_res.get("inbox") {
            if let Some(threads_arr) = inbox.get("threads").and_then(|t| t.as_array()) {
                for thread_val in threads_arr {
                    if let Ok(th) = serde_json::from_value::<Thread>(thread_val.clone()) {
                        threads.push(th);
                    }
                }
            }
        }
        
        let has_more = json_res.get("inbox").and_then(|i| i.get("has_older")).and_then(|h| h.as_bool()).unwrap_or(false);
        let next_cursor = json_res.get("inbox").and_then(|i| i.get("oldest_cursor")).and_then(|c| c.as_str()).map(|s| s.to_string());
        
        Ok(InboxResult {
            threads,
            has_more,
            next_cursor,
        })
    }

    pub async fn ensure_thread(&self, user_pks: &[&str]) -> Result<Thread> {
        let qs = url::form_urlencoded::Serializer::new(String::new())
            .append_pair("recipient_users", &format!("[{}]", user_pks.join(",")))
            .finish();
            
        let url = format!("{}/api/v1/direct_v2/threads/get_by_participants/?{}", crate::constants::HOST, qs);
        
        let res = self.get(&url).send().await?;
        let text = res.text().await.map_err(InstagramError::NetworkError)?;
        let json_res: serde_json::Value = serde_json::from_str(&text).map_err(InstagramError::SerdeError)?;
        
        let thread_val = json_res.get("thread").ok_or_else(|| InstagramError::Unknown("Missing thread obj".into()))?;
        let thread: Thread = serde_json::from_value(thread_val.clone()).map_err(InstagramError::SerdeError)?;
        Ok(thread)
    }
}
