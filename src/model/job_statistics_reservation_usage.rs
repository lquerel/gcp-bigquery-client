use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JobStatisticsReservationUsage {
    /// [Output-only] Reservation name or \"unreserved\" for on-demand resources usage.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// [Output-only] Slot-milliseconds the job spent in the given reservation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slot_ms: Option<String>,
}
