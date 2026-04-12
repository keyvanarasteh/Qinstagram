use hmac::{Hmac, Mac};
use sha2::Sha256;
use url::form_urlencoded;

use crate::constants::SIGNATURE_KEY;
use crate::error::{InstagramError, Result};

type HmacSha256 = Hmac<Sha256>;

pub fn generate_signature(payload: &str) -> String {
    let mut mac = HmacSha256::new_from_slice(SIGNATURE_KEY.as_bytes())
        .expect("HMAC can take key of any size");
    mac.update(payload.as_bytes());
    let result = mac.finalize();
    let bytes = result.into_bytes();
    hex::encode(bytes)
}

pub fn sign_request(payload: &serde_json::Value) -> Result<String> {
    let payload_str = serde_json::to_string(payload).map_err(InstagramError::SerdeError)?;
    let signature = generate_signature(&payload_str);
    
    let signed_body = format!(
        "ig_sig_key_version={}&signed_body={}.{}",
        crate::constants::IG_SIG_KEY_VERSION,
        signature,
        form_urlencoded::byte_serialize(payload_str.as_bytes()).collect::<String>()
    );
    
    Ok(signed_body)
}
