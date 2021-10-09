use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BigtableColumn {
    /// [Optional] The encoding of the values when the type is not STRING. Acceptable encoding values are: TEXT - indicates values are alphanumeric text strings. BINARY - indicates values are encoded using HBase Bytes.toBytes family of functions. 'encoding' can also be set at the column family level. However, the setting at this level takes precedence if 'encoding' is set at both levels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding: Option<String>,
    /// [Optional] If the qualifier is not a valid BigQuery field identifier i.e. does not match [a-zA-Z][a-zA-Z0-9_]*, a valid identifier must be provided as the column field name and is used as field name in queries.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_name: Option<String>,
    /// [Optional] If this is set, only the latest version of value in this column are exposed. 'onlyReadLatest' can also be set at the column family level. However, the setting at this level takes precedence if 'onlyReadLatest' is set at both levels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub only_read_latest: Option<bool>,
    /// [Required] Qualifier of the column. Columns in the parent column family that has this exact qualifier are exposed as . field. If the qualifier is valid UTF-8 string, it can be specified in the qualifier_string field. Otherwise, a base-64 encoded value must be set to qualifier_encoded. The column field name is the same as the column qualifier. However, if the qualifier is not a valid BigQuery field identifier i.e. does not match [a-zA-Z][a-zA-Z0-9_]*, a valid identifier must be provided as field_name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qualifier_encoded: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qualifier_string: Option<String>,
    /// [Optional] The type to convert the value in cells of this column. The values are expected to be encoded using HBase Bytes.toBytes function when using the BINARY encoding value. Following BigQuery types are allowed (case-sensitive) - BYTES STRING INTEGER FLOAT BOOLEAN Default type is BYTES. 'type' can also be set at the column family level. However, the setting at this level takes precedence if 'type' is set at both levels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}
