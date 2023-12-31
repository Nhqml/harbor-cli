/*
 * Harbor API
 *
 * These APIs provide services for manipulating Harbor project.
 *
 * The version of the OpenAPI document: 2.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// FilterStyle : The style of the resource filter

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FilterStyle {
    /// The filter type
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// The filter style
    #[serde(rename = "style", skip_serializing_if = "Option::is_none")]
    pub style: Option<String>,
    /// The filter values
    #[serde(rename = "values", skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

impl FilterStyle {
    /// The style of the resource filter
    pub fn new() -> FilterStyle {
        FilterStyle {
            r#type: None,
            style: None,
            values: None,
        }
    }
}
