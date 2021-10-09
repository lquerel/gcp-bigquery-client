use crate::error::BQError;
use crate::model::dataset_reference::DatasetReference;
use crate::model::table::Table;
use crate::Client;
use std::collections::HashMap;

#[derive(Debug, Default, Serialize, Deserialize)]
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

    /// Returns the project id of this dataset.
    pub fn project_id(&self) -> &String {
        &self.dataset_reference.project_id
    }

    /// Returns the dataset id of this dataset.
    pub fn dataset_id(&self) -> &String {
        &self.dataset_reference.dataset_id
    }

    /// Sets the location of this dataset.
    /// # Arguments
    /// * `location` - The location of this dataset
    pub fn location(mut self, location: &str) -> Self {
        self.location = Some(location.into());
        self
    }

    /// Sets the friendly name of this dataset.
    /// # Arguments
    /// * `friendly_name` - The friendly name of this dataset
    pub fn friendly_name(mut self, friendly_name: &str) -> Self {
        self.friendly_name = Some(friendly_name.into());
        self
    }

    /// Adds a label to this dataset.
    /// # Arguments
    /// * `key` - The label name.
    /// * `value` - The label value.
    pub fn label(mut self, key: &str, value: &str) -> Self {
        if let Some(labels) = self.labels.as_mut() {
            labels.insert(key.into(), value.into());
        } else {
            let mut labels = HashMap::default();
            labels.insert(key.into(), value.into());
            self.labels = Some(labels);
        }
        self
    }

    /// Creates a new table.
    /// # Arguments
    /// * `client` - The client API.
    /// * `table` - The table definition.
    pub async fn create_table(&self, client: &Client, table: Table) -> Result<Table, BQError> {
        client.table().create(table).await
    }

    /// Deletes an existing table.
    /// # Arguments
    /// * `client` - The client API.
    /// * `delete_contents` - A flag defining if a dataset must be deleted even if it contains some tables, views, ...
    pub async fn delete(self, client: &Client, delete_contents: bool) -> Result<(), BQError> {
        client
            .dataset()
            .delete(self.project_id(), self.dataset_id(), delete_contents)
            .await
    }
}
