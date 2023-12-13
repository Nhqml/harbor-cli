# \RetentionApi

All URIs are relative to *http://localhost/api/v2.0*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_retention**](RetentionApi.md#create_retention) | **POST** /retentions | Create Retention Policy
[**delete_retention**](RetentionApi.md#delete_retention) | **DELETE** /retentions/{id} | Delete Retention Policy
[**get_rentenition_metadata**](RetentionApi.md#get_rentenition_metadata) | **GET** /retentions/metadatas | Get Retention Metadatas
[**get_retention**](RetentionApi.md#get_retention) | **GET** /retentions/{id} | Get Retention Policy
[**get_retention_task_log**](RetentionApi.md#get_retention_task_log) | **GET** /retentions/{id}/executions/{eid}/tasks/{tid} | Get Retention job task log
[**list_retention_executions**](RetentionApi.md#list_retention_executions) | **GET** /retentions/{id}/executions | Get Retention executions
[**list_retention_tasks**](RetentionApi.md#list_retention_tasks) | **GET** /retentions/{id}/executions/{eid}/tasks | Get Retention tasks
[**operate_retention_execution**](RetentionApi.md#operate_retention_execution) | **PATCH** /retentions/{id}/executions/{eid} | Stop a Retention execution
[**trigger_retention_execution**](RetentionApi.md#trigger_retention_execution) | **POST** /retentions/{id}/executions | Trigger a Retention Execution
[**update_retention**](RetentionApi.md#update_retention) | **PUT** /retentions/{id} | Update Retention Policy



## create_retention

> create_retention(policy, x_request_id)
Create Retention Policy

Create Retention Policy, you can reference metadatas API for the policy model. You can check project metadatas to find whether a retention policy is already binded. This method should only be called when no retention policy binded to project yet.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**policy** | [**RetentionPolicy**](RetentionPolicy.md) | Create Retention Policy successfully. | [required] |
**x_request_id** | Option<**String**> | An unique ID for the request |  |

### Return type

 (empty response body)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_retention

> delete_retention(id, x_request_id)
Delete Retention Policy

Delete Retention Policy, you can reference metadatas API for the policy model. You can check project metadatas to find whether a retention policy is already binded. This method should only be called when retention policy has already binded to project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** | Retention ID. | [required] |
**x_request_id** | Option<**String**> | An unique ID for the request |  |

### Return type

 (empty response body)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_rentenition_metadata

> crate::models::RetentionMetadata get_rentenition_metadata(x_request_id)
Get Retention Metadatas

Get Retention Metadatas.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_request_id** | Option<**String**> | An unique ID for the request |  |

### Return type

[**crate::models::RetentionMetadata**](RetentionMetadata.md)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_retention

> crate::models::RetentionPolicy get_retention(id, x_request_id)
Get Retention Policy

Get Retention Policy.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** | Retention ID. | [required] |
**x_request_id** | Option<**String**> | An unique ID for the request |  |

### Return type

[**crate::models::RetentionPolicy**](RetentionPolicy.md)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_retention_task_log

> String get_retention_task_log(id, eid, tid, x_request_id)
Get Retention job task log

Get Retention job task log, tags ratain or deletion detail will be shown in a table.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** | Retention ID. | [required] |
**eid** | **i64** | Retention execution ID. | [required] |
**tid** | **i64** | Retention execution ID. | [required] |
**x_request_id** | Option<**String**> | An unique ID for the request |  |

### Return type

**String**

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_retention_executions

> Vec<crate::models::RetentionExecution> list_retention_executions(id, x_request_id, page, page_size)
Get Retention executions

Get Retention executions, execution status may be delayed before job service schedule it up.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** | Retention ID. | [required] |
**x_request_id** | Option<**String**> | An unique ID for the request |  |
**page** | Option<**i64**> | The page number. |  |
**page_size** | Option<**i64**> | The size of per page. |  |

### Return type

[**Vec<crate::models::RetentionExecution>**](RetentionExecution.md)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_retention_tasks

> Vec<crate::models::RetentionExecutionTask> list_retention_tasks(id, eid, x_request_id, page, page_size)
Get Retention tasks

Get Retention tasks, each repository as a task.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** | Retention ID. | [required] |
**eid** | **i64** | Retention execution ID. | [required] |
**x_request_id** | Option<**String**> | An unique ID for the request |  |
**page** | Option<**i64**> | The page number. |  |
**page_size** | Option<**i64**> | The size of per page. |  |

### Return type

[**Vec<crate::models::RetentionExecutionTask>**](RetentionExecutionTask.md)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## operate_retention_execution

> operate_retention_execution(id, eid, body, x_request_id)
Stop a Retention execution

Stop a Retention execution, only support \"stop\" action now.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** | Retention ID. | [required] |
**eid** | **i64** | Retention execution ID. | [required] |
**body** | [**OperateRetentionExecutionRequest**](OperateRetentionExecutionRequest.md) | The action, only support \"stop\" now. | [required] |
**x_request_id** | Option<**String**> | An unique ID for the request |  |

### Return type

 (empty response body)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## trigger_retention_execution

> trigger_retention_execution(id, body, x_request_id)
Trigger a Retention Execution

Trigger a Retention Execution, if dry_run is True, nothing would be deleted actually.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** | Retention ID. | [required] |
**body** | [**TriggerRetentionExecutionRequest**](TriggerRetentionExecutionRequest.md) |  | [required] |
**x_request_id** | Option<**String**> | An unique ID for the request |  |

### Return type

 (empty response body)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_retention

> update_retention(id, policy, x_request_id)
Update Retention Policy

Update Retention Policy, you can reference metadatas API for the policy model. You can check project metadatas to find whether a retention policy is already binded. This method should only be called when retention policy has already binded to project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** | Retention ID. | [required] |
**policy** | [**RetentionPolicy**](RetentionPolicy.md) |  | [required] |
**x_request_id** | Option<**String**> | An unique ID for the request |  |

### Return type

 (empty response body)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

