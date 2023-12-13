# \OidcApi

All URIs are relative to *http://localhost/api/v2.0*

Method | HTTP request | Description
------------- | ------------- | -------------
[**ping_oidc**](OidcApi.md#ping_oidc) | **POST** /system/oidc/ping | Test the OIDC endpoint.



## ping_oidc

> ping_oidc(endpoint, x_request_id)
Test the OIDC endpoint.

Test the OIDC endpoint, the setting of the endpoint is provided in the request.  This API can only be called by system admin. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**endpoint** | [**PingOidcRequest**](PingOidcRequest.md) | Request body for OIDC endpoint to be tested. | [required] |
**x_request_id** | Option<**String**> | An unique ID for the request |  |

### Return type

 (empty response body)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

