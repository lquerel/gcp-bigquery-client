use crate::model::bqml_training_run::BqmlTrainingRun;
use crate::model::model_definition_model_options::ModelDefinitionModelOptions;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModelDefinition {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_options: Option<ModelDefinitionModelOptions>,
    /// [Output-only, Beta] Information about ml training runs, each training run comprises of multiple iterations and there may be multiple training runs for the model if warm start is used or if a user decides to continue a previously cancelled query.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub training_runs: Option<Vec<BqmlTrainingRun>>,
}
