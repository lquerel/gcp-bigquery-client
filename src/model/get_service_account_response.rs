#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetServiceAccountResponse {
    /// The resource type of the response.
    pub kind: Option<String>,
    /// The service account email address.
    pub email: Option<String>,
}
