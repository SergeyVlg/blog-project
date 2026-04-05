use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use serde::Serialize;
use serde_json::json;
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
    #[error("unauthorized")]
    Unauthorized,
}

#[derive(Serialize)]
struct ErrorBody {
    error: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    details: Option<serde_json::Value>,
}

impl ResponseError for BlogError {
    fn status_code(&self) -> StatusCode {
        match self {
            BlogError::Validation(_) => StatusCode::BAD_REQUEST,
            BlogError::NotFound(_) => StatusCode::NOT_FOUND,
            BlogError::Unauthorized => StatusCode::UNAUTHORIZED,
            BlogError::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let message = self.to_string();
        let details = match self {
            BlogError::Validation(msg) => Some(json!({ "message": msg })),
            BlogError::NotFound(resource) => Some(json!({ "resource": resource })),
            BlogError::Unauthorized => None,
            BlogError::Internal(_) => None,
        };
        let body = ErrorBody {
            error: message,
            details,
        };
        HttpResponse::build(self.status_code()).json(body)
    }
}