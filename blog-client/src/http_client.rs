use std::time::Duration;
use tonic::async_trait;
use uuid::Uuid;
use crate::dto::{CreatePostRequest, GetPostsRequest, GetPostsResponse, LoginRequest, Post, RegisterRequest, UpdatePostRequest, UserWithToken};
use crate::error::{Result};
use crate::transport::BlogTransport;

#[derive(Debug, Clone)]
pub struct HttpClient {
    client: reqwest::Client,
    url: String,
}

impl HttpClient {
    pub(super) fn new(url: String) -> Result<Self> {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(10))
            .build()?;

        Ok(Self {client, url})
    }
}

#[async_trait]
impl BlogTransport for HttpClient {
    async fn register(&self, name: String, email: String, password: String) -> Result<UserWithToken> {
        let url = format!("{}/api/register", self.url);
        let req = RegisterRequest { name, email, password };
        let user = self.client
            .post(url)
            .json(&req)
            .send()
            .await?
            .error_for_status()?
            .json::<UserWithToken>()
            .await?;

        Ok(user)
    }

    async fn login(&self, name: String, password: String) -> Result<UserWithToken> {
        let url = format!("{}/api/login", self.url);
        let req = LoginRequest { name, password };
        let user = self.client
            .get(url)
            .json(&req)
            .send()
            .await?
            .error_for_status()?
            .json::<UserWithToken>()
            .await?;

        Ok(user)
    }

    async fn create_post(&self, token: String, title: String, content: String) -> Result<Post> {
        let url = format!("{}/posts", self.url);
        let req = CreatePostRequest { title, content };
        let post = self.client
            .post(url)
            .bearer_auth(token)
            .json(&req)
            .send()
            .await?
            .error_for_status()?
            .json::<Post>()
            .await?;

        Ok(post)
    }

    async fn get_post(&self, post_id: Uuid) -> Result<Post> {
        let url = format!("{}/posts/{}", self.url, post_id);
        let post = self.client
            .get(url)
            .send()
            .await?
            .error_for_status()?
            .json::<Post>()
            .await?;

        Ok(post)
    }

    async fn update_post(&self, token: String, post_id: Uuid, title: String, content: String) -> Result<Post> {
        let url = format!("{}/posts/{}", self.url, post_id);
        let req = UpdatePostRequest { title, content };
        let post = self.client
            .put(url)
            .bearer_auth(token)
            .json(&req)
            .send()
            .await?
            .error_for_status()?
            .json::<Post>()
            .await?;

        Ok(post)
    }

    async fn delete_post(&self, token: String, post_id: Uuid) -> Result<()> {
        let url = format!("{}/posts/{}", self.url, post_id);
        self.client
            .delete(url)
            .bearer_auth(token)
            .send()
            .await?
            .error_for_status()?;

        Ok(())
    }

    async fn list_posts(&self, limit: u32, offset: u32) -> Result<GetPostsResponse> {
        let url = format!("{}/posts/", self.url);
        let req = GetPostsRequest { limit, offset };
        let response = self.client
            .get(url)
            .query(&req)
            .send()
            .await?
            .error_for_status()?
            .json::<GetPostsResponse>()
            .await?;

        Ok(response)
    }
}
