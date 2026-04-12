use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct UserWithToken {
    pub user: User,
    pub token: String,
}

#[derive(Debug, Serialize)]
pub struct RegisterRequest {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginRequest {
    pub name: String,
    pub password: String,
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct Post {
    pub id: Uuid,
    pub author_id: Uuid,
    pub title: String,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct CreatePostRequest {
    pub title: String,
    pub content: String,
}

#[derive(Debug, Serialize)]
pub struct UpdatePostRequest {
    pub title: String,
    pub content: String,
}

#[derive(Debug, Serialize)]
pub struct GetPostsRequest {
    #[serde(default = "GetPostsRequest::set_default_limit")]
    pub limit: u32,
    pub offset: u32,
}

impl GetPostsRequest {
    const DEFAULT_LIMIT: u32 = 10;
    fn set_default_limit() -> u32 { Self::DEFAULT_LIMIT }
}

#[derive(Debug, Deserialize)]
pub struct GetPostsResponse {
    pub posts: Vec<Post>,
    pub total: u32,
    pub limit: u32,
    pub offset: u32,
}