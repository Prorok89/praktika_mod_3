use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Post {
	#[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    pub title: String,
    pub content: String,
    pub author_id: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PostCreateOrUpdate {
    pub title: String,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct QueryParams {
	#[serde(default="default_limit")]
	pub limit : i64,
	#[serde(default="default_offset")]
	pub offset : i64
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

fn default_limit() -> i64 { 10 }
fn default_offset() -> i64 { 0 }