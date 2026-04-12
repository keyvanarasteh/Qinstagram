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
