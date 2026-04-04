use tonic::transport::Server;

use actix_web::{App, HttpServer, web};
use actix_web_httpauth::middleware::HttpAuthentication;

use crate::{
    application::{auth_service::AuthService, blog_service::BlogService},
    domain::error::BlogError,
    infrastructure::{config::Config, database, logging},
    presentation::{
        grpc_service::{BlogGrpcService, blog_service_server::BlogServiceServer},
        http_handlers,
        middleware::{configure_cors, jwt_validator},
    },
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

    let config_clone = config.clone();

    let grpc_addr = "0.0.0.0:50051".parse().unwrap();
    let grpc_service = BlogGrpcService::new(pool.clone(), &config.jwt_secret);

    tokio::spawn(async move {
        _ = Server::builder()
            .add_service(BlogServiceServer::new(grpc_service))
            .serve(grpc_addr)
            .await
            .map_err(|e| BlogError::ErrorNotKnow(format!("gRPC server error: {}", e)));
    });

    _ = HttpServer::new(move || {
        let auth_service = AuthService::new();
        let blog_service = BlogService::new();
        let cors = configure_cors(&config_clone);

        let auth = HttpAuthentication::bearer(jwt_validator);

        App::new()
            .wrap(cors)
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(auth_service))
            .app_data(web::Data::new(blog_service))
            .app_data(web::Data::new(config_clone.clone()))
            .route(
                "/api/auth/register",
                web::post().to(http_handlers::auth_register),
            )
            .route("/api/auth/login", web::post().to(http_handlers::auth_login))
            .route("/api/post/{id}", web::get().to(http_handlers::get_post))
            .route("/api/posts", web::get().to(http_handlers::get_posts))
            // Protected routes with JWT
            .service(
                web::scope("/api")
                    .wrap(auth)
                    .route("/posts", web::post().to(http_handlers::create_posts))
                    .route("/posts/{id}", web::put().to(http_handlers::put_post))
                    .route("/posts/{id}", web::delete().to(http_handlers::delete_post)),
            )
    })
    .bind(("0.0.0.0", config.port))
    .map_err(|e| BlogError::ErrorNotKnow(e.to_string()))?
    .run()
    .await;

    Ok(())
}
