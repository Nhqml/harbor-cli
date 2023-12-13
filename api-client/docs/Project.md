# Project

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**project_id** | Option<**i32**> | Project ID | [optional]
**owner_id** | Option<**i32**> | The owner ID of the project always means the creator of the project. | [optional]
**name** | Option<**String**> | The name of the project. | [optional]
**registry_id** | Option<**i64**> | The ID of referenced registry when the project is a proxy cache project. | [optional]
**creation_time** | Option<**String**> | The creation time of the project. | [optional]
**update_time** | Option<**String**> | The update time of the project. | [optional]
**deleted** | Option<**bool**> | A deletion mark of the project. | [optional]
**owner_name** | Option<**String**> | The owner name of the project. | [optional]
**togglable** | Option<**bool**> | Correspond to the UI about whether the project's publicity is  updatable (for UI) | [optional]
**current_user_role_id** | Option<**i32**> | The role ID with highest permission of the current user who triggered the API (for UI).  This attribute is deprecated and will be removed in future versions. | [optional]
**current_user_role_ids** | Option<**Vec<i32>**> | The list of role ID of the current user who triggered the API (for UI) | [optional]
**repo_count** | Option<**i32**> | The number of the repositories under this project. | [optional]
**metadata** | Option<[**crate::models::ProjectMetadata**](ProjectMetadata.md)> |  | [optional]
**cve_allowlist** | Option<[**crate::models::CveAllowlist**](CVEAllowlist.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


