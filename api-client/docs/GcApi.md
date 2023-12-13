# \GcApi

All URIs are relative to *http://localhost/api/v2.0*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_gc_schedule**](GcApi.md#create_gc_schedule) | **POST** /system/gc/schedule | Create a gc schedule.
[**get_gc**](GcApi.md#get_gc) | **GET** /system/gc/{gc_id} | Get gc status.
[**get_gc_history**](GcApi.md#get_gc_history) | **GET** /system/gc | Get gc results.
[**get_gc_log**](GcApi.md#get_gc_log) | **GET** /system/gc/{gc_id}/log | Get gc job log.
[**get_gc_schedule**](GcApi.md#get_gc_schedule) | **GET** /system/gc/schedule | Get gc's schedule.
[**stop_gc**](GcApi.md#stop_gc) | **PUT** /system/gc/{gc_id} | Stop the specific GC execution
[**update_gc_schedule**](GcApi.md#update_gc_schedule) | **PUT** /system/gc/schedule | Update gc's schedule.



## create_gc_schedule

> create_gc_schedule(schedule, x_request_id)
Create a gc schedule.

This endpoint is for update gc schedule. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**schedule** | [**Schedule**](Schedule.md) | Updates of gc's schedule. | [required] |
**x_request_id** | Option<**String**> | An unique ID for the request |  |

### Return type

 (empty response body)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_gc

> crate::models::GcHistory get_gc(gc_id, x_request_id)
Get gc status.

This endpoint let user get gc status filtered by specific ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**gc_id** | **i64** | The ID of the gc log | [required] |
**x_request_id** | Option<**String**> | An unique ID for the request |  |

### Return type

[**crate::models::GcHistory**](GCHistory.md)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_gc_history

> Vec<crate::models::GcHistory> get_gc_history(x_request_id, q, sort, page, page_size)
Get gc results.

This endpoint let user get gc execution history.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_request_id** | Option<**String**> | An unique ID for the request |  |
**q** | Option<**String**> | Query string to query resources. Supported query patterns are \"exact match(k=v)\", \"fuzzy match(k=~v)\", \"range(k=[min~max])\", \"list with union releationship(k={v1 v2 v3})\" and \"list with intersetion relationship(k=(v1 v2 v3))\". The value of range and list can be string(enclosed by \" or '), integer or time(in format \"2020-04-09 02:36:00\"). All of these query patterns should be put in the query string \"q=xxx\" and splitted by \",\". e.g. q=k1=v1,k2=~v2,k3=[min~max] |  |
**sort** | Option<**String**> | Sort the resource list in ascending or descending order. e.g. sort by field1 in ascending order and field2 in descending order with \"sort=field1,-field2\" |  |
**page** | Option<**i64**> | The page number |  |[default to 1]
**page_size** | Option<**i64**> | The size of per page |  |[default to 10]

### Return type

[**Vec<crate::models::GcHistory>**](GCHistory.md)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_gc_log

> String get_gc_log(gc_id, x_request_id)
Get gc job log.

This endpoint let user get gc job logs filtered by specific ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**gc_id** | **i64** | The ID of the gc log | [required] |
**x_request_id** | Option<**String**> | An unique ID for the request |  |

### Return type

**String**

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_gc_schedule

> crate::models::GcHistory get_gc_schedule(x_request_id)
Get gc's schedule.

This endpoint is for get schedule of gc job.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_request_id** | Option<**String**> | An unique ID for the request |  |

### Return type

[**crate::models::GcHistory**](GCHistory.md)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stop_gc

> stop_gc(gc_id, x_request_id)
Stop the specific GC execution

Stop the GC execution specified by ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**gc_id** | **i64** | The ID of the gc log | [required] |
**x_request_id** | Option<**String**> | An unique ID for the request |  |

### Return type

 (empty response body)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_gc_schedule

> update_gc_schedule(schedule, x_request_id)
Update gc's schedule.

This endpoint is for update gc schedule. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**schedule** | [**Schedule**](Schedule.md) | Updates of gc's schedule. | [required] |
**x_request_id** | Option<**String**> | An unique ID for the request |  |

### Return type

 (empty response body)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

