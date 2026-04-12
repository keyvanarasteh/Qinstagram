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

    pub async fn search_threads_by_title(&self, query: &str) -> Result<Vec<(Thread, f64)>> {
        let mut threads_cache = Vec::new();
        let mut cursor = None;

        for _ in 0..2 {
            let res = self.get_threads(cursor.as_deref()).await?;
            threads_cache.extend(res.threads);
            cursor = res.next_cursor;
            if !res.has_more || cursor.is_none() {
                break;
            }
        }

        let mut results = Vec::new();
        for thread in threads_cache {
            // jaro_winkler returns ~0.0 for completely different, ~1.0 for same.
            let score = strsim::jaro_winkler(&thread.title.to_lowercase(), &query.to_lowercase());
            if score >= 0.4 {
                results.push((thread, score));
            }
        }

        results.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        Ok(results)
    }

    pub async fn search_thread_by_username(&self, username: &str) -> Result<Option<Thread>> {
        let mut cursor = None;
        for _ in 0..10 {
            let res = self.get_threads(cursor.as_deref()).await?;
            for thread in res.threads {
                if thread.users.iter().any(|u| u.username == username) {
                    return Ok(Some(thread));
                }
            }
            cursor = res.next_cursor;
            if !res.has_more || cursor.is_none() {
                break;
            }
        }
        Ok(None)
    }
}
