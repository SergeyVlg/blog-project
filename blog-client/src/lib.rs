use crate::error::Result;
use crate::grpc_client::GrpcClient;
use crate::http_client::HttpClient;
use serde::{Deserialize, Serialize};

mod http_client;
mod grpc_client;
mod error;
mod dto;
mod proto;

enum Transport {
    Http(String),
    Grpc(String),
}

struct BlogClient {
    transport: Transport,
    http_client: Option<HttpClient>,
    grpc_client: Option<GrpcClient>,
    token: Option<String>,
}

impl BlogClient {
    fn new(transport: Transport) -> Self {
        Self { transport, http_client: None, token: None, grpc_client: None }
    }

    fn set_token(&mut self, token: String) {
        self.token = Some(token);
    }

    fn get_token(&self) -> Option<String> {
        self.token.clone()
    }

    async fn register(&mut self, username: String, email: String, password: String) -> Result<()> {
        if let Some(client) = &self.http_client {
            client.register(username, email, password).await?;
        }

        Ok(())
    }

    fn login(&self, username: String, password: String) {
        todo!()
    }

    fn create_post(&self, title: String, content: String) {
        todo!()
    }

    fn get_post(&self, post_id: String) {
        todo!()
    }

    fn update_post(&self, post_id: String, title: String, content: String) {
        todo!()
    }

    fn delete_post(&self, post_id: String) {
        todo!()
    }

    fn list_posts(&self, limit: u32, offset: u32) {
        todo!()
    }
}