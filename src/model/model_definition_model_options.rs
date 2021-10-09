use serde::{Deserialize, Serialize};

/// ModelDefinitionModelOptions : [Output-only, Beta] Model options used for the first training run. These options are immutable for subsequent training runs. Default values are used for any options not specified in the input query.

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModelDefinitionModelOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loss_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_type: Option<String>,
}
