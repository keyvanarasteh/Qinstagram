use reqwest::header::{HeaderMap, HeaderValue, HeaderName};
use reqwest::header::{ACCEPT_LANGUAGE, USER_AGENT};
use std::time::{SystemTime, UNIX_EPOCH};

pub fn base_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(
        HeaderName::from_static("x-ig-app-id"),
        HeaderValue::from_static(crate::constants::FACEBOOK_ANALYTICS_APP_ID),
    );
    headers.insert(
        HeaderName::from_static("x-ig-app-locale"),
        HeaderValue::from_static("en_US"),
    );
    headers.insert(
        HeaderName::from_static("x-ig-device-locale"),
        HeaderValue::from_static("en_US"),
    );
    headers.insert(
        HeaderName::from_static("x-ig-mapped-locale"),
        HeaderValue::from_static("en_US"),
    );
    headers.insert(
        HeaderName::from_static("x-ig-connection-speed"),
        HeaderValue::from_static("-1kbps"),
    );
    headers.insert(
        HeaderName::from_static("x-ig-bandwidth-speed-kbps"),
        HeaderValue::from_static("-1.000"),
    );
    headers.insert(
        HeaderName::from_static("x-ig-bandwidth-totalbytes-b"),
        HeaderValue::from_static("0"),
    );
    headers.insert(
        HeaderName::from_static("x-ig-bandwidth-totaltime-ms"),
        HeaderValue::from_static("0"),
    );
    headers.insert(
        HeaderName::from_static("x-ig-www-claim"),
        HeaderValue::from_static("0"),
    );
    headers.insert(
        HeaderName::from_static("x-ig-connection-type"),
        HeaderValue::from_static("WIFI"),
    );
    headers.insert(
        ACCEPT_LANGUAGE,
        HeaderValue::from_static("en-US,en;q=0.8"),
    );
    headers
}

pub fn request_headers(user_agent: &str) -> HeaderMap {
    let mut headers = base_headers();
    if let Ok(ua) = HeaderValue::from_str(user_agent) {
        headers.insert(USER_AGENT, ua);
    }
    
    if let Ok(duration) = SystemTime::now().duration_since(UNIX_EPOCH) {
        let timestamp = format!("{:.3}", duration.as_secs_f64());
        if let Ok(val) = HeaderValue::from_str(&timestamp) {
            headers.insert(HeaderName::from_static("x-pigeon-rawclienttime"), val);
        }
    }
    
    headers
}

/// Headers for webview-style Instagram endpoints that expect a Chrome `webUserAgent`
/// (avoids HTTP 467 when app version is modern but Chrome UA is ancient).
pub fn web_request_headers(web_user_agent: &str) -> HeaderMap {
    request_headers(web_user_agent)
}
