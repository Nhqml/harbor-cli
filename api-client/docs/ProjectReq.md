# ProjectReq

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**project_name** | Option<**String**> | The name of the project. | [optional]
**public** | Option<**bool**> | deprecated, reserved for project creation in replication | [optional]
**metadata** | Option<[**crate::models::ProjectMetadata**](ProjectMetadata.md)> |  | [optional]
**cve_allowlist** | Option<[**crate::models::CveAllowlist**](CVEAllowlist.md)> |  | [optional]
**storage_limit** | Option<**i64**> | The storage quota of the project. | [optional]
**registry_id** | Option<**i64**> | The ID of referenced registry when creating the proxy cache project | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


