use crate::error::Result;

pub struct RealtimeClient {
    // Uses rumqttc when feature is enabled
}

impl RealtimeClient {
    pub fn new() -> Self {
        Self {}
    }
    
    pub async fn connect(&mut self) -> Result<()> {
        Err(crate::error::InstagramError::NotImplemented("MQTT Connect".into()))
    }
}
