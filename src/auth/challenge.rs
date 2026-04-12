use serde_json::json;

use crate::error::{InstagramError, Result};
use crate::transport::client::InstagramHttpClient;
use crate::transport::signing::sign_request;
use crate::auth::login::LoginResult;

impl InstagramHttpClient {
    pub async fn start_challenge(&mut self, challenge_url: &str) -> Result<()> {
        let csrf_token = self.get_cookie_value("csrftoken").unwrap_or_else(|| "missing".to_string());
        
        let payload = json!({
            "choice": "0",
            "_csrftoken": csrf_token,
            "guid": self.device.uuid,
            "device_id": self.device.device_id,
        });
        
        let signed_body = sign_request(&payload)?;
        let target_url = format!("{}{}", crate::constants::HOST, challenge_url);

        let res = self.post(&target_url)
            .header("content-type", "application/x-www-form-urlencoded; charset=UTF-8")
            .body(signed_body)
            .send()
            .await?;

        let _text = res.text().await.map_err(InstagramError::NetworkError)?;
        Ok(())
    }

    pub async fn send_challenge_code(&mut self, challenge_url: &str, code: &str, username: &str) -> Result<LoginResult> {
        let csrf_token = self.get_cookie_value("csrftoken").unwrap_or_else(|| "missing".to_string());
        
        let payload = json!({
            "security_code": code,
            "_csrftoken": csrf_token,
            "guid": self.device.uuid,
            "device_id": self.device.device_id,
        });

        let signed_body = sign_request(&payload)?;
        let target_url = format!("{}{}", crate::constants::HOST, challenge_url);

        let res = self.post(&target_url)
            .header("content-type", "application/x-www-form-urlencoded; charset=UTF-8")
            .body(signed_body)
            .send()
            .await?;

        let text = res.text().await.map_err(InstagramError::NetworkError)?;
        let json_res: serde_json::Value = serde_json::from_str(&text).map_err(InstagramError::SerdeError)?;

        if json_res["status"] == "ok" {
             return Ok(LoginResult {
                 success: true,
                 error: None,
                 username: Some(username.to_string()),
                 checkpoint_error: None,
                 two_factor_info: None,
                 bad_password: false,
             });
        }

        let err_msg = json_res["message"].as_str().unwrap_or("Unknown").to_string();
        Ok(LoginResult {
             success: false,
             error: Some(err_msg),
             username: None,
             checkpoint_error: None,
             two_factor_info: None,
             bad_password: false,
        })
    }
}
