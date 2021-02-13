use serde::{Serialize, Deserialize};
use crate::model::policy::Policy;

/// SetIamPolicyRequest : Request message for `SetIamPolicy` method.

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetIamPolicyRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<Policy>,
    /// OPTIONAL: A FieldMask specifying which fields of the policy to modify. Only the fields in the mask will be modified. If no mask is provided, the following default mask is used: `paths: \"bindings, etag\"`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_mask: Option<String>,
}
