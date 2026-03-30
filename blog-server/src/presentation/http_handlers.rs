/*
POST /api/auth/register
POST /api/auth/login

POST /api/posts 
GET /api/posts/{id} 
PUT /api/posts/{id}
DELETE /api/posts/{id}
GET /api/posts
*/

use actix_web::{HttpResponse, get, post, put, delete, web::{self, ServiceConfig}};

use crate::domain::error::BlogError;

pub fn configure(cfg: &mut ServiceConfig)
{
    let scope_auth = web::scope("/auth")
        .service(auth_register)
        .service(auth_login);

    let scope_api = 
        web::scope("/api")
            .service(scope_auth)
            .service(create_posts)
            .service(get_post)
            .service(get_posts)
            .service(put_post)
            .service(delete_post)
            ;

    cfg.service(scope_api);
}

#[post("/register")]
async fn auth_register() -> Result<HttpResponse, BlogError>{
    // username, email, password
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "test" : "1"
    })))
}
#[post("/login")]
async fn auth_login() -> HttpResponse{
    // username, password
    HttpResponse::Ok().json(serde_json::json!({
        "test" : "1"
    }))
}

#[post("/posts")]
async fn create_posts() -> HttpResponse{
    HttpResponse::Ok().json(serde_json::json!({
        "test" : "1"
    }))
}

#[get("/posts/{id}")]
async fn get_post() -> HttpResponse{
    HttpResponse::Ok().json(serde_json::json!({
        "test" : "1"
    }))
}

#[get("/posts")]
async fn get_posts() -> HttpResponse{
    HttpResponse::Ok().json(serde_json::json!({
        "test" : "1"
    }))
}

#[put("/posts/{id}")]
async fn put_post() -> HttpResponse{
    HttpResponse::Ok().json(serde_json::json!({
        "test" : "1"
    }))
}

#[delete("/posts/{id}")]
async fn delete_post() -> HttpResponse{
    HttpResponse::Ok().json(serde_json::json!({
        "test" : "1"
    }))
}

