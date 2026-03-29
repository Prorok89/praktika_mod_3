use argon2::{
    Algorithm, Argon2, Params, PasswordHash, PasswordHasher, PasswordVerifier, password_hash::{self, Salt, SaltString, rand_core::OsRng}
};
use chrono::{self, DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::error::BlogError;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
}

impl User {
    pub fn new(email: &str, password: &str, username: &str) -> Result<Self, BlogError> {
        let password_hash =
            hash_password(&password).map_err(|e| BlogError::ErrorNotKnow(e.to_string()))?;

        Ok(Self {
            id: Uuid::new_v4(),
            email: email.to_string(),
            password_hash,
            username: username.to_string(),
            created_at: Utc::now(),
        })
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

fn hash_password(password: &str) -> Result<String, argon2::password_hash::Error> {
    let salt = SaltString::generate(&mut OsRng);

    let params = Params::new(19 * 1024, 2, 1, None)?;

    let argon2 = Argon2::new(argon2::Algorithm::Argon2id, argon2::Version::V0x13, params);

    let password_hash = argon2.hash_password(password.as_bytes(), &salt)?;

    Ok(password_hash.to_string())
}

fn verify_password(password: &str, hash: &str) ->Result<bool, argon2::password_hash::Error>
{
    let parsed_hash = PasswordHash::new(hash)?;
    let argon2 = Argon2::default();

    Ok(argon2.verify_password(password.as_bytes(), &parsed_hash).is_ok())
}
