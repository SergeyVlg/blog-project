use chrono::{DateTime, Utc};
use uuid::Uuid;

pub(crate) struct Post {
    pub id: Uuid,
    pub author_id: Uuid,
    pub title: String,
    pub content: String,
    pub created_at: DateTime<Utc>,
}

impl Post {
    pub fn new(author_id: Uuid, title: String, content: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            author_id,
            title,
            content,
            created_at: Utc::now(),
        }
    }
}