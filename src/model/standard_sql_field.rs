//! A field or a column.
use crate::model::standard_sql_data_type::StandardSqlDataType;

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StandardSqlField {
    /// Optional. The name of this field. Can be absent for struct fields.
    pub name: Option<String>,
    /// Optional. The type of this parameter. Absent if not explicitly specified (e.g., CREATE FUNCTION statement can omit the return type; in this case the output parameter does not have this "type" field).
    pub r#type: Option<StandardSqlDataType>,
}
