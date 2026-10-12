use thiserror::Error;

#[derive(Debug, Error)]
pub enum LlmError {
    #[error("network request failed: {0}")]
    Network(#[from] reqwest::Error),

    #[error("request timed out after {0} seconds")]
    Timeout(u64),

    #[error("rate limited by provider, retry after {0} seconds")]
    RateLimited(u64),

    #[error("provider returned error, status {status}: {message}")]
    ApiError { status: u16, message: String },

    #[error("failed to parse response body: {0}")]
    Deserialize(#[from] serde_json::Error),

    #[error("unsupported provider: {0}")]
    UnsupportedProvider(String),
}
