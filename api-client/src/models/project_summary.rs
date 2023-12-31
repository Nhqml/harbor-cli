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
pub struct ProjectSummary {
    /// The number of the repositories under this project.
    #[serde(rename = "repo_count", skip_serializing_if = "Option::is_none")]
    pub repo_count: Option<i32>,
    /// The total number of project admin members.
    #[serde(
        rename = "project_admin_count",
        skip_serializing_if = "Option::is_none"
    )]
    pub project_admin_count: Option<i32>,
    /// The total number of maintainer members.
    #[serde(rename = "maintainer_count", skip_serializing_if = "Option::is_none")]
    pub maintainer_count: Option<i32>,
    /// The total number of developer members.
    #[serde(rename = "developer_count", skip_serializing_if = "Option::is_none")]
    pub developer_count: Option<i32>,
    /// The total number of guest members.
    #[serde(rename = "guest_count", skip_serializing_if = "Option::is_none")]
    pub guest_count: Option<i32>,
    /// The total number of limited guest members.
    #[serde(
        rename = "limited_guest_count",
        skip_serializing_if = "Option::is_none"
    )]
    pub limited_guest_count: Option<i32>,
    #[serde(rename = "quota", skip_serializing_if = "Option::is_none")]
    pub quota: Option<Box<crate::models::ProjectSummaryQuota>>,
    #[serde(rename = "registry", skip_serializing_if = "Option::is_none")]
    pub registry: Option<Box<crate::models::Registry>>,
}

impl ProjectSummary {
    pub fn new() -> ProjectSummary {
        ProjectSummary {
            repo_count: None,
            project_admin_count: None,
            maintainer_count: None,
            developer_count: None,
            guest_count: None,
            limited_guest_count: None,
            quota: None,
            registry: None,
        }
    }
}
