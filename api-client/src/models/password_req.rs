/*
 * Harbor API
 *
 * These APIs provide services for manipulating Harbor project.
 *
 * The version of the OpenAPI document: 2.0
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PasswordReq {
    /// The user's existing password.
    #[serde(rename = "old_password", skip_serializing_if = "Option::is_none")]
    pub old_password: Option<String>,
    /// New password for marking as to be updated.
    #[serde(rename = "new_password", skip_serializing_if = "Option::is_none")]
    pub new_password: Option<String>,
}

impl PasswordReq {
    pub fn new() -> PasswordReq {
        PasswordReq {
            old_password: None,
            new_password: None,
        }
    }
}
