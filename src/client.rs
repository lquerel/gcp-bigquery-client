use crate::auth::service_account_authenticator;
use crate::job::JobApi;
use crate::dataset::DatasetApi;
use crate::tabledata::TableDataApi;
use crate::table::TableApi;

pub struct Client {
    dataset_api: DatasetApi,
    table_api: TableApi,
    job_api: JobApi,
    tabledata_api: TableDataApi,
}

impl Client {
    pub async fn new(sa_key_file: &str) -> Self {
        let scopes = vec!["https://www.googleapis.com/auth/bigquery"];
        let sa_auth = service_account_authenticator(scopes, sa_key_file).await.expect("expecting a valid key");

        let access_token = sa_auth.access_token().await.expect("expecting a valid token");
        let client = reqwest::Client::new();
        Self {
            dataset_api: DatasetApi::new(client.clone(), access_token.clone()),
            table_api: TableApi::new(client.clone(), access_token.clone()),
            job_api: JobApi::new(client.clone(), access_token.clone()),
            tabledata_api: TableDataApi::new(client.clone(), access_token.clone()),
        }
    }

    pub fn dataset(&self) -> &DatasetApi {
        &self.dataset_api
    }

    pub fn table(&self) -> &TableApi {
        &self.table_api
    }

    pub fn job(&self) -> &JobApi {
        &self.job_api
    }

    pub fn tabledata(&self) -> &TableDataApi {
        &self.tabledata_api
    }
}