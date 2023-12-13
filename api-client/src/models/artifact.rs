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
pub struct Artifact {
    /// The ID of the artifact
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// The type of the artifact, e.g. image, chart, etc
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// The media type of the artifact
    #[serde(rename = "media_type", skip_serializing_if = "Option::is_none")]
    pub media_type: Option<String>,
    /// The manifest media type of the artifact
    #[serde(
        rename = "manifest_media_type",
        skip_serializing_if = "Option::is_none"
    )]
    pub manifest_media_type: Option<String>,
    /// The ID of the project that the artifact belongs to
    #[serde(rename = "project_id", skip_serializing_if = "Option::is_none")]
    pub project_id: Option<i64>,
    /// The ID of the repository that the artifact belongs to
    #[serde(rename = "repository_id", skip_serializing_if = "Option::is_none")]
    pub repository_id: Option<i64>,
    /// The digest of the artifact
    #[serde(rename = "digest", skip_serializing_if = "Option::is_none")]
    pub digest: Option<String>,
    /// The size of the artifact
    #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    /// The digest of the icon
    #[serde(rename = "icon", skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    /// The push time of the artifact
    #[serde(rename = "push_time", skip_serializing_if = "Option::is_none")]
    pub push_time: Option<String>,
    /// The latest pull time of the artifact
    #[serde(rename = "pull_time", skip_serializing_if = "Option::is_none")]
    pub pull_time: Option<String>,
    #[serde(rename = "extra_attrs", skip_serializing_if = "Option::is_none")]
    pub extra_attrs: Option<::std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "annotations", skip_serializing_if = "Option::is_none")]
    pub annotations: Option<::std::collections::HashMap<String, String>>,
    #[serde(rename = "references", skip_serializing_if = "Option::is_none")]
    pub references: Option<Vec<crate::models::Reference>>,
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<crate::models::Tag>>,
    #[serde(rename = "addition_links", skip_serializing_if = "Option::is_none")]
    pub addition_links: Option<::std::collections::HashMap<String, crate::models::AdditionLink>>,
    #[serde(rename = "labels", skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<crate::models::Label>>,
    /// The scan overview attached in the metadata of tag
    #[serde(rename = "scan_overview", skip_serializing_if = "Option::is_none")]
    pub scan_overview:
        Option<::std::collections::HashMap<String, crate::models::NativeReportSummary>>,
    #[serde(rename = "accessories", skip_serializing_if = "Option::is_none")]
    pub accessories: Option<Vec<crate::models::Accessory>>,
}

impl Artifact {
    pub fn new() -> Artifact {
        Artifact {
            id: None,
            r#type: None,
            media_type: None,
            manifest_media_type: None,
            project_id: None,
            repository_id: None,
            digest: None,
            size: None,
            icon: None,
            push_time: None,
            pull_time: None,
            extra_attrs: None,
            annotations: None,
            references: None,
            tags: None,
            addition_links: None,
            labels: None,
            scan_overview: None,
            accessories: None,
        }
    }
}