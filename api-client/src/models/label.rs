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
pub struct Label {
    /// The ID of the label
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// The name the label
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The description the label
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The color the label
    #[serde(rename = "color", skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    /// The scope the label
    #[serde(rename = "scope", skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    /// The ID of project that the label belongs to
    #[serde(rename = "project_id", skip_serializing_if = "Option::is_none")]
    pub project_id: Option<i64>,
    /// The creation time the label
    #[serde(rename = "creation_time", skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<String>,
    /// The update time of the label
    #[serde(rename = "update_time", skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
}

impl Label {
    pub fn new() -> Label {
        Label {
            id: None,
            name: None,
            description: None,
            color: None,
            scope: None,
            project_id: None,
            creation_time: None,
            update_time: None,
        }
    }
}
