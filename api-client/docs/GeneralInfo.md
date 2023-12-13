# GeneralInfo

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**banner_message** | Option<**String**> | The banner message for the UI. It is the stringified result of the banner message object. | [optional]
**current_time** | Option<**String**> | The current time of the server. | [optional]
**registry_url** | Option<**String**> | The url of registry against which the docker command should be issued. | [optional]
**external_url** | Option<**String**> | The external URL of Harbor, with protocol. | [optional]
**auth_mode** | Option<**String**> | The auth mode of current Harbor instance. | [optional]
**primary_auth_mode** | Option<**bool**> | The flag to indicate whether the current auth mode should consider as a primary one. | [optional]
**project_creation_restriction** | Option<**String**> | Indicate who can create projects, it could be 'adminonly' or 'everyone'. | [optional]
**self_registration** | Option<**bool**> | Indicate whether the Harbor instance enable user to register himself. | [optional]
**has_ca_root** | Option<**bool**> | Indicate whether there is a ca root cert file ready for download in the file system. | [optional]
**harbor_version** | Option<**String**> | The build version of Harbor. | [optional]
**registry_storage_provider_name** | Option<**String**> | The storage provider's name of Harbor registry | [optional]
**read_only** | Option<**bool**> | The flag to indicate whether Harbor is in readonly mode. | [optional]
**notification_enable** | Option<**bool**> | The flag to indicate whether notification mechanism is enabled on Harbor instance. | [optional]
**authproxy_settings** | Option<[**crate::models::AuthproxySetting**](AuthproxySetting.md)> |  | [optional]
**oidc_provider_name** | Option<**String**> | The OIDC provider name, empty if current auth is not OIDC_auth or OIDC provider is not configured. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


