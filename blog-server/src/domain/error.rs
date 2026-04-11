use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use serde::Serialize;
use serde_json::json;
use thiserror::Error;
use tonic::Status;

#[derive(Debug, Error)]
pub(crate) enum DomainError {
    #[error("validation error: {0}")]
    Validation(String),
    #[error("internal server error: {0}")]
    Internal(String),
    #[error("user not found: {0}")]
    UserNotFound(String),
    #[error("user already exists: {0}")]
    UserAlreadyExists(String),
    #[error("post not found: {0}")]
    PostNotFound(String),
    #[error("forbidden")]
    Forbidden
}

#[derive(Debug, Error)]
pub(crate) enum BlogError {
    #[error("validation error: {0}")]
    Validation(String),
    #[error("internal server error: {0}")]
    Internal(String),
    #[error("unauthorized")]
    Unauthorized,
    #[error("not found: {0}")]
    NotFound(String),
    #[error("user already exists")]
    UserAlreadyExists,
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
            BlogError::UserAlreadyExists => StatusCode::CONFLICT,
            BlogError::Unauthorized => StatusCode::UNAUTHORIZED,
            BlogError::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let message = self.to_string();
        let details = match self {
            BlogError::Validation(msg) => Some(json!({ "message": msg })),
            BlogError::NotFound(resource) => Some(json!({ "resource": resource })),
            BlogError::UserAlreadyExists => None,
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

impl From<DomainError> for BlogError {
    fn from(value: DomainError) -> Self {
        match value {
            DomainError::Validation(msg) => BlogError::Validation(msg),
            DomainError::UserNotFound(id) => BlogError::NotFound(format!("user {}", id)),
            DomainError::PostNotFound(id) => BlogError::NotFound(format!("post {}", id)),
            DomainError::Forbidden => BlogError::Validation("forbidden to edit post".to_owned()),
            DomainError::UserAlreadyExists(_id) => BlogError::UserAlreadyExists,
            DomainError::Internal(msg) => BlogError::Internal(msg),
        }
    }
}

impl From<BlogError> for Status {
    fn from(value: BlogError) -> Self {
        match value {
            BlogError::Validation(err) => Status::invalid_argument(err),
            BlogError::Internal(err) => Status::internal(err),
            BlogError::Unauthorized => Status::unauthenticated("unauthorized"),
            BlogError::NotFound(err) => Status::not_found(err),
            BlogError::UserAlreadyExists => Status::already_exists("user already exists"),
        }
    }
}