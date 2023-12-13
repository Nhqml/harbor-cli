# \PermissionsApi

All URIs are relative to *http://localhost/api/v2.0*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_permissions**](PermissionsApi.md#get_permissions) | **GET** /permissions | Get system or project level permissions info.



## get_permissions

> crate::models::Permissions get_permissions(x_request_id)
Get system or project level permissions info.

This endpoint is for retrieving resource and action info that only provides for admin user(system admin and project admin). 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_request_id** | Option<**String**> | An unique ID for the request |  |

### Return type

[**crate::models::Permissions**](Permissions.md)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

