# Instance

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> | Unique ID | [optional]
**name** | Option<**String**> | Instance name | [optional]
**description** | Option<**String**> | Description of instance | [optional]
**vendor** | Option<**String**> | Based on which driver, identified by ID | [optional]
**endpoint** | Option<**String**> | The service endpoint of this instance | [optional]
**auth_mode** | Option<**String**> | The authentication way supported | [optional]
**auth_info** | Option<**::std::collections::HashMap<String, String>**> | The auth credential data if exists | [optional]
**status** | Option<**String**> | The health status | [optional]
**enabled** | Option<**bool**> | Whether the instance is activated or not | [optional]
**default** | Option<**bool**> | Whether the instance is default or not | [optional]
**insecure** | Option<**bool**> | Whether the instance endpoint is insecure or not | [optional]
**setup_timestamp** | Option<**i64**> | The timestamp of instance setting up | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


