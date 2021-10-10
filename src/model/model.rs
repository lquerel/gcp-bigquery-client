use crate::model::encryption_configuration::EncryptionConfiguration;
use crate::model::model_reference::ModelReference;
use crate::model::standard_sql_field::StandardSqlField;
use crate::model::training_run::TrainingRun;
use std::collections::HashMap;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Model {
    /// Optional. A user-friendly description of this model.
    pub description: Option<String>,
    /// Output only. The geographic location where the model resides. This value is inherited from the dataset.
    pub location: Option<String>,
    /// Optional. The time when this model expires, in milliseconds since the epoch. If not present, the model will persist indefinitely. Expired models will be deleted and their storage reclaimed. The defaultTableExpirationMs property of the encapsulating dataset can be used to set a default expirationTime on newly created models.
    pub expiration_time: Option<i64>,
    /// Output only. Information for all training runs in increasing order of start_time.
    pub training_runs: Option<Vec<TrainingRun>>,
    /// Output only. A hash of this resource.
    pub etag: Option<String>,
    /// Output only. The time when this model was created, in millisecs since the epoch.
    pub creation_time: Option<i64>,
    /// Output only. The time when this model was last modified, in millisecs since the epoch.
    pub last_modified_time: Option<i64>,
    /// Output only. Input feature columns that were used to train this model.
    pub feature_columns: Option<Vec<StandardSqlField>>,
    /// Required. Unique identifier for this model.
    pub model_reference: ModelReference,
    /// Output only. Label columns that were used to train this model. The output of the model will have a "predicted_" prefix to these columns.
    pub label_columns: Option<Vec<StandardSqlField>>,
    /// Custom encryption configuration (e.g., Cloud KMS keys). This shows the encryption configuration of the model data while stored in BigQuery storage. This field can be used with PatchModel to update encryption key for an already encrypted model.
    pub encryption_configuration: Option<EncryptionConfiguration>,
    /// Optional. A descriptive name for this model.
    pub friendly_name: Option<String>,
    /// The labels associated with this model. You can use these to organize and group your models. Label keys and values can be no longer than 63 characters, can only contain lowercase letters, numeric characters, underscores and dashes. International characters are allowed. Label values are optional. Label keys must start with a letter and each label in the list must have a different key.
    pub labels: Option<HashMap<String, String>>,
    /// Output only. Type of the model resource.
    pub model_type: Option<ModelType>,
}

/// Output only. Type of the model resource.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ModelType {
    ///
    ModelTypeUnspecified,
    /// Linear regression model.
    LinearRegression,
    /// Logistic regression based classification model.
    LogisticRegression,
    /// K-means clustering model.
    Kmean,
    /// Matrix factorization model.
    MatrixFactorization,
    /// DNN classifier model.
    DnnClassifier,
    /// An imported TensorFlow model.
    Tensorflow,
    /// DNN regressor model.
    DnnRegressor,
    /// Boosted tree regressor model.
    BoostedTreeRegressor,
    /// Boosted tree classifier model.
    BoostedTreeClassifier,
    /// ARIMA model.
    Arima,
    /// [Beta] AutoML Tables regression model.
    AutomlRegressor,
    /// [Beta] AutoML Tables classification model.
    AutomlClassifier,
}
