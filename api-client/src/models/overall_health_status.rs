/*
 * Harbor API
 *
 * These APIs provide services for manipulating Harbor project.
 *
 * The version of the OpenAPI document: 2.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// OverallHealthStatus : The system health status

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OverallHealthStatus {
    /// The overall health status. It is \"healthy\" only when all the components' status are \"healthy\"
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "components", skip_serializing_if = "Option::is_none")]
    pub components: Option<Vec<crate::models::ComponentHealthStatus>>,
}

impl OverallHealthStatus {
    /// The system health status
    pub fn new() -> OverallHealthStatus {
        OverallHealthStatus {
            status: None,
            components: None,
        }
    }
}