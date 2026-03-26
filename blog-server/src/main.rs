use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::{
    data::user_repository,
    domain::{error::BlogError, user::User},
    infrastructure::{config::Config, database},
};

mod data;
mod domain;
mod infrastructure;

#[tokio::main]
async fn main() {
    if let Err(e) = run().await {
        println!("{}", e);
    }
}

async fn run() -> Result<(), BlogError> {
    dotenvy::dotenv().ok();

    let config = Config::from_env()?;

    let pool = database::create_pool(&config.database_url)
        .await
        .map_err(|e| BlogError::ErrorNotKnow(e.to_string()))?;

    database::migration(&pool)
        .await
        .map_err(|e| BlogError::ErrorNotKnow(e.to_string()))?;

    println!("{:?}", config);

    let user = User::new("test@test1", "test11", "loginlogin1");

    _ = user_repository::create_user(&pool, &user)
        .await
        .map_err(|e| BlogError::ErrorNotKnow(e.to_string()))?;

    Ok(())
}
