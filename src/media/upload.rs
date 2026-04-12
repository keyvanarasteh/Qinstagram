use crate::error::{InstagramError, Result};
use crate::transport::client::InstagramHttpClient;
use std::path::Path;
use tokio::fs;

impl InstagramHttpClient {
    pub async fn send_photo(&self, thread_id: &str, file_path: &Path) -> Result<String> {
        let file_data = fs::read(file_path).await.map_err(InstagramError::IoError)?;
        
        let upload_id = chrono::Utc::now().timestamp_millis().to_string();
        let rand_suffix: u64 = rand::random::<u64>() % 9000000000 + 1000000000;
        let name = format!("{}_0_{}", upload_id, rand_suffix);
        let waterfall_id = uuid::Uuid::new_v4().to_string();
        
        // 1. RUpload
        let rupload_params = serde_json::json!({
            "retry_context": "{\"num_step_auto_retry\":0,\"num_reupload\":0,\"num_step_manual_retry\":0}",
            "media_type": "1",
            "upload_id": upload_id,
            "xsharing_user_ids": "[]",
            "image_compression": "{\"lib_name\":\"moz\",\"lib_version\":\"3.1.m\",\"quality\":\"80\"}"
        });

        let content_length = file_data.len();
        let rupload_url = format!("https://rupload.facebook.com/igphoto/{}", name);

        let res = self.post(&rupload_url)
            .header("X-FB-PHOTO-WATERFALL-ID", waterfall_id)
            .header("X-Entity-Type", "image/jpeg")
            .header("Offset", "0")
            .header("X-Instagram-Rupload-Params", serde_json::to_string(&rupload_params).unwrap_or_default())
            .header("X-Entity-Name", &name)
            .header("X-Entity-Length", content_length.to_string())
            .header("Content-Type", "application/octet-stream")
            .body(file_data)
            .send()
            .await?;
            
        let _text = res.text().await.map_err(InstagramError::NetworkError)?;
        
        // 2. Configure Photo
        let configure_payload = serde_json::json!({
            "action": "send_item",
            "thread_ids": format!("[{}]", thread_id),
            "client_context": uuid::Uuid::new_v4().to_string(),
            "upload_id": upload_id,
            "allow_full_aspect_ratio": "true",
            "_csrftoken": self.get_cookie_value("csrftoken").unwrap_or_default(),
            "device_id": self.device.device_id,
            "guid": self.device.uuid,
        });

        let signed_body = crate::transport::signing::sign_request(&configure_payload)?;
        let config_url = format!("{}/api/v1/direct_v2/threads/broadcast/configure_photo/", crate::constants::HOST);
        
        let conf_res = self.post(&config_url)
            .header("content-type", "application/x-www-form-urlencoded; charset=UTF-8")
            .body(signed_body)
            .send()
            .await?;
            
        let text_res = conf_res.text().await.map_err(InstagramError::NetworkError)?;
        let json_res: serde_json::Value = serde_json::from_str(&text_res).map_err(InstagramError::SerdeError)?;
        
        if json_res["status"] == "ok" {
            Ok(json_res["item_id"].as_str().unwrap_or("").to_string())
        } else {
            Err(InstagramError::ApiError(json_res["message"].as_str().unwrap_or("Failed").into()))
        }
    }

    pub async fn send_video(&self, thread_id: &str, file_path: &Path) -> Result<String> {
        let file_data = fs::read(file_path).await.map_err(InstagramError::IoError)?;
        
        let upload_id = chrono::Utc::now().timestamp_millis().to_string();
        let rand_suffix: u64 = rand::random::<u64>() % 9000000000 + 1000000000;
        let name = format!("{}_0_{}", upload_id, rand_suffix);
        let waterfall_id = uuid::Uuid::new_v4().to_string();
        
        // Basic Rupload for Video without segmentation
        let rupload_params = serde_json::json!({
            "retry_context": "{\"num_step_auto_retry\":0,\"num_reupload\":0,\"num_step_manual_retry\":0}",
            "media_type": "2",
            "xsharing_user_ids": "[]",
            "upload_id": upload_id,
            "upload_media_duration_ms": "5000",
            "direct_v2": "1",
        });

        let content_length = file_data.len();
        let rupload_url = format!("https://rupload.facebook.com/igvideo/{}", name);

        let res = self.post(&rupload_url)
            .header("X-FB-VIDEO-WATERFALL-ID", waterfall_id)
            .header("X-Entity-Type", "video/mp4")
            .header("Offset", "0")
            .header("X-Instagram-Rupload-Params", serde_json::to_string(&rupload_params).unwrap_or_default())
            .header("X-Entity-Name", &name)
            .header("X-Entity-Length", content_length.to_string())
            .header("Content-Type", "application/octet-stream")
            .body(file_data)
            .send()
            .await?;
            
        let _text = res.text().await.map_err(InstagramError::NetworkError)?;
        
        // 2. Transcode Finish (Upload Finish for Video)
        let finish_payload = serde_json::json!({
            "upload_id": upload_id,
            "source_type": "2",
            "video": { "length": 5.0 },
            "_csrftoken": self.get_cookie_value("csrftoken").unwrap_or_default(),
            "device_id": self.device.device_id,
            "guid": self.device.uuid,
        });
        
        let signed_finish = crate::transport::signing::sign_request(&finish_payload)?;
        let _ = self.post(&format!("{}/api/v1/media/upload_finish/", crate::constants::HOST))
            .header("content-type", "application/x-www-form-urlencoded; charset=UTF-8")
            .body(signed_finish)
            .send()
            .await?;
        
        // 3. Configure Video
        let configure_payload = serde_json::json!({
            "action": "send_item",
            "thread_ids": format!("[{}]", thread_id),
            "client_context": uuid::Uuid::new_v4().to_string(),
            "upload_id": upload_id,
            "video_result": "",
            "sampled": "true",
            "_csrftoken": self.get_cookie_value("csrftoken").unwrap_or_default(),
            "device_id": self.device.device_id,
            "guid": self.device.uuid,
        });

        let signed_body = crate::transport::signing::sign_request(&configure_payload)?;
        let config_url = format!("{}/api/v1/direct_v2/threads/broadcast/configure_video/", crate::constants::HOST);
        
        let conf_res = self.post(&config_url)
            .header("content-type", "application/x-www-form-urlencoded; charset=UTF-8")
            .body(signed_body)
            .send()
            .await?;
            
        let text_res = conf_res.text().await.map_err(InstagramError::NetworkError)?;
        let json_res: serde_json::Value = serde_json::from_str(&text_res).map_err(InstagramError::SerdeError)?;
        
        if json_res["status"] == "ok" {
            Ok(json_res["item_id"].as_str().unwrap_or("").to_string())
        } else {
            Err(InstagramError::ApiError(json_res["message"].as_str().unwrap_or("Failed").into()))
        }
    }
}
