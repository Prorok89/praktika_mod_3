use chrono::{self, DateTime, Utc};
use uuid::Uuid;

pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
}

impl User {
    pub fn new(email : &str, password : &str, username : &str) -> Self {
        Self {
            id: Uuid::new_v4(),
            email : email.to_string(),
            password_hash: password.to_string(),
            username: username.to_string(),
            created_at: Utc::now(),
        }
    }
}

pub struct FormReg {
    username: String,
    email: String,
    password: String,
}

pub struct FormAuth {
    username: String,
    password: String,
}
