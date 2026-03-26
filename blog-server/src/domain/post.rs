use chrono::{DateTime, Utc};
use uuid::Uuid;

pub struct Post {
    pub id: Uuid,
    pub title: String,
    pub content: String,
    pub author_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub struct PostCreateOrUpdate {
    title: String,
    content: String,
}

impl Post {
    pub fn new(title : String, content : String) -> Self {
        Self {
            id: Uuid::new_v4(),
            title,
            content,
            author_id: Uuid::new_v4(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}