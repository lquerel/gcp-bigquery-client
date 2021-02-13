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
    use std::env;

    pub const PROJECT_ID: &str = "XXX";
    pub const DATASET_ID: &str = "test_ds";
    pub const TABLE_ID: &str = "test_table";
    pub const SA_KEY: &str = "XXX";

    pub fn env_vars() -> (String, String, String, String) {
        let project_id = env::var("PROJECT_ID").expect("PROJECT_ID env var not defined");
        let dataset_id = env::var("DATASET_ID").expect("DATASET_ID env var not defined");
        let table_id = env::var("TABLE_ID").expect("TABLE_ID env var not defined");
        let gcp_sa_key = env::var("GCP_SA_KEY").expect("GCP_SA_KEY env var not defined");

        (project_id, dataset_id, table_id, gcp_sa_key)
    }
}
