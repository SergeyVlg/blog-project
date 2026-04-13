use uuid::Uuid;
use crate::dto::{GetPostsResponse, Post, User, UserWithToken};
use crate::error::{BlogClientError, Result};
use crate::grpc_client::GrpcClient;
use crate::http_client::HttpClient;
use crate::transport::BlogTransport;

mod http_client;
mod grpc_client;
mod error;
mod dto;
mod proto;
mod transport;

pub(crate) struct BlogClient<T>
where T: BlogTransport,
{
    transport: T,
    token: Option<String>,
}

impl<T> BlogClient<T>
where
    T: BlogTransport,
{
    fn with_transport(transport: T) -> Self {
        Self { transport, token: None }
    }

    pub(crate) fn set_token(&mut self, token: String) {
        self.token = Some(token);
    }

    pub(crate) fn clear_token(&mut self) {
        self.token = None;
    }

    fn require_token(&self) -> Result<&str> {
        self.token.as_deref().ok_or(BlogClientError::MissingToken)
    }

    pub(crate) async fn register(&mut self, username: String, email: String, password: String) -> Result<User> {
        let user_with_token = self.transport.register(username, email, password).await?;
        let UserWithToken { user, token } = user_with_token;
        self.token = Some(token);

        Ok(user)
    }

    pub(crate) async fn login(&mut self, username: String, password: String) -> Result<User> {
        let user_with_token = self.transport.login(username, password).await?;
        let UserWithToken { user, token } = user_with_token;
        self.token = Some(token);

        Ok(user)
    }

    pub(crate) async fn create_post(&self, title: String, content: String) -> Result<Post> {
        let token = self.require_token()?.to_owned();

        self.transport.create_post(token, title, content).await
    }

    pub(crate) async fn get_post(&self, post_id: Uuid) -> Result<Post> {
        self.transport.get_post(post_id).await
    }

    pub(crate) async fn update_post(&self, post_id: Uuid, title: String, content: String) -> Result<Post> {
        let token = self.require_token()?.to_owned();

        self.transport.update_post(token, post_id, title, content).await
    }

    pub(crate) async fn delete_post(&self, post_id: Uuid) -> Result<()> {
        let token = self.require_token()?.to_owned();

        self.transport.delete_post(token, post_id).await
    }

    pub(crate) async fn list_posts(&self, limit: u32, offset: u32) -> Result<GetPostsResponse> {
        self.transport.list_posts(limit, offset).await
    }
}

impl BlogClient<HttpClient> {
    pub(crate) fn new(url: String) -> Result<Self> {
        let transport = HttpClient::new(url)?;

        Ok(Self::with_transport(transport))
    }
}

impl BlogClient<GrpcClient> {
    pub(crate) async fn new(url: String) -> Result<Self> {
        let transport = GrpcClient::new(url).await?;

        Ok(Self::with_transport(transport))
    }
}