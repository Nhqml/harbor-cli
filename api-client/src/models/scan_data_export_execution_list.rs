/*
 * Harbor API
 *
 * These APIs provide services for manipulating Harbor project.
 *
 * The version of the OpenAPI document: 2.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// ScanDataExportExecutionList : The list of scan data export executions

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScanDataExportExecutionList {
    /// The list of scan data export executions
    #[serde(rename = "items", skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<crate::models::ScanDataExportExecution>>,
}

impl ScanDataExportExecutionList {
    /// The list of scan data export executions
    pub fn new() -> ScanDataExportExecutionList {
        ScanDataExportExecutionList { items: None }
    }
}
