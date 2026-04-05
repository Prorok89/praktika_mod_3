use thiserror::Error;

#[derive(Error, Debug)]
pub enum BlogClientError {
    #[error("HTTP error: {0}")]
    HttpError(#[from] reqwest::Error),

    #[error("gRPC error: {0}")]
    GrpcError(#[from] tonic::Status),

    #[error("Transport error: {0}")]
    TransportError(#[from] tonic::transport::Error),

    #[error("Resource not found: {0}")]
    NotFound(String),

    #[error("Unauthorized")]
    Unauthorized,
	
    #[error("Invalid request: {0}")]
    InvalidRequest(String),

    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),

    #[error("Internal error: {0}")]
    InternalError(String),
}

pub type Result<T> = std::result::Result<T, BlogClientError>;
