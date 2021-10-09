//! Options used in model training.
use crate::model::arima_order::ArimaOrder;
use std::collections::HashMap;

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrainingOptions {
    /// Whether to train a model from the last checkpoint.
    pub warm_start: Option<bool>,
    /// L1 regularization coefficient.
    pub l_1_regularization: Option<f64>,
    /// Name of input label columns in training data.
    pub input_label_columns: Option<Vec<String>>,
    /// Feedback type that specifies which algorithm to run for matrix factorization.
    pub feedback_type: Option<FeedbackType>,
    /// Distance type for clustering models.
    pub distance_type: Option<DistanceType>,
    /// Learning rate in training. Used only for iterative training algorithms.
    pub learn_rate: Option<f64>,
    /// Optimization strategy for training linear regression models.
    pub optimization_strategy: Option<OptimizationStrategy>,
    /// The data split type for training and evaluation, e.g. RANDOM.
    pub data_split_method: Option<DataSplitMethod>,
    /// Item column specified for matrix factorization models.
    pub item_column: Option<String>,
    /// The fraction of evaluation data over the whole input data. The rest of data will be used as training data. The format should be double. Accurate to two decimal places. Default value is 0.2.
    pub data_split_eval_fraction: Option<f64>,
    /// Hidden units for dnn models.
    pub hidden_units: Option<Vec<i64>>,
    /// Number of clusters for clustering models.
    pub num_clusters: Option<i64>,
    /// Num factors specified for matrix factorization models.
    pub num_factors: Option<i64>,
    /// Specifies the initial learning rate for the line search learn rate strategy.
    pub initial_learn_rate: Option<f64>,
    /// Type of loss function used during training run.
    pub loss_type: Option<LossType>,
    /// When early_stop is true, stops training when accuracy improvement is less than 'min_relative_progress'. Used only for iterative training algorithms.
    pub min_relative_progress: Option<f64>,
    /// Dropout probability for dnn models.
    pub dropout: Option<f64>,
    /// The number of periods ahead that need to be forecasted.
    pub horizon: Option<i64>,
    /// Google Cloud Storage URI from which the model was imported. Only applicable for imported models.
    pub model_uri: Option<String>,
    /// Minimum split loss for boosted tree models.
    pub min_split_loss: Option<f64>,
    /// Batch size for dnn models.
    pub batch_size: Option<i64>,
    /// Column to be designated as time series timestamp for ARIMA model.
    pub time_series_timestamp_column: Option<String>,
    /// Whether to enable auto ARIMA or not.
    pub auto_arima: Option<bool>,
    /// Hyperparameter for matrix factoration when implicit feedback type is specified.
    pub wals_alpha: Option<f64>,
    /// The column used to provide the initial centroids for kmeans algorithm when kmeans_initialization_method is CUSTOM.
    pub kmeans_initialization_column: Option<String>,
    /// The maximum number of iterations in training. Used only for iterative training algorithms.
    pub max_iterations: Option<i64>,
    /// Whether to preserve the input structs in output feature names. Suppose there is a struct A with field b. When false (default), the output feature name is A_b. When true, the output feature name is A.b.
    pub preserve_input_structs: Option<bool>,
    /// Weights associated with each label class, for rebalancing the training data. Only applicable for classification models.
    pub label_class_weights: Option<HashMap<String, f64>>,
    /// The strategy to determine learn rate for the current iteration.
    pub learn_rate_strategy: Option<LearnRateStrategy>,
    /// The method used to initialize the centroids for kmeans algorithm.
    pub kmeans_initialization_method: Option<KmeansInitializationMethod>,
    /// User column specified for matrix factorization models.
    pub user_column: Option<String>,
    /// Subsample fraction of the training data to grow tree to prevent overfitting for boosted tree models.
    pub subsample: Option<f64>,
    /// L2 regularization coefficient.
    pub l_2_regularization: Option<f64>,
    /// The max value of non-seasonal p and q.
    pub auto_arima_max_order: Option<i64>,
    /// Maximum depth of a tree for boosted tree models.
    pub max_tree_depth: Option<i64>,
    /// Column to be designated as time series data for ARIMA model.
    pub time_series_data_column: Option<String>,
    /// The column to split data with. This column won't be used as a feature. 1. When data_split_method is CUSTOM, the corresponding column should be boolean. The rows with true value tag are eval data, and the false are training data. 2. When data_split_method is SEQ, the first DATA_SPLIT_EVAL_FRACTION rows (from smallest to largest) in the corresponding column are used as training data, and the rest are eval data. It respects the order in Orderable data types: https://cloud.google.com/bigquery/docs/reference/standard-sql/data-types#data-type-properties
    pub data_split_column: Option<String>,
    /// A specification of the non-seasonal part of the ARIMA model: the three components (p, d, q) are the AR order, the degree of differencing, and the MA order.
    pub non_seasonal_order: Option<ArimaOrder>,
    /// Include drift when fitting an ARIMA model.
    pub include_drift: Option<bool>,
    /// Whether to stop early when the loss doesn't improve significantly any more (compared to min_relative_progress). Used only for iterative training algorithms.
    pub early_stop: Option<bool>,
    /// The geographical region based on which the holidays are considered in time series modeling. If a valid value is specified, then holiday effects modeling is enabled.
    pub holiday_region: Option<HolidayRegion>,
    /// The data frequency of a time series.
    pub data_frequency: Option<DataFrequency>,
    /// The time series id column that was used during ARIMA model training.
    pub time_series_id_column: Option<String>,
}

