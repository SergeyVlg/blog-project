use tonic::{Request, Response, Status};
use crate::presentation::proto::blog_service_server::BlogService as GrpcBlogService;
use crate::presentation::proto::{CreatePostRequest, CreatePostResponse, DeletePostRequest, DeletePostResponse, GetPostRequest, GetPostResponse, ListPostsRequest, ListPostsResponse, LoginRequest, LoginResponse, RegisterRequest, RegisterResponse, UpdatePostRequest, UpdatePostResponse};

struct BlogGrpcService;

#[tonic::async_trait]
impl GrpcBlogService for BlogGrpcService {
    async fn register(&self, request: Request<RegisterRequest>) -> Result<Response<RegisterResponse>, Status> {
        todo!()
    }

    async fn login(&self, request: Request<LoginRequest>) -> Result<Response<LoginResponse>, Status> {
        todo!()
    }

    async fn create_post(&self, request: Request<CreatePostRequest>) -> Result<Response<CreatePostResponse>, Status> {
        todo!()
    }

    async fn get_post(&self, request: Request<GetPostRequest>) -> Result<Response<GetPostResponse>, Status> {
        todo!()
    }

    async fn update_post(&self, request: Request<UpdatePostRequest>) -> Result<Response<UpdatePostResponse>, Status> {
        todo!()
    }

    async fn delete_post(&self, request: Request<DeletePostRequest>) -> Result<Response<DeletePostResponse>, Status> {
        todo!()
    }

    async fn list_posts(&self, request: Request<ListPostsRequest>) -> Result<Response<ListPostsResponse>, Status> {
        todo!()
    }
}