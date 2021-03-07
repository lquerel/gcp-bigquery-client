#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetServiceAccountResponse {
    /// The service account email address.
    pub email: Option<String>,
    /// The resource type of the response.
    pub kind: Option<String>,
}
