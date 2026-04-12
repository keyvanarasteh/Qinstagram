use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum InstagramError {
    #[error("API request failed: {0}")]
    ApiError(String),

    #[error("Network error: {0}")]
    NetworkError(#[from] reqwest::Error),

    #[error("Serialization/Deserialization error: {0}")]
    SerdeError(#[from] serde_json::Error),

    #[error("I/O error: {0}")]
    IoError(#[from] io::Error),

    #[error("Authentication required")]
    AuthRequired,
    
    #[error("Not implemented: {0}")]
    NotImplemented(String),

    #[error("Unknown error: {0}")]
    Unknown(String),
}

pub type Result<T> = std::result::Result<T, InstagramError>;
