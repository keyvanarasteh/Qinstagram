use md5::{Md5, Digest};
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use uuid::Uuid;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceInfo {
    pub device_id: String,
    pub phone_id: String,
    pub uuid: String,
    pub device_string: String,
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
        
        let device_string = format!(
            "Instagram {} Android ({}/{}; {}; {}; {}; {}; {}; {}; en_US; {})",
            crate::constants::APP_VERSION,
            android_version,
            android_release,
            dpi,
            resolution,
            manufacturer,
            model,
            device,
            cpu,
            crate::constants::APP_VERSION_CODE
        );

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
        }
    }
}
