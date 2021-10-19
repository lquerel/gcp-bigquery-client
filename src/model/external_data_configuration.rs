use crate::model::bigtable_options::BigtableOptions;
use crate::model::csv_options::CsvOptions;
use crate::model::google_sheets_options::GoogleSheetsOptions;
use crate::model::hive_partitioning_options::HivePartitioningOptions;
use crate::model::table_schema::TableSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExternalDataConfiguration {
    /// Try to detect schema and format options automatically. Any option specified explicitly will be honored.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub autodetect: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bigtable_options: Option<BigtableOptions>,
    /// [Optional] The compression type of the data source. Possible values include GZIP and NONE. The default value is NONE. This setting is ignored for Google Cloud Bigtable, Google Cloud Datastore backups and Avro formats.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compression: Option<String>,
    /// [Optional, Trusted Tester] Connection for external data source.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub csv_options: Option<CsvOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub google_sheets_options: Option<GoogleSheetsOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hive_partitioning_options: Option<HivePartitioningOptions>,
    /// [Optional] Indicates if BigQuery should allow extra values that are not represented in the table schema. If true, the extra values are ignored. If false, records with extra columns are treated as bad records, and if there are too many bad records, an invalid error is returned in the job result. The default value is false. The sourceFormat property determines what BigQuery treats as an extra value: CSV: Trailing columns JSON: Named values that don't match any column names Google Cloud Bigtable: This setting is ignored. Google Cloud Datastore backups: This setting is ignored. Avro: This setting is ignored.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore_unknown_values: Option<bool>,
    /// [Optional] The maximum number of bad records that BigQuery can ignore when reading data. If the number of bad records exceeds this value, an invalid error is returned in the job result. This is only valid for CSV, JSON, and Google Sheets. The default value is 0, which requires that all records are valid. This setting is ignored for Google Cloud Bigtable, Google Cloud Datastore backups and Avro formats.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_bad_records: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<TableSchema>,
    /// [Required] The data format. For CSV files, specify \"CSV\". For Google sheets, specify \"GOOGLE_SHEETS\". For newline-delimited JSON, specify \"NEWLINE_DELIMITED_JSON\". For Avro files, specify \"AVRO\". For Google Cloud Datastore backups, specify \"DATASTORE_BACKUP\". [Beta] For Google Cloud Bigtable, specify \"BIGTABLE\".
    pub source_format: String,
    /// [Required] The fully-qualified URIs that point to your data in Google Cloud. For Google Cloud Storage URIs: Each URI can contain one '*' wildcard character and it must come after the 'bucket' name. Size limits related to load jobs apply to external data sources. For Google Cloud Bigtable URIs: Exactly one URI can be specified and it has be a fully specified and valid HTTPS URL for a Google Cloud Bigtable table. For Google Cloud Datastore backups, exactly one URI can be specified. Also, the '*' wildcard character is not allowed.
    pub source_uris: Vec<String>,
}
