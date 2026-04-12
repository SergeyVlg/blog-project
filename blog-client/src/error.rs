use thiserror::Error;

#[derive(Debug, Error)]
pub(crate) enum BlogClientError {
    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),

    #[error("invalid URL: {0}")]
    InvalidUrl(String),
}

pub(crate) type Result<T> = std::result::Result<T, BlogClientError>;