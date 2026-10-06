use thiserror::Error;

#[derive(Error, Debug)]
pub enum KkhayError {
    #[error("API error [HTTP {status}]: {message}")]
    Api {
        status: u16,
        message: String,
        code: Option<String>,
    },

    #[error("HTTP client error: {0}")]
    Reqwest(#[from] reqwest::Error),

    #[error("JSON serialization error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Invalid webhook signature: Request rejected")]
    InvalidSignature,

    #[error("Configuration error: {0}")]
    Config(String),
}

