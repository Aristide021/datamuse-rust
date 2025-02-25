use thiserror::Error;

#[derive(Error, Debug)]
pub enum DatamuseError {
    #[error("Request failed: {0}")]
    RequestError(#[from] reqwest::Error),
}
