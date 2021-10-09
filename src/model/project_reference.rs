#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectReference {
    /// [Required] ID of the project. Can be either the numeric ID or the assigned ID of the project.
    pub project_id: String,
}
