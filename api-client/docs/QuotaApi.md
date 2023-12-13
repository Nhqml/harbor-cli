# \QuotaApi

All URIs are relative to *http://localhost/api/v2.0*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_quota**](QuotaApi.md#get_quota) | **GET** /quotas/{id} | Get the specified quota
[**list_quotas**](QuotaApi.md#list_quotas) | **GET** /quotas | List quotas
[**update_quota**](QuotaApi.md#update_quota) | **PUT** /quotas/{id} | Update the specified quota



## get_quota

> crate::models::Quota get_quota(id, x_request_id)
Get the specified quota

Get the specified quota

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | Quota ID | [required] |
**x_request_id** | Option<**String**> | An unique ID for the request |  |

### Return type

[**crate::models::Quota**](Quota.md)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_quotas

> Vec<crate::models::Quota> list_quotas(x_request_id, page, page_size, reference, reference_id, sort)
List quotas

List quotas

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_request_id** | Option<**String**> | An unique ID for the request |  |
**page** | Option<**i64**> | The page number |  |[default to 1]
**page_size** | Option<**i64**> | The size of per page |  |[default to 10]
**reference** | Option<**String**> | The reference type of quota. |  |
**reference_id** | Option<**String**> | The reference id of quota. |  |
**sort** | Option<**String**> | Sort method, valid values include: 'hard.resource_name', '-hard.resource_name', 'used.resource_name', '-used.resource_name'. Here '-' stands for descending order, resource_name should be the real resource name of the quota.  |  |

### Return type

[**Vec<crate::models::Quota>**](Quota.md)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_quota

> update_quota(id, hard, x_request_id)
Update the specified quota

Update hard limits of the specified quota

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | Quota ID | [required] |
**hard** | [**QuotaUpdateReq**](QuotaUpdateReq.md) | The new hard limits for the quota | [required] |
**x_request_id** | Option<**String**> | An unique ID for the request |  |

### Return type

 (empty response body)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

