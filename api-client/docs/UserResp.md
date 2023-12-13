# UserResp

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**email** | Option<**String**> |  | [optional]
**realname** | Option<**String**> |  | [optional]
**comment** | Option<**String**> |  | [optional]
**user_id** | Option<**i32**> |  | [optional]
**username** | Option<**String**> |  | [optional]
**sysadmin_flag** | Option<**bool**> |  | [optional]
**admin_role_in_auth** | Option<**bool**> | indicate the admin privilege is grant by authenticator (LDAP), is always false unless it is the current login user | [optional]
**oidc_user_meta** | Option<[**crate::models::OidcUserInfo**](OIDCUserInfo.md)> |  | [optional]
**creation_time** | Option<**String**> | The creation time of the user. | [optional]
**update_time** | Option<**String**> | The update time of the user. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


