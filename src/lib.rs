use serde::Deserialize;
use reqwest::Response;
use crate::error::BQError;

pub mod auth;
pub mod error;
pub mod dataset;
pub mod client;
pub mod job;
pub mod model;
pub mod tabledata;
pub mod table;

pub fn urlencode<T: AsRef<str>>(s: T) -> String {
    url::form_urlencoded::byte_serialize(s.as_ref().as_bytes()).collect()
}

async fn process_response<T: for<'de> Deserialize<'de>>(resp: Response) -> Result<T, BQError> {
    if resp.status().is_success() {
        Ok(resp.json().await?)
    } else {
        Err(BQError::ResponseError { error: resp.json().await? })
    }
}

#[cfg(test)]
pub mod tests {
    use crate::auth::service_account_authenticator;

    pub const PROJECT_ID: &str = "XXX";
    pub const DATASET_ID: &str = "test_ds";
    pub const TABLE_ID: &str = "test_table";
    pub const SA_KEY: &str = "XXX";
}
