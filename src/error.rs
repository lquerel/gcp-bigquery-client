#[derive(thiserror::Error, Debug)]
pub enum BQError {
    #[error("Invalid service account key")]
    InvalidServiceAccountKey(#[from] std::io::Error),

    #[error("Invalid service account authenticator")]
    InvalidServiceAccountAuthenticator(std::io::Error),

    #[error("Authentication error")]
    AuthError(#[from] yup_oauth2::error::AuthError),

    #[error("Authentication error")]
    YupAuthError(#[from] yup_oauth2::Error),

    #[error("Request error")]
    RequestError(#[from] reqwest::Error),

    #[error("Response error")]
    ResponseError { error: serde_json::Value },

    #[error("No data available. The result set is positioned before the first or after the last row. Try to call the method next on your result set.")]
    NoDataAvailable,

    #[error("Invalid column index")]
    InvalidColumnIndex { col_index: usize },

    #[error("Invalid column name")]
    InvalidColumnName { col_name: String },

    #[error("Data conversion error")]
    InvalidColumnType {
        col_index: usize,
        column_type: String,
        type_requested: String,
    },

    #[error("Serialization error")]
    SerializationError(#[from] serde_json::Error),
}
