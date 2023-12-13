# \SysteminfoApi

All URIs are relative to *http://localhost/api/v2.0*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_cert**](SysteminfoApi.md#get_cert) | **GET** /systeminfo/getcert | Get default root certificate.
[**get_system_info**](SysteminfoApi.md#get_system_info) | **GET** /systeminfo | Get general system info
[**get_volumes**](SysteminfoApi.md#get_volumes) | **GET** /systeminfo/volumes | Get system volume info (total/free size).



## get_cert

> std::path::PathBuf get_cert(x_request_id)
Get default root certificate.

This endpoint is for downloading a default root certificate. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_request_id** | Option<**String**> | An unique ID for the request |  |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/octet-stream

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_system_info

> crate::models::GeneralInfo get_system_info(x_request_id)
Get general system info

This API is for retrieving general system info, this can be called by anonymous request.  Some attributes will be omitted in the response when this API is called by anonymous request. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_request_id** | Option<**String**> | An unique ID for the request |  |

### Return type

[**crate::models::GeneralInfo**](GeneralInfo.md)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_volumes

> crate::models::SystemInfo get_volumes(x_request_id)
Get system volume info (total/free size).

This endpoint is for retrieving system volume info that only provides for admin user.  Note that the response only reflects the storage status of local disk. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_request_id** | Option<**String**> | An unique ID for the request |  |

### Return type

[**crate::models::SystemInfo**](SystemInfo.md)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

