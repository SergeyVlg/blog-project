use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;
use crate::data::user_repository::{PostgresUserRepository, UserRepository};
use crate::domain::error::DomainError;
use crate::domain::post::Post;

#[async_trait]
pub(crate) trait PostRepository: Send + Sync {
    async fn create(&self, post: Post) -> Result<Post, DomainError>;
    async fn update(&self, post_id: Uuid, title: String, content: String) -> Result<Post, DomainError>;
    async fn list(&self, limit: u32, offset: u32) -> Result<Vec<Post>, DomainError>;
}

#[derive(Clone)]
pub(crate) struct PostgresPostRepository {
    pool: PgPool,
}

impl PostgresPostRepository {
    pub(crate) fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl PostRepository for PostgresPostRepository {
    async fn create(&self, post: Post) -> Result<Post, DomainError> {
        todo!()
    }

    async fn update(&self, post_id: Uuid, title: String, content: String) -> Result<Post, DomainError> {
        todo!()
    }

    async fn list(&self, limit: u32, offset: u32) -> Result<Vec<Post>, DomainError> {
        todo!()
    }
}
