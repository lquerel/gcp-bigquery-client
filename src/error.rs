//! List of the BigQuery errors supported by this crate.

use std::collections::HashMap;
use tokio::sync::AcquireError;
use tonic::{metadata::errors::InvalidMetadataValue, Status};

#[allow(clippy::upper_case_acronyms)]
#[derive(thiserror::Error, Debug)]
pub enum BQError {
    #[error("Invalid service account key (error: {0})")]
    InvalidServiceAccountKey(#[from] std::io::Error),

    #[error("Invalid service account authenticator (error: {0})")]
    InvalidServiceAccountAuthenticator(std::io::Error),

    #[error("Invalid installed flow authenticator (error: {0})")]
    InvalidInstalledFlowAuthenticator(std::io::Error),

    #[error("Invalid installed application default credentials authenticator (error: {0})")]
    InvalidApplicationDefaultCredentialsAuthenticator(std::io::Error),

    #[error("Invalid authorized user authenticator (error: {0})")]
    InvalidAuthorizedUserAuthenticator(std::io::Error),

    #[error("Authentication error (error: {0})")]
    AuthError(#[from] yup_oauth2::error::AuthError),

    #[error("Authentication error (error: {0})")]
    YupAuthError(#[from] yup_oauth2::Error),

    #[error("No token")]
    NoToken,

    #[error("Request error (error: {0})")]
    RequestError(#[from] reqwest::Error),

    #[error("Response error (error: {error:?})")]
    ResponseError { error: ResponseError },

    #[error("No data available. The result set is positioned before the first or after the last row. Try to call the method next on your result set.")]
    NoDataAvailable,

    #[error("Invalid column index (col_index: {col_index})")]
    InvalidColumnIndex { col_index: usize },

    #[error("Invalid column name (col_name: {col_name})")]
    InvalidColumnName { col_name: String },

    #[error("Invalid column type (col_index: {col_index}, col_type: {col_type}, type_requested: {type_requested})")]
    InvalidColumnType {
        col_index: usize,
        col_type: String,
        type_requested: String,
    },

    #[error("Json serialization error (error: {0})")]
    SerializationError(#[from] serde_json::Error),

    #[error("Tonic transport error (error: {0}")]
    TonicTransportError(#[from] tonic::transport::Error),

    #[error("Tonic invalid metadata value error (error: {0}")]
    TonicInvalidMetadataValueError(#[from] InvalidMetadataValue),

    #[error("Tonic status error (error: {0}")]
    TonicStatusError(#[from] Status),

    #[error("Failed to acquire semaphore permit (error: {0})")]
    SemaphorePermitError(#[from] AcquireError),

    #[error("Failed to join tokio task (error: {0})")]
    TokioTaskError(#[from] tokio::task::JoinError),

    #[error("Connection pool error (error: {0})")]
    ConnectionPoolError(String),
}

#[derive(Debug, Deserialize)]
pub struct ResponseError {
    pub error: NestedResponseError,
}

#[derive(Debug, Deserialize)]
pub struct NestedResponseError {
    pub code: i64,
    pub errors: Vec<HashMap<String, String>>,
    pub message: String,
    #[serde(default)]
    pub status: String,
}
