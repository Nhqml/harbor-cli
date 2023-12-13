# \UserApi

All URIs are relative to *http://localhost/api/v2.0*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_user**](UserApi.md#create_user) | **POST** /users | Create a local user.
[**delete_user**](UserApi.md#delete_user) | **DELETE** /users/{user_id} | Mark a registered user as be removed.
[**get_current_user_info**](UserApi.md#get_current_user_info) | **GET** /users/current | Get current user info.
[**get_current_user_permissions**](UserApi.md#get_current_user_permissions) | **GET** /users/current/permissions | Get current user permissions.
[**get_user**](UserApi.md#get_user) | **GET** /users/{user_id} | Get a user's profile.
[**list_users**](UserApi.md#list_users) | **GET** /users | List users
[**search_users**](UserApi.md#search_users) | **GET** /users/search | Search users by username
[**set_cli_secret**](UserApi.md#set_cli_secret) | **PUT** /users/{user_id}/cli_secret | Set CLI secret for a user.
[**set_user_sys_admin**](UserApi.md#set_user_sys_admin) | **PUT** /users/{user_id}/sysadmin | Update a registered user to change to be an administrator of Harbor.
[**update_user_password**](UserApi.md#update_user_password) | **PUT** /users/{user_id}/password | Change the password on a user that already exists.
[**update_user_profile**](UserApi.md#update_user_profile) | **PUT** /users/{user_id} | Update user's profile.



## create_user

> create_user(user_req, x_request_id)
Create a local user.

This API can be used only when the authentication mode is for local DB.  When self registration is disabled.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_req** | [**UserCreationReq**](UserCreationReq.md) | The new user | [required] |
**x_request_id** | Option<**String**> | An unique ID for the request |  |

### Return type

 (empty response body)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_user

> delete_user(user_id, x_request_id)
Mark a registered user as be removed.

This endpoint let administrator of Harbor mark a registered user as removed.It actually won't be deleted from DB. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **i32** | User ID for marking as to be removed. | [required] |
**x_request_id** | Option<**String**> | An unique ID for the request |  |

### Return type

 (empty response body)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_current_user_info

> crate::models::UserResp get_current_user_info(x_request_id)
Get current user info.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_request_id** | Option<**String**> | An unique ID for the request |  |

### Return type

[**crate::models::UserResp**](UserResp.md)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_current_user_permissions

> Vec<crate::models::Permission> get_current_user_permissions(x_request_id, scope, relative)
Get current user permissions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_request_id** | Option<**String**> | An unique ID for the request |  |
**scope** | Option<**String**> | The scope for the permission |  |
**relative** | Option<**bool**> | If true, the resources in the response are relative to the scope, eg for resource '/project/1/repository' if relative is 'true' then the resource in response will be 'repository'.  |  |

### Return type

[**Vec<crate::models::Permission>**](Permission.md)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user

> crate::models::UserResp get_user(user_id, x_request_id)
Get a user's profile.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **i32** |  | [required] |
**x_request_id** | Option<**String**> | An unique ID for the request |  |

### Return type

[**crate::models::UserResp**](UserResp.md)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_users

> Vec<crate::models::UserResp> list_users(x_request_id, q, sort, page, page_size)
List users

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_request_id** | Option<**String**> | An unique ID for the request |  |
**q** | Option<**String**> | Query string to query resources. Supported query patterns are \"exact match(k=v)\", \"fuzzy match(k=~v)\", \"range(k=[min~max])\", \"list with union releationship(k={v1 v2 v3})\" and \"list with intersetion relationship(k=(v1 v2 v3))\". The value of range and list can be string(enclosed by \" or '), integer or time(in format \"2020-04-09 02:36:00\"). All of these query patterns should be put in the query string \"q=xxx\" and splitted by \",\". e.g. q=k1=v1,k2=~v2,k3=[min~max] |  |
**sort** | Option<**String**> | Sort the resource list in ascending or descending order. e.g. sort by field1 in ascending order and field2 in descending order with \"sort=field1,-field2\" |  |
**page** | Option<**i64**> | The page number |  |[default to 1]
**page_size** | Option<**i64**> | The size of per page |  |[default to 10]

### Return type

[**Vec<crate::models::UserResp>**](UserResp.md)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_users

> Vec<crate::models::UserSearchRespItem> search_users(username, x_request_id, page, page_size)
Search users by username

This endpoint is to search the users by username.  It's open for all authenticated requests. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | **String** | Username for filtering results. | [required] |
**x_request_id** | Option<**String**> | An unique ID for the request |  |
**page** | Option<**i64**> | The page number |  |[default to 1]
**page_size** | Option<**i64**> | The size of per page |  |[default to 10]

### Return type

[**Vec<crate::models::UserSearchRespItem>**](UserSearchRespItem.md)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_cli_secret

> set_cli_secret(user_id, secret, x_request_id)
Set CLI secret for a user.

This endpoint let user generate a new CLI secret for himself.  This API only works when auth mode is set to 'OIDC'. Once this API returns with successful status, the old secret will be invalid, as there will be only one CLI secret for a user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **i32** | User ID | [required] |
**secret** | [**OidcCliSecretReq**](OidcCliSecretReq.md) |  | [required] |
**x_request_id** | Option<**String**> | An unique ID for the request |  |

### Return type

 (empty response body)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_user_sys_admin

> set_user_sys_admin(user_id, sysadmin_flag, x_request_id)
Update a registered user to change to be an administrator of Harbor.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **i32** |  | [required] |
**sysadmin_flag** | [**UserSysAdminFlag**](UserSysAdminFlag.md) | Toggle a user to admin or not. | [required] |
**x_request_id** | Option<**String**> | An unique ID for the request |  |

### Return type

 (empty response body)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_user_password

> update_user_password(user_id, password, x_request_id)
Change the password on a user that already exists.

This endpoint is for user to update password. Users with the admin role can change any user's password. Regular users can change only their own password. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **i32** |  | [required] |
**password** | [**PasswordReq**](PasswordReq.md) | Password to be updated, the attribute 'old_password' is optional when the API is called by the system administrator. | [required] |
**x_request_id** | Option<**String**> | An unique ID for the request |  |

### Return type

 (empty response body)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_user_profile

> update_user_profile(user_id, profile, x_request_id)
Update user's profile.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **i32** | Registered user ID | [required] |
**profile** | [**UserProfile**](UserProfile.md) | Only email, realname and comment can be modified. | [required] |
**x_request_id** | Option<**String**> | An unique ID for the request |  |

### Return type

 (empty response body)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

