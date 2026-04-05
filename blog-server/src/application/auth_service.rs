use std::sync::Arc;
use tracing::instrument;
use crate::data::user_repository::UserRepository;
use crate::domain::error::{BlogError};
use crate::domain::user::{User, UserWithToken};
use crate::infrastructure::jwt::{JwtKeys};
use crate::infrastructure::jwt as jwt;

#[derive(Clone)]
pub(crate) struct AuthService<R: UserRepository/* + 'static*/> {
    repo: Arc<R>,
    keys: JwtKeys,
}

impl<R> AuthService<R>
where
    R: UserRepository/* + 'static*/,
{
    /*pub fn new(repo: Arc<R>, keys: JwtKeys) -> Self {
        Self { repo, keys }
    }

    pub fn keys(&self) -> &JwtKeys {
        &self.keys
    }

    pub async fn get_user(&self, id: uuid::Uuid) -> Result<User, BankError> {
        self.repo
            .find_by_id(id)
            .await
            .map_err(BankError::from)?
            .ok_or_else(|| BankError::NotFound(format!("user {}", id)))
    }*/

    #[instrument(skip(self))]
    pub(crate) async fn register(&self, name: String, email: String, password: String) -> Result<UserWithToken, BlogError> {
        let hash = jwt::hash_password(&password).map_err(|err| BlogError::Internal(err.to_string()))?;
        let user = User::new(name, email.to_lowercase(), hash);
        let token = self.keys
            .generate_token(user.id, user.name.to_string())
            .map_err(|err| BlogError::Internal(err.to_string()))?;

        let user = self.repo.create(user).await.map_err(BlogError::from)?;
        
        Ok(UserWithToken::new(user, token))
    }

    #[instrument(skip(self))]
    pub(crate) async fn login(&self, email: &str, password: &str) -> Result<UserWithToken, BlogError> {
        let user = self
            .repo
            .find_by_email(&email.to_lowercase())
            .await
            .map_err(BlogError::from)?
            .ok_or_else(|| BlogError::NotFound(format!("User with email {} not found", email)))?;

        let valid = jwt::verify_password(password, &user.password_hash)
            .map_err(|err| BlogError::Internal(err.to_string()))?;

        if !valid {
            return Err(BlogError::Unauthorized);
        }

        let token = self.keys
            .generate_token(user.id.to_owned(), user.name.to_owned())
            .map_err(|err| BlogError::Internal(err.to_string()))?;

        Ok(UserWithToken::new(user, token))
    }
}