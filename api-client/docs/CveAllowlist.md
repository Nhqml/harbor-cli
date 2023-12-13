# CveAllowlist

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> | ID of the allowlist | [optional]
**project_id** | Option<**i32**> | ID of the project which the allowlist belongs to.  For system level allowlist this attribute is zero. | [optional]
**expires_at** | Option<**i32**> | the time for expiration of the allowlist, in the form of seconds since epoch.  This is an optional attribute, if it's not set the CVE allowlist does not expire. | [optional]
**items** | Option<[**Vec<crate::models::CveAllowlistItem>**](CVEAllowlistItem.md)> |  | [optional]
**creation_time** | Option<**String**> | The creation time of the allowlist. | [optional]
**update_time** | Option<**String**> | The update time of the allowlist. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


