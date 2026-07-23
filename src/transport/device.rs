use md5::{Md5, Digest};
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use uuid::Uuid;
use serde::{Serialize, Deserialize};

use crate::constants::{APP_VERSION, APP_VERSION_CODE, WEB_CHROME_VERSION};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceInfo {
    pub device_id: String,
    pub phone_id: String,
    pub uuid: String,
    /// Instagram Android application User-Agent (`User-Agent` header for private API).
    pub device_string: String,
    /// Android WebView / web User-Agent (Chrome 131+). Used for webview-style endpoints.
    /// Defaults on deserialize so older session files without this field still load.
    #[serde(default = "default_web_user_agent")]
    pub web_user_agent: String,
}

fn default_web_user_agent() -> String {
    // Matches the hardcoded device used in `DeviceInfo::generate` (SM-G930F / Android 7.0).
    build_web_user_agent("7.0", "SM-G930F", "NRD90M", &default_app_user_agent())
}

fn default_app_user_agent() -> String {
    format!(
        "Instagram {} Android ({}/{}; {}; {}; {}; {}; {}; {}; en_US; {})",
        APP_VERSION,
        "24",
        "7.0",
        "640dpi",
        "1440x2560",
        "samsung",
        "SM-G930F",
        "herolte",
        "samsungexynos8890",
        APP_VERSION_CODE
    )
}

fn build_web_user_agent(
    android_release: &str,
    model: &str,
    build: &str,
    app_user_agent: &str,
) -> String {
    format!(
        "Mozilla/5.0 (Linux; Android {}; {} Build/{}; wv) AppleWebKit/537.36 (KHTML, like Gecko) Version/4.0 Chrome/{} Mobile Safari/537.36 {}",
        android_release,
        model,
        build,
        WEB_CHROME_VERSION,
        app_user_agent
    )
}

impl DeviceInfo {
    pub fn generate(username: &str) -> Self {
        let mut hasher = Md5::new();
        hasher.update(username.as_bytes());
        let seed = hasher.finalize();
        
        let mut seed_arr = [0u8; 32];
        for i in 0..16 {
            seed_arr[i] = seed[i];
        }
        let mut rng = StdRng::from_seed(seed_arr);
        
        let android_version = "24";
        let android_release = "7.0";
        let dpi = "640dpi";
        let resolution = "1440x2560";
        let manufacturer = "samsung";
        let model = "SM-G930F";
        let device = "herolte";
        let cpu = "samsungexynos8890";
        let build = "NRD90M";
        
        let device_string = format!(
            "Instagram {} Android ({}/{}; {}; {}; {}; {}; {}; {}; en_US; {})",
            APP_VERSION,
            android_version,
            android_release,
            dpi,
            resolution,
            manufacturer,
            model,
            device,
            cpu,
            APP_VERSION_CODE
        );

        let web_user_agent = build_web_user_agent(android_release, model, build, &device_string);

        let device_id = format!("android-{:x}{:x}{:x}{:x}{:x}{:x}{:x}{:x}", 
            rng.gen::<u16>(), rng.gen::<u16>(), rng.gen::<u16>(), rng.gen::<u16>(),
            rng.gen::<u16>(), rng.gen::<u16>(), rng.gen::<u16>(), rng.gen::<u16>());
            
        let mut phone_id_bytes: [u8; 16] = rng.gen();
        phone_id_bytes[6] = (phone_id_bytes[6] & 0x0f) | 0x40;
        phone_id_bytes[8] = (phone_id_bytes[8] & 0x3f) | 0x80;
        let phone_id = Uuid::from_bytes(phone_id_bytes).to_string();

        let mut uuid_bytes: [u8; 16] = rng.gen();
        uuid_bytes[6] = (uuid_bytes[6] & 0x0f) | 0x40;
        uuid_bytes[8] = (uuid_bytes[8] & 0x3f) | 0x80;
        let uuid = Uuid::from_bytes(uuid_bytes).to_string();

        DeviceInfo {
            device_id,
            phone_id,
            uuid,
            device_string,
            web_user_agent,
        }
    }

    /// Ensure `web_user_agent` is present and uses a modern Chrome version.
    /// Call after loading sessions that predate this field (serde default already
    /// covers missing fields; this upgrades stale Chrome/70 strings if any).
    pub fn ensure_modern_web_user_agent(&mut self) {
        if self.web_user_agent.is_empty() || self.web_user_agent.contains("Chrome/70.") {
            self.web_user_agent = default_web_user_agent();
            // Prefer pairing with this device's app UA when available.
            if !self.device_string.is_empty() {
                self.web_user_agent = build_web_user_agent(
                    "7.0",
                    "SM-G930F",
                    "NRD90M",
                    &self.device_string,
                );
            }
        }
    }
}
