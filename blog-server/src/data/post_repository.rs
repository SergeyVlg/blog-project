use async_trait::async_trait;
use sqlx::{PgPool, Row};
use sqlx::postgres::PgRow;
use tracing::{error, info};
use uuid::Uuid;
use crate::domain::error::DomainError;
use crate::domain::post::Post;

#[async_trait]
pub(crate) trait PostRepository: Send + Sync {
    async fn create(&self, post: Post) -> Result<Post, DomainError>;
    async fn update(&self, author_id: Uuid, post_id: Uuid, title: String, content: String) -> Result<Post, DomainError>;
    async fn delete(&self, author_id: Uuid, post_id: Uuid) -> Result<(), DomainError>;
    async fn list(&self, limit: u32, offset: u32) -> Result<Vec<Post>, DomainError>;
    async fn get(&self, post_id: Uuid) -> Result<Post, DomainError>;
}

#[derive(Clone)]
pub(crate) struct PostgresPostRepository {
    pool: PgPool,
}

impl PostgresPostRepository {
    pub(crate) fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

fn map_row(row: PgRow) -> Result<Post, DomainError> {
    let decode_err = |e: sqlx::Error| DomainError::Internal(format!("row decode error: {}", e));

    Ok(Post {
        id: row.try_get("id").map_err(decode_err)?,
        author_id: row.try_get("author_id").map_err(decode_err)?,
        title: row.try_get("title").map_err(decode_err)?,
        content: row.try_get("content").map_err(decode_err)?,
        created_at: row.try_get("created_at").map_err(decode_err)?,
        updated_at: row.try_get("updated_at").map_err(decode_err)?,
    })
}

#[async_trait]
impl PostRepository for PostgresPostRepository {
    async fn create(&self, post: Post) -> Result<Post, DomainError> {
        sqlx::query(
            r#"
            INSERT INTO posts (id, title, content, author_id, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6)
            "#,
        )
            .bind(post.id)
            .bind(&post.title)
            .bind(&post.content)
            .bind(&post.author_id)
            .bind(&post.created_at)
            .bind(&post.updated_at)
            .execute(&self.pool)
            .await
            .map_err(|e| {
                error!("failed to create post: {}", e);

                return DomainError::Internal(format!("database error: {}", e));
            })?;

        info!(post_id = %post.id, author_id = %post.author_id, title = %post.title, "post created");
        Ok(post)
    }

    async fn update(&self, author_id: Uuid, post_id: Uuid, title: String, content: String) -> Result<Post, DomainError> {
        let row = sqlx::query(
            r#"
            UPDATE posts
            SET title = $1, content = $2, updated_at = NOW()
            WHERE id = $3 AND author_id = $4
            RETURNING id, author_id, title, content, created_at, updated_at
            "#,
        )
            .bind(title)
            .bind(content)
            .bind(post_id)
            .bind(author_id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| {
                error!("failed to create post: {}", e);

                return DomainError::Internal(format!("database error: {}", e));
            })?;

        match row {
            Some(row) => {
                let post = map_row(row)?;
                info!(post_id = %post.id, author_id = %post.author_id, title = %post.title, "post created");
                Ok(post)
            },
            None => {
                Err(DomainError::PostNotFound(format!("post {} not found", post_id)))
            }
        }
    }

    async fn delete(&self, author_id: Uuid, post_id: Uuid) -> Result<(), DomainError> {
        let result = sqlx::query(
            r#"
            DELETE FROM posts
            WHERE id = $1 AND author_id = $2
            "#,
        )
            .bind(post_id)
            .bind(author_id)
            .execute(&self.pool)
            .await
            .map_err(|e| {
                error!("failed to delete post: {}", e);

                return DomainError::Internal(format!("database error: {}", e));
            })?;

        if result.rows_affected() == 0 {
            Err(DomainError::PostNotFound(format!("post {} not found", post_id)))
        } else {
            info!(post_id = %post_id, author_id = %author_id, "post deleted");
            Ok(())
        }
    }

    async fn list(&self, limit: u32, offset: u32) -> Result<Vec<Post>, DomainError> {
        let rows = sqlx::query(
            r#"
            SELECT id, author_id, title, content, created_at, updated_at
            FROM posts
            ORDER BY created_at DESC
            LIMIT $1 OFFSET $2
            "#,
        )
            .bind(limit as i64)
            .bind(offset as i64)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| {
                error!("failed to list posts: {}", e);

                return DomainError::Internal(format!("database error: {}", e));
            })?;

        let posts = rows.into_iter().map(map_row).collect::<Result<Vec<_>, _>>()?;
        Ok(posts)
    }

    async fn get(&self, post_id: Uuid) -> Result<Post, DomainError> {
        let row = sqlx::query(
            r#"
            SELECT id, author_id, title, content, created_at, updated_at
            FROM posts
            WHERE id = $1
            "#,
        )
            .bind(post_id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| {
                error!("failed to get post {}: {}", post_id, e);

                return DomainError::Internal(format!("database error: {}", e));
            })?;

        match row {
            Some(row) => {
                let post = map_row(row)?;
                Ok(post)
            },
            None => {
                Err(DomainError::PostNotFound(format!("post {} not found", post_id)))
            }
        }
    }
}
