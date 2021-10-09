use crate::model::range_partitioning_range::RangePartitioningRange;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RangePartitioning {
    /// [TrustedTester] [Required] The table is partitioned by this field. The field must be a top-level NULLABLE/REQUIRED field. The only supported type is INTEGER/INT64.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range: Option<RangePartitioningRange>,
}
