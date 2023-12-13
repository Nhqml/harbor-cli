# \PurgeApi

All URIs are relative to *http://localhost/api/v2.0*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_purge_schedule**](PurgeApi.md#create_purge_schedule) | **POST** /system/purgeaudit/schedule | Create a purge job schedule.
[**get_purge_history**](PurgeApi.md#get_purge_history) | **GET** /system/purgeaudit | Get purge job results.
[**get_purge_job**](PurgeApi.md#get_purge_job) | **GET** /system/purgeaudit/{purge_id} | Get purge job status.
[**get_purge_job_log**](PurgeApi.md#get_purge_job_log) | **GET** /system/purgeaudit/{purge_id}/log | Get purge job log.
[**get_purge_schedule**](PurgeApi.md#get_purge_schedule) | **GET** /system/purgeaudit/schedule | Get purge's schedule.
[**stop_purge**](PurgeApi.md#stop_purge) | **PUT** /system/purgeaudit/{purge_id} | Stop the specific purge audit log execution
[**update_purge_schedule**](PurgeApi.md#update_purge_schedule) | **PUT** /system/purgeaudit/schedule | Update purge job's schedule.



## create_purge_schedule

> create_purge_schedule(schedule, x_request_id)
Create a purge job schedule.

This endpoint is for update purge job schedule. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**schedule** | [**Schedule**](Schedule.md) | The purge job's schedule, it is a json object. | The sample format is | {\"parameters\":{\"audit_retention_hour\":168,\"dry_run\":true, \"include_operations\":\"create,delete,pull\"},\"schedule\":{\"type\":\"Hourly\",\"cron\":\"0 0 * * * *\"}} | the include_operation should be a comma separated string, e.g. create,delete,pull, if it is empty, no operation will be purged.  | [required] |
**x_request_id** | Option<**String**> | An unique ID for the request |  |

### Return type

 (empty response body)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_purge_history

> Vec<crate::models::ExecHistory> get_purge_history(x_request_id, q, sort, page, page_size)
Get purge job results.

get purge job execution history.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_request_id** | Option<**String**> | An unique ID for the request |  |
**q** | Option<**String**> | Query string to query resources. Supported query patterns are \"exact match(k=v)\", \"fuzzy match(k=~v)\", \"range(k=[min~max])\", \"list with union releationship(k={v1 v2 v3})\" and \"list with intersetion relationship(k=(v1 v2 v3))\". The value of range and list can be string(enclosed by \" or '), integer or time(in format \"2020-04-09 02:36:00\"). All of these query patterns should be put in the query string \"q=xxx\" and splitted by \",\". e.g. q=k1=v1,k2=~v2,k3=[min~max] |  |
**sort** | Option<**String**> | Sort the resource list in ascending or descending order. e.g. sort by field1 in ascending order and field2 in descending order with \"sort=field1,-field2\" |  |
**page** | Option<**i64**> | The page number |  |[default to 1]
**page_size** | Option<**i64**> | The size of per page |  |[default to 10]

### Return type

[**Vec<crate::models::ExecHistory>**](ExecHistory.md)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_purge_job

> crate::models::ExecHistory get_purge_job(purge_id, x_request_id)
Get purge job status.

This endpoint let user get purge job status filtered by specific ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**purge_id** | **i64** | The ID of the purge log | [required] |
**x_request_id** | Option<**String**> | An unique ID for the request |  |

### Return type

[**crate::models::ExecHistory**](ExecHistory.md)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_purge_job_log

> String get_purge_job_log(purge_id, x_request_id)
Get purge job log.

This endpoint let user get purge job logs filtered by specific ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**purge_id** | **i64** | The ID of the purge log | [required] |
**x_request_id** | Option<**String**> | An unique ID for the request |  |

### Return type

**String**

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_purge_schedule

> crate::models::ExecHistory get_purge_schedule(x_request_id)
Get purge's schedule.

This endpoint is for get schedule of purge job.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_request_id** | Option<**String**> | An unique ID for the request |  |

### Return type

[**crate::models::ExecHistory**](ExecHistory.md)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stop_purge

> stop_purge(purge_id, x_request_id)
Stop the specific purge audit log execution

Stop the purge audit log execution specified by ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**purge_id** | **i64** | The ID of the purge log | [required] |
**x_request_id** | Option<**String**> | An unique ID for the request |  |

### Return type

 (empty response body)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_purge_schedule

> update_purge_schedule(schedule, x_request_id)
Update purge job's schedule.

This endpoint is for update purge job schedule. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**schedule** | [**Schedule**](Schedule.md) | The purge job's schedule, it is a json object. | The sample format is | {\"parameters\":{\"audit_retention_hour\":168,\"dry_run\":true, \"include_operations\":\"create,delete,pull\"},\"schedule\":{\"type\":\"Hourly\",\"cron\":\"0 0 * * * *\"}} | the include_operation should be a comma separated string, e.g. create,delete,pull, if it is empty, no operation will be purged.  | [required] |
**x_request_id** | Option<**String**> | An unique ID for the request |  |

### Return type

 (empty response body)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

