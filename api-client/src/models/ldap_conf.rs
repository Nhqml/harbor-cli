/*
 * Harbor API
 *
 * These APIs provide services for manipulating Harbor project.
 *
 * The version of the OpenAPI document: 2.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// LdapConf : The ldap configure properties

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LdapConf {
    /// The url of ldap service.
    #[serde(rename = "ldap_url", skip_serializing_if = "Option::is_none")]
    pub ldap_url: Option<String>,
    /// The search dn of ldap service.
    #[serde(rename = "ldap_search_dn", skip_serializing_if = "Option::is_none")]
    pub ldap_search_dn: Option<String>,
    /// The search password of ldap service.
    #[serde(
        rename = "ldap_search_password",
        skip_serializing_if = "Option::is_none"
    )]
    pub ldap_search_password: Option<String>,
    /// The base dn of ldap service.
    #[serde(rename = "ldap_base_dn", skip_serializing_if = "Option::is_none")]
    pub ldap_base_dn: Option<String>,
    /// The serach filter of ldap service.
    #[serde(rename = "ldap_filter", skip_serializing_if = "Option::is_none")]
    pub ldap_filter: Option<String>,
    /// The serach uid from ldap service attributes.
    #[serde(rename = "ldap_uid", skip_serializing_if = "Option::is_none")]
    pub ldap_uid: Option<String>,
    /// The serach scope of ldap service.
    #[serde(rename = "ldap_scope", skip_serializing_if = "Option::is_none")]
    pub ldap_scope: Option<i64>,
    /// The connect timeout of ldap service(second).
    #[serde(
        rename = "ldap_connection_timeout",
        skip_serializing_if = "Option::is_none"
    )]
    pub ldap_connection_timeout: Option<i64>,
    /// Verify Ldap server certificate.
    #[serde(rename = "ldap_verify_cert", skip_serializing_if = "Option::is_none")]
    pub ldap_verify_cert: Option<bool>,
}

impl LdapConf {
    /// The ldap configure properties
    pub fn new() -> LdapConf {
        LdapConf {
            ldap_url: None,
            ldap_search_dn: None,
            ldap_search_password: None,
            ldap_base_dn: None,
            ldap_filter: None,
            ldap_uid: None,
            ldap_scope: None,
            ldap_connection_timeout: None,
            ldap_verify_cert: None,
        }
    }
}