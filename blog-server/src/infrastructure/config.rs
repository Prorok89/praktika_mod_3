use crate::domain::error::BlogError;

#[derive(Debug)]
pub struct Config {
    pub database_url: String,
    pub port: u16
}

impl Config {
    pub fn from_env() -> Result<Self, BlogError> {
        let database_url = std::env::var("DATABASE_URL")?;
        let port = std::env::var("PORT")?.parse::<u16>().map_err(|e|BlogError::ErrorNotKnow(e.to_string()))?;

        Ok(Self { database_url, port })
    }
}
