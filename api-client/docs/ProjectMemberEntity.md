# ProjectMemberEntity

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> | the project member id | [optional]
**project_id** | Option<**i32**> | the project id | [optional]
**entity_name** | Option<**String**> | the name of the group member. | [optional]
**role_name** | Option<**String**> | the name of the role | [optional]
**role_id** | Option<**i32**> | the role id | [optional]
**entity_id** | Option<**i32**> | the id of entity, if the member is a user, it is user_id in user table. if the member is a user group, it is the user group's ID in user_group table. | [optional]
**entity_type** | Option<**String**> | the entity's type, u for user entity, g for group entity. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


