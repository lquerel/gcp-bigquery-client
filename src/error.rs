#[derive(thiserror::Error, Debug)]
pub enum BQError {
    #[error("Invalid service account key (error: {0})")]
    InvalidServiceAccountKey(#[from] std::io::Error),

    #[error("Invalid service account authenticator (error: {0})")]
    InvalidServiceAccountAuthenticator(std::io::Error),

    #[error("Authentication error (error: {0})")]
    AuthError(#[from] yup_oauth2::error::AuthError),

    #[error("Authentication error (error: {0})")]
    YupAuthError(#[from] yup_oauth2::Error),

    #[error("Request error (error: {0})")]
    RequestError(#[from] reqwest::Error),

    #[error("Response error (error: {error:?})")]
    ResponseError { error: serde_json::Value },

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
}
