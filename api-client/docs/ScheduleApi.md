# \ScheduleApi

All URIs are relative to *http://localhost/api/v2.0*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_schedule_paused**](ScheduleApi.md#get_schedule_paused) | **GET** /schedules/{job_type}/paused | 
[**list_schedules**](ScheduleApi.md#list_schedules) | **GET** /schedules | 



## get_schedule_paused

> crate::models::SchedulerStatus get_schedule_paused(job_type, x_request_id)


Get scheduler paused status

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_type** | **String** | The type of the job. 'all' stands for all job types, current only support query with all | [required] |
**x_request_id** | Option<**String**> | An unique ID for the request |  |

### Return type

[**crate::models::SchedulerStatus**](SchedulerStatus.md)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_schedules

> Vec<crate::models::ScheduleTask> list_schedules(x_request_id, page, page_size)


List schedules

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_request_id** | Option<**String**> | An unique ID for the request |  |
**page** | Option<**i64**> | The page number |  |[default to 1]
**page_size** | Option<**i64**> | The size of per page |  |[default to 10]

### Return type

[**Vec<crate::models::ScheduleTask>**](ScheduleTask.md)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

