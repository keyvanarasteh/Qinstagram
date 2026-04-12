use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tokio::fs;

use crate::error::{InstagramError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub language: String,
    pub login: LoginConfig,
    pub chat: ChatConfig,
    pub privacy: PrivacyConfig,
    pub feed: FeedConfig,
    pub image: ImageConfig,
    pub advanced: AdvancedConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LoginConfig {
    pub default_account: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ChatConfig {
    pub show_seen_receipts: bool,
    pub show_typing_indicators: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PrivacyConfig {
    pub anonymous_story_viewing: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FeedConfig {
    pub hide_ads: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ImageConfig {
    pub disable_cache: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AdvancedConfig {
    pub proxy_url: Option<String>,
    pub debug_mode: bool,
}

pub struct ConfigManager {
    config_path: PathBuf,
    pub config: Config,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            language: "en".to_string(),
            login: Default::default(),
            chat: ChatConfig { show_seen_receipts: true, show_typing_indicators: true },
            privacy: Default::default(),
            feed: Default::default(),
            image: Default::default(),
            advanced: Default::default(),
        }
    }
}

impl ConfigManager {
    pub async fn new() -> Result<Self> {
        let mut home_dir = dirs::home_dir().ok_or_else(|| InstagramError::Unknown("Cannot find home directory".to_string()))?;
        home_dir.push(".instagram-cli");
        
        let mut config_path = home_dir.clone();
        config_path.push("config.ts.yaml");
        
        if !home_dir.exists() {
            fs::create_dir_all(&home_dir).await.map_err(InstagramError::IoError)?;
        }

        let mut manager = Self {
            config_path,
            config: Config::default(),
        };

        manager.load().await?;
        Ok(manager)
    }

    pub async fn load(&mut self) -> Result<()> {
        if self.config_path.exists() {
            let content = fs::read_to_string(&self.config_path).await.map_err(InstagramError::IoError)?;
            self.config = serde_yaml::from_str(&content).map_err(|e| InstagramError::Unknown(e.to_string()))?;
        } else {
            self.save().await?;
        }
        Ok(())
    }

    pub async fn save(&self) -> Result<()> {
        let content = serde_yaml::to_string(&self.config).map_err(|e| InstagramError::Unknown(e.to_string()))?;
        fs::write(&self.config_path, content).await.map_err(InstagramError::IoError)?;
        Ok(())
    }
}
