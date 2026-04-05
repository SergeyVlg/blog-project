use std::sync::Arc;
use tracing::instrument;
use crate::data::user_repository::UserRepository;
use crate::domain::error::AuthError;
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
    pub(crate) async fn register(&self, name: String, email: String, password: String) -> Result<UserWithToken, AuthError> {
        let hash = jwt::hash_password(&password).map_err(|err| AuthError::Internal(err.to_string()))?;
        let user = User::new(name, email.to_lowercase(), hash);
        let token = self.keys
            .generate_token(user.id, user.name.to_string())
            .map_err(|err| AuthError::Internal(err.to_string()))?;

        self.repo.create(user).await.map_err(AuthError::from)?;
        
        Ok(UserWithToken::new(user, token))
    }

    #[instrument(skip(self))]
    pub(crate) async fn login(&self, email: &str, password: &str) -> Result<String, AuthError> {
        let user = self
            .repo
            .find_by_email(&email.to_lowercase())
            .await
            .map_err(AuthError::from)?
            .ok_or_else(|| AuthError::NotFound)?;

        let valid = jwt::verify_password(password, &user.password_hash)
            .map_err(|_| AuthError::Unauthorized)?;

        if !valid {
            return Err(AuthError::Unauthorized);
        }

        self.keys
            .generate_token(user.id, user.name)
            .map_err(|err| AuthError::Internal(err.to_string()))
    }
}