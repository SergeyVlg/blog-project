use chrono::{Utc, DateTime};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
}

impl User {
    pub fn new(name: String, email: String, password_hash: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            email,
            password_hash,
            created_at: Utc::now(),
        }
    }
}

pub(crate) struct UserWithToken {
    pub user: User,
    pub token: String,
}

impl UserWithToken {
    pub fn new(user: User, token: String) -> Self {
        Self { user, token }
    }
}