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
pub struct Statistic {
    /// The count of the private projects
    #[serde(
        rename = "private_project_count",
        skip_serializing_if = "Option::is_none"
    )]
    pub private_project_count: Option<i64>,
    /// The count of the private repositories
    #[serde(rename = "private_repo_count", skip_serializing_if = "Option::is_none")]
    pub private_repo_count: Option<i64>,
    /// The count of the public projects
    #[serde(
        rename = "public_project_count",
        skip_serializing_if = "Option::is_none"
    )]
    pub public_project_count: Option<i64>,
    /// The count of the public repositories
    #[serde(rename = "public_repo_count", skip_serializing_if = "Option::is_none")]
    pub public_repo_count: Option<i64>,
    /// The count of the total projects, only be seen by the system admin
    #[serde(
        rename = "total_project_count",
        skip_serializing_if = "Option::is_none"
    )]
    pub total_project_count: Option<i64>,
    /// The count of the total repositories, only be seen by the system admin
    #[serde(rename = "total_repo_count", skip_serializing_if = "Option::is_none")]
    pub total_repo_count: Option<i64>,
    /// The total storage consumption of blobs, only be seen by the system admin
    #[serde(
        rename = "total_storage_consumption",
        skip_serializing_if = "Option::is_none"
    )]
    pub total_storage_consumption: Option<i64>,
}

impl Statistic {
    pub fn new() -> Statistic {
        Statistic {
            private_project_count: None,
            private_repo_count: None,
            public_project_count: None,
            public_repo_count: None,
            total_project_count: None,
            total_repo_count: None,
            total_storage_consumption: None,
        }
    }
}
