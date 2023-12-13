# ReplicationPolicy

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i64**> | The policy ID. | [optional]
**name** | Option<**String**> | The policy name. | [optional]
**description** | Option<**String**> | The description of the policy. | [optional]
**src_registry** | Option<[**crate::models::Registry**](Registry.md)> |  | [optional]
**dest_registry** | Option<[**crate::models::Registry**](Registry.md)> |  | [optional]
**dest_namespace** | Option<**String**> | The destination namespace. | [optional]
**dest_namespace_replace_count** | Option<**i32**> | Specify how many path components will be replaced by the provided destination namespace. The default value is -1 in which case the legacy mode will be applied. | [optional]
**trigger** | Option<[**crate::models::ReplicationTrigger**](ReplicationTrigger.md)> |  | [optional]
**filters** | Option<[**Vec<crate::models::ReplicationFilter>**](ReplicationFilter.md)> | The replication policy filter array. | [optional]
**replicate_deletion** | Option<**bool**> | Whether to replicate the deletion operation. | [optional]
**deletion** | Option<**bool**> | Deprecated, use \"replicate_deletion\" instead. Whether to replicate the deletion operation. | [optional]
**r#override** | Option<**bool**> | Whether to override the resources on the destination registry. | [optional]
**enabled** | Option<**bool**> | Whether the policy is enabled or not. | [optional]
**creation_time** | Option<**String**> | The create time of the policy. | [optional]
**update_time** | Option<**String**> | The update time of the policy. | [optional]
**speed** | Option<**i32**> | speed limit for each task | [optional]
**copy_by_chunk** | Option<**bool**> | Whether to enable copy by chunk. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


