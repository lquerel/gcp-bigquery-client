//! Principal component infos, used only for eigen decomposition based models, e.g., PCA. Ordered by explained_variance in the descending order.

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PrincipalComponentInfo {
    /// The explained_variance is pre-ordered in the descending order to compute the cumulative explained variance ratio.
    pub cumulative_explained_variance_ratio: Option<f64>,
    /// Explained_variance over the total explained variance.
    pub explained_variance_ratio: Option<f64>,
    /// Id of the principal component.
    pub principal_component_id: Option<i64>,
    /// Explained variance by this principal component, which is simply the eigenvalue.
    pub explained_variance: Option<f64>,
}
