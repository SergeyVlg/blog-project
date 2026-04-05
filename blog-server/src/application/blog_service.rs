use std::sync::Arc;
use uuid::Uuid;
use crate::data::post_repository::PostRepository;
use crate::domain::error::{BlogError};
use crate::domain::post::Post;

#[derive(Clone)]
pub(crate) struct BlogService<R: PostRepository/* + 'static*/> {
    repo: Arc<R>,
}

impl <R> BlogService<R>
where R: PostRepository/* + 'static*/,
{
    pub(crate) fn new(repo: Arc<R>) -> Self {
        Self { repo }
    }

    pub(crate) async fn create_post(&self, author_id: Uuid, title: String, content: String) -> Result<Post, BlogError> {
        let post = Post::new(author_id, title, content);
        self.repo.create(post).await.map_err(BlogError::from)
    }

    pub(crate) async fn update_post(&self, post_id: Uuid, title: String, content: String) -> Result<Post, BlogError> {
        self.repo.update(post_id, title, content).await.map_err(BlogError::from)
    }

    pub(crate) async fn list_posts(&self, limit: u32, offset: u32) -> Result<Vec<Post>, BlogError> {
        self.repo.list(limit, offset).await.map_err(BlogError::from)
    }
}
