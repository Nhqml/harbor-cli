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
pub struct GeneralInfo {
    /// The banner message for the UI. It is the stringified result of the banner message object.
    #[serde(
        rename = "banner_message",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub banner_message: Option<Option<String>>,
    /// The current time of the server.
    #[serde(
        rename = "current_time",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub current_time: Option<Option<String>>,
    /// The url of registry against which the docker command should be issued.
    #[serde(
        rename = "registry_url",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub registry_url: Option<Option<String>>,
    /// The external URL of Harbor, with protocol.
    #[serde(
        rename = "external_url",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub external_url: Option<Option<String>>,
    /// The auth mode of current Harbor instance.
    #[serde(
        rename = "auth_mode",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub auth_mode: Option<Option<String>>,
    /// The flag to indicate whether the current auth mode should consider as a primary one.
    #[serde(
        rename = "primary_auth_mode",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub primary_auth_mode: Option<Option<bool>>,
    /// Indicate who can create projects, it could be 'adminonly' or 'everyone'.
    #[serde(
        rename = "project_creation_restriction",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub project_creation_restriction: Option<Option<String>>,
    /// Indicate whether the Harbor instance enable user to register himself.
    #[serde(
        rename = "self_registration",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub self_registration: Option<Option<bool>>,
    /// Indicate whether there is a ca root cert file ready for download in the file system.
    #[serde(
        rename = "has_ca_root",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub has_ca_root: Option<Option<bool>>,
    /// The build version of Harbor.
    #[serde(
        rename = "harbor_version",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub harbor_version: Option<Option<String>>,
    /// The storage provider's name of Harbor registry
    #[serde(
        rename = "registry_storage_provider_name",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub registry_storage_provider_name: Option<Option<String>>,
    /// The flag to indicate whether Harbor is in readonly mode.
    #[serde(
        rename = "read_only",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub read_only: Option<Option<bool>>,
    /// The flag to indicate whether notification mechanism is enabled on Harbor instance.
    #[serde(
        rename = "notification_enable",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub notification_enable: Option<Option<bool>>,
    #[serde(rename = "authproxy_settings", skip_serializing_if = "Option::is_none")]
    pub authproxy_settings: Option<Box<crate::models::AuthproxySetting>>,
    /// The OIDC provider name, empty if current auth is not OIDC_auth or OIDC provider is not configured.
    #[serde(
        rename = "oidc_provider_name",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub oidc_provider_name: Option<Option<String>>,
}

impl GeneralInfo {
    pub fn new() -> GeneralInfo {
        GeneralInfo {
            banner_message: None,
            current_time: None,
            registry_url: None,
            external_url: None,
            auth_mode: None,
            primary_auth_mode: None,
            project_creation_restriction: None,
            self_registration: None,
            has_ca_root: None,
            harbor_version: None,
            registry_storage_provider_name: None,
            read_only: None,
            notification_enable: None,
            authproxy_settings: None,
            oidc_provider_name: None,
        }
    }
}