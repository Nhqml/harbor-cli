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
pub struct Access {
    /// The resource of the access. Possible resources are *, artifact, artifact-addition, artifact-label, audit-log, catalog, configuration, distribution, garbage-collection, helm-chart, helm-chart-version, helm-chart-version-label, immutable-tag, label, ldap-user, log, member, metadata, notification-policy, preheat-instance, preheat-policy, project, quota, registry, replication, replication-adapter, replication-policy, repository, robot, scan, scan-all, scanner, system-volumes, tag, tag-retention, user, user-group or \"\" (for self-reference).
    #[serde(rename = "resource", skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    /// The action of the access. Possible actions are *, pull, push, create, read, update, delete, list, operate, scanner-pull and stop.
    #[serde(rename = "action", skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    /// The effect of the access
    #[serde(rename = "effect", skip_serializing_if = "Option::is_none")]
    pub effect: Option<String>,
}

impl Access {
    pub fn new() -> Access {
        Access {
            resource: None,
            action: None,
            effect: None,
        }
    }
}
