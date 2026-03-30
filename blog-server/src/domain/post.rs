use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Post {
    pub id: Option<i64>,
    pub title: String,
    pub content: String,
    pub author_id: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub struct PostCreateOrUpdate {
    title: String,
    content: String,
}

impl Post {
    pub fn new(title : String, content : String, author_id : i64) -> Self {
        Self {
            id: None,
            title,
            content,
            author_id,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}