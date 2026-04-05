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
    tracing::info!("Starting server ...");

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

    tracing::info!("HTTP server listening on http://0.0.0.0:{}", config.port);
    tracing::info!("gRPC server listening on {}", grpc_addr);

    let grpc_handle = tokio::spawn(async move {
        Server::builder()
            .add_service(BlogServiceServer::new(grpc_service))
            .serve(grpc_addr)
            .await
            .map_err(|e| BlogError::ErrorNotKnow(format!("gRPC server error: {}", e)))
    });

    let auth_service = AuthService::new();
    let blog_service = BlogService::new();

    let http_server = HttpServer::new(move || {
        let cors = configure_cors(&config_clone);
        let auth = HttpAuthentication::bearer(jwt_validator);

        App::new()
            .wrap(cors)
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(auth_service.clone()))
            .app_data(web::Data::new(blog_service.clone()))
            .app_data(web::Data::new(config_clone.clone()))
            .route("/api/auth/register", web::post().to(http_handlers::auth_register))
            .route("/api/auth/login", web::post().to(http_handlers::auth_login))
            .route("/api/posts/{id}", web::get().to(http_handlers::get_post))
            .route("/api/posts", web::get().to(http_handlers::get_posts))
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
    .run();

    let http_handle = tokio::spawn(async move {
        http_server.await.map_err(|e| BlogError::ErrorNotKnow(e.to_string()))
    });

    tokio::select! {
        http_result = http_handle => {
            match http_result {
                Ok(Ok(())) => {
                    tracing::info!("HTTP server exited normally");
                    Ok(())
                }
                Ok(Err(e)) => {
                    tracing::error!("HTTP server error: {}", e);
                    Err(e)
                }
                Err(e) => {
                    tracing::error!("HTTP task panicked: {}", e);
                    Err(BlogError::ErrorNotKnow(format!("HTTP task panicked: {}", e)))
                }
            }
        }
        grpc_result = grpc_handle => {
            match grpc_result {
                Ok(Ok(())) => {
                    tracing::info!("gRPC server exited normally");
                    Ok(())
                }
                Ok(Err(e)) => {
                    tracing::error!("gRPC server error: {}", e);
                    Err(e)
                }
                Err(e) => {
                    tracing::error!("gRPC task panicked: {}", e);
                    Err(BlogError::ErrorNotKnow(format!("gRPC task panicked: {}", e)))
                }
            }
        }
    }
}
