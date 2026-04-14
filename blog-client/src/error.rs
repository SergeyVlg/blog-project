use thiserror::Error;

#[derive(Debug, Error)]
pub enum BlogClientError {
    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),

    #[error("invalid URL: {0}")]
    InvalidUrl(String),

    #[error("gRPC transport failed: {0}")]
    GrpcTransport(#[from] tonic::transport::Error),

    #[error("gRPC request failed: {0}")]
    GrpcStatus(#[from] tonic::Status),

    #[error("invalid gRPC response: {0}")]
    InvalidResponse(String),

    #[error("authentication token is missing")]
    MissingToken,
}

pub type Result<T> = std::result::Result<T, BlogClientError>;