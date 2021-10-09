use crate::model::model_reference::ModelReference;
use crate::model::table_reference::TableReference;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JobConfigurationExtract {
    /// [Optional] The compression type to use for exported files. Possible values include GZIP, DEFLATE, SNAPPY, and NONE. The default value is NONE. DEFLATE and SNAPPY are only supported for Avro. Not applicable when extracting models.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compression: Option<String>,
    /// [Optional] The exported file format. Possible values include CSV, NEWLINE_DELIMITED_JSON, PARQUET or AVRO for tables and ML_TF_SAVED_MODEL or ML_XGBOOST_BOOSTER for models. The default value for tables is CSV. Tables with nested or repeated fields cannot be exported as CSV. The default value for models is ML_TF_SAVED_MODEL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_format: Option<String>,
    /// [Pick one] DEPRECATED: Use destinationUris instead, passing only one URI as necessary. The fully-qualified Google Cloud Storage URI where the extracted table should be written.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_uri: Option<String>,
    /// [Pick one] A list of fully-qualified Google Cloud Storage URIs where the extracted table should be written.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_uris: Option<Vec<String>>,
    /// [Optional] Delimiter to use between fields in the exported data. Default is ','. Not applicable when extracting models.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_delimiter: Option<String>,
    /// [Optional] Whether to print out a header row in the results. Default is true. Not applicable when extracting models.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub print_header: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_model: Option<ModelReference>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_table: Option<TableReference>,
    /// [Optional] If destinationFormat is set to \"AVRO\", this flag indicates whether to enable extracting applicable column types (such as TIMESTAMP) to their corresponding AVRO logical types (timestamp-micros), instead of only using their raw types (avro-long). Not applicable when extracting models.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_avro_logical_types: Option<bool>,
}
