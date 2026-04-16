use std::sync::Arc;
use crate::application::auth_service::AuthService;
use crate::application::blog_service::BlogService;
use crate::data::post_repository::PostgresPostRepository;
use crate::data::user_repository::PostgresUserRepository;
use crate::domain;
use crate::presentation::auth::{jwt_validator, AuthenticatedUser};
use crate::presentation::proto::blog::blog_service_server::BlogService as BlogServiceContract;
use crate::presentation::proto::blog::*;
use futures_util::TryFutureExt;
use tonic::metadata::MetadataMap;
use tonic::{Request, Response, Status};
use uuid::Uuid;

pub struct BlogGrpcService {
    pub auth_service: Arc<AuthService<PostgresUserRepository>>,
    pub blog_service: Arc<BlogService<PostgresPostRepository>>
}

impl BlogGrpcService {
    async fn ensure_authentication(&self, metadata: &MetadataMap) -> Result<AuthenticatedUser, Status> {
        if let Some(token) = metadata.get("authorization") {
            let token_str = token.to_str().map_err(|e| Status::unauthenticated(e.to_string()))?;
            let token = token_str
                .strip_prefix("Bearer ")
                .ok_or(Status::unauthenticated("invalid authorization header"))?;

            let authenticated_user = jwt_validator(token, self.auth_service.keys(), &self.auth_service)
                .map_err(|e| Status::unauthenticated(e.to_string()))
                .await?;

            return Ok(authenticated_user)
        }

        Err(Status::unauthenticated("authorization header not found"))
    }
}

#[tonic::async_trait]
impl BlogServiceContract for BlogGrpcService {
    async fn register(&self, request: Request<RegisterRequest>) -> Result<Response<RegisterResponse>, Status> {
        let req = request.into_inner();
        let user_with_token = self.auth_service.register(req.name, req.email, req.password).await?;
        let user = user_with_token.user;
        let response = RegisterResponse {
            user: Some(user.into()),
            token: user_with_token.token,
        };

        Ok(Response::new(response))
    }

    async fn login(&self, request: Request<LoginRequest>) -> Result<Response<LoginResponse>, Status> {
        let req = request.into_inner();
        let user_with_token = self.auth_service.login(&req.name, &req.password).await?;
        let user = user_with_token.user;

        let response = LoginResponse {
            user: Some(user.into()),
            token: user_with_token.token,
        };

        Ok(Response::new(response))
    }

    async fn create_post(&self, request: Request<CreatePostRequest>) -> Result<Response<CreatePostResponse>, Status> {
        let authenticated_user = self.ensure_authentication(request.metadata()).await?;
        let req = request.into_inner();
        let post = self.blog_service.create_post(authenticated_user.id, req.title, req.content).await?;

        let response = CreatePostResponse {
            post: Some(post.into())
        };

        Ok(Response::new(response))
    }

    async fn get_post(&self, request: Request<GetPostRequest>) -> Result<Response<GetPostResponse>, Status> {
        let req = request.into_inner();
        let post_id = Uuid::parse_str(&req.id).map_err(|e| Status::invalid_argument(e.to_string()))?;
        let post = self.blog_service.get_post(post_id).await?;

        let response = GetPostResponse {
            post: Some(post.into())
        };

        Ok(Response::new(response))
    }

    async fn update_post(&self, request: Request<UpdatePostRequest>) -> Result<Response<UpdatePostResponse>, Status> {
        let authenticated_user = self.ensure_authentication(request.metadata()).await?;

        let req = request.into_inner();
        let post_id = Uuid::parse_str(&req.id).map_err(|e| Status::invalid_argument(e.to_string()))?;
        let post = self.blog_service.update_post(authenticated_user.id, post_id, req.title, req.content).await?;

        let response = UpdatePostResponse {
            post: Some(post.into())
        };

        Ok(Response::new(response))
    }

    async fn delete_post(&self, request: Request<DeletePostRequest>) -> Result<Response<DeletePostResponse>, Status> {
        let authenticated_user = self.ensure_authentication(request.metadata()).await?;

        let req = request.into_inner();
        let post_id = Uuid::parse_str(&req.id).map_err(|e| Status::invalid_argument(e.to_string()))?;

        self.blog_service.delete_post(authenticated_user.id, post_id).await?;

        Ok(Response::new(DeletePostResponse {}))
    }

    async fn list_posts(&self, request: Request<ListPostsRequest>) -> Result<Response<ListPostsResponse>, Status> {
        let req = request.into_inner();
        let data_posts = self.blog_service.list_posts(req.limit, req.offset).await?;
        let total = data_posts.len() as u32;
        let posts = data_posts.into_iter().map(|p| p.into()).collect();

        Ok(Response::new(ListPostsResponse {
            posts,
            total,
            limit: req.limit,
            offset: req.offset,
        }))
    }
}

impl From<domain::post::Post> for Post {
    fn from(value: domain::post::Post) -> Self {
        Post {
            id: value.id.to_string(),
            author_id: value.author_id.to_string(),
            title: value.title,
            content: value.content,
            created_at: value.created_at.timestamp(),
            updated_at: value.updated_at.timestamp()
        }
    }
}

impl From<domain::user::User> for User {
    fn from(value: domain::user::User) -> Self {
        User {
            id: value.id.to_string(),
            name: value.name,
            email: value.email,
            created_at: value.created_at.timestamp(),
        }
    }
}

