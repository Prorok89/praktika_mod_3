use actix_cors::Cors;

use crate::{domain::error::BlogError, infrastructure::config::Config};

pub fn configure_cors(config: &Config) -> Cors {
    let mut cors = Cors::default()
        .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
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
