use crate::error::BQError;
use crate::model::dataset_reference::DatasetReference;
use crate::model::table::Table;
use crate::model::table_schema::TableSchema;
use crate::Client;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Dataset {
    /// [Required] A reference that identifies the dataset.
    pub dataset_reference: DatasetReference,
    /// A descriptive name for the dataset, if one exists.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
    /// The fully-qualified, unique, opaque ID of the dataset.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The resource type. This property always returns the value \"bigquery#dataset\".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// The labels associated with this dataset. You can use these to organize and group your datasets.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<std::collections::HashMap<String, String>>,
    /// The geographic location where the data resides.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
}

impl Dataset {
    pub fn new(project_id: &str, dataset_id: &str) -> Self {
        Self {
            dataset_reference: DatasetReference {
                dataset_id: dataset_id.into(),
                project_id: project_id.into(),
            },
            friendly_name: None,
            id: None,
            kind: None,
            labels: None,
            location: None,
        }
    }

    pub fn project_id(&self) -> &String {
        &self.dataset_reference.project_id
    }

    pub fn dataset_id(&self) -> &String {
        &self.dataset_reference.dataset_id
    }

    pub fn friendly_name(&mut self, friendly_name: String) -> &mut Dataset {
        self.friendly_name = Some(friendly_name);
        self
    }

    pub async fn create_table(
        &self,
        client: &Client,
        table_id: &str,
        table_schema: TableSchema,
    ) -> Result<Table, BQError> {
        let table_decl = Table::new(self.project_id(), self.dataset_id(), table_id, table_schema);
        client.table().create(table_decl).await
    }

    pub async fn delete(self, client: &Client, delete_contents: bool) -> Result<(), BQError> {
        client
            .dataset()
            .delete(self.project_id(), self.dataset_id(), delete_contents)
            .await
    }
}
