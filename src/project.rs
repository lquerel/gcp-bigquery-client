//! There is no persistent data associated with this resource.
use std::sync::Arc;

use reqwest::Client;

use crate::auth::Authenticator;
use crate::error::BQError;
use crate::model::get_service_account_response::GetServiceAccountResponse;
use crate::model::project_list::ProjectList;
use crate::{process_response, urlencode, BIG_QUERY_V2_URL};

/// A project API handler.
#[derive(Clone)]
pub struct ProjectApi {
    client: Client,
    auth: Arc<dyn Authenticator>,
    base_url: String,
}

impl ProjectApi {
    pub(crate) fn new(client: Client, auth: Arc<dyn Authenticator>) -> Self {
        Self {
            client,
            sa_auth,
            base_url: BIG_QUERY_V2_URL.to_string(),
        }
    }

    pub(crate) fn with_base_url(&mut self, base_url: String) -> &mut Self {
        self.base_url = base_url;
        self
    }

    /// RPC to get the service account for a project used for interactions with Google Cloud KMS.
    /// # Arguments
    /// * `project_id`- ID of the project
    pub async fn get_service_account(&self, project_id: &str) -> Result<GetServiceAccountResponse, BQError> {
        let req_url = &format!(
            "{base_url}/projects/{project_id}/serviceAccount",
            base_url = self.base_url,
            project_id = urlencode(project_id),
        );

        let access_token = self.auth.access_token().await?;

        let request = self.client.get(req_url).bearer_auth(access_token).build()?;
        let response = self.client.execute(request).await?;

        process_response(response).await
    }

    /// RPC to list projects to which the user has been granted any project role.
    ///
    /// Users of this method are encouraged to consider the Resource Manager API, which provides
    /// the underlying data for this method and has more capabilities.
    /// # Arguments
    /// * `options` - Get options.
    pub async fn list(&self, options: GetOptions) -> Result<ProjectList, BQError> {
        let req_url = format!("{base_url}/projects", base_url = self.base_url);

        let access_token = self.auth.access_token().await?;

        let request = self
            .client
            .get(req_url)
            .bearer_auth(access_token)
            .query(&options)
            .build()?;

        let resp = self.client.execute(request).await?;

        process_response(resp).await
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetOptions {
    max_results: Option<u64>,
    page_token: Option<String>,
}

impl GetOptions {
    /// The maximum number of results to return in a single response page. Leverage the page tokens
    /// to iterate through the entire collection.
    pub fn max_results(mut self, value: u64) -> Self {
        self.max_results = Some(value);
        self
    }

    /// Page token, returned by a previous call, to request the next page of results
    pub fn page_token(mut self, value: String) -> Self {
        self.page_token = Some(value);
        self
    }
}
