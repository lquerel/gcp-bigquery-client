use crate::model::clustering::Clustering;
use crate::model::destination_table_properties::DestinationTableProperties;
use crate::model::encryption_configuration::EncryptionConfiguration;
use crate::model::hive_partitioning_options::HivePartitioningOptions;
use crate::model::range_partitioning::RangePartitioning;
use crate::model::table_reference::TableReference;
use crate::model::table_schema::TableSchema;
use crate::model::time_partitioning::TimePartitioning;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JobConfigurationLoad {
    /// [Optional] Accept rows that are missing trailing optional columns. The missing values are treated as nulls. If false, records with missing trailing columns are treated as bad records, and if there are too many bad records, an invalid error is returned in the job result. The default value is false. Only applicable to CSV, ignored for other formats.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_jagged_rows: Option<bool>,
    /// Indicates if BigQuery should allow quoted data sections that contain newline characters in a CSV file. The default value is false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_quoted_newlines: Option<bool>,
    /// [Optional] Indicates if we should automatically infer the options and schema for CSV and JSON sources.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub autodetect: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clustering: Option<Clustering>,
    /// [Optional] Specifies whether the job is allowed to create new tables. The following values are supported: CREATE_IF_NEEDED: If the table does not exist, BigQuery creates the table. CREATE_NEVER: The table must already exist. If it does not, a 'notFound' error is returned in the job result. The default value is CREATE_IF_NEEDED. Creation, truncation and append actions occur as one atomic update upon job completion.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_disposition: Option<String>,
    /// Defines the list of possible SQL data types to which the source decimal values are converted. This list and the precision and the scale parameters of the decimal field determine the target type. In the order of NUMERIC, BIGNUMERIC ([Preview](/products/#product-launch-stages)), and STRING, a type is picked if it is in the specified list and if it supports the precision and the scale. STRING supports all precision and scale values. If none of the listed types supports the precision and the scale, the type supporting the widest range in the specified list is picked, and if a value exceeds the supported range when reading the data, an error will be thrown. Example: Suppose the value of this field is [\"NUMERIC\", \"BIGNUMERIC\"]. If (precision,scale) is: * (38,9) -> NUMERIC; * (39,9) -> BIGNUMERIC (NUMERIC cannot hold 30 integer digits); * (38,10) -> BIGNUMERIC (NUMERIC cannot hold 10 fractional digits); * (76,38) -> BIGNUMERIC; * (77,38) -> BIGNUMERIC (error if value exeeds supported range). This field cannot contain duplicate types. The order of the types in this field is ignored. For example, [\"BIGNUMERIC\", \"NUMERIC\"] is the same as [\"NUMERIC\", \"BIGNUMERIC\"] and NUMERIC always takes precedence over BIGNUMERIC. Defaults to [\"NUMERIC\", \"STRING\"] for ORC and [\"NUMERIC\"] for the other file formats.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decimal_target_types: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_encryption_configuration: Option<EncryptionConfiguration>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_table: Option<TableReference>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_table_properties: Option<DestinationTableProperties>,
    /// [Optional] The character encoding of the data. The supported values are UTF-8 or ISO-8859-1. The default value is UTF-8. BigQuery decodes the data after the raw, binary data has been split using the values of the quote and fieldDelimiter properties.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding: Option<String>,
    /// [Optional] The separator for fields in a CSV file. The separator can be any ISO-8859-1 single-byte character. To use a character in the range 128-255, you must encode the character as UTF8. BigQuery converts the string to ISO-8859-1 encoding, and then uses the first byte of the encoded string to split the data in its raw, binary state. BigQuery also supports the escape sequence \"\\t\" to specify a tab separator. The default value is a comma (',').
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_delimiter: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hive_partitioning_options: Option<HivePartitioningOptions>,
    /// [Optional] Indicates if BigQuery should allow extra values that are not represented in the table schema. If true, the extra values are ignored. If false, records with extra columns are treated as bad records, and if there are too many bad records, an invalid error is returned in the job result. The default value is false. The sourceFormat property determines what BigQuery treats as an extra value: CSV: Trailing columns JSON: Named values that don't match any column names
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore_unknown_values: Option<bool>,
    /// [Optional] If sourceFormat is set to newline-delimited JSON, indicates whether it should be processed as a JSON variant such as GeoJSON. For a sourceFormat other than JSON, omit this field. If the sourceFormat is newline-delimited JSON: - for newline-delimited GeoJSON: set to GEOJSON.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub json_extension: Option<String>,
    /// [Optional] The maximum number of bad records that BigQuery can ignore when running the job. If the number of bad records exceeds this value, an invalid error is returned in the job result. This is only valid for CSV and JSON. The default value is 0, which requires that all records are valid.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_bad_records: Option<i32>,
    /// [Optional] Specifies a string that represents a null value in a CSV file. For example, if you specify \"\\N\", BigQuery interprets \"\\N\" as a null value when loading a CSV file. The default value is the empty string. If you set this property to a custom value, BigQuery throws an error if an empty string is present for all data types except for STRING and BYTE. For STRING and BYTE columns, BigQuery interprets the empty string as an empty value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub null_marker: Option<String>,
    /// If sourceFormat is set to \"DATASTORE_BACKUP\", indicates which entity properties to load into BigQuery from a Cloud Datastore backup. Property names are case sensitive and must be top-level properties. If no properties are specified, BigQuery loads all properties. If any named property isn't found in the Cloud Datastore backup, an invalid error is returned in the job result.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub projection_fields: Option<Vec<String>>,
    /// [Optional] The value that is used to quote data sections in a CSV file. BigQuery converts the string to ISO-8859-1 encoding, and then uses the first byte of the encoded string to split the data in its raw, binary state. The default value is a double-quote ('\"'). If your data does not contain quoted sections, set the property value to an empty string. If your data contains quoted newline characters, you must also set the allowQuotedNewlines property to true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range_partitioning: Option<RangePartitioning>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<TableSchema>,
    /// [Deprecated] The inline schema. For CSV schemas, specify as \"Field1:Type1[,Field2:Type2]*\". For example, \"foo:STRING, bar:INTEGER, baz:FLOAT\".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_inline: Option<String>,
    /// [Deprecated] The format of the schemaInline property.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_inline_format: Option<String>,
    /// Allows the schema of the destination table to be updated as a side effect of the load job if a schema is autodetected or supplied in the job configuration. Schema update options are supported in two cases: when writeDisposition is WRITE_APPEND; when writeDisposition is WRITE_TRUNCATE and the destination table is a partition of a table, specified by partition decorators. For normal tables, WRITE_TRUNCATE will always overwrite the schema. One or more of the following values are specified: ALLOW_FIELD_ADDITION: allow adding a nullable field to the schema. ALLOW_FIELD_RELAXATION: allow relaxing a required field in the original schema to nullable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_update_options: Option<Vec<String>>,
    /// [Optional] The number of rows at the top of a CSV file that BigQuery will skip when loading the data. The default value is 0. This property is useful if you have header rows in the file that should be skipped.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_leading_rows: Option<i32>,
    /// [Optional] The format of the data files. For CSV files, specify \"CSV\". For datastore backups, specify \"DATASTORE_BACKUP\". For newline-delimited JSON, specify \"NEWLINE_DELIMITED_JSON\". For Avro, specify \"AVRO\". For parquet, specify \"PARQUET\". For orc, specify \"ORC\". The default value is CSV.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_format: Option<String>,
    /// [Required] The fully-qualified URIs that point to your data in Google Cloud. For Google Cloud Storage URIs: Each URI can contain one '*' wildcard character and it must come after the 'bucket' name. Size limits related to load jobs apply to external data sources. For Google Cloud Bigtable URIs: Exactly one URI can be specified and it has be a fully specified and valid HTTPS URL for a Google Cloud Bigtable table. For Google Cloud Datastore backups: Exactly one URI can be specified. Also, the '*' wildcard character is not allowed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_uris: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_partitioning: Option<TimePartitioning>,
    /// [Optional] If sourceFormat is set to \"AVRO\", indicates whether to enable interpreting logical types into their corresponding types (ie. TIMESTAMP), instead of only using their raw types (ie. INTEGER).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_avro_logical_types: Option<bool>,
    /// [Optional] Specifies the action that occurs if the destination table already exists. The following values are supported: WRITE_TRUNCATE: If the table already exists, BigQuery overwrites the table data. WRITE_APPEND: If the table already exists, BigQuery appends the data to the table. WRITE_EMPTY: If the table already exists and contains data, a 'duplicate' error is returned in the job result. The default value is WRITE_APPEND. Each action is atomic and only occurs if BigQuery is able to complete the job successfully. Creation, truncation and append actions occur as one atomic update upon job completion.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub write_disposition: Option<String>,
}
