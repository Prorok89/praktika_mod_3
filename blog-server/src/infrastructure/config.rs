use crate::domain::error::BlogError;

#[derive(Debug)]
pub struct Config {
    pub database_url: String,
}

impl Config {
    pub fn from_env() -> Result<Self, BlogError> {
        let database_url = std::env::var("DATABASE_URL")?;

        Ok(Self { database_url })
    }
}
