use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};

use crate::domain::error::BlogError;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) struct Claims {
    pub user_id: i64,
    pub username: String,
    exp: usize,
}

#[derive(Debug, Clone)]
pub struct JwtService {
    encoding: EncodingKey,
    decoding: DecodingKey,
}

impl JwtService {
    pub fn new(secret: &str) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret.as_bytes()),
            decoding: DecodingKey::from_secret(secret.as_bytes()),
        }
    }

    pub fn generate_token(&self, user_id: i64, username: String) -> Result<String, BlogError> {
        let cur_time = Utc::now();
        let exp = cur_time + Duration::hours(24);

        let claims = Claims {
            user_id,
            username,
            exp: exp.timestamp() as usize,
        };

        let token = encode(&Header::default(), &claims, &self.encoding)
            .map_err(|e| BlogError::ErrorNotKnow(e.to_string()))?;

        Ok(token)
    }

    pub fn verify_token(&self, token: &str) -> Result<Claims, BlogError> {
        let mut validaion = Validation::default();
        validaion.validate_exp = true;

        let token_data = decode::<Claims>(token, &self.decoding, &validaion)
            .map_err(|e| BlogError::ErrorNotKnow(e.to_string()))?;

        Ok(token_data.claims)
    }
}