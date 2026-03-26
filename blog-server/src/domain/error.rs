use std::env::VarError;

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
}
