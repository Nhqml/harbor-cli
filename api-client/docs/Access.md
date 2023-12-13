# Access

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**resource** | Option<**String**> | The resource of the access. Possible resources are *, artifact, artifact-addition, artifact-label, audit-log, catalog, configuration, distribution, garbage-collection, helm-chart, helm-chart-version, helm-chart-version-label, immutable-tag, label, ldap-user, log, member, metadata, notification-policy, preheat-instance, preheat-policy, project, quota, registry, replication, replication-adapter, replication-policy, repository, robot, scan, scan-all, scanner, system-volumes, tag, tag-retention, user, user-group or \"\" (for self-reference). | [optional]
**action** | Option<**String**> | The action of the access. Possible actions are *, pull, push, create, read, update, delete, list, operate, scanner-pull and stop. | [optional]
**effect** | Option<**String**> | The effect of the access | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


