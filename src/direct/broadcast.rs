use crate::error::{InstagramError, Result};
use crate::transport::client::InstagramHttpClient;
use crate::types::message::Message;

impl InstagramHttpClient {
    pub async fn send_message(&self, thread_id: &str, text: &str) -> Result<String> {
        self.broadcast_text(thread_id, text, None).await
    }

    pub async fn send_reply(&self, thread_id: &str, text: &str, reply_to: &Message) -> Result<String> {
        self.broadcast_text(thread_id, text, Some(reply_to)).await
    }

    async fn broadcast_text(&self, thread_id: &str, text: &str, reply_to: Option<&Message>) -> Result<String> {
        use serde_json::json;
        use crate::transport::signing;
        
        let client_context = uuid::Uuid::new_v4().to_string();
        let mut payload = json!({
            "thread_ids": format!("[{}]", thread_id),
            "client_context": client_context,
            "text": text,
            "action": "send_item"
        });

        if let Some(msg) = reply_to {
            if let Some(obj) = payload.as_object_mut() {
                let item_id = match msg {
                    Message::Text(m) => &m.base.item_id,
                    Message::Media(m) => &m.base.item_id,
                    Message::Link(m) => &m.base.item_id,
                    Message::Placeholder(m) => &m.base.item_id,
                    Message::MediaShare(m) => &m.base.item_id,
                };
                if let Some(id) = item_id {
                    obj.insert("replied_to_action_source".into(), json!("messaging_controls"));
                    obj.insert("replied_to_item_id".into(), json!(id));
                    let ctx = match msg {
                        Message::Text(m) => &m.base.client_context,
                        Message::Media(m) => &m.base.client_context,
                        Message::Link(m) => &m.base.client_context,
                        Message::Placeholder(m) => &m.base.client_context,
                        Message::MediaShare(m) => &m.base.client_context,
                    };
                    if let Some(c) = ctx {
                        obj.insert("replied_to_client_context".into(), json!(c));
                    }
                }
            }
        }
        
        let signed_body = signing::sign_request(&payload)?;
        let url = format!("{}/api/v1/direct_v2/threads/broadcast/text/", crate::constants::HOST);
        
        let res = self.post(&url)
            .header("content-type", "application/x-www-form-urlencoded; charset=UTF-8")
            .body(signed_body)
            .send()
            .await?;
            
        let text_res = res.text().await.map_err(InstagramError::NetworkError)?;
        let json_res: serde_json::Value = serde_json::from_str(&text_res).map_err(InstagramError::SerdeError)?;
        
        if json_res["status"] == "ok" {
            Ok(client_context)
        } else {
            Err(InstagramError::ApiError(json_res["message"].as_str().unwrap_or("Failed to send message").to_string()))
        }
    }

    pub async fn unsend_message(&self, thread_id: &str, item_id: &str) -> Result<()> {
        let url = format!("{}/api/v1/direct_v2/threads/{}/items/{}/delete/", crate::constants::HOST, thread_id, item_id);
        use serde_json::json;
        let payload = json!({
            "client_context": uuid::Uuid::new_v4().to_string(),
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
            Err(InstagramError::ApiError("Failed to unsend message".into()))
        }
    }

    pub async fn mark_thread_as_seen(&self, thread_id: &str, item_id: &str) -> Result<()> {
        use serde_json::json;
        use crate::transport::signing::sign_request;
        let action = "mark_seen";
        let url = format!("{}/api/v1/direct_v2/threads/{}/items/{}/seen/", crate::constants::HOST, thread_id, item_id);
        
        let payload = json!({
            "action": action,
            "thread_id": thread_id,
            "item_id": item_id,
            "_csrftoken": self.get_cookie_value("csrftoken").unwrap_or_default(),
            "device_id": self.device.device_id,
            "guid": self.device.uuid,
        });
        
        let signed_body = sign_request(&payload)?;
        
        let res = self.post(&url)
            .header("content-type", "application/x-www-form-urlencoded; charset=UTF-8")
            .body(signed_body)
            .send()
            .await?;
            
        let _text = res.text().await.map_err(InstagramError::NetworkError)?;
        Ok(())
    }

    /// Unified seen state dispatcher modeling the TypeScript client's fallback mechanics.
    /// In a fully-enabled realtime build, this would try MQTT `RealtimeClient::mark_as_seen` first,
    /// before falling back to HTTP.
    pub async fn mark_item_as_seen(&self, thread_id: &str, item_id: &str) -> Result<()> {
        // Fallback directly to HTTP 
        self.mark_thread_as_seen(thread_id, item_id).await
    }
}
