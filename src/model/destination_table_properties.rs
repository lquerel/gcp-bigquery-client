use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinationTableProperties {
    /// [Optional] The description for the destination table. This will only be used if the destination table is newly created. If the table already exists and a value different than the current description is provided, the job will fail.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// [Optional] The friendly name for the destination table. This will only be used if the destination table is newly created. If the table already exists and a value different than the current friendly name is provided, the job will fail.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
    /// [Optional] The labels associated with this table. You can use these to organize and group your tables. This will only be used if the destination table is newly created. If the table already exists and labels are different than the current labels are provided, the job will fail.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<::std::collections::HashMap<String, String>>,
}
