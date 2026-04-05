use crate::error::{BlogClientError, Result};
use reqwest::Client;
use serde::{de::DeserializeOwned, Serialize};

pub struct HttpClient {
    client: Client,
    base_url: String,
}

impl HttpClient {
    pub fn new(base_url: String) -> Self {
        Self {
            client: Client::new(),
            base_url,
        }
    }

    pub async fn get<T: DeserializeOwned>(&self, path: &str) -> Result<T> {
        let url = format!("{}{}", self.base_url, path);
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
            Err(BlogClientError::HttpError(
                response.error_for_status().unwrap_err(),
            ))
        }
    }

    pub async fn get_with_auth<T: DeserializeOwned>(&self, path: &str, token: &str) -> Result<T> {
        let url = format!("{}{}", self.base_url, path);
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

    pub async fn post<T: Serialize, R: DeserializeOwned>(&self, path: &str, body: &T) -> Result<R> {
        let url = format!("{}{}", self.base_url, path);
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
            Err(BlogClientError::HttpError(
                response.error_for_status().unwrap_err(),
            ))
        }
    }

    pub async fn post_with_auth<T: Serialize, R: DeserializeOwned>(
        &self,
        path: &str,
        body: &T,
        token: &str,
    ) -> Result<R> {
        let url = format!("{}{}", self.base_url, path);
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

    pub async fn put_with_auth<T: Serialize, R: DeserializeOwned>(
        &self,
        path: &str,
        body: &T,
        token: &str,
    ) -> Result<R> {
        let url = format!("{}{}", self.base_url, path);
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

    pub async fn delete_with_auth(&self, path: &str, token: &str) -> Result<()> {
        let url = format!("{}{}", self.base_url, path);
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
}
