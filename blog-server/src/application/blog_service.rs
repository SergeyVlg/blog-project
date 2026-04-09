use crate::data::post_repository::PostRepository;
use crate::domain::error::BlogError;
use crate::domain::post::Post;
use uuid::Uuid;

#[derive(Clone)]
pub(crate) struct BlogService<R: PostRepository/* + 'static*/> {
    repo: R,
}

impl <R> BlogService<R>
where R: PostRepository/* + 'static*/,
{
    pub(crate) fn new(repo: R) -> Self {
        Self { repo }
    }

    pub(crate) async fn create_post(&self, author_id: Uuid, title: String, content: String) -> Result<Post, BlogError> {
        if title.is_empty() || content.is_empty() {
            return Err(BlogError::Validation("title or content cannot be empty".into()));
        }

        let post = Post::new(author_id, title, content);

        self.repo.create(post).await.map_err(BlogError::from)
    }

    pub(crate) async fn update_post(&self, author_id: Uuid, post_id: Uuid, title: String, content: String) -> Result<Post, BlogError> {
        if title.is_empty() || content.is_empty() {
            return Err(BlogError::Validation("title or content cannot be empty".into()));
        }
        
        self.repo.update(author_id, post_id, title, content).await.map_err(BlogError::from)
    }

    pub(crate) async fn delete_post(&self, author_id: Uuid, post_id: Uuid) -> Result<(), BlogError> {
        self.repo.delete(author_id, post_id).await.map_err(BlogError::from)
    }

    pub(crate) async fn list_posts(&self, limit: u32, offset: u32) -> Result<Vec<Post>, BlogError> {
        self.repo.list(limit, offset).await.map_err(BlogError::from)
    }

    pub(crate) async fn get_post(&self, post_id: Uuid) -> Result<Post, BlogError> {
        self.repo.get(post_id).await.map_err(BlogError::from)
    }
}
