# WebhookPolicy

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i64**> | The webhook policy ID. | [optional]
**name** | Option<**String**> | The name of webhook policy. | [optional]
**description** | Option<**String**> | The description of webhook policy. | [optional]
**project_id** | Option<**i32**> | The project ID of webhook policy. | [optional]
**targets** | Option<[**Vec<crate::models::WebhookTargetObject>**](WebhookTargetObject.md)> |  | [optional]
**event_types** | Option<**Vec<String>**> |  | [optional]
**creator** | Option<**String**> | The creator of the webhook policy. | [optional]
**creation_time** | Option<**String**> | The create time of the webhook policy. | [optional]
**update_time** | Option<**String**> | The update time of the webhook policy. | [optional]
**enabled** | Option<**bool**> | Whether the webhook policy is enabled or not. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


