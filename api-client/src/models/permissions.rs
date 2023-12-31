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
pub struct Permissions {
    /// The system level permissions
    #[serde(rename = "system", skip_serializing_if = "Option::is_none")]
    pub system: Option<Vec<crate::models::Permission>>,
    /// The project level permissions
    #[serde(rename = "project", skip_serializing_if = "Option::is_none")]
    pub project: Option<Vec<crate::models::Permission>>,
}

impl Permissions {
    pub fn new() -> Permissions {
        Permissions {
            system: None,
            project: None,
        }
    }
}
