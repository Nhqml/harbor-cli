/*
 * Harbor API
 *
 * These APIs provide services for manipulating Harbor project.
 *
 * The version of the OpenAPI document: 2.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// RobotSec : The response for refresh/update robot account secret.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RobotSec {
    /// The secret of the robot
    #[serde(rename = "secret", skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
}

impl RobotSec {
    /// The response for refresh/update robot account secret.
    pub fn new() -> RobotSec {
        RobotSec { secret: None }
    }
}
