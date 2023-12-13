# \JobserviceApi

All URIs are relative to *http://localhost/api/v2.0*

Method | HTTP request | Description
------------- | ------------- | -------------
[**action_get_job_log**](JobserviceApi.md#action_get_job_log) | **GET** /jobservice/jobs/{job_id}/log | Get job log by job id
[**action_pending_jobs**](JobserviceApi.md#action_pending_jobs) | **PUT** /jobservice/queues/{job_type} | stop and clean, pause, resume pending jobs in the queue
[**get_worker_pools**](JobserviceApi.md#get_worker_pools) | **GET** /jobservice/pools | Get worker pools
[**get_workers**](JobserviceApi.md#get_workers) | **GET** /jobservice/pools/{pool_id}/workers | Get workers
[**list_job_queues**](JobserviceApi.md#list_job_queues) | **GET** /jobservice/queues | list job queues
[**stop_running_job**](JobserviceApi.md#stop_running_job) | **PUT** /jobservice/jobs/{job_id} | Stop running job



## action_get_job_log

> String action_get_job_log(job_id, x_request_id)
Get job log by job id

Get job log by job id, it is only used by administrator

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_id** | **String** | The id of the job. | [required] |
**x_request_id** | Option<**String**> | An unique ID for the request |  |

### Return type

**String**

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## action_pending_jobs

> action_pending_jobs(job_type, action_request, x_request_id)
stop and clean, pause, resume pending jobs in the queue

stop and clean, pause, resume pending jobs in the queue

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_type** | **String** | The type of the job. 'all' stands for all job types | [required] |
**action_request** | [**ActionRequest**](ActionRequest.md) |  | [required] |
**x_request_id** | Option<**String**> | An unique ID for the request |  |

### Return type

 (empty response body)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_worker_pools

> Vec<crate::models::WorkerPool> get_worker_pools(x_request_id)
Get worker pools

Get worker pools

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_request_id** | Option<**String**> | An unique ID for the request |  |

### Return type

[**Vec<crate::models::WorkerPool>**](WorkerPool.md)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_workers

> Vec<crate::models::Worker> get_workers(pool_id, x_request_id)
Get workers

Get workers in current pool

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pool_id** | **String** | The name of the pool. 'all' stands for all pools | [required] |
**x_request_id** | Option<**String**> | An unique ID for the request |  |

### Return type

[**Vec<crate::models::Worker>**](Worker.md)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_job_queues

> Vec<crate::models::JobQueue> list_job_queues(x_request_id)
list job queues

list job queue

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_request_id** | Option<**String**> | An unique ID for the request |  |

### Return type

[**Vec<crate::models::JobQueue>**](JobQueue.md)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stop_running_job

> stop_running_job(job_id, x_request_id)
Stop running job

Stop running job

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_id** | **String** | The id of the job. | [required] |
**x_request_id** | Option<**String**> | An unique ID for the request |  |

### Return type

 (empty response body)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

