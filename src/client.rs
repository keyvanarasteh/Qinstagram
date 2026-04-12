pub use crate::transport::client::InstagramHttpClient as InstagramClient;

use crate::auth::session::SessionManager;
use crate::transport::device::DeviceInfo;
use crate::error::Result;

pub struct ClientBuilder {
    pub username: String,
    pub session_manager: Option<SessionManager>,
}

impl ClientBuilder {
    pub fn new(username: &str) -> Self {
        Self {
            username: username.to_string(),
            session_manager: None,
        }
    }
    
    pub fn with_session_manager(mut self, sm: SessionManager) -> Self {
        self.session_manager = Some(sm);
        self
    }
    
    pub async fn build(self) -> Result<InstagramClient> {
        let device = DeviceInfo::generate(&self.username);
        let client = InstagramClient::default_client(device)?;
        Ok(client)
    }
}

impl InstagramClient {
    pub async fn cleanup_sessions() -> Result<()> {
        SessionManager::cleanup_sessions().await
    }

    pub async fn logout(username: Option<&str>) -> Result<()> {
        let mut config_mgr = crate::config::ConfigManager::new().await?;
        
        let target_username = if let Some(u) = username {
            u.to_string()
        } else {
            config_mgr.config.login.current_username.clone().unwrap_or_default()
        };
        
        if !target_username.is_empty() {
             let sm = SessionManager::new(&target_username).await?;
             sm.delete_session().await?;
             if config_mgr.config.login.current_username.as_deref() == Some(&target_username) {
                 config_mgr.config.login.current_username = None;
                 config_mgr.save().await?;
             }
        } else {
             config_mgr.config.login.current_username = None;
             config_mgr.save().await?;
        }
        Ok(())
    }

    pub async fn switch_user(username: &str) -> Result<()> {
        let sm = SessionManager::new(username).await?;
        let state = sm.load_state().await?;
        if state.is_none() {
            return Err(crate::error::InstagramError::Unknown(format!("No session found for @{}", username)));
        }
        
        let mut config_mgr = crate::config::ConfigManager::new().await?;
        config_mgr.config.login.current_username = Some(username.to_string());
        config_mgr.save().await?;
        Ok(())
    }
}
