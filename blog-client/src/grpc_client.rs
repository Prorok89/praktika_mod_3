#[cfg(not(target_arch = "wasm32"))]
use crate::error::{BlogClientError, Result};
#[cfg(not(target_arch = "wasm32"))]
use tonic::transport::Channel;
#[cfg(not(target_arch = "wasm32"))]
use tonic::Request;

#[cfg(not(target_arch = "wasm32"))]
pub struct GrpcClient {
    inner: crate::proto::blog_service_client::BlogServiceClient<Channel>,
}

#[cfg(not(target_arch = "wasm32"))]
impl GrpcClient {
    pub async fn new(addr: String) -> Result<Self> {
        let channel = Channel::from_shared(addr).map_err(|e| {
            BlogClientError::InvalidRequest(format!("Invalid gRPC address: {}", e))
        })?;
        let inner = crate::proto::blog_service_client::BlogServiceClient::connect(channel).await?;
        Ok(Self { inner })
    }

    pub fn with_channel(channel: Channel) -> Self {
        Self {
            inner: crate::proto::blog_service_client::BlogServiceClient::new(channel),
        }
    }

    pub fn set_metadata_token(&mut self, _token: &str) {
    }

    pub async fn register(
        &mut self,
        username: String,
        email: String,
        password: String,
    ) -> Result<crate::proto::AuthResponse> {
        let request = Request::new(crate::proto::RegisterRequest {
            username,
            email,
            password,
        });

        let response = self.inner.register(request).await?;
        Ok(response.into_inner())
    }

    pub async fn login(
        &mut self,
        username: String,
        password: String,
    ) -> Result<crate::proto::AuthResponse> {
        let request = Request::new(crate::proto::LoginRequest {
            username,
            password,
        });

        let response = self.inner.login(request).await?;
        Ok(response.into_inner())
    }

    pub async fn create_post(
        &mut self,
        token: &str,
        title: String,
        content: String,
    ) -> Result<crate::proto::Post> {
        use tonic::metadata::MetadataValue;
        let mut request = Request::new(crate::proto::CreatePostRequest {
            id: 0,
            title,
            content,
        });

        let auth_header = format!("Bearer {}", token);
        let val = MetadataValue::try_from(auth_header)
            .map_err(|e| BlogClientError::InternalError(e.to_string()))?;
        request.metadata_mut().insert("authorization", val);

        let response = self.inner.create_post(request).await?;
        Ok(response.into_inner())
    }

    pub async fn get_post(&mut self, id: i64) -> Result<crate::proto::Post> {
        let request = Request::new(crate::proto::PostId { id });
        let response = self.inner.get_post(request).await?;
        Ok(response.into_inner())
    }

    pub async fn update_post(
        &mut self,
        token: &str,
        id: i64,
        title: String,
        content: String,
    ) -> Result<crate::proto::Post> {
        use tonic::metadata::MetadataValue;
        let mut request = Request::new(crate::proto::CreatePostRequest {
            id,
            title,
            content,
        });

        let auth_header = format!("Bearer {}", token);
        let val = MetadataValue::try_from(auth_header)
            .map_err(|e| BlogClientError::InternalError(e.to_string()))?;
        request.metadata_mut().insert("authorization", val);

        let response = self.inner.update_post(request).await?;
        Ok(response.into_inner())
    }

    pub async fn delete_post(&mut self, token: &str, id: i64) -> Result<crate::proto::EmptyResponse> {
        use tonic::metadata::MetadataValue;
        let mut request = Request::new(crate::proto::PostId { id });

        let auth_header = format!("Bearer {}", token);
        let val = MetadataValue::try_from(auth_header)
            .map_err(|e| BlogClientError::InternalError(e.to_string()))?;
        request.metadata_mut().insert("authorization", val);

        let response = self.inner.delete_post(request).await?;
        Ok(response.into_inner())
    }

    pub async fn list_posts(
        &mut self,
        limit: i64,
        offset: i64,
    ) -> Result<crate::proto::PostResponse> {
        let request = Request::new(crate::proto::ListParams { limit, offset });
        let response = self.inner.list_posts(request).await?;
        Ok(response.into_inner())
    }
}
