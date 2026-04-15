pub mod dto;

pub use crate::dto::{GetPostsResponse, Post};

#[cfg(feature = "native-clients")]
use crate::dto::UserWithToken;
#[cfg(feature = "native-clients")]
use crate::grpc_client::GrpcClient;
#[cfg(feature = "native-clients")]
use crate::http_client::HttpClient;
#[cfg(feature = "native-clients")]
use crate::transport::BlogTransport;
#[cfg(feature = "native-clients")]
use tonic::async_trait;
#[cfg(feature = "native-clients")]
use uuid::Uuid;

#[cfg(feature = "native-clients")]
mod http_client;
#[cfg(feature = "native-clients")]
mod grpc_client;
#[cfg(feature = "native-clients")]
mod error;
#[cfg(feature = "native-clients")]
mod proto;
#[cfg(feature = "native-clients")]
mod transport;

#[cfg(feature = "native-clients")]
pub use crate::error::{BlogClientError, Result};

#[cfg(feature = "native-clients")]
#[async_trait]
pub trait BlogClientApi: Send + Sync {
    fn set_token(&mut self, token: String);

    async fn register(&mut self, username: String, email: String, password: String) -> Result<UserWithToken>;
    async fn login(&mut self, username: String, password: String) -> Result<UserWithToken>;
    async fn create_post(&self, title: String, content: String) -> Result<Post>;

    async fn get_post(&self, post_id: Uuid) -> Result<Post>;
    async fn update_post(&self, post_id: Uuid, title: String, content: String) -> Result<Post>;
    async fn delete_post(&self, post_id: Uuid) -> Result<()>;
    async fn list_posts(&self, limit: u32, offset: u32) -> Result<GetPostsResponse>;
}

#[cfg(feature = "native-clients")]
pub struct BlogClient<T: BlogTransport>
{
    transport: T,
    token: Option<String>,
}

#[cfg(feature = "native-clients")]
impl<T: BlogTransport> BlogClient<T>
{
    fn with_transport(transport: T) -> Self {
        Self { transport, token: None }
    }

    fn get_token(&self) -> Result<&str> {
        self.token.as_deref().ok_or(BlogClientError::MissingToken)
    }
}
#[cfg(feature = "native-clients")]
#[async_trait]
impl<T: BlogTransport> BlogClientApi for BlogClient<T> {
    fn set_token(&mut self, token: String) {
        self.token = Some(token);
    }

    async fn register(&mut self, username: String, email: String, password: String) -> Result<UserWithToken> {
        let user_with_token = self.transport.register(username, email, password).await?;

        Ok(user_with_token)
    }

    async fn login(&mut self, username: String, password: String) -> Result<UserWithToken> {
        let user_with_token = self.transport.login(username, password).await?;

        Ok(user_with_token)
    }

    async fn create_post(&self, title: String, content: String) -> Result<Post> {
        let token = self.get_token()?.to_owned();

        self.transport.create_post(token, title, content).await
    }

    async fn get_post(&self, post_id: Uuid) -> Result<Post> {
        self.transport.get_post(post_id).await
    }

    async fn update_post(&self, post_id: Uuid, title: String, content: String) -> Result<Post> {
        let token = self.get_token()?.to_owned();

        self.transport.update_post(token, post_id, title, content).await
    }

    async fn delete_post(&self, post_id: Uuid) -> Result<()> {
        let token = self.get_token()?.to_owned();


        self.transport.delete_post(token, post_id).await
    }

    async fn list_posts(&self, limit: u32, offset: u32) -> Result<GetPostsResponse> {
        self.transport.list_posts(limit, offset).await
    }
}

#[cfg(feature = "native-clients")]
impl BlogClient<HttpClient> {
    pub fn new_http(url: String) -> Result<Self> {
        let transport = HttpClient::new(url)?;

        Ok(Self::with_transport(transport))
    }
}

#[cfg(feature = "native-clients")]
impl BlogClient<GrpcClient> {
    pub async fn new_grpc(url: String) -> Result<Self> {
        let transport = GrpcClient::new(url).await?;

        Ok(Self::with_transport(transport))
    }
}