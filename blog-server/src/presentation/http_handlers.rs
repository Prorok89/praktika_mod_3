/*
POST /api/auth/register - public
POST /api/auth/login - public

GET /api/posts/{id} - public
GET /api/posts - public

POST /api/posts - private
PUT /api/posts/{id} - private
DELETE /api/posts/{id} - private
*/

use actix_web::{HttpMessage, HttpRequest, HttpResponse, web};
use sqlx::PgPool;

use crate::{
    application::auth_service::AuthService,
    domain::{
        error::BlogError,
        user::{FormAuth, FormReg, verify_password},
    },
    infrastructure::{config::Config, jwt::JwtService},
    presentation::middleware::AuthenticatedUser,
};

pub async fn auth_register(
    user: web::Json<FormReg>,
    auth_service: web::Data<AuthService>,
    pool: web::Data<PgPool>,
    config: web::Data<Config>,
) -> Result<HttpResponse, BlogError> {
    // username, email, password

    match auth_service.create_user(&user, &pool).await {
        Ok(new_user) => {
            let token = JwtService::new(&config.jwt_secret)
                .generate_token(new_user.id.unwrap(), new_user.username.clone())?;

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

pub async fn auth_login(
    user: web::Json<FormAuth>,
    auth_service: web::Data<AuthService>,
    pool: web::Data<PgPool>,
    config: web::Data<Config>,
) -> Result<HttpResponse, BlogError> {
    match auth_service.login_user(&user, &pool).await {
        Ok(l_user) => {
            if !verify_password(&user.password, &l_user.password_hash)
                .map_err(|e| BlogError::ErrorNotKnow(e.to_string()))?
            {
                return Err(BlogError::InvalidCredentials);
            }

            let token = JwtService::new(&config.jwt_secret)
                .generate_token(l_user.id.unwrap(), l_user.username.clone())?;

            Ok(HttpResponse::Created().json(serde_json::json!({
                "token": token,
                "user": {
                    "username": l_user.username,
                    "email": l_user.email
                }
            })))
        }
        Err(e) => Err(e),
    }
}

pub async fn create_posts(req: HttpRequest) -> Result<HttpResponse, BlogError> {
    match req.extensions().get::<AuthenticatedUser>() {
        Some(user) => Ok(HttpResponse::Ok().json(serde_json::json!({
            "test" : &user.user_id
        }))),
        None => Err(BlogError::ErrorNotKnow("E".to_string())),
    }
}

pub async fn get_post() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({
        "test" : "1"
    }))
}

pub async fn get_posts() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({
        "test" : "get_posts"
    }))
}

pub async fn put_post() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({
        "test" : "put_post"
    }))
}

pub async fn delete_post() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({
        "test" : "delete_post"
    }))
}
