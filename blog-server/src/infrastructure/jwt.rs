use actix_web::dev::{ServiceRequest, Transform};
use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};

use crate::domain::error::BlogError;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Claims {
    user_id: u64,
    username: String,
    exp: usize,
}

struct JwtService {
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

    pub fn generate_token(&self, user_id: u64, username: String) -> Result<String, BlogError> {
        let cur_time = Utc::now();
        let exp = cur_time + Duration::hours(24);

        let claims = Claims {
            user_id,
            username,
            exp : exp.timestamp() as usize,
        };

        let token = encode(&Header::default(), &claims, &self.encoding).map_err(|e| BlogError::ErrorNotKnow(e.to_string()))?;

		Ok(token)
    }

    pub fn verify_token(&self, token: String) -> Result<Claims, BlogError>{
		let mut validaion = Validation::default();
		validaion.validate_exp = true;

		let token_data = decode::<Claims> (token, &self.decoding, &validaion).map_err(|e| BlogError::ErrorNotKnow(e.to_string()))?;

		Ok(token_data.claims)
	}
}

impl<S, B> Transform<S, ServiceRequest> for  JwtService {
	type Response;

	type Error;

	type Transform;

	type InitError;

	type Future;

	fn new_transform(&self, service: S) -> Self::Future {
		todo!()
	}
}