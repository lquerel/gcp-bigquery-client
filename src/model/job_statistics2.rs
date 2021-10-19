use crate::model::bigquery_model_training::BigQueryModelTraining;
use crate::model::explain_query_stage::ExplainQueryStage;
use crate::model::job_statistics_reservation_usage::JobStatisticsReservationUsage;
use crate::model::query_parameter::QueryParameter;
use crate::model::query_timeline_sample::QueryTimelineSample;
use crate::model::routine_reference::RoutineReference;
use crate::model::row_access_policy_reference::RowAccessPolicyReference;
use crate::model::table_reference::TableReference;
use crate::model::table_schema::TableSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JobStatistics2 {
    /// [Output-only] Billing tier for the job.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_tier: Option<i32>,
    /// [Output-only] Whether the query result was fetched from the query cache.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_hit: Option<bool>,
    /// [Output-only] [Preview] The number of row access policies affected by a DDL statement. Present only for DROP ALL ROW ACCESS POLICIES queries.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ddl_affected_row_access_policy_count: Option<String>,
    /// The DDL operation performed, possibly dependent on the pre-existence of the DDL target. Possible values (new values might be added in the future): \"CREATE\": The query created the DDL target. \"SKIP\": No-op. Example cases: the query is CREATE TABLE IF NOT EXISTS while the table already exists, or the query is DROP TABLE IF EXISTS while the table does not exist. \"REPLACE\": The query replaced the DDL target. Example case: the query is CREATE OR REPLACE TABLE, and the table already exists. \"DROP\": The query deleted the DDL target.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ddl_operation_performed: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ddl_target_routine: Option<RoutineReference>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ddl_target_row_access_policy: Option<RowAccessPolicyReference>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ddl_target_table: Option<TableReference>,
    /// [Output-only] The original estimate of bytes processed for the job.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_bytes_processed: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_training: Option<BigQueryModelTraining>,
    /// [Output-only, Beta] Deprecated; do not use.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_training_current_iteration: Option<i32>,
    /// [Output-only, Beta] Deprecated; do not use.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_training_expected_total_iteration: Option<String>,
    /// [Output-only] The number of rows affected by a DML statement. Present only for DML statements INSERT, UPDATE or DELETE.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_dml_affected_rows: Option<String>,
    /// [Output-only] Describes execution plan for the query.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_plan: Option<Vec<ExplainQueryStage>>,
    /// [Output-only] Referenced routines (persistent user-defined functions and stored procedures) for the job.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub referenced_routines: Option<Vec<RoutineReference>>,
    /// [Output-only] Referenced tables for the job. Queries that reference more than 50 tables will not have a complete list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub referenced_tables: Option<Vec<TableReference>>,
    /// [Output-only] Job resource usage breakdown by reservation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reservation_usage: Option<Vec<JobStatisticsReservationUsage>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<TableSchema>,
    /// The type of query statement, if valid. Possible values (new values might be added in the future): \"SELECT\": SELECT query. \"INSERT\": INSERT query; see https://cloud.google.com/bigquery/docs/reference/standard-sql/data-manipulation-language. \"UPDATE\": UPDATE query; see https://cloud.google.com/bigquery/docs/reference/standard-sql/data-manipulation-language. \"DELETE\": DELETE query; see https://cloud.google.com/bigquery/docs/reference/standard-sql/data-manipulation-language. \"MERGE\": MERGE query; see https://cloud.google.com/bigquery/docs/reference/standard-sql/data-manipulation-language. \"ALTER_TABLE\": ALTER TABLE query. \"ALTER_VIEW\": ALTER VIEW query. \"ASSERT\": ASSERT condition AS 'description'. \"CREATE_FUNCTION\": CREATE FUNCTION query. \"CREATE_MODEL\": CREATE [OR REPLACE] MODEL ... AS SELECT ... . \"CREATE_PROCEDURE\": CREATE PROCEDURE query. \"CREATE_TABLE\": CREATE [OR REPLACE] TABLE without AS SELECT. \"CREATE_TABLE_AS_SELECT\": CREATE [OR REPLACE] TABLE ... AS SELECT ... . \"CREATE_VIEW\": CREATE [OR REPLACE] VIEW ... AS SELECT ... . \"DROP_FUNCTION\" : DROP FUNCTION query. \"DROP_PROCEDURE\": DROP PROCEDURE query. \"DROP_TABLE\": DROP TABLE query. \"DROP_VIEW\": DROP VIEW query.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_type: Option<String>,
    /// [Output-only] [Beta] Describes a timeline of job execution.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeline: Option<Vec<QueryTimelineSample>>,
    /// [Output-only] Total bytes billed for the job.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_bytes_billed: Option<String>,
    /// [Output-only] Total bytes processed for the job.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_bytes_processed: Option<String>,
    /// [Output-only] For dry-run jobs, totalBytesProcessed is an estimate and this field specifies the accuracy of the estimate. Possible values can be: UNKNOWN: accuracy of the estimate is unknown. PRECISE: estimate is precise. LOWER_BOUND: estimate is lower bound of what the query would cost. UPPER_BOUND: estimate is upper bound of what the query would cost.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_bytes_processed_accuracy: Option<String>,
    /// [Output-only] Total number of partitions processed from all partitioned tables referenced in the job.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_partitions_processed: Option<String>,
    /// [Output-only] Slot-milliseconds for the job.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_slot_ms: Option<String>,
    /// Standard SQL only: list of undeclared query parameters detected during a dry run validation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub undeclared_query_parameters: Option<Vec<QueryParameter>>,
}
