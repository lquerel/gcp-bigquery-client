#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Schemata {
    /// The name of the project that contains the dataset
    pub catalog_name: String,
    /// The dataset's name also referred to as the datasetId
    pub schema_name: String,
    /// The value is always NULL
    pub schema_owner: Option<String>,
    /// The dataset's creation time
    pub creation_time: String, // ToDo should a TIMESTAMP
    /// The dataset's last modified time
    pub last_modified_time: String, // ToDo should a TIMESTAMP
    /// The dataset's geographic location
    pub location: String,
}
