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
pub struct ProjectDeletable {
    /// Whether the project can be deleted.
    #[serde(rename = "deletable", skip_serializing_if = "Option::is_none")]
    pub deletable: Option<bool>,
    /// The detail message when the project can not be deleted.
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

impl ProjectDeletable {
    pub fn new() -> ProjectDeletable {
        ProjectDeletable {
            deletable: None,
            message: None,
        }
    }
}
