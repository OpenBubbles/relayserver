use std::sync::Arc;

use thiserror::Error;



#[derive(Error, Debug)]
pub enum RelayError {
    #[error("Plist parsing error: {0}")]
    PlistError(#[from] plist::Error),
    #[error("HTTP error: {0}")]
    RequestError(#[from] reqwest::Error),
    #[error("NAC error: {0}")]
    NacError(u64),
    #[error("Resource Timeout")]
    ResourceTimeout,
    #[error("Resource Failure")]
    ResourceFailure(#[from] Arc<RelayError>),
    #[error("Resource Panic {0}")]
    ResourcePanic(String),
    #[error("Do not retry {0}")]
    DoNotRetry(Box<RelayError>),
    #[error("WS error: {0}")]
    WSError(#[from] tokio_tungstenite::tungstenite::Error),
    #[error("JSON error: {0}")]
    JSONError(#[from] serde_json::Error),
}

