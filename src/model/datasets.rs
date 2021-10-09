use crate::model::dataset::Dataset;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Datasets {
    /// An array of the dataset resources in the project. Each resource contains basic information. For full information about a particular dataset resource, use the Datasets: get method. This property is omitted when there are no datasets in the project.
    //#[serde(skip_serializing_if = "Option::is_none")]
    pub datasets: Vec<Dataset>,
    /// A hash value of the results page. You can use this property to determine if the page has changed since the last request.
    //#[serde(skip_serializing_if = "Option::is_none")]
    pub etag: String,
    /// The list type. This property always returns the value \"bigquery#datasetList\".
    //#[serde(skip_serializing_if = "Option::is_none")]
    pub kind: String,
    /// A token that can be used to request the next results page. This property is omitted on the final results page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}
