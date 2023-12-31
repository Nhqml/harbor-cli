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
pub struct GcHistory {
    /// the id of gc job.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    /// the job name of gc job.
    #[serde(rename = "job_name", skip_serializing_if = "Option::is_none")]
    pub job_name: Option<String>,
    /// the job kind of gc job.
    #[serde(rename = "job_kind", skip_serializing_if = "Option::is_none")]
    pub job_kind: Option<String>,
    /// the job parameters of gc job.
    #[serde(rename = "job_parameters", skip_serializing_if = "Option::is_none")]
    pub job_parameters: Option<String>,
    #[serde(rename = "schedule", skip_serializing_if = "Option::is_none")]
    pub schedule: Option<Box<crate::models::ScheduleObj>>,
    /// the status of gc job.
    #[serde(rename = "job_status", skip_serializing_if = "Option::is_none")]
    pub job_status: Option<String>,
    /// if gc job was deleted.
    #[serde(rename = "deleted", skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,
    /// the creation time of gc job.
    #[serde(rename = "creation_time", skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<String>,
    /// the update time of gc job.
    #[serde(rename = "update_time", skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
}

impl GcHistory {
    pub fn new() -> GcHistory {
        GcHistory {
            id: None,
            job_name: None,
            job_kind: None,
            job_parameters: None,
            schedule: None,
            job_status: None,
            deleted: None,
            creation_time: None,
            update_time: None,
        }
    }
}
