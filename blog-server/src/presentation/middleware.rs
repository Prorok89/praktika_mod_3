use actix_cors::Cors;
use actix_web::{Error, HttpMessage, dev::ServiceRequest, error::ErrorUnauthorized, web};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use serde::{Deserialize, Serialize};

use crate::infrastructure::{config::Config, jwt::JwtService};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AuthenticatedUser {
    pub user_id: i64,
    pub username: String,
}

pub fn configure_cors(config: &Config) -> Cors {
    let mut cors = Cors::default()
        .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])
        .allowed_headers(vec![
            actix_web::http::header::CONTENT_TYPE,
            actix_web::http::header::AUTHORIZATION,
        ])
        .max_age(3600);

    if config.debug {
        cors = cors.allow_any_origin();
    }

    cors
}

pub async fn jwt_validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    let app_config = match req.app_data::<web::Data<Config>>() {
        Some(config) => config,
        None => {
            return Err((ErrorUnauthorized("Config not found"), req));
        }
    };

    let jwt_service = JwtService::new(&app_config.jwt_secret);

    match jwt_service.verify_token(credentials.token()) {
        Ok(claims) => {
            req.extensions_mut().insert(AuthenticatedUser {
                user_id: claims.user_id,
                username: claims.username,
            });
            return Ok(req);
        }
        Err(e) => {
            return Err((ErrorUnauthorized(e.to_string()), req));
        }
    }
}
