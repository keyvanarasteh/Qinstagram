use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::error::{InstagramError, Result};
use crate::transport::client::InstagramHttpClient;
use crate::transport::signing::sign_request;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "graphql", derive(async_graphql::SimpleObject))]
pub struct CheckpointError {
    pub message: String,
    pub challenge_url: String,
    pub step_name: String,
}

use crate::auth::session::SessionManager;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "graphql", derive(async_graphql::SimpleObject))]
pub struct TwoFactorInfo {
    pub two_factor_identifier: String,
    pub obfuscated_phone_number: String,
    pub totp_two_factor_on: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "graphql", derive(async_graphql::SimpleObject))]
pub struct LoginResult {
    pub success: bool,
    pub error: Option<String>,
    pub username: Option<String>,
    pub checkpoint_error: Option<CheckpointError>,
    pub two_factor_info: Option<TwoFactorInfo>,
    pub bad_password: bool,
}

impl InstagramHttpClient {
    pub async fn login(&mut self, username: &str, password: &str) -> Result<LoginResult> {
        let payload = json!({
            "username": username,
            "enc_password": format!("#PWD_INSTAGRAM_BROWSER:0:{}:{}", 
                std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_else(|_| std::time::Duration::from_secs(0)).as_secs(), 
                password),
            "phone_id": self.device.phone_id,
            "device_id": self.device.device_id,
            "guid": self.device.uuid,
            "login_attempt_count": "0"
        });

        let signed_body = sign_request(&payload)?;

        let res = self.post(&format!("{}/api/v1/accounts/login/", crate::constants::HOST))
            .header("content-type", "application/x-www-form-urlencoded; charset=UTF-8")
            .body(signed_body)
            .send()
            .await?;

        let text = res.text().await.map_err(InstagramError::NetworkError)?;
        let json_res: serde_json::Value = serde_json::from_str(&text).map_err(InstagramError::SerdeError)?;

        if let Some(two_factor) = json_res.get("two_factor_info") {
            return Ok(LoginResult {
                success: false,
                error: None,
                username: None,
                checkpoint_error: None,
                two_factor_info: serde_json::from_value(two_factor.clone()).ok(),
                bad_password: false,
            });
        }

        if let Some(challenge_obj) = json_res.get("challenge") {
            let ch_err = CheckpointError {
                 message: json_res["message"].as_str().unwrap_or("").to_string(),
                 challenge_url: challenge_obj["api_path"].as_str().unwrap_or("").to_string(),
                 step_name: challenge_obj["step_name"].as_str().unwrap_or("").to_string(),
            };
            return Ok(LoginResult {
                success: false,
                error: Some("challenge_required".to_string()),
                username: None,
                checkpoint_error: Some(ch_err),
                two_factor_info: None,
                bad_password: false,
            });
        }

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
             error: Some(err_msg.clone()),
             username: None,
             checkpoint_error: None,
             two_factor_info: None,
             bad_password: err_msg.contains("password"),
        })
    }

    pub async fn two_factor_login(&mut self, code: &str, identifier: &str, username: &str) -> Result<LoginResult> {
        let payload = json!({
            "username": username,
            "verification_code": code,
            "two_factor_identifier": identifier,
            "device_id": self.device.device_id,
            "guid": self.device.uuid,
        });

        let signed_body = sign_request(&payload)?;

        let res = self.post(&format!("{}/api/v1/accounts/two_factor_login/", crate::constants::HOST))
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

        Ok(LoginResult {
             success: false,
             error: Some(json_res["message"].as_str().unwrap_or("Unknown").to_string()),
             username: None,
             checkpoint_error: None,
             two_factor_info: None,
             bad_password: false,
        })
    }

    #[allow(deprecated)]
    pub async fn login_by_session(&mut self, session_manager: &SessionManager) -> Result<LoginResult> {
        if let Some(state) = session_manager.load_state().await? {
            let cookies_bytes = state.cookies.into_bytes();
            if let Ok(store) = cookie_store::CookieStore::load_json(&cookies_bytes[..]) {
                if let Ok(mut guard) = self.cookie_store.lock() {
                    *guard = store;
                }
            }
            let mut device = state.device;
            device.ensure_modern_web_user_agent();
            self.device = device;
            
            return Ok(LoginResult {
                success: true,
                error: None,
                username: Some(session_manager.username.clone()),
                checkpoint_error: None,
                two_factor_info: None,
                bad_password: false,
            });
        }
        
        Ok(LoginResult {
             success: false,
             error: Some("No active session found".into()),
             username: None,
             checkpoint_error: None,
             two_factor_info: None,
             bad_password: false,
        })
    }

    pub async fn pre_login_flow(&self) -> Result<()> {
        // Normally hits /api/v1/launcher/sync/ but we can just stub it for now
        // Ignores errors to match TypeScript client's behavior
        let _ = self.post(&format!("{}/api/v1/launcher/sync/", crate::constants::HOST))
            .send()
            .await;
        Ok(())
    }

    pub async fn post_login_flow(&self) -> Result<()> {
        // Matches typescript client behavior
        Ok(())
    }
}
