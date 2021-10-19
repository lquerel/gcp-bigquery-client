use serde::{Deserialize, Serialize};

/// TestIamPermissionsRequest : Request message for `TestIamPermissions` method.

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TestIamPermissionsRequest {
    /// The set of permissions to check for the `resource`. Permissions with wildcards (such as '*' or 'storage.*')
    /// are not allowed. For more information see [IAM Overview](https://cloud.google.com/iam/docs/overview#permissions).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<String>>,
}
