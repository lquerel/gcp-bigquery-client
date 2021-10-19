//! Evaluation metrics used by weighted-ALS models specified by feedback_type=implicit.

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RankingMetrics {
    /// Calculates a precision per user for all the items by ranking them and then averages all the precisions across all the users.
    pub mean_average_precision: Option<f64>,
    /// A metric to determine the goodness of a ranking calculated from the predicted confidence by comparing it to an ideal rank measured by the original ratings.
    pub normalized_discounted_cumulative_gain: Option<f64>,
    /// Similar to the mean squared error computed in regression and explicit recommendation models except instead of computing the rating directly, the output from evaluate is computed against a preference which is 1 or 0 depending on if the rating exists or not.
    pub mean_squared_error: Option<f64>,
    /// Determines the goodness of a ranking by computing the percentile rank from the predicted confidence and dividing it by the original rank.
    pub average_rank: Option<f64>,
}
