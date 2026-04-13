use uuid::Uuid;
use crate::dto::{GetPostsResponse, Post, UserWithToken};
use crate::error::Result;

pub(crate) trait BlogTransport: Send + Sync {
    async fn register(&self, name: String, email: String, password: String) -> Result<UserWithToken>;
    async fn login(&self, name: String, password: String) -> Result<UserWithToken>;
    
    async fn create_post(&self, token: String, title: String, content: String) -> Result<Post>;
    async fn get_post(&self, post_id: Uuid) -> Result<Post>;
    async fn update_post(&self, token: String, post_id: Uuid, title: String, content: String) -> Result<Post>;
    
    async fn delete_post(&self, token: String, post_id: Uuid) -> Result<()>;
    async fn list_posts(&self, limit: u32, offset: u32) -> Result<GetPostsResponse>;
}

