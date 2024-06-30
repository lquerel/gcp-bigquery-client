//! Manage BigQuery dataset.
use std::sync::Arc;

use tonic::transport::{Channel, ClientTlsConfig, Error};

use crate::{
    auth::Authenticator, google::cloud::bigquery::storage::v1::big_query_write_client::BigQueryWriteClient,
    BIG_QUERY_V2_URL,
};

static BIG_QUERY_STORAGE_API_URL: &str = "https://bigquerystorage.googleapis.com";
static BIGQUERY_STORAGE_API_DOMAIN: &str = "bigquerystorage.googleapis.com";

/// A dataset API handler.
#[derive(Clone)]
pub struct StorageApi {
    write_client: BigQueryWriteClient<Channel>,
    auth: Arc<dyn Authenticator>,
    base_url: String,
}

impl StorageApi {
    pub(crate) fn new(write_client: BigQueryWriteClient<Channel>, auth: Arc<dyn Authenticator>) -> Self {
        Self {
            write_client,
            auth,
            base_url: BIG_QUERY_V2_URL.to_string(),
        }
    }

    pub(crate) async fn new_write_client() -> Result<BigQueryWriteClient<Channel>, Error> {
        let tls_config = ClientTlsConfig::new().domain_name(BIGQUERY_STORAGE_API_DOMAIN);
        let channel = Channel::from_static(BIG_QUERY_STORAGE_API_URL)
            .tls_config(tls_config)?
            .connect()
            .await?;
        let write_client = BigQueryWriteClient::new(channel);

        Ok(write_client)
    }

    pub(crate) fn with_base_url(&mut self, base_url: String) -> &mut Self {
        self.base_url = base_url;
        self
    }
}
