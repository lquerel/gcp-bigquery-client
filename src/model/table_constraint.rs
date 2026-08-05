use crate::model::table_reference::TableReference;
use serde::{Deserialize, Serialize};

/// The primary key and foreign key constraints defined on a table.
///
/// Table constraints are not enforced by BigQuery.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TableConstraints {
    /// [Optional] The table's primary key constraint.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_key: Option<PrimaryKey>,
    /// [Optional] The table's foreign key constraints.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foreign_keys: Option<Vec<ForeignKey>>,
}

impl TableConstraints {
    pub fn primary_key(mut self, primary_key: PrimaryKey) -> Self {
        self.primary_key = Some(primary_key);
        self
    }

    pub fn foreign_key(mut self, foreign_key: ForeignKey) -> Self {
        self.foreign_keys.get_or_insert_with(Vec::new).push(foreign_key);
        self
    }
}

/// A primary key constraint on a table's columns.
///
/// Primary key constraints are not enforced by BigQuery.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PrimaryKey {
    /// [Required] The columns that compose the primary key constraint.
    pub columns: Vec<String>,
}

impl PrimaryKey {
    pub fn new(columns: Vec<String>) -> Self {
        Self { columns }
    }
}

/// A foreign key constraint on a table's columns.
///
/// Foreign key constraints are not enforced by BigQuery.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ForeignKey {
    /// [Optional] The name of the foreign key constraint.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// [Required] The table containing the referenced primary key.
    pub referenced_table: TableReference,
    /// [Required] The columns that compose the foreign key.
    pub column_references: Vec<ColumnReference>,
}

impl ForeignKey {
    pub fn new(referenced_table: TableReference, column_references: Vec<ColumnReference>) -> Self {
        Self {
            name: None,
            referenced_table,
            column_references,
        }
    }

    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.into());
        self
    }
}

/// The pair of the foreign key column and the referenced primary key column.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ColumnReference {
    /// [Required] The column that composes the foreign key.
    pub referencing_column: String,
    /// [Required] The primary key column referenced by `referencing_column`.
    pub referenced_column: String,
}

impl ColumnReference {
    pub fn new(referencing_column: &str, referenced_column: &str) -> Self {
        Self {
            referencing_column: referencing_column.into(),
            referenced_column: referenced_column.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{ColumnReference, ForeignKey, PrimaryKey, TableConstraints};
    use crate::model::field_type::FieldType;
    use crate::model::table::Table;
    use crate::model::table_field_schema::TableFieldSchema;
    use crate::model::table_reference::TableReference;
    use crate::model::table_schema::TableSchema;
    use serde_json::json;

    #[test]
    fn serializes_and_deserializes_table_constraints() {
        let constraints = TableConstraints::default()
            .primary_key(PrimaryKey::new(vec!["id".into(), "version".into()]))
            .foreign_key(
                ForeignKey::new(
                    TableReference::new("project", "dataset", "parent"),
                    vec![ColumnReference::new("parent_id", "id")],
                )
                .name("parent_fk"),
            );
        let table = Table::new(
            "project",
            "dataset",
            "child",
            TableSchema::new(vec![
                TableFieldSchema::new("id", FieldType::String),
                TableFieldSchema::new("version", FieldType::Int64),
                TableFieldSchema::new("parent_id", FieldType::String),
            ]),
        )
        .table_constraints(constraints);

        let value = serde_json::to_value(&table).unwrap();
        assert_eq!(
            value["tableConstraints"],
            json!({
                "primaryKey": { "columns": ["id", "version"] },
                "foreignKeys": [{
                    "name": "parent_fk",
                    "referencedTable": {
                        "projectId": "project",
                        "datasetId": "dataset",
                        "tableId": "parent"
                    },
                    "columnReferences": [{
                        "referencingColumn": "parent_id",
                        "referencedColumn": "id"
                    }]
                }]
            })
        );

        let decoded: Table = serde_json::from_value(value).unwrap();
        let decoded_constraints = decoded.table_constraints.unwrap();
        assert_eq!(decoded_constraints.primary_key.unwrap().columns, ["id", "version"]);
        let foreign_key = &decoded_constraints.foreign_keys.unwrap()[0];
        assert_eq!(foreign_key.name.as_deref(), Some("parent_fk"));
        assert_eq!(foreign_key.referenced_table.table_id, "parent");
        assert_eq!(foreign_key.column_references[0].referencing_column, "parent_id");
        assert_eq!(foreign_key.column_references[0].referenced_column, "id");
    }
}
