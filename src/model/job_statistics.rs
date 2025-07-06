use crate::model::job_statistics2::JobStatistics2;
use crate::model::job_statistics3::JobStatistics3;
use crate::model::job_statistics4::JobStatistics4;
use crate::model::job_statistics_reservation_usage::JobStatisticsReservationUsage;
use crate::model::row_level_security_statistics::RowLevelSecurityStatistics;
use crate::model::script_statistics::ScriptStatistics;
use crate::model::transaction_info::TransactionInfo;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JobStatistics {
    /// [TrustedTester] [Output-only] Job progress (0.0 -> 1.0) for LOAD and EXTRACT jobs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion_ratio: Option<f64>,
    /// [Output-only] Creation time of this job, in milliseconds since the epoch. This field will be present on all jobs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<String>,
    /// [Output-only] End time of this job, in milliseconds since the epoch. This field will be present whenever a job is in the DONE state.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extract: Option<JobStatistics4>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load: Option<JobStatistics3>,
    /// [Output-only] Number of child jobs executed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_child_jobs: Option<String>,
    /// [Output-only] If this is a child job, the id of the parent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_job_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<JobStatistics2>,
    /// [Output-only] Quotas which delayed this job's start time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quota_deferments: Option<Vec<String>>,
    /// [Output-only] Job resource usage breakdown by reservation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reservation_usage: Option<Vec<JobStatisticsReservationUsage>>,
    /// [Output-only] Name of the primary reservation assigned to this job. Note that this could be different than reservations reported in the reservation usage field if parent reservations were used to execute this job.
    #[serde(alias = "reservation_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reservation_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub row_level_security_statistics: Option<RowLevelSecurityStatistics>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub script_statistics: Option<ScriptStatistics>,
    /// [Output-only] Start time of this job, in milliseconds since the epoch. This field will be present when the job transitions from the PENDING state to either RUNNING or DONE.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// [Output-only] [Deprecated] Use the bytes processed in the query statistics instead.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_bytes_processed: Option<String>,
    /// [Output-only] Slot-milliseconds for the job.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_slot_ms: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_info_template: Option<TransactionInfo>,
}
