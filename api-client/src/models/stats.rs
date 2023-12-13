/*
 * Harbor API
 *
 * These APIs provide services for manipulating Harbor project.
 *
 * The version of the OpenAPI document: 2.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// Stats : Stats provides the overall progress of the scan all process.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Stats {
    /// The total number of scan processes triggered by the scan all action
    #[serde(rename = "total", skip_serializing_if = "Option::is_none")]
    pub total: Option<i32>,
    /// The number of the finished scan processes triggered by the scan all action
    #[serde(rename = "completed", skip_serializing_if = "Option::is_none")]
    pub completed: Option<i32>,
    /// The metrics data for the each status
    #[serde(rename = "metrics", skip_serializing_if = "Option::is_none")]
    pub metrics: Option<::std::collections::HashMap<String, i32>>,
    /// A flag indicating job status of scan all.
    #[serde(rename = "ongoing", skip_serializing_if = "Option::is_none")]
    pub ongoing: Option<bool>,
    /// The trigger of the scan all job.
    #[serde(rename = "trigger", skip_serializing_if = "Option::is_none")]
    pub trigger: Option<Trigger>,
}

impl Stats {
    /// Stats provides the overall progress of the scan all process.
    pub fn new() -> Stats {
        Stats {
            total: None,
            completed: None,
            metrics: None,
            ongoing: None,
            trigger: None,
        }
    }
}

/// The trigger of the scan all job.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Trigger {
    #[serde(rename = "Manual")]
    Manual,
    #[serde(rename = "Schedule")]
    Schedule,
    #[serde(rename = "Event")]
    Event,
}

impl Default for Trigger {
    fn default() -> Trigger {
        Self::Manual
    }
}