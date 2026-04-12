use reqwest_cookie_store::CookieStoreMutex;
use reqwest::{Client, RequestBuilder};
use std::sync::Arc;

use crate::transport::device::DeviceInfo;
use crate::transport::headers;
use crate::error::Result;

pub struct InstagramHttpClient {
    pub client: Client,
    pub cookie_store: Arc<CookieStoreMutex>,
    pub device: DeviceInfo,
}

impl InstagramHttpClient {
    pub fn new(device: DeviceInfo, cookie_store: Arc<CookieStoreMutex>) -> Result<Self> {
        let mut req_headers = headers::request_headers(&device.device_string);
        
        if let Ok(uuid_val) = reqwest::header::HeaderValue::from_str(&device.uuid) {
            req_headers.insert(
                reqwest::header::HeaderName::from_static("x-pigeon-session-id"),
                uuid_val
            );
        }

        let reqwest_client = Client::builder()
            .cookie_provider(Arc::clone(&cookie_store))
            .default_headers(req_headers)
            .build()?;

        Ok(Self {
            client: reqwest_client,
            cookie_store,
            device,
        })
    }

    pub fn default_client(device: DeviceInfo) -> Result<Self> {
        Self::new(device, Arc::new(CookieStoreMutex::default()))
    }

    pub fn get(&self, url: &str) -> RequestBuilder {
        self.client.get(url)
    }

    pub fn post(&self, url: &str) -> RequestBuilder {
        self.client.post(url)
    }
}
