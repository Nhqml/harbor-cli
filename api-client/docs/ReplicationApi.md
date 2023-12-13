# \ReplicationApi

All URIs are relative to *http://localhost/api/v2.0*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_replication_policy**](ReplicationApi.md#create_replication_policy) | **POST** /replication/policies | Create a replication policy
[**delete_replication_policy**](ReplicationApi.md#delete_replication_policy) | **DELETE** /replication/policies/{id} | Delete the specific replication policy
[**get_replication_execution**](ReplicationApi.md#get_replication_execution) | **GET** /replication/executions/{id} | Get the specific replication execution
[**get_replication_log**](ReplicationApi.md#get_replication_log) | **GET** /replication/executions/{id}/tasks/{task_id}/log | Get the log of the specific replication task
[**get_replication_policy**](ReplicationApi.md#get_replication_policy) | **GET** /replication/policies/{id} | Get the specific replication policy
[**list_replication_executions**](ReplicationApi.md#list_replication_executions) | **GET** /replication/executions | List replication executions
[**list_replication_policies**](ReplicationApi.md#list_replication_policies) | **GET** /replication/policies | List replication policies
[**list_replication_tasks**](ReplicationApi.md#list_replication_tasks) | **GET** /replication/executions/{id}/tasks | List replication tasks for a specific execution
[**start_replication**](ReplicationApi.md#start_replication) | **POST** /replication/executions | Start one replication execution
[**stop_replication**](ReplicationApi.md#stop_replication) | **PUT** /replication/executions/{id} | Stop the specific replication execution
[**update_replication_policy**](ReplicationApi.md#update_replication_policy) | **PUT** /replication/policies/{id} | Update the replication policy



## create_replication_policy

> create_replication_policy(policy, x_request_id)
Create a replication policy

Create a replication policy

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**policy** | [**ReplicationPolicy**](ReplicationPolicy.md) | The replication policy | [required] |
**x_request_id** | Option<**String**> | An unique ID for the request |  |

### Return type

 (empty response body)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_replication_policy

> delete_replication_policy(id, x_request_id)
Delete the specific replication policy

Delete the specific replication policy

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** | Replication policy ID | [required] |
**x_request_id** | Option<**String**> | An unique ID for the request |  |

### Return type

 (empty response body)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_replication_execution

> crate::models::ReplicationExecution get_replication_execution(id, x_request_id)
Get the specific replication execution

Get the replication execution specified by ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** | The ID of the execution. | [required] |
**x_request_id** | Option<**String**> | An unique ID for the request |  |

### Return type

[**crate::models::ReplicationExecution**](ReplicationExecution.md)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_replication_log

> String get_replication_log(id, task_id, x_request_id)
Get the log of the specific replication task

Get the log of the specific replication task

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** | The ID of the execution that the tasks belongs to. | [required] |
**task_id** | **i64** | The ID of the task. | [required] |
**x_request_id** | Option<**String**> | An unique ID for the request |  |

### Return type

**String**

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_replication_policy

> crate::models::ReplicationPolicy get_replication_policy(id, x_request_id)
Get the specific replication policy

Get the specific replication policy

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** | Policy ID | [required] |
**x_request_id** | Option<**String**> | An unique ID for the request |  |

### Return type

[**crate::models::ReplicationPolicy**](ReplicationPolicy.md)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_replication_executions

> Vec<crate::models::ReplicationExecution> list_replication_executions(x_request_id, sort, page, page_size, policy_id, status, trigger)
List replication executions

List replication executions

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_request_id** | Option<**String**> | An unique ID for the request |  |
**sort** | Option<**String**> | Sort the resource list in ascending or descending order. e.g. sort by field1 in ascending order and field2 in descending order with \"sort=field1,-field2\" |  |
**page** | Option<**i64**> | The page number |  |[default to 1]
**page_size** | Option<**i64**> | The size of per page |  |[default to 10]
**policy_id** | Option<**i32**> | The ID of the policy that the executions belong to. |  |
**status** | Option<**String**> | The execution status. |  |
**trigger** | Option<**String**> | The trigger mode. |  |

### Return type

[**Vec<crate::models::ReplicationExecution>**](ReplicationExecution.md)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_replication_policies

> Vec<crate::models::ReplicationPolicy> list_replication_policies(x_request_id, q, sort, page, page_size, name)
List replication policies

List replication policies

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_request_id** | Option<**String**> | An unique ID for the request |  |
**q** | Option<**String**> | Query string to query resources. Supported query patterns are \"exact match(k=v)\", \"fuzzy match(k=~v)\", \"range(k=[min~max])\", \"list with union releationship(k={v1 v2 v3})\" and \"list with intersetion relationship(k=(v1 v2 v3))\". The value of range and list can be string(enclosed by \" or '), integer or time(in format \"2020-04-09 02:36:00\"). All of these query patterns should be put in the query string \"q=xxx\" and splitted by \",\". e.g. q=k1=v1,k2=~v2,k3=[min~max] |  |
**sort** | Option<**String**> | Sort the resource list in ascending or descending order. e.g. sort by field1 in ascending order and field2 in descending order with \"sort=field1,-field2\" |  |
**page** | Option<**i64**> | The page number |  |[default to 1]
**page_size** | Option<**i64**> | The size of per page |  |[default to 10]
**name** | Option<**String**> | Deprecated, use \"query\" instead. The policy name. |  |

### Return type

[**Vec<crate::models::ReplicationPolicy>**](ReplicationPolicy.md)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_replication_tasks

> Vec<crate::models::ReplicationTask> list_replication_tasks(id, x_request_id, sort, page, page_size, status, resource_type)
List replication tasks for a specific execution

List replication tasks for a specific execution

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** | The ID of the execution that the tasks belongs to. | [required] |
**x_request_id** | Option<**String**> | An unique ID for the request |  |
**sort** | Option<**String**> | Sort the resource list in ascending or descending order. e.g. sort by field1 in ascending order and field2 in descending order with \"sort=field1,-field2\" |  |
**page** | Option<**i64**> | The page number |  |[default to 1]
**page_size** | Option<**i64**> | The size of per page |  |[default to 10]
**status** | Option<**String**> | The task status. |  |
**resource_type** | Option<**String**> | The resource type. |  |

### Return type

[**Vec<crate::models::ReplicationTask>**](ReplicationTask.md)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## start_replication

> start_replication(execution, x_request_id)
Start one replication execution

Start one replication execution according to the policy

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**execution** | [**StartReplicationExecution**](StartReplicationExecution.md) | The ID of policy that the execution belongs to | [required] |
**x_request_id** | Option<**String**> | An unique ID for the request |  |

### Return type

 (empty response body)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stop_replication

> stop_replication(id, x_request_id)
Stop the specific replication execution

Stop the replication execution specified by ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** | The ID of the execution. | [required] |
**x_request_id** | Option<**String**> | An unique ID for the request |  |

### Return type

 (empty response body)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_replication_policy

> update_replication_policy(id, policy, x_request_id)
Update the replication policy

Update the replication policy

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** | The policy ID | [required] |
**policy** | [**ReplicationPolicy**](ReplicationPolicy.md) | The replication policy | [required] |
**x_request_id** | Option<**String**> | An unique ID for the request |  |

### Return type

 (empty response body)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

