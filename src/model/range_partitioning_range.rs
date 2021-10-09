use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RangePartitioningRange {
    /// [TrustedTester] [Required] The end of range partitioning, exclusive.
    pub end: String,
    /// [TrustedTester] [Required] The width of each interval.
    pub interval: String,
    /// [TrustedTester] [Required] The start of range partitioning, inclusive.
    pub start: String,
}
