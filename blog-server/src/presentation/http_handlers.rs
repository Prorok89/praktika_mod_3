/*
POST /api/auth/register - public
POST /api/auth/login - public

GET /api/posts/{id} - public
GET /api/posts - public
POST /api/posts - private
PUT /api/posts/{id} - private
DELETE /api/posts/{id} - private
*/

use actix_web::{
    HttpResponse, Scope, delete, get, post, put,
    web::{self},
};
use sqlx::PgPool;

use crate::{
    application::auth_service::AuthService,
    domain::{error::BlogError, user::FormReg}, infrastructure::{config::Config, jwt::JwtService},
};

pub fn scope_private() -> Scope {
    let scope_api = web::scope("/api")
        .service(create_posts)
        .service(put_post)
        .service(delete_post);

    scope_api
}

pub fn scope_public() -> Scope {
    let scope_auth = web::scope("/auth")
        .service(auth_register)
        .service(auth_login);

    let scope_api = web::scope("/api")
        .service(scope_auth)
        .service(get_post)
        .service(get_posts);

    scope_api
}

#[post("/register")]
async fn auth_register(
    user: web::Json<FormReg>,
    auth_service: web::Data<AuthService>,
    pool: web::Data<PgPool>,
    config: web::Data<Config>,
) -> Result<HttpResponse, BlogError> {
    // username, email, password

    match auth_service.create_user(&user, &pool).await {
        Ok(new_user) => {
            let token = JwtService::new(&config.jwt_secret).generate_token(
                new_user.id.unwrap(),
                new_user.username.clone(),
            )?;

            Ok(HttpResponse::Created().json(serde_json::json!({
                "token": token,
                "user": {
                    "username": new_user.username,
                    "email": new_user.email
                }
            })))
        }
        Err(BlogError::UserAlreadyExists) => Ok(HttpResponse::Conflict().json(serde_json::json!({
            "error": "User already exists"
        }))),
        Err(e) => Err(e),
    }
}
#[post("/login")]
async fn auth_login() -> HttpResponse {
    // username, password
    HttpResponse::Ok().json(serde_json::json!({
        "test" : "1"
    }))
}

#[post("/posts")]
async fn create_posts() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({
        "test" : "1"
    }))
}

#[get("/posts/{id}")]
async fn get_post() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({
        "test" : "1"
    }))
}

#[get("/posts")]
async fn get_posts() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({
        "test" : "1"
    }))
}

#[put("/posts/{id}")]
async fn put_post() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({
        "test" : "1"
    }))
}

#[delete("/posts/{id}")]
async fn delete_post() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({
        "test" : "1"
    }))
}
