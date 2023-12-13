# \UsergroupApi

All URIs are relative to *http://localhost/api/v2.0*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_user_group**](UsergroupApi.md#create_user_group) | **POST** /usergroups | Create user group
[**delete_user_group**](UsergroupApi.md#delete_user_group) | **DELETE** /usergroups/{group_id} | Delete user group
[**get_user_group**](UsergroupApi.md#get_user_group) | **GET** /usergroups/{group_id} | Get user group information
[**list_user_groups**](UsergroupApi.md#list_user_groups) | **GET** /usergroups | Get all user groups information
[**search_user_groups**](UsergroupApi.md#search_user_groups) | **GET** /usergroups/search | Search groups by groupname
[**update_user_group**](UsergroupApi.md#update_user_group) | **PUT** /usergroups/{group_id} | Update group information



## create_user_group

> create_user_group(x_request_id, usergroup)
Create user group

Create user group information

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_request_id** | Option<**String**> | An unique ID for the request |  |
**usergroup** | Option<[**UserGroup**](UserGroup.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_user_group

> delete_user_group(group_id, x_request_id)
Delete user group

Delete user group

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **i32** |  | [required] |
**x_request_id** | Option<**String**> | An unique ID for the request |  |

### Return type

 (empty response body)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_group

> crate::models::UserGroup get_user_group(group_id, x_request_id)
Get user group information

Get user group information

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **i64** | Group ID | [required] |
**x_request_id** | Option<**String**> | An unique ID for the request |  |

### Return type

[**crate::models::UserGroup**](UserGroup.md)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_user_groups

> Vec<crate::models::UserGroup> list_user_groups(x_request_id, page, page_size, ldap_group_dn, group_name)
Get all user groups information

Get all user groups information, it is open for system admin

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_request_id** | Option<**String**> | An unique ID for the request |  |
**page** | Option<**i64**> | The page number |  |[default to 1]
**page_size** | Option<**i64**> | The size of per page |  |[default to 10]
**ldap_group_dn** | Option<**String**> | search with ldap group DN |  |
**group_name** | Option<**String**> | group name need to search, fuzzy matches |  |

### Return type

[**Vec<crate::models::UserGroup>**](UserGroup.md)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_user_groups

> Vec<crate::models::UserGroupSearchItem> search_user_groups(groupname, x_request_id, page, page_size)
Search groups by groupname

This endpoint is to search groups by group name.  It's open for all authenticated requests. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**groupname** | **String** | Group name for filtering results. | [required] |
**x_request_id** | Option<**String**> | An unique ID for the request |  |
**page** | Option<**i64**> | The page number |  |[default to 1]
**page_size** | Option<**i64**> | The size of per page |  |[default to 10]

### Return type

[**Vec<crate::models::UserGroupSearchItem>**](UserGroupSearchItem.md)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_user_group

> update_user_group(group_id, x_request_id, usergroup)
Update group information

Update user group information

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **i64** | Group ID | [required] |
**x_request_id** | Option<**String**> | An unique ID for the request |  |
**usergroup** | Option<[**UserGroup**](UserGroup.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

