use thiserror::Error;

#[derive(Debug, Error)]
pub enum PplxError {
    #[error("API Error: {0}")]
    ApiError(String),

    #[error("Request Error: {0}")]
    RequestError(#[from] reqwest::Error),

    #[error("Environment Error: {0}")]
    EnvError(#[from] std::env::VarError),

    #[error("Model Not Available: {0}")]
    ModelNotAvailable(String),

    #[error("Invalid Configuration: {0}")]
    InvalidConfig(String),

    #[error("Rate Limit Exceeded")]
    RateLimitExceeded,

    #[error("Authentication Error: {0}")]
    AuthenticationError(String),
}

impl From<serde_json::Error> for PplxError {
    fn from(err: serde_json::Error) -> Self {
        PplxError::ApiError(err.to_string())
    }
}