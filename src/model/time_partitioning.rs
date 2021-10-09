use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TimePartitioning {
    /// [Optional] Number of milliseconds for which to keep the storage for partitions in the table. The storage in a partition will have an expiration time of its partition time plus this value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_ms: Option<String>,
    /// [Beta] [Optional] If not set, the table is partitioned by pseudo column, referenced via either '_PARTITIONTIME' as TIMESTAMP type, or '_PARTITIONDATE' as DATE type. If field is specified, the table is instead partitioned by this field. The field must be a top-level TIMESTAMP or DATE field. Its mode must be NULLABLE or REQUIRED.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_partition_filter: Option<bool>,
    /// [Required] The supported types are DAY, HOUR, MONTH, and YEAR, which will generate one partition per day, hour, month, and year, respectively. When the type is not specified, the default behavior is DAY.
    #[serde(rename = "type")]
    pub r#type: String,
}

impl TimePartitioning {
    /// Creates a new time partitioning.
    /// # Argument
    /// * `type` - The type of the partitioning (HOUR, DAY, MONTH or YEAR)
    pub fn new(r#type: String) -> Self {
        Self {
            expiration_ms: None,
            field: None,
            require_partition_filter: None,
            r#type,
        }
    }

    /// Creates a time partitioning per hour.
    pub fn per_hour() -> Self {
        Self {
            expiration_ms: None,
            field: None,
            require_partition_filter: None,
            r#type: "HOUR".to_string(),
        }
    }

    /// Creates a time partitioning per day.
    pub fn per_day() -> Self {
        Self {
            expiration_ms: None,
            field: None,
            require_partition_filter: None,
            r#type: "DAY".to_string(),
        }
    }

    /// Creates a time partitioning per month.
    pub fn per_month() -> Self {
        Self {
            expiration_ms: None,
            field: None,
            require_partition_filter: None,
            r#type: "MONTH".to_string(),
        }
    }

    /// Creates a time partitioning per year.
    pub fn per_year() -> Self {
        Self {
            expiration_ms: None,
            field: None,
            require_partition_filter: None,
            r#type: "YEAR".to_string(),
        }
    }

    /// Sets the expiration time per partition
    pub fn expiration_ms(mut self, duration: Duration) -> Self {
        self.expiration_ms = Some(duration.as_millis().to_string());
        self
    }

    /// Sets the field used for the time partitioning
    pub fn field(mut self, field: &str) -> Self {
        self.field = Some(field.to_string());
        self
    }
}
