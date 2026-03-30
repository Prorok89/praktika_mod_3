use actix_web::{App, HttpServer};

use crate::{
    data::user_repository,
    domain::{error::BlogError},
    infrastructure::{config::Config, database},
};

mod data;
mod domain;
mod infrastructure;
mod presentation;

#[actix_web::main]
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

    database::run_migrations(&pool)
        .await
        .map_err(|e| BlogError::ErrorNotKnow(e.to_string()))?;

    
    _ = HttpServer::new(move || {
        App::new()
            .configure(presentation::http_handlers::configure)
    })
    .bind(("0.0.0.0", config.port)).map_err(|e| BlogError::ErrorNotKnow(e.to_string()))?
    .run()
    .await;
    
    Ok(())
    
}
