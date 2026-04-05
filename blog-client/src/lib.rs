pub mod error;
pub mod http_client;
pub mod grpc_client;

pub use error::{BlogClientError, Result};
use serde::{Deserialize, Serialize};

pub mod proto {
    tonic::include_proto!("blog");
}

#[derive(Debug, Clone)]
pub enum Transport {
    Http(String),
    Grpc(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthResponse {
    pub token: String,
    pub user: User,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    pub username: String,
    pub email: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Post {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    pub title: String,
    pub content: String,
    pub author_id: i64,
    #[serde(default, deserialize_with = "parse_datetime")]
    pub created_at: i64,
    #[serde(default, deserialize_with = "parse_datetime")]
    pub updated_at: i64,
}

fn parse_datetime<'de, D>(deserializer: D) -> std::result::Result<i64, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::Deserialize;
    let s = String::deserialize(deserializer).map_err(serde::de::Error::custom)?;
    // Пробуем парсить как ISO 8601 формат
    if let Ok(dt) = chrono::DateTime::parse_from_rfc3339(&s) {
        Ok(dt.timestamp())
    } else if let Ok(ts) = s.parse::<i64>() {
        Ok(ts)
    } else {
        Ok(0)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pagination {
    pub limit: i64,
    pub offset: i64,
}

impl Default for Pagination {
    fn default() -> Self {
        Self { limit: 10, offset: 0 }
    }
}

pub struct BlogClient {
    transport: Transport,
    token: Option<String>,
}

impl BlogClient {
    pub fn new(transport: Transport) -> Self {
        Self {
            transport,
            token: None,
        }
    }

    pub fn set_token(&mut self, token: String) {
        self.token = Some(token);
    }

    pub fn get_token(&self) -> Option<&str> {
        self.token.as_deref()
    }

    pub fn clear_token(&mut self) {
        self.token = None;
    }

    pub async fn register(
        &mut self,
        username: String,
        email: String,
        password: String,
    ) -> Result<AuthResponse> {
        match &self.transport {
            Transport::Http(url) => {
                use crate::http_client::HttpClient;
                let client = HttpClient::new(url.clone());
                let response: AuthResponse = client
                    .post("/api/auth/register", &serde_json::json!({
                        "username": username,
                        "email": email,
                        "password": password
                    }))
                    .await?;
                self.token = Some(response.token.clone());
                Ok(response)
            }
            Transport::Grpc(addr) => {
                use crate::grpc_client::GrpcClient;
                let mut client = GrpcClient::new(addr.clone()).await?;
                let response = client.register(username, email, password).await?;
                let auth_response = AuthResponse {
                    token: response.token.clone(),
                    user: User {
                        id: None,
                        username: response.user.as_ref().map(|u| u.username.clone()).unwrap_or_default(),
                        email: response.user.as_ref().map(|u| u.email.clone()).unwrap_or_default(),
                    },
                };
                self.token = Some(response.token);
                Ok(auth_response)
            }
        }
    }

    pub async fn login(&mut self, username: String, password: String) -> Result<AuthResponse> {
        match &self.transport {
            Transport::Http(url) => {
                use crate::http_client::HttpClient;
                let client = HttpClient::new(url.clone());
                let response: AuthResponse = client
                    .post("/api/auth/login", &serde_json::json!({
                        "username": username,
                        "password": password
                    }))
                    .await?;
                self.token = Some(response.token.clone());
                Ok(response)
            }
            Transport::Grpc(addr) => {
                use crate::grpc_client::GrpcClient;
                let mut client = GrpcClient::new(addr.clone()).await?;
                let response = client.login(username, password).await?;
                let auth_response = AuthResponse {
                    token: response.token.clone(),
                    user: User {
                        id: None,
                        username: response.user.as_ref().map(|u| u.username.clone()).unwrap_or_default(),
                        email: response.user.as_ref().map(|u| u.email.clone()).unwrap_or_default(),
                    },
                };
                self.token = Some(response.token);
                Ok(auth_response)
            }
        }
    }

    pub async fn create_post(&self, title: String, content: String) -> Result<Post> {
        let token = self.token.as_ref().ok_or(BlogClientError::Unauthorized)?;

        match &self.transport {
            Transport::Http(url) => {
                use crate::http_client::HttpClient;
                let client = HttpClient::new(url.clone());
                let response: Post = client
                    .post_with_auth("/api/posts", &serde_json::json!({
                        "title": title,
                        "content": content
                    }), token)
                    .await?;
                Ok(response)
            }
            Transport::Grpc(addr) => {
                use crate::grpc_client::GrpcClient;
                let mut client = GrpcClient::new(addr.clone()).await?;
                let response = client.create_post(token, title, content).await?;
                Ok(Post {
                    id: Some(response.id),
                    title: response.title,
                    content: response.content,
                    author_id: response.author_id,
                    created_at: response.created_at,
                    updated_at: response.updated_at,
                })
            }
        }
    }

    pub async fn get_post(&self, id: i64) -> Result<Post> {
        match &self.transport {
            Transport::Http(url) => {
                use crate::http_client::HttpClient;
                let client = HttpClient::new(url.clone());
                let response: Post = client
                    .get(&format!("/api/posts/{}", id))
                    .await?;
                Ok(response)
            }
            Transport::Grpc(addr) => {
                use crate::grpc_client::GrpcClient;
                let mut client = GrpcClient::new(addr.clone()).await?;
                let response = client.get_post(id).await?;
                Ok(Post {
                    id: Some(response.id),
                    title: response.title,
                    content: response.content,
                    author_id: response.author_id,
                    created_at: response.created_at,
                    updated_at: response.updated_at,
                })
            }
        }
    }

    pub async fn update_post(&self, id: i64, title: String, content: String) -> Result<Post> {
        let token = self.token.as_ref().ok_or(BlogClientError::Unauthorized)?;

        match &self.transport {
            Transport::Http(url) => {
                use crate::http_client::HttpClient;
                let client = HttpClient::new(url.clone());
                let response: Post = client
                    .put_with_auth(
                        &format!("/api/posts/{}", id),
                        &serde_json::json!({
                            "title": title,
                            "content": content
                        }),
                        token,
                    )
                    .await?;
                Ok(response)
            }
            Transport::Grpc(addr) => {
                use crate::grpc_client::GrpcClient;
                let mut client = GrpcClient::new(addr.clone()).await?;
                let response = client.update_post(token, id, title, content).await?;
                Ok(Post {
                    id: Some(response.id),
                    title: response.title,
                    content: response.content,
                    author_id: response.author_id,
                    created_at: response.created_at,
                    updated_at: response.updated_at,
                })
            }
        }
    }

    pub async fn delete_post(&self, id: i64) -> Result<()> {
        let token = self.token.as_ref().ok_or(BlogClientError::Unauthorized)?;

        match &self.transport {
            Transport::Http(url) => {
                use crate::http_client::HttpClient;
                let client = HttpClient::new(url.clone());
                client
                    .delete_with_auth(&format!("/api/posts/{}", id), token)
                    .await?;
                Ok(())
            }
            Transport::Grpc(addr) => {
                use crate::grpc_client::GrpcClient;
                let mut client = GrpcClient::new(addr.clone()).await?;
                client.delete_post(token, id).await?;
                Ok(())
            }
        }
    }

    pub async fn list_posts(&self, limit: i64, offset: i64) -> Result<Vec<Post>> {
        match &self.transport {
            Transport::Http(url) => {
                use crate::http_client::HttpClient;
                let client = HttpClient::new(url.clone());
                #[derive(Deserialize)]
                struct PostResponse {
                    posts: Vec<Post>,
                }
                let response: PostResponse = client
                    .get(&format!("/api/posts?limit={}&offset={}", limit, offset))
                    .await?;
                Ok(response.posts)
            }
            Transport::Grpc(addr) => {
                use crate::grpc_client::GrpcClient;
                let mut client = GrpcClient::new(addr.clone()).await?;
                let response = client.list_posts(limit, offset).await?;
                Ok(response
                    .posts
                    .into_iter()
                    .map(|p| Post {
                        id: Some(p.id),
                        title: p.title,
                        content: p.content,
                        author_id: p.author_id,
                        created_at: p.created_at,
                        updated_at: p.updated_at,
                    })
                    .collect())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transport_clone() {
        let http = Transport::Http("http://localhost:8080".to_string());
        let grpc = Transport::Grpc("http://localhost:9090".to_string());

        assert!(matches!(http, Transport::Http(_)));
        assert!(matches!(grpc, Transport::Grpc(_)));
    }

    #[tokio::test]
    async fn test_client_creation() {
        let client = BlogClient::new(Transport::Http("http://localhost:8080".to_string()));
        assert!(client.get_token().is_none());
    }
}
