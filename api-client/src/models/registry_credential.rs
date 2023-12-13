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
pub struct RegistryCredential {
    /// Credential type, such as 'basic', 'oauth'.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// Access key, e.g. user name when credential type is 'basic'.
    #[serde(rename = "access_key", skip_serializing_if = "Option::is_none")]
    pub access_key: Option<String>,
    /// Access secret, e.g. password when credential type is 'basic'.
    #[serde(rename = "access_secret", skip_serializing_if = "Option::is_none")]
    pub access_secret: Option<String>,
}

impl RegistryCredential {
    pub fn new() -> RegistryCredential {
        RegistryCredential {
            r#type: None,
            access_key: None,
            access_secret: None,
        }
    }
}
