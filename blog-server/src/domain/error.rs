use thiserror::Error;

#[derive(Debug, Error)]
pub(crate) enum AuthError {
    #[error("validation error: {0}")]
    Validation(String),
    #[error("user not found")]
    NotFound,
    #[error("unauthorized")]
    Unauthorized,
    #[error("internal server error: {0}")]
    Internal(String),
}

#[derive(Debug, Error)]
pub(crate) enum DomainError {
    #[error("validation error: {0}")]
    Validation(String),
    #[error("internal server error: {0}")]
    Internal(String),
}

#[derive(Debug, Error)]
pub(crate) enum BlogError {
    #[error("validation error: {0}")]
    Validation(String),
    #[error("internal server error: {0}")]
    Internal(String),
}