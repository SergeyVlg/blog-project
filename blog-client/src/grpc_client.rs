use chrono::{DateTime, Utc};
use uuid::Uuid;
use crate::dto::{User, UserWithToken};
use crate::error::{BlogClientError, Result};
use crate::proto::blog::blog_service_client::BlogServiceClient;
use crate::proto::blog::RegisterRequest;
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

	pub(crate) async fn register(&self, name: String, email: String, password: String) -> Result<UserWithToken> {
		let request = RegisterRequest { name, email, password };
		let response = self.client.clone().register(request).await?.into_inner();
		let user = response
			.user
			.ok_or_else(|| BlogClientError::InvalidResponse("register response does not contain user".into()))?;

		let id = Uuid::parse_str(&user.id)
			.map_err(|err| BlogClientError::InvalidResponse(format!("invalid user id in register response: {err}")))?;
		let created_at = DateTime::<Utc>::from_timestamp(user.created_at, 0)
			.ok_or_else(|| BlogClientError::InvalidResponse(format!("invalid created_at timestamp in register response: {}", user.created_at)))?;

		Ok(UserWithToken {
			user: User {
				id,
				name: user.name,
				email: user.email,
				created_at,
			},
			token: response.token,
		})
	}
}
