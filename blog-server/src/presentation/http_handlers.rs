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
    application::{auth_service::AuthService, blog_service::BlogService},
    domain::{
        error::BlogError,
        post::{PostCreateOrUpdate, QueryParams},
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
					"id" : l_user.id,
                    "username": l_user.username,
                    "email": l_user.email
                }
            })))
        }
        Err(e) => Err(e),
    }
}

pub async fn create_posts(
    req: HttpRequest,
    post: web::Json<PostCreateOrUpdate>,
    blog_service: web::Data<BlogService>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, BlogError> {
    match req.extensions().get::<AuthenticatedUser>() {
        Some(user) => {
            let post_new = blog_service.create_post(&post, &user, &pool).await?;
            Ok(HttpResponse::Created().json(serde_json::json!(post_new)))
        }
        None => Err(BlogError::ErrorNotKnow("E1".to_string())),
    }
}

pub async fn get_post(
    path: web::Path<i64>,
    blog_service: web::Data<BlogService>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, BlogError> {
    let user_id = path.into_inner();
    match blog_service.get_post(user_id, &pool).await {
        Ok(post) => Ok(HttpResponse::Created().json(serde_json::json!(post))),
        Err(e) => Err(BlogError::PostNotFound(e.to_string())),
    }
}

pub async fn get_posts(
    query: web::Query<QueryParams>,
    blog_service: web::Data<BlogService>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, BlogError> {
    let limit = query.limit;
    let offset = query.offset;

    let (posts, count) = blog_service.get_posts(limit, offset, &pool).await?;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "posts": posts,
        "total": count,
        "limit": limit,
        "offset": offset
    })))
}

pub async fn put_post(
    path: web::Path<i64>,
    req: HttpRequest,
    post: web::Json<PostCreateOrUpdate>,
    blog_service: web::Data<BlogService>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, BlogError> {
    match req.extensions().get::<AuthenticatedUser>() {
        Some(user) => {
            let post_id = path.into_inner();
            let updated_post = blog_service
                .update_post(post_id, &post, &user, &pool)
                .await?;
            Ok(HttpResponse::Ok().json(serde_json::json!(updated_post)))
        }
        None => Err(BlogError::ErrorNotKnow("E2".to_string())),
    }
}

pub async fn delete_post(
    path: web::Path<i64>,
    req: HttpRequest,
    blog_service: web::Data<BlogService>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, BlogError> {
    match req.extensions().get::<AuthenticatedUser>() {
        Some(user) => {
            let post_id = path.into_inner();
            blog_service.delete_post(post_id, &user, &pool).await?;

            Ok(HttpResponse::NoContent().into())
        }
        None => Err(BlogError::ErrorNotKnow("E3".to_string())),
    }
}
