#[cfg(target_arch = "wasm32")]
use gloo_net::http::Request;
#[cfg(not(target_arch = "wasm32"))]
use reqwest::Client;

use crate::error::{BlogClientError, Result};
use serde::{Serialize, de::DeserializeOwned};

#[derive(serde::Deserialize)]
struct ApiErrorResponse {
    error: String,
}

pub struct HttpClient {
    #[cfg(not(target_arch = "wasm32"))]
    client: Client,
    base_url: String,
}

impl HttpClient {
    pub fn new(base_url: String) -> Self {
        Self {
            #[cfg(not(target_arch = "wasm32"))]
            client: Client::new(),
            base_url,
        }
    }

    pub async fn get<T: DeserializeOwned>(&self, path: &str) -> Result<T> {
        let url = format!("{}{}", self.base_url, path);
        #[cfg(not(target_arch = "wasm32"))]
        {
            let response = self.client.get(&url).send().await?;

            if response.status().is_success() {
                Ok(response.json().await?)
            } else if response.status() == reqwest::StatusCode::NOT_FOUND {
                Err(BlogClientError::NotFound(path.to_string()))
            } else if response.status() == reqwest::StatusCode::UNAUTHORIZED
                || response.status() == reqwest::StatusCode::FORBIDDEN
            {
                Err(BlogClientError::Unauthorized)
            } else {
                let status = response.status();
                let body_text = response.text().await.unwrap_or_default();

                if let Ok(api_err) = serde_json::from_str::<ApiErrorResponse>(&body_text) {
                    Err(BlogClientError::InternalError(api_err.error))
                } else {
                    Err(BlogClientError::InternalError(format!(
                        "HTTP error: {}",
                        status
                    )))
                }
            }
        }
        #[cfg(target_arch = "wasm32")]
        {
            let resp = Request::get(&url)
                .send()
                .await
                .map_err(|e| BlogClientError::InternalError(format!("Gloo-net error: {}", e)))?;

            if resp.ok() {
                resp.json()
                    .await
                    .map_err(|e| BlogClientError::InternalError(format!("JSON error: {}", e)))
            } else if resp.status() == 404 {
                Err(BlogClientError::NotFound(path.to_string()))
            } else if resp.status() == 401 || resp.status() == 403 {
                Err(BlogClientError::Unauthorized)
            } else {
				let status = resp.status();
                let body_text = resp.text().await.unwrap_or_default();

                if let Ok(api_err) = serde_json::from_str::<ApiErrorResponse>(&body_text) {
                    Err(BlogClientError::InternalError(api_err.error))
                } else {
                    Err(BlogClientError::InternalError(format!(
                        "HTTP error: {}",
                        status
                    )))
                }
            }
        }
    }

    pub async fn get_with_auth<T: DeserializeOwned>(&self, path: &str, token: &str) -> Result<T> {
        let url = format!("{}{}", self.base_url, path);
        #[cfg(not(target_arch = "wasm32"))]
        {
            let response = self
                .client
                .get(&url)
                .header("Authorization", format!("Bearer {}", token))
                .send()
                .await?;

            if response.status().is_success() {
                Ok(response.json().await?)
            } else if response.status() == reqwest::StatusCode::NOT_FOUND {
                Err(BlogClientError::NotFound(path.to_string()))
            } else if response.status() == reqwest::StatusCode::UNAUTHORIZED
                || response.status() == reqwest::StatusCode::FORBIDDEN
            {
                Err(BlogClientError::Unauthorized)
            } else {
                Err(BlogClientError::HttpError(
                    response.error_for_status().unwrap_err(),
                ))
            }
        }
        #[cfg(target_arch = "wasm32")]
        {
            let resp = Request::get(&url)
                .header("Authorization", &format!("Bearer {}", token).to_string())
                .send()
                .await
                .map_err(|e| BlogClientError::InternalError(format!("Gloo-net error: {}", e)))?;

            if resp.ok() {
                resp.json()
                    .await
                    .map_err(|e| BlogClientError::InternalError(format!("JSON error: {}", e)))
            } else if resp.status() == 404 {
                Err(BlogClientError::NotFound(path.to_string()))
            } else if resp.status() == 401 || resp.status() == 403 {
                Err(BlogClientError::Unauthorized)
            } else {
                Err(BlogClientError::InternalError(format!(
                    "HTTP error: {}",
                    resp.status()
                )))
            }
        }
    }

    pub async fn post<T: Serialize, R: DeserializeOwned>(&self, path: &str, body: &T) -> Result<R> {
        let url = format!("{}{}", self.base_url, path);
        #[cfg(not(target_arch = "wasm32"))]
        {
            let response = self.client.post(&url).json(body).send().await?;

            if response.status().is_success() {
                Ok(response.json().await?)
            } else if response.status() == reqwest::StatusCode::NOT_FOUND {
                Err(BlogClientError::NotFound(path.to_string()))
            } else if response.status() == reqwest::StatusCode::UNAUTHORIZED
                || response.status() == reqwest::StatusCode::FORBIDDEN
            {
                Err(BlogClientError::Unauthorized)
            } else {
                let status = response.status();
                let body_text = response.text().await.unwrap_or_default();

                if let Ok(api_err) = serde_json::from_str::<ApiErrorResponse>(&body_text) {
                    Err(BlogClientError::InternalError(api_err.error))
                } else {
                    Err(BlogClientError::InternalError(format!(
                        "HTTP error: {}",
                        status
                    )))
                }
            }
        }
        #[cfg(target_arch = "wasm32")]
        {
            let resp =
                Request::post(&url).json(body)?.send().await.map_err(|e| {
                    BlogClientError::InternalError(format!("Gloo-net error: {}", e))
                })?;

            if resp.ok() {
                resp.json()
                    .await
                    .map_err(|e| BlogClientError::InternalError(format!("JSON error: {}", e)))
            } else if resp.status() == 404 {
                Err(BlogClientError::NotFound(path.to_string()))
            } else if resp.status() == 401 || resp.status() == 403 {
                Err(BlogClientError::Unauthorized)
            } else {
				let status = resp.status();
                let body_text = resp.text().await.unwrap_or_default();

                if let Ok(api_err) = serde_json::from_str::<ApiErrorResponse>(&body_text) {
                    Err(BlogClientError::InternalError(api_err.error))
                } else {
                    Err(BlogClientError::InternalError(format!(
                        "HTTP error: {}",
                        status
                    )))
                }
            }
        }
    }

