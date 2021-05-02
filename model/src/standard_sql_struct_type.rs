use crate::standard_sql_field::StandardSqlField;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StandardSqlStructType {
    pub fields: Option<Vec<StandardSqlField>>,
}
