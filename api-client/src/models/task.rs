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
pub struct Task {
    /// The ID of task
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    /// The ID of task execution
    #[serde(rename = "execution_id", skip_serializing_if = "Option::is_none")]
    pub execution_id: Option<i32>,
    /// The status of task
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// The status message of task
    #[serde(rename = "status_message", skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
    /// The count of task run
    #[serde(rename = "run_count", skip_serializing_if = "Option::is_none")]
    pub run_count: Option<i32>,
    #[serde(rename = "extra_attrs", skip_serializing_if = "Option::is_none")]
    pub extra_attrs: Option<::std::collections::HashMap<String, serde_json::Value>>,
    /// The creation time of task
    #[serde(rename = "creation_time", skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<String>,
    /// The update time of task
    #[serde(rename = "update_time", skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
    /// The start time of task
    #[serde(rename = "start_time", skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// The end time of task
    #[serde(rename = "end_time", skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
}

impl Task {
    pub fn new() -> Task {
        Task {
            id: None,
            execution_id: None,
            status: None,
            status_message: None,
            run_count: None,
            extra_attrs: None,
            creation_time: None,
            update_time: None,
            start_time: None,
            end_time: None,
        }
    }
}
