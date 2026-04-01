use crate::domain::error::BlogError;

#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub port: u16,
    pub debug: bool,
    pub jwt_secret: String,
}

impl Config {
    pub fn from_env() -> Result<Self, BlogError> {
        let database_url = std::env::var("DATABASE_URL")?;
        let jwt_secret = std::env::var("JWT_SECRET")?;
        let port = std::env::var("PORT")?
            .parse::<u16>()
            .map_err(|e| BlogError::ErrorNotKnow(e.to_string()))?;
        let debug = std::env::var("DEBUG")?
            .parse::<bool>()
            .map_err(|e| BlogError::ErrorNotKnow(e.to_string()))?;

        Ok(Self {
            database_url,
            port,
            debug,
			jwt_secret
        })
    }
}
