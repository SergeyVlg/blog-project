use async_trait::async_trait;
use uuid::Uuid;
use crate::domain::error::DomainError;
use crate::domain::post::Post;

#[async_trait]
pub(crate) trait PostRepository: Send + Sync {
    async fn create(&self, post: Post) -> Result<Post, DomainError>;
    async fn update(&self, post_id: Uuid, title: String, content: String) -> Result<Post, DomainError>;
}