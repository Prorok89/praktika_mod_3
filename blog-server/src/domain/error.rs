use std::env::VarError;

use actix_web::ResponseError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum BlogError {
    #[error("user not found")]
    UserNotFound,
    #[error("user already exists")]
    UserAlreadyExists,
    #[error("invalid credentials")]
    InvalidCredentials,
    #[error("post not found")]
    PostNotFound,
    #[error("forbidden")]
    Forbidden,
    #[error("var on .env not found : {0}")]
    VarEnvNotFound(#[from] VarError),
    #[error("надо поправить: {0}")]
    ErrorNotKnow(String),
    #[error("sql error: {0}")]
    SqlError(String),
}

impl ResponseError for BlogError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            BlogError::UserNotFound => actix_web::http::StatusCode::NOT_FOUND,
            BlogError::UserAlreadyExists => actix_web::http::StatusCode::CONFLICT,
            BlogError::InvalidCredentials => actix_web::http::StatusCode::UNAUTHORIZED,
            BlogError::PostNotFound => actix_web::http::StatusCode::NOT_FOUND,
            BlogError::Forbidden => actix_web::http::StatusCode::FORBIDDEN,
            BlogError::VarEnvNotFound(_) => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            BlogError::ErrorNotKnow(_) => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            BlogError::SqlError(_) => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
