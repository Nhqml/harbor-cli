# \SystemCveAllowlistApi

All URIs are relative to *http://localhost/api/v2.0*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_system_cve_allowlist**](SystemCveAllowlistApi.md#get_system_cve_allowlist) | **GET** /system/CVEAllowlist | Get the system level allowlist of CVE.
[**put_system_cve_allowlist**](SystemCveAllowlistApi.md#put_system_cve_allowlist) | **PUT** /system/CVEAllowlist | Update the system level allowlist of CVE.



## get_system_cve_allowlist

> crate::models::CveAllowlist get_system_cve_allowlist(x_request_id)
Get the system level allowlist of CVE.

Get the system level allowlist of CVE.  This API can be called by all authenticated users.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_request_id** | Option<**String**> | An unique ID for the request |  |

### Return type

[**crate::models::CveAllowlist**](CVEAllowlist.md)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_system_cve_allowlist

> put_system_cve_allowlist(x_request_id, allowlist)
Update the system level allowlist of CVE.

This API overwrites the system level allowlist of CVE with the list in request body.  Only system Admin has permission to call this API.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_request_id** | Option<**String**> | An unique ID for the request |  |
**allowlist** | Option<[**CveAllowlist**](CveAllowlist.md)> | The allowlist with new content |  |

### Return type

 (empty response body)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

