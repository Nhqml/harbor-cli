# \MemberApi

All URIs are relative to *http://localhost/api/v2.0*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_project_member**](MemberApi.md#create_project_member) | **POST** /projects/{project_name_or_id}/members | Create project member
[**delete_project_member**](MemberApi.md#delete_project_member) | **DELETE** /projects/{project_name_or_id}/members/{mid} | Delete project member
[**get_project_member**](MemberApi.md#get_project_member) | **GET** /projects/{project_name_or_id}/members/{mid} | Get the project member information
[**list_project_members**](MemberApi.md#list_project_members) | **GET** /projects/{project_name_or_id}/members | Get all project member information
[**update_project_member**](MemberApi.md#update_project_member) | **PUT** /projects/{project_name_or_id}/members/{mid} | Update project member



## create_project_member

> create_project_member(project_name_or_id, x_request_id, x_is_resource_name, project_member)
Create project member

Create project member relationship, the member can be one of the user_member and group_member,  The user_member need to specify user_id or username. If the user already exist in harbor DB, specify the user_id,  If does not exist in harbor DB, it will SearchAndOnBoard the user. The group_member need to specify id or ldap_group_dn. If the group already exist in harbor DB. specify the user group's id,  If does not exist, it will SearchAndOnBoard the group. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_name_or_id** | **String** | The name or id of the project | [required] |
**x_request_id** | Option<**String**> | An unique ID for the request |  |
**x_is_resource_name** | Option<**bool**> | The flag to indicate whether the parameter which supports both name and id in the path is the name of the resource. When the X-Is-Resource-Name is false and the parameter can be converted to an integer, the parameter will be as an id, otherwise, it will be as a name. |  |[default to false]
**project_member** | Option<[**ProjectMember**](ProjectMember.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_project_member

> delete_project_member(project_name_or_id, mid, x_request_id, x_is_resource_name)
Delete project member

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_name_or_id** | **String** | The name or id of the project | [required] |
**mid** | **i64** | Member ID. | [required] |
**x_request_id** | Option<**String**> | An unique ID for the request |  |
**x_is_resource_name** | Option<**bool**> | The flag to indicate whether the parameter which supports both name and id in the path is the name of the resource. When the X-Is-Resource-Name is false and the parameter can be converted to an integer, the parameter will be as an id, otherwise, it will be as a name. |  |[default to false]

### Return type

 (empty response body)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_project_member

> crate::models::ProjectMemberEntity get_project_member(project_name_or_id, mid, x_request_id, x_is_resource_name)
Get the project member information

Get the project member information

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_name_or_id** | **String** | The name or id of the project | [required] |
**mid** | **i64** | The member ID | [required] |
**x_request_id** | Option<**String**> | An unique ID for the request |  |
**x_is_resource_name** | Option<**bool**> | The flag to indicate whether the parameter which supports both name and id in the path is the name of the resource. When the X-Is-Resource-Name is false and the parameter can be converted to an integer, the parameter will be as an id, otherwise, it will be as a name. |  |[default to false]

### Return type

[**crate::models::ProjectMemberEntity**](ProjectMemberEntity.md)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_project_members

> Vec<crate::models::ProjectMemberEntity> list_project_members(project_name_or_id, x_request_id, x_is_resource_name, page, page_size, entityname)
Get all project member information

Get all project member information

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_name_or_id** | **String** | The name or id of the project | [required] |
**x_request_id** | Option<**String**> | An unique ID for the request |  |
**x_is_resource_name** | Option<**bool**> | The flag to indicate whether the parameter which supports both name and id in the path is the name of the resource. When the X-Is-Resource-Name is false and the parameter can be converted to an integer, the parameter will be as an id, otherwise, it will be as a name. |  |[default to false]
**page** | Option<**i64**> | The page number |  |[default to 1]
**page_size** | Option<**i64**> | The size of per page |  |[default to 10]
**entityname** | Option<**String**> | The entity name to search. |  |

### Return type

[**Vec<crate::models::ProjectMemberEntity>**](ProjectMemberEntity.md)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_project_member

> update_project_member(project_name_or_id, mid, x_request_id, x_is_resource_name, role)
Update project member

Update project member relationship

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_name_or_id** | **String** | The name or id of the project | [required] |
**mid** | **i64** | Member ID. | [required] |
**x_request_id** | Option<**String**> | An unique ID for the request |  |
**x_is_resource_name** | Option<**bool**> | The flag to indicate whether the parameter which supports both name and id in the path is the name of the resource. When the X-Is-Resource-Name is false and the parameter can be converted to an integer, the parameter will be as an id, otherwise, it will be as a name. |  |[default to false]
**role** | Option<[**RoleRequest**](RoleRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

