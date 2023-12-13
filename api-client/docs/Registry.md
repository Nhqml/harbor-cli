# Registry

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i64**> | The registry ID. | [optional]
**url** | Option<**String**> | The registry URL string. | [optional]
**name** | Option<**String**> | The registry name. | [optional]
**credential** | Option<[**crate::models::RegistryCredential**](RegistryCredential.md)> |  | [optional]
**r#type** | Option<**String**> | Type of the registry, e.g. 'harbor'. | [optional]
**insecure** | Option<**bool**> | Whether or not the certificate will be verified when Harbor tries to access the server. | [optional]
**description** | Option<**String**> | Description of the registry. | [optional]
**status** | Option<**String**> | Health status of the registry. | [optional]
**creation_time** | Option<**String**> | The create time of the policy. | [optional]
**update_time** | Option<**String**> | The update time of the policy. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


