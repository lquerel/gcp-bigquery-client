//! Represents access on a subset of rows on the specified table, defined by its filter predicate. Access to the subset of rows is controlled by its IAM policy.
use crate::model::row_access_policy_reference::RowAccessPolicyReference;
use time::OffsetDateTime;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RowAccessPolicy {
    /// Output only. The time when this row access policy was created, in milliseconds since the epoch.
    #[serde(with = "time::serde::rfc3339")]
    pub creation_time: OffsetDateTime,
    /// Output only. A hash of this resource.
    pub etag: Option<String>,
    /// Required. Reference describing the ID of this row access policy.
    pub row_access_policy_reference: RowAccessPolicyReference,
    /// Output only. The time when this row access policy was last modified, in milliseconds since the epoch.
    #[serde(with = "time::serde::rfc3339")]
    pub last_modified_time: OffsetDateTime,
    /// Required. A SQL boolean expression that represents the rows defined by this row access policy, similar to the boolean expression in a WHERE clause of a SELECT query on a table. References to other tables, routines, and temporary functions are not supported. Examples: region="EU" date_field = CAST('2019-9-27' as DATE) nullable_field is not NULL numeric_field BETWEEN 1.0 AND 5.0
    pub filter_predicate: String,
}

impl Default for RowAccessPolicy {
    fn default() -> Self {
        Self {
            creation_time: OffsetDateTime::now_utc(),
            etag: None,
            row_access_policy_reference: RowAccessPolicyReference {
                dataset_id: None,
                policy_id: None,
                project_id: None,
                table_id: None,
            },
            last_modified_time: OffsetDateTime::now_utc(),
            filter_predicate: "".to_string(),
        }
    }
}
