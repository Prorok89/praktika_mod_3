use actix_web::{
    HttpMessage, HttpResponse,
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    error::ErrorUnauthorized,
    http::header::AUTHORIZATION,
};
use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use std::{
    future::{Ready, ready},
    pin::Pin,
    task::{Context, Poll},
};

use crate::domain::error::BlogError;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) struct Claims {
    user_id: i64,
    username: String,
    exp: usize,
}

#[derive(Debug, Clone)]
pub struct JwtService {
    encoding: EncodingKey,
    decoding: DecodingKey,
}

impl JwtService {
    pub fn new(secret: &str) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret.as_bytes()),
            decoding: DecodingKey::from_secret(secret.as_bytes()),
        }
    }

    pub fn generate_token(&self, user_id: i64, username: String) -> Result<String, BlogError> {
        let cur_time = Utc::now();
        let exp = cur_time + Duration::hours(24);

        let claims = Claims {
            user_id,
            username,
            exp: exp.timestamp() as usize,
        };

        let token = encode(&Header::default(), &claims, &self.encoding)
            .map_err(|e| BlogError::ErrorNotKnow(e.to_string()))?;

        Ok(token)
    }

    pub fn verify_token(&self, token: &str) -> Result<Claims, BlogError> {
        let mut validaion = Validation::default();
        validaion.validate_exp = true;

        let token_data = decode::<Claims>(token, &self.decoding, &validaion)
            .map_err(|e| BlogError::ErrorNotKnow(e.to_string()))?;

        Ok(token_data.claims)
    }
}
pub struct JwtMiddleware {
    secret: String,
}

impl JwtMiddleware {
    pub fn new(secret: String) -> Self {
        Self { secret }
    }
}

impl<S, B> Transform<S, ServiceRequest> for JwtMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::Error> + 'static,
    B: actix_web::body::MessageBody,
{
    type Response = ServiceResponse<B>;
    type Error = actix_web::Error;
    type InitError = ();
    type Transform = JwtMiddlewareService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(JwtMiddlewareService {
            service,
            jwt: JwtService::new(&self.secret),
        }))
    }
}

pub struct JwtMiddlewareService<S> {
    service: S,
    jwt: JwtService,
}

impl<S, B> Service<ServiceRequest> for JwtMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::Error> + 'static,
    B: actix_web::body::MessageBody,
{
    type Response = ServiceResponse<B>;
    type Error = actix_web::Error;
    type Future = Pin<Box<dyn std::future::Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx).map_err(|e| e.into())
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        // Извлекаем токен из Authorization: Bearer <token>
        let auth_header = req
            .headers()
            .get(AUTHORIZATION)
            .and_then(|h| h.to_str().ok());

        if let Some(header) = auth_header {
            if let Some(token) = header.strip_prefix("Bearer ") {
                match self.jwt.verify_token(token) {
                    Ok(claims) => {
                        // Добавляем user_id и username в extensions
                        req.extensions_mut().insert(claims.user_id);
                        req.extensions_mut().insert(claims.username);

                        let fut = self.service.call(req);
                        return Box::pin(async move {
                            let res = fut.await?;
                            Ok(res)
                        });
                    }
                    Err(_) => {
                        return Box::pin(async move {
                            Err(actix_web::error::ErrorUnauthorized(
                                "Invalid or expired token".to_string(),
                            ))
                        });
                    }
                }
            }
        }

        // Нет токена

        Box::pin(async move {
            Err(actix_web::error::ErrorUnauthorized(
                "Authorization required".to_string(),
            ))
        })
    }
}
