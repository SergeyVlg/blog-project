use chrono::{DateTime, Utc};
use uuid::Uuid;
use crate::dto::{GetPostsResponse, Post, User, UserWithToken};
use crate::error::{BlogClientError, Result};
use crate::transport::BlogTransport;
use crate::proto::blog::blog_service_client::BlogServiceClient;
use crate::proto::blog::{
	CreatePostRequest,
	DeletePostRequest,
	GetPostRequest,
	ListPostsRequest,
	LoginRequest,
	Post as ProtoPost,
	RegisterRequest,
	UpdatePostRequest,
	User as ProtoUser,
};
use tonic::metadata::MetadataValue;
use tonic::Request;
use tonic::transport::Channel;

#[derive(Debug, Clone)]
pub(super) struct GrpcClient {
	client: BlogServiceClient<Channel>,
}

impl GrpcClient {
	pub(super) async fn new(url: String) -> Result<Self> {
		let client = BlogServiceClient::connect(url).await?;

		Ok(Self { client })
	}

	fn add_bearer_token<T>(request: &mut Request<T>, token: &str) -> Result<()> {
		let metadata = MetadataValue::try_from(format!("Bearer {token}"))
			.map_err(|err| BlogClientError::InvalidResponse(format!("invalid authorization metadata: {err}")))?;
		request.metadata_mut().insert("authorization", metadata);

		Ok(())
	}

	fn map_user_with_token(user: Option<ProtoUser>, token: String, operation: &str) -> Result<UserWithToken> {
		let user = user.ok_or_else(|| {
			BlogClientError::InvalidResponse(format!("{operation} response does not contain user"))
		})?;

		Ok(UserWithToken {
			user: Self::map_user(user, operation)?,
			token,
		})
	}

	fn map_user(user: ProtoUser, operation: &str) -> Result<User> {
		let id = Uuid::parse_str(&user.id)
			.map_err(|err| BlogClientError::InvalidResponse(format!("invalid user id in {operation} response: {err}")))?;
		let created_at = DateTime::<Utc>::from_timestamp(user.created_at, 0).ok_or_else(|| {
			BlogClientError::InvalidResponse(format!(
				"invalid created_at timestamp in {operation} response: {}",
				user.created_at,
			))
		})?;

		Ok(User {
			id,
			name: user.name,
			email: user.email,
			created_at,
		})
	}

	fn map_post(post: ProtoPost, operation: &str) -> Result<Post> {
		let id = Uuid::parse_str(&post.id)
			.map_err(|err| BlogClientError::InvalidResponse(format!("invalid post id in {operation} response: {err}")))?;

		let author_id = Uuid::parse_str(&post.author_id).map_err(|err| {
			BlogClientError::InvalidResponse(format!("invalid author id in {operation} response: {err}"))
		})?;

		let created_at = DateTime::<Utc>::from_timestamp(post.created_at, 0).ok_or_else(|| {
			BlogClientError::InvalidResponse(format!(
				"invalid created_at timestamp in {operation} response: {}",
				post.created_at,
			))
		})?;

		let updated_at = DateTime::<Utc>::from_timestamp(post.updated_at, 0).ok_or_else(|| {
			BlogClientError::InvalidResponse(format!(
				"invalid updated_at timestamp in {operation} response: {}",
				post.updated_at,
			))
		})?;

		Ok(Post {
			id,
			author_id,
			title: post.title,
			content: post.content,
			created_at,
			updated_at,
		})
	}
}

impl BlogTransport for GrpcClient {
	async fn register(&self, name: String, email: String, password: String) -> Result<UserWithToken> {
		let request = RegisterRequest { name, email, password };
		let response = self.client.clone().register(request).await?.into_inner();

		Self::map_user_with_token(response.user, response.token, "register")
	}

	async fn login(&self, name: String, password: String) -> Result<UserWithToken> {
		let request = LoginRequest { name, password };
		let response = self.client.clone().login(request).await?.into_inner();

		Self::map_user_with_token(response.user, response.token, "login")
	}

	async fn create_post(&self, token: String, title: String, content: String) -> Result<Post> {
		let mut request = Request::new(CreatePostRequest { title, content });
		Self::add_bearer_token(&mut request, &token)?;

		let response = self.client.clone().create_post(request).await?.into_inner();
		let post = response
			.post
			.ok_or_else(|| BlogClientError::InvalidResponse("create_post response does not contain post".into()))?;

		Self::map_post(post, "create_post")
	}

	async fn get_post(&self, post_id: Uuid) -> Result<Post> {
		let request = GetPostRequest { id: post_id.to_string() };
		let response = self.client.clone().get_post(request).await?.into_inner();
		let post = response
			.post
			.ok_or_else(|| BlogClientError::InvalidResponse("get_post response does not contain post".into()))?;

		Self::map_post(post, "get_post")
	}

	async fn update_post(&self, token: String, post_id: Uuid, title: String, content: String) -> Result<Post> {
		let mut request = Request::new(UpdatePostRequest {
			id: post_id.to_string(),
			title,
			content,
		});
		Self::add_bearer_token(&mut request, &token)?;

		let response = self.client.clone().update_post(request).await?.into_inner();
		let post = response
			.post
			.ok_or_else(|| BlogClientError::InvalidResponse("update_post response does not contain post".into()))?;

		Self::map_post(post, "update_post")
	}

	async fn delete_post(&self, token: String, post_id: Uuid) -> Result<()> {
		let mut request = Request::new(DeletePostRequest {
			id: post_id.to_string(),
		});
		Self::add_bearer_token(&mut request, &token)?;

		self.client.clone().delete_post(request).await?;

		Ok(())
	}

	async fn list_posts(&self, limit: u32, offset: u32) -> Result<GetPostsResponse> {
		let request = Request::new(ListPostsRequest { limit, offset });
		let response = self.client.clone().list_posts(request).await?.into_inner();
		let posts = response
			.posts
			.into_iter()
			.map(|post| Self::map_post(post, "list_posts"))
			.collect::<Result<Vec<_>>>()?;

		Ok(GetPostsResponse {
			posts,
			total: response.total,
			limit: response.limit,
			offset: response.offset,
		})
	}
}

