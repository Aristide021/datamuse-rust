use thiserror::Error;

#[derive(Error, Debug)]
pub enum DatamuseError {
    #[error("Network error: {0}")]
    NetworkError(#[from] reqwest::Error),

    #[error("Invalid parameter: {0}")]
    InvalidParameter(String),

    #[error("JSON parsing error: {0}")]
    ParsingError(String),

    #[error("Metadata parsing error: {0}")]
    MetadataError(String),

    #[error("Rate limit exceeded")]
    RateLimitError,

    #[error("API error: {0}")]
    ApiError(String),
}
