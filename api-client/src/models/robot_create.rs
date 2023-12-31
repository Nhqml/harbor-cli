/*
 * Harbor API
 *
 * These APIs provide services for manipulating Harbor project.
 *
 * The version of the OpenAPI document: 2.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// RobotCreate : The request for robot account creation.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RobotCreate {
    /// The name of the robot
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The description of the robot
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The secret of the robot
    #[serde(rename = "secret", skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
    /// The level of the robot, project or system
    #[serde(rename = "level", skip_serializing_if = "Option::is_none")]
    pub level: Option<String>,
    /// The disable status of the robot
    #[serde(rename = "disable", skip_serializing_if = "Option::is_none")]
    pub disable: Option<bool>,
    /// The duration of the robot in days
    #[serde(rename = "duration", skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,
    #[serde(rename = "permissions", skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<crate::models::RobotPermission>>,
}

impl RobotCreate {
    /// The request for robot account creation.
    pub fn new() -> RobotCreate {
        RobotCreate {
            name: None,
            description: None,
            secret: None,
            level: None,
            disable: None,
            duration: None,
            permissions: None,
        }
    }
}
