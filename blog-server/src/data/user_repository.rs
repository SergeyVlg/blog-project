use async_trait::async_trait;
use sqlx::{PgPool, Row};
use tracing::{error, info};
use uuid::Uuid;
use crate::domain::error::DomainError;
use crate::domain::user::User;

#[async_trait]
pub(crate) trait UserRepository: Send + Sync {
    async fn create(&self, user: User) -> Result<User, DomainError>;
    async fn find_by_name(&self, name: &str) -> Result<Option<User>, DomainError>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, DomainError>;
}

#[derive(Clone)]
pub(crate) struct PostgresUserRepository {
    pool: PgPool,
}

impl PostgresUserRepository {
    pub(crate) fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for PostgresUserRepository {
    async fn create(&self, user: User) -> Result<User, DomainError> {
        sqlx::query(
            r#"
            INSERT INTO users (id, username, email, password_hash)
            VALUES ($1, $2, $3, $4)
            "#,
        )
            .bind(user.id)
            .bind(&user.name)
            .bind(&user.email)
            .bind(&user.password_hash)
            .execute(&self.pool)
            .await
            .map_err(|e| {
                error!("failed to create user: {}", e);

                return match e.as_database_error().and_then(|db| db.constraint()) {
                    Some(err) if err.contains("users_email") => DomainError::Validation("email already registered".into()),
                    Some(err) if err.contains("users_username") => DomainError::Validation("name already registered".into()),
                    Some(err) => DomainError::Internal(format!("database error: {}", err)),
                    None => DomainError::Internal(format!("database error: {}", e))
                }
            })?;

        info!(user_id = %user.id, email = %user.email, "user created");
        Ok(user)
    }

    async fn find_by_name(&self, name: &str) -> Result<Option<User>, DomainError> {
        let row = sqlx::query(
            r#"
            SELECT id, username, email, password_hash, created_at
            FROM users
            WHERE username = $1
            "#,
        )
            .bind(name)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| {
                error!("failed to find user by name {}: {}", name, e);
                DomainError::Internal(format!("database error: {}", e))
            })?;

        Ok(row.map(|row| User {
            id: row.get("id"),
            name: row.get("username"),
            email: row.get("email"),
            password_hash: row.get("password_hash"),
            created_at: row.get("created_at"),
        }))
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, DomainError> {
        let row = sqlx::query(
            r#"
            SELECT id, username, email, password_hash, created_at
            FROM users
            WHERE id = $1
            "#,
        )
            .bind(id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| {
                error!("failed to find user by id {}: {}", id, e);
                DomainError::Internal(format!("database error: {}", e))
            })?;

        Ok(row.map(|row| User {
            id: row.get("id"),
            email: row.get("email"),
            name: row.get("username"),
            password_hash: row.get("password_hash"),
            created_at: row.get("created_at"),
        }))
    }
}