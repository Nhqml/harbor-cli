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
pub struct OidcUserInfo {
    /// the ID of the OIDC info record
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    /// the ID of the user
    #[serde(rename = "user_id", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i32>,
    /// the concatenation of sub and issuer in the ID token
    #[serde(rename = "subiss", skip_serializing_if = "Option::is_none")]
    pub subiss: Option<String>,
    /// the secret of the OIDC user that can be used for CLI to push/pull artifacts
    #[serde(rename = "secret", skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
    /// The creation time of the OIDC user info record.
    #[serde(rename = "creation_time", skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<String>,
    /// The update time of the OIDC user info record.
    #[serde(rename = "update_time", skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
}

impl OidcUserInfo {
    pub fn new() -> OidcUserInfo {
        OidcUserInfo {
            id: None,
            user_id: None,
            subiss: None,
            secret: None,
            creation_time: None,
            update_time: None,
        }
    }
}