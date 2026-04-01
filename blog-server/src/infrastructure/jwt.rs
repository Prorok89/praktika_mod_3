use chrono::{DateTime, Utc};

struct Claims {
    user_id: u64,
    username: String,
    exp: DateTime<Utc>,
}

struct JwtService {
    encoding: String,
    decoding: String,
}

impl JwtService {
    pub fn new(secret: &str) {}
    pub fn generate_token(user_id: u64, username: String) {}
    pub fn verify_token(token: String) {}
}
