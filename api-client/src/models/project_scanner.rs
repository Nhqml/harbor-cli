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
pub struct ProjectScanner {
    /// The identifier of the scanner registration
    #[serde(rename = "uuid")]
    pub uuid: String,
}

impl ProjectScanner {
    pub fn new(uuid: String) -> ProjectScanner {
        ProjectScanner { uuid }
    }
}
