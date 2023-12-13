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
pub struct UserGroup {
    /// The ID of the user group
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    /// The name of the user group
    #[serde(rename = "group_name", skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
    /// The group type, 1 for LDAP group, 2 for HTTP group, 3 for OIDC group.
    #[serde(rename = "group_type", skip_serializing_if = "Option::is_none")]
    pub group_type: Option<i32>,
    /// The DN of the LDAP group if group type is 1 (LDAP group).
    #[serde(rename = "ldap_group_dn", skip_serializing_if = "Option::is_none")]
    pub ldap_group_dn: Option<String>,
}

impl UserGroup {
    pub fn new() -> UserGroup {
        UserGroup {
            id: None,
            group_name: None,
            group_type: None,
            ldap_group_dn: None,
        }
    }
}
