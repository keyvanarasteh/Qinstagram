use reqwest_cookie_store::CookieStoreMutex;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tokio::fs;

use crate::error::{InstagramError, Result};
use crate::transport::device::DeviceInfo;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppState {
    pub device: DeviceInfo,
    pub cookies: String,
}

pub struct SessionManager {
    pub username: String,
    pub session_dir: PathBuf,
}

impl SessionManager {
    pub async fn new(username: &str) -> Result<Self> {
        let mut home_dir = dirs::home_dir().ok_or_else(|| InstagramError::Unknown("Cannot find home directory".to_string()))?;
        home_dir.push(".instagram-cli");
        home_dir.push("users");
        home_dir.push(username);
        
        if !home_dir.exists() {
            fs::create_dir_all(&home_dir).await.map_err(InstagramError::IoError)?;
        }
        
        Ok(Self {
            username: username.to_string(),
            session_dir: home_dir,
        })
    }
    
    pub fn session_file_path(&self) -> PathBuf {
        let mut path = self.session_dir.clone();
        path.push("session.rust.json");
        path
    }

    pub async fn load_state(&self) -> Result<Option<AppState>> {
        let path = self.session_file_path();
        if path.exists() {
            let content = fs::read_to_string(&path).await.map_err(InstagramError::IoError)?;
            let state: AppState = serde_json::from_str(&content).map_err(InstagramError::SerdeError)?;
            Ok(Some(state))
        } else {
            Ok(None)
        }
    }

    pub async fn save_state(&self, device: &DeviceInfo, cookie_store: &CookieStoreMutex) -> Result<()> {
        let path = self.session_file_path();
        let mut buffer = Vec::new();
        {
            #[allow(deprecated)]
            let guard = cookie_store.lock().map_err(|_| InstagramError::Unknown("Failed to lock cookie store".into()))?;
            #[allow(deprecated)]
            guard.save_json(&mut buffer).map_err(|e| InstagramError::Unknown(e.to_string()))?;
        }
        let cookies_str = String::from_utf8(buffer).map_err(|e| InstagramError::Unknown(e.to_string()))?;
        
        let state = AppState {
            device: device.clone(),
            cookies: cookies_str,
        };
        
        let content = serde_json::to_string_pretty(&state).map_err(InstagramError::SerdeError)?;
        fs::write(&path, content).await.map_err(InstagramError::IoError)?;
        Ok(())
    }
}
