# \ConfigureApi

All URIs are relative to *http://localhost/api/v2.0*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_configurations**](ConfigureApi.md#get_configurations) | **GET** /configurations | Get system configurations.
[**get_internalconfig**](ConfigureApi.md#get_internalconfig) | **GET** /internalconfig | Get internal configurations.
[**update_configurations**](ConfigureApi.md#update_configurations) | **PUT** /configurations | Modify system configurations.



## get_configurations

> crate::models::ConfigurationsResponse get_configurations(x_request_id)
Get system configurations.

This endpoint is for retrieving system configurations that only provides for admin user. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_request_id** | Option<**String**> | An unique ID for the request |  |

### Return type

[**crate::models::ConfigurationsResponse**](ConfigurationsResponse.md)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_internalconfig

> ::std::collections::HashMap<String, crate::models::InternalConfigurationValue> get_internalconfig(x_request_id)
Get internal configurations.

This endpoint is for retrieving system configurations that only provides for internal api call. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_request_id** | Option<**String**> | An unique ID for the request |  |

### Return type

[**::std::collections::HashMap<String, crate::models::InternalConfigurationValue>**](InternalConfigurationValue.md)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_configurations

> update_configurations(configurations, x_request_id)
Modify system configurations.

This endpoint is for modifying system configurations that only provides for admin user. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**configurations** | [**Configurations**](Configurations.md) | The configuration map can contain a subset of the attributes of the schema, which are to be updated. | [required] |
**x_request_id** | Option<**String**> | An unique ID for the request |  |

### Return type

 (empty response body)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

