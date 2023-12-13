# \PingApi

All URIs are relative to *http://localhost/api/v2.0*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_ping**](PingApi.md#get_ping) | **GET** /ping | Ping Harbor to check if the API server is alive.



## get_ping

> String get_ping(x_request_id)
Ping Harbor to check if the API server is alive.

This API simply replies a pong to indicate the process to handle API is up, disregarding the health status of dependent components. This path does not require any authentication.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_request_id** | Option<**String**> | An unique ID for the request |  |

### Return type

**String**

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

