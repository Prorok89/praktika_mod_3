use std::sync::Arc;

use actix_web::{App, HttpServer, web};

use crate::{
    application::{auth_service::AuthService, blog_service::BlogService},
    domain::error::BlogError,
    infrastructure::{config::Config, database, logging},
};

mod application;
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

    logging::init();
    tracing::info!("Satrting server ...");

    dotenvy::dotenv().ok();

    let config = Config::from_env()?;

    let pool = database::create_pool(&config.database_url)
        .await
        .map_err(|e| BlogError::ErrorNotKnow(e.to_string()))?;

    database::run_migrations(&pool)
        .await
        .map_err(|e| BlogError::ErrorNotKnow(e.to_string()))?;

    let blog_service = Arc::new(BlogService {});

    _ = HttpServer::new(move || {
        let auth_service= AuthService::new();
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(auth_service))
            .app_data(web::Data::new(blog_service.clone()))
            .configure(presentation::http_handlers::configure)
    })
    .bind(("0.0.0.0", config.port))
    .map_err(|e| BlogError::ErrorNotKnow(e.to_string()))?
    .run()
    .await;

    Ok(())
}