/// Feedback type that specifies which algorithm to run for matrix factorization.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum FeedbackType {
    ///
    FeedbackTypeUnspecified,
    /// Use weighted-als for implicit feedback problems.
    Implicit,
    /// Use nonweighted-als for explicit feedback problems.
    Explicit,
}

/// Distance type for clustering models.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DistanceType {
    ///
    DistanceTypeUnspecified,
    /// Eculidean distance.
    Euclidean,
    /// Cosine distance.
    Cosine,
}

/// Optimization strategy for training linear regression models.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OptimizationStrategy {
    ///
    OptimizationStrategyUnspecified,
    /// Uses an iterative batch gradient descent algorithm.
    BatchGradientDescent,
    /// Uses a normal equation to solve linear regression problem.
    NormalEquation,
}

/// The data split type for training and evaluation, e.g. RANDOM.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DataSplitMethod {
    ///
    DataSplitMethodUnspecified,
    /// Splits data randomly.
    Random,
    /// Splits data with the user provided tags.
    Custom,
    /// Splits data sequentially.
    Sequential,
    /// Data split will be skipped.
    NoSplit,
    /// Splits data automatically: Uses NO_SPLIT if the data size is small. Otherwise uses RANDOM.
    AutoSplit,
}

/// Type of loss function used during training run.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LossType {
    ///
    LossTypeUnspecified,
    /// Mean squared loss, used for linear regression.
    MeanSquaredLoss,
    /// Mean log loss, used for logistic regression.
    MeanLogLoss,
}

/// The strategy to determine learn rate for the current iteration.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LearnRateStrategy {
    ///
    LearnRateStrategyUnspecified,
    /// Use line search to determine learning rate.
    LineSearch,
    /// Use a constant learning rate.
    Constant,
}

/// The method used to initialize the centroids for kmeans algorithm.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum KmeansInitializationMethod {
    /// Unspecified initialization method.
    KmeansInitializationMethodUnspecified,
    /// Initializes the centroids randomly.
    Random,
    /// Initializes the centroids using data specified in kmeans_initialization_column.
    Custom,
    /// Initializes with kmeans++.
    KmeansPlusPlu,
}

/// The geographical region based on which the holidays are considered in time series modeling. If a valid value is specified, then holiday effects modeling is enabled.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum HolidayRegion {
    /// Holiday region unspecified.
    HolidayRegionUnspecified,
    /// Global.
    Global,
    /// North America.
    Na,
    /// Japan and Asia Pacific: Korea, Greater China, India, Australia, and New Zealand.
    Japac,
    /// Europe, the Middle East and Africa.
    Emea,
    /// Latin America and the Caribbean.
    Lac,
    /// United Arab Emirates
    Ae,
    /// Argentina
    Ar,
    /// Austria
    At,
    /// Australia
    Au,
    /// Belgium
    Be,
    /// Brazil
    Br,
    /// Canada
    Ca,
    /// Switzerland
    Ch,
    /// Chile
    Cl,
    /// China
    Cn,
    /// Colombia
    Co,
    /// Czechoslovakia
    C,
    /// Czech Republic
    Cz,
    /// Germany
    De,
    /// Denmark
    Dk,
    /// Algeria
    Dz,
    /// Ecuador
    Ec,
    /// Estonia
    Ee,
    /// Egypt
    Eg,
    /// Spain
    E,
    /// Finland
    Fi,
    /// France
    Fr,
    /// Great Britain (United Kingdom)
    Gb,
    /// Greece
    Gr,
    /// Hong Kong
    Hk,
    /// Hungary
    Hu,
    /// Indonesia
    Id,
    /// Ireland
    Ie,
    /// Israel
    Il,
    /// India
    In,
    /// Iran
    Ir,
    /// Italy
    It,
    /// Japan
    Jp,
    /// Korea (South)
    Kr,
    /// Latvia
    Lv,
    /// Morocco
    Ma,
    /// Mexico
    Mx,
    /// Malaysia
    My,
    /// Nigeria
    Ng,
    /// Netherlands
    Nl,
    /// Norway
    No,
    /// New Zealand
    Nz,
    /// Peru
    Pe,
    /// Philippines
    Ph,
    /// Pakistan
    Pk,
    /// Poland
    Pl,
    /// Portugal
    Pt,
    /// Romania
    Ro,
    /// Serbia
    R,
    /// Russian Federation
    Ru,
    /// Saudi Arabia
    Sa,
    /// Sweden
    Se,
    /// Singapore
    Sg,
    /// Slovenia
    Si,
    /// Slovakia
    Sk,
    /// Thailand
    Th,
    /// Turkey
    Tr,
    /// Taiwan
    Tw,
    /// Ukraine
    Ua,
    /// United States
    U,
    /// Venezuela
    Ve,
    /// Viet Nam
    Vn,
    /// South Africa
    Za,
}

/// The data frequency of a time series.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DataFrequency {
    ///
    DataFrequencyUnspecified,
    /// Automatically inferred from timestamps.
    AutoFrequency,
    /// Yearly data.
    Yearly,
    /// Quarterly data.
    Quarterly,
    /// Monthly data.
    Monthly,
    /// Weekly data.
    Weekly,
    /// Daily data.
    Daily,
    /// Hourly data.
    Hourly,
    /// Per-minute data.
    PerMinute,
}
