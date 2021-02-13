use serde::{Serialize, Deserialize};
use crate::model::dataset_reference::DatasetReference;

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
    pub fn new(id: &str) -> Self {
        Self {
            dataset_reference: DatasetReference { dataset_id: id.into(), project_id: None },
            friendly_name: None,
            id: None,
            kind: None,
            labels: None,
            location: None
        }
    }

    pub fn friendly_name(&mut self, friendly_name: String) -> &mut Dataset {
        self.friendly_name = Some(friendly_name);
        self
    }
}