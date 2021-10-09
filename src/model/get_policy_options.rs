use serde::{Deserialize, Serialize};

/// GetPolicyOptions : Encapsulates settings provided to GetIamPolicy.

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetPolicyOptions {
    /// Optional. The policy format version to be returned. Valid values are 0, 1, and 3. Requests specifying an invalid
    /// value will be rejected. Requests for policies with any conditional bindings must specify version 3. Policies
    /// without any conditional bindings may specify any valid value or leave the field unset. To learn which resources
    /// support conditions in their IAM policies,
    /// see the [IAM documentation](https://cloud.google.com/iam/help/conditions/resource-policies).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_policy_version: Option<i32>,
}