    pub async fn post_with_auth<T: Serialize, R: DeserializeOwned>(
        &self,
        path: &str,
        body: &T,
        token: &str,
    ) -> Result<R> {
        let url = format!("{}{}", self.base_url, path);
        #[cfg(not(target_arch = "wasm32"))]
        {
            let response = self
                .client
                .post(&url)
                .header("Authorization", format!("Bearer {}", token))
                .json(body)
                .send()
                .await?;

            if response.status().is_success() {
                Ok(response.json().await?)
            } else if response.status() == reqwest::StatusCode::NOT_FOUND {
                Err(BlogClientError::NotFound(path.to_string()))
            } else if response.status() == reqwest::StatusCode::UNAUTHORIZED
                || response.status() == reqwest::StatusCode::FORBIDDEN
            {
                Err(BlogClientError::Unauthorized)
            } else {
                Err(BlogClientError::HttpError(
                    response.error_for_status().unwrap_err(),
                ))
            }
        }
        #[cfg(target_arch = "wasm32")]
        {
            let resp = Request::post(&url)
                .header("Authorization", &format!("Bearer {}", token))
                .json(body)?
                .send()
                .await
                .map_err(|e| BlogClientError::InternalError(format!("Gloo-net error: {}", e)))?;

            if resp.ok() {
                resp.json()
                    .await
                    .map_err(|e| BlogClientError::InternalError(format!("JSON error: {}", e)))
            } else if resp.status() == 404 {
                Err(BlogClientError::NotFound(path.to_string()))
            } else if resp.status() == 401 || resp.status() == 403 {
                Err(BlogClientError::Unauthorized)
            } else {
                Err(BlogClientError::InternalError(format!(
                    "HTTP error: {}",
                    resp.status()
                )))
            }
        }
    }

    pub async fn put_with_auth<T: Serialize, R: DeserializeOwned>(
        &self,
        path: &str,
        body: &T,
        token: &str,
    ) -> Result<R> {
        let url = format!("{}{}", self.base_url, path);
        #[cfg(not(target_arch = "wasm32"))]
        {
            let response = self
                .client
                .put(&url)
                .header("Authorization", format!("Bearer {}", token))
                .json(body)
                .send()
                .await?;

            if response.status().is_success() {
                Ok(response.json().await?)
            } else if response.status() == reqwest::StatusCode::NOT_FOUND {
                Err(BlogClientError::NotFound(path.to_string()))
            } else if response.status() == reqwest::StatusCode::UNAUTHORIZED
                || response.status() == reqwest::StatusCode::FORBIDDEN
            {
                Err(BlogClientError::Unauthorized)
            } else {
                Err(BlogClientError::HttpError(
                    response.error_for_status().unwrap_err(),
                ))
            }
        }
        #[cfg(target_arch = "wasm32")]
        {
            let resp = Request::put(&url)
                .header("Authorization", &format!("Bearer {}", token))
                .json(body)?
                .send()
                .await
                .map_err(|e| BlogClientError::InternalError(format!("Gloo-net error: {}", e)))?;

            if resp.ok() {
                resp.json()
                    .await
                    .map_err(|e| BlogClientError::InternalError(format!("JSON error: {}", e)))
            } else if resp.status() == 404 {
                Err(BlogClientError::NotFound(path.to_string()))
            } else if resp.status() == 401 || resp.status() == 403 {
                Err(BlogClientError::Unauthorized)
            } else {
                Err(BlogClientError::InternalError(format!(
                    "HTTP error: {}",
                    resp.status()
                )))
            }
        }
    }

    pub async fn delete_with_auth(&self, path: &str, token: &str) -> Result<()> {
        let url = format!("{}{}", self.base_url, path);
        #[cfg(not(target_arch = "wasm32"))]
        {
            let response = self
                .client
                .delete(&url)
                .header("Authorization", format!("Bearer {}", token))
                .send()
                .await?;

            if response.status().is_success() {
                Ok(())
            } else if response.status() == reqwest::StatusCode::NOT_FOUND {
                Err(BlogClientError::NotFound(path.to_string()))
            } else if response.status() == reqwest::StatusCode::UNAUTHORIZED
                || response.status() == reqwest::StatusCode::FORBIDDEN
            {
                Err(BlogClientError::Unauthorized)
            } else {
                Err(BlogClientError::HttpError(
                    response.error_for_status().unwrap_err(),
                ))
            }
        }
        #[cfg(target_arch = "wasm32")]
        {
            let resp = Request::delete(&url)
                .header("Authorization", &format!("Bearer {}", token).to_string())
                .send()
                .await
                .map_err(|e| BlogClientError::InternalError(format!("Gloo-net error: {}", e)))?;

            if resp.ok() {
                Ok(())
            } else if resp.status() == 404 {
                Err(BlogClientError::NotFound(path.to_string()))
            } else if resp.status() == 401 || resp.status() == 403 {
                Err(BlogClientError::Unauthorized)
            } else {
                Err(BlogClientError::InternalError(format!(
                    "HTTP error: {}",
                    resp.status()
                )))
            }
        }
    }
}
