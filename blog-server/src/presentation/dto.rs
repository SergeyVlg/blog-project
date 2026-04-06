use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::domain::user::User;

#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct RegisterResponse {
    pub token: String,
    pub user: User,
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct CreatePostRequest {
    pub title: String,
    pub content: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdatePostRequest {
    pub title: String,
    pub content: String,
}

#[derive(Debug, Deserialize)]
pub struct GetPostsRequest {
    #[serde(default = "GetPostsRequest::set_default_limit")]
    pub limit: u32,
    pub offset: u32,
}

impl GetPostsRequest {
    const DEFAULT_LIMIT: u32 = 10;
    fn set_default_limit() -> u32 { Self::DEFAULT_LIMIT }
}

#[derive(Debug, Serialize)]
pub struct HealthResponse {
    pub status: &'static str,
    pub timestamp: DateTime<Utc>,
}