# \ScanAllApi

All URIs are relative to *http://localhost/api/v2.0*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_scan_all_schedule**](ScanAllApi.md#create_scan_all_schedule) | **POST** /system/scanAll/schedule | Create a schedule or a manual trigger for the scan all job.
[**get_latest_scan_all_metrics**](ScanAllApi.md#get_latest_scan_all_metrics) | **GET** /scans/all/metrics | Get the metrics of the latest scan all process
[**get_latest_scheduled_scan_all_metrics**](ScanAllApi.md#get_latest_scheduled_scan_all_metrics) | **GET** /scans/schedule/metrics | Get the metrics of the latest scheduled scan all process
[**get_scan_all_schedule**](ScanAllApi.md#get_scan_all_schedule) | **GET** /system/scanAll/schedule | Get scan all's schedule.
[**stop_scan_all**](ScanAllApi.md#stop_scan_all) | **POST** /system/scanAll/stop | Stop scanAll job execution
[**update_scan_all_schedule**](ScanAllApi.md#update_scan_all_schedule) | **PUT** /system/scanAll/schedule | Update scan all's schedule.



## create_scan_all_schedule

> create_scan_all_schedule(schedule, x_request_id)
Create a schedule or a manual trigger for the scan all job.

This endpoint is for creating a schedule or a manual trigger for the scan all job, which scans all of images in Harbor.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**schedule** | [**Schedule**](Schedule.md) | Create a schedule or a manual trigger for the scan all job. | [required] |
**x_request_id** | Option<**String**> | An unique ID for the request |  |

### Return type

 (empty response body)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_latest_scan_all_metrics

> crate::models::Stats get_latest_scan_all_metrics(x_request_id)
Get the metrics of the latest scan all process

Get the metrics of the latest scan all process

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_request_id** | Option<**String**> | An unique ID for the request |  |

### Return type

[**crate::models::Stats**](Stats.md)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_latest_scheduled_scan_all_metrics

> crate::models::Stats get_latest_scheduled_scan_all_metrics(x_request_id)
Get the metrics of the latest scheduled scan all process

Get the metrics of the latest scheduled scan all process

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_request_id** | Option<**String**> | An unique ID for the request |  |

### Return type

[**crate::models::Stats**](Stats.md)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_scan_all_schedule

> crate::models::Schedule get_scan_all_schedule(x_request_id)
Get scan all's schedule.

This endpoint is for getting a schedule for the scan all job, which scans all of images in Harbor.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_request_id** | Option<**String**> | An unique ID for the request |  |

### Return type

[**crate::models::Schedule**](Schedule.md)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stop_scan_all

> stop_scan_all(x_request_id)
Stop scanAll job execution

Stop scanAll job execution

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_request_id** | Option<**String**> | An unique ID for the request |  |

### Return type

 (empty response body)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_scan_all_schedule

> update_scan_all_schedule(schedule, x_request_id)
Update scan all's schedule.

This endpoint is for updating the schedule of scan all job, which scans all of images in Harbor.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**schedule** | [**Schedule**](Schedule.md) | Updates the schedule of scan all job, which scans all of images in Harbor. | [required] |
**x_request_id** | Option<**String**> | An unique ID for the request |  |

### Return type

 (empty response body)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

