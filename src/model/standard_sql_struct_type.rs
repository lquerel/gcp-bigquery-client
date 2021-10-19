use crate::model::standard_sql_field::StandardSqlField;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StandardSqlStructType {
    pub fields: Option<Vec<StandardSqlField>>,
}
