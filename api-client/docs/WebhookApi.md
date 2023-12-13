# \WebhookApi

All URIs are relative to *http://localhost/api/v2.0*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_webhook_policy_of_project**](WebhookApi.md#create_webhook_policy_of_project) | **POST** /projects/{project_name_or_id}/webhook/policies | Create project webhook policy.
[**delete_webhook_policy_of_project**](WebhookApi.md#delete_webhook_policy_of_project) | **DELETE** /projects/{project_name_or_id}/webhook/policies/{webhook_policy_id} | Delete webhook policy of a project
[**get_logs_of_webhook_task**](WebhookApi.md#get_logs_of_webhook_task) | **GET** /projects/{project_name_or_id}/webhook/policies/{webhook_policy_id}/executions/{execution_id}/tasks/{task_id}/log | Get logs for a specific webhook task
[**get_supported_event_types**](WebhookApi.md#get_supported_event_types) | **GET** /projects/{project_name_or_id}/webhook/events | Get supported event types and notify types.
[**get_webhook_policy_of_project**](WebhookApi.md#get_webhook_policy_of_project) | **GET** /projects/{project_name_or_id}/webhook/policies/{webhook_policy_id} | Get project webhook policy
[**last_trigger**](WebhookApi.md#last_trigger) | **GET** /projects/{project_name_or_id}/webhook/lasttrigger | Get project webhook policy last trigger info
[**list_executions_of_webhook_policy**](WebhookApi.md#list_executions_of_webhook_policy) | **GET** /projects/{project_name_or_id}/webhook/policies/{webhook_policy_id}/executions | List executions for a specific webhook policy
[**list_tasks_of_webhook_execution**](WebhookApi.md#list_tasks_of_webhook_execution) | **GET** /projects/{project_name_or_id}/webhook/policies/{webhook_policy_id}/executions/{execution_id}/tasks | List tasks for a specific webhook execution
[**list_webhook_policies_of_project**](WebhookApi.md#list_webhook_policies_of_project) | **GET** /projects/{project_name_or_id}/webhook/policies | List project webhook policies.
[**update_webhook_policy_of_project**](WebhookApi.md#update_webhook_policy_of_project) | **PUT** /projects/{project_name_or_id}/webhook/policies/{webhook_policy_id} | Update webhook policy of a project.



## create_webhook_policy_of_project

> create_webhook_policy_of_project(project_name_or_id, policy, x_request_id, x_is_resource_name)
Create project webhook policy.

This endpoint create a webhook policy if the project does not have one. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_name_or_id** | **String** | The name or id of the project | [required] |
**policy** | [**WebhookPolicy**](WebhookPolicy.md) | Properties \"targets\" and \"event_types\" needed. | [required] |
**x_request_id** | Option<**String**> | An unique ID for the request |  |
**x_is_resource_name** | Option<**bool**> | The flag to indicate whether the parameter which supports both name and id in the path is the name of the resource. When the X-Is-Resource-Name is false and the parameter can be converted to an integer, the parameter will be as an id, otherwise, it will be as a name. |  |[default to false]

### Return type

 (empty response body)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_webhook_policy_of_project

> delete_webhook_policy_of_project(project_name_or_id, webhook_policy_id, x_request_id, x_is_resource_name)
Delete webhook policy of a project

This endpoint is aimed to delete webhookpolicy of a project. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_name_or_id** | **String** | The name or id of the project | [required] |
**webhook_policy_id** | **i64** | The ID of the webhook policy | [required] |
**x_request_id** | Option<**String**> | An unique ID for the request |  |
**x_is_resource_name** | Option<**bool**> | The flag to indicate whether the parameter which supports both name and id in the path is the name of the resource. When the X-Is-Resource-Name is false and the parameter can be converted to an integer, the parameter will be as an id, otherwise, it will be as a name. |  |[default to false]

### Return type

 (empty response body)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_logs_of_webhook_task

> String get_logs_of_webhook_task(project_name_or_id, webhook_policy_id, execution_id, task_id, x_request_id, x_is_resource_name)
Get logs for a specific webhook task

This endpoint returns the logs of a specific webhook task. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_name_or_id** | **String** | The name or id of the project | [required] |
**webhook_policy_id** | **i64** | The ID of the webhook policy | [required] |
**execution_id** | **i32** | Execution ID | [required] |
**task_id** | **i32** | Task ID | [required] |
**x_request_id** | Option<**String**> | An unique ID for the request |  |
**x_is_resource_name** | Option<**bool**> | The flag to indicate whether the parameter which supports both name and id in the path is the name of the resource. When the X-Is-Resource-Name is false and the parameter can be converted to an integer, the parameter will be as an id, otherwise, it will be as a name. |  |[default to false]

### Return type

**String**

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_supported_event_types

> crate::models::SupportedWebhookEventTypes get_supported_event_types(project_name_or_id, x_request_id, x_is_resource_name)
Get supported event types and notify types.

Get supported event types and notify types.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_name_or_id** | **String** | The name or id of the project | [required] |
**x_request_id** | Option<**String**> | An unique ID for the request |  |
**x_is_resource_name** | Option<**bool**> | The flag to indicate whether the parameter which supports both name and id in the path is the name of the resource. When the X-Is-Resource-Name is false and the parameter can be converted to an integer, the parameter will be as an id, otherwise, it will be as a name. |  |[default to false]

### Return type

[**crate::models::SupportedWebhookEventTypes**](SupportedWebhookEventTypes.md)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_webhook_policy_of_project

> crate::models::WebhookPolicy get_webhook_policy_of_project(project_name_or_id, webhook_policy_id, x_request_id, x_is_resource_name)
Get project webhook policy

This endpoint returns specified webhook policy of a project. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_name_or_id** | **String** | The name or id of the project | [required] |
**webhook_policy_id** | **i64** | The ID of the webhook policy | [required] |
**x_request_id** | Option<**String**> | An unique ID for the request |  |
**x_is_resource_name** | Option<**bool**> | The flag to indicate whether the parameter which supports both name and id in the path is the name of the resource. When the X-Is-Resource-Name is false and the parameter can be converted to an integer, the parameter will be as an id, otherwise, it will be as a name. |  |[default to false]

### Return type

[**crate::models::WebhookPolicy**](WebhookPolicy.md)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## last_trigger

> Vec<crate::models::WebhookLastTrigger> last_trigger(project_name_or_id, x_request_id, x_is_resource_name)
Get project webhook policy last trigger info

This endpoint returns last trigger information of project webhook policy. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_name_or_id** | **String** | The name or id of the project | [required] |
**x_request_id** | Option<**String**> | An unique ID for the request |  |
**x_is_resource_name** | Option<**bool**> | The flag to indicate whether the parameter which supports both name and id in the path is the name of the resource. When the X-Is-Resource-Name is false and the parameter can be converted to an integer, the parameter will be as an id, otherwise, it will be as a name. |  |[default to false]

### Return type

[**Vec<crate::models::WebhookLastTrigger>**](WebhookLastTrigger.md)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_executions_of_webhook_policy

> Vec<crate::models::Execution> list_executions_of_webhook_policy(project_name_or_id, webhook_policy_id, x_request_id, x_is_resource_name, page, page_size, q, sort)
List executions for a specific webhook policy

This endpoint returns the executions of a specific webhook policy. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_name_or_id** | **String** | The name or id of the project | [required] |
**webhook_policy_id** | **i64** | The ID of the webhook policy | [required] |
**x_request_id** | Option<**String**> | An unique ID for the request |  |
**x_is_resource_name** | Option<**bool**> | The flag to indicate whether the parameter which supports both name and id in the path is the name of the resource. When the X-Is-Resource-Name is false and the parameter can be converted to an integer, the parameter will be as an id, otherwise, it will be as a name. |  |[default to false]
**page** | Option<**i64**> | The page number |  |[default to 1]
**page_size** | Option<**i64**> | The size of per page |  |[default to 10]
**q** | Option<**String**> | Query string to query resources. Supported query patterns are \"exact match(k=v)\", \"fuzzy match(k=~v)\", \"range(k=[min~max])\", \"list with union releationship(k={v1 v2 v3})\" and \"list with intersetion relationship(k=(v1 v2 v3))\". The value of range and list can be string(enclosed by \" or '), integer or time(in format \"2020-04-09 02:36:00\"). All of these query patterns should be put in the query string \"q=xxx\" and splitted by \",\". e.g. q=k1=v1,k2=~v2,k3=[min~max] |  |
**sort** | Option<**String**> | Sort the resource list in ascending or descending order. e.g. sort by field1 in ascending order and field2 in descending order with \"sort=field1,-field2\" |  |

### Return type

[**Vec<crate::models::Execution>**](Execution.md)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_tasks_of_webhook_execution

> Vec<crate::models::Task> list_tasks_of_webhook_execution(project_name_or_id, webhook_policy_id, execution_id, x_request_id, x_is_resource_name, page, page_size, q, sort)
List tasks for a specific webhook execution

This endpoint returns the tasks of a specific webhook execution. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_name_or_id** | **String** | The name or id of the project | [required] |
**webhook_policy_id** | **i64** | The ID of the webhook policy | [required] |
**execution_id** | **i32** | Execution ID | [required] |
**x_request_id** | Option<**String**> | An unique ID for the request |  |
**x_is_resource_name** | Option<**bool**> | The flag to indicate whether the parameter which supports both name and id in the path is the name of the resource. When the X-Is-Resource-Name is false and the parameter can be converted to an integer, the parameter will be as an id, otherwise, it will be as a name. |  |[default to false]
**page** | Option<**i64**> | The page number |  |[default to 1]
**page_size** | Option<**i64**> | The size of per page |  |[default to 10]
**q** | Option<**String**> | Query string to query resources. Supported query patterns are \"exact match(k=v)\", \"fuzzy match(k=~v)\", \"range(k=[min~max])\", \"list with union releationship(k={v1 v2 v3})\" and \"list with intersetion relationship(k=(v1 v2 v3))\". The value of range and list can be string(enclosed by \" or '), integer or time(in format \"2020-04-09 02:36:00\"). All of these query patterns should be put in the query string \"q=xxx\" and splitted by \",\". e.g. q=k1=v1,k2=~v2,k3=[min~max] |  |
**sort** | Option<**String**> | Sort the resource list in ascending or descending order. e.g. sort by field1 in ascending order and field2 in descending order with \"sort=field1,-field2\" |  |

### Return type

[**Vec<crate::models::Task>**](Task.md)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_webhook_policies_of_project

> Vec<crate::models::WebhookPolicy> list_webhook_policies_of_project(project_name_or_id, x_request_id, x_is_resource_name, sort, q, page, page_size)
List project webhook policies.

This endpoint returns webhook policies of a project. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_name_or_id** | **String** | The name or id of the project | [required] |
**x_request_id** | Option<**String**> | An unique ID for the request |  |
**x_is_resource_name** | Option<**bool**> | The flag to indicate whether the parameter which supports both name and id in the path is the name of the resource. When the X-Is-Resource-Name is false and the parameter can be converted to an integer, the parameter will be as an id, otherwise, it will be as a name. |  |[default to false]
**sort** | Option<**String**> | Sort the resource list in ascending or descending order. e.g. sort by field1 in ascending order and field2 in descending order with \"sort=field1,-field2\" |  |
**q** | Option<**String**> | Query string to query resources. Supported query patterns are \"exact match(k=v)\", \"fuzzy match(k=~v)\", \"range(k=[min~max])\", \"list with union releationship(k={v1 v2 v3})\" and \"list with intersetion relationship(k=(v1 v2 v3))\". The value of range and list can be string(enclosed by \" or '), integer or time(in format \"2020-04-09 02:36:00\"). All of these query patterns should be put in the query string \"q=xxx\" and splitted by \",\". e.g. q=k1=v1,k2=~v2,k3=[min~max] |  |
**page** | Option<**i64**> | The page number |  |[default to 1]
**page_size** | Option<**i64**> | The size of per page |  |[default to 10]

### Return type

[**Vec<crate::models::WebhookPolicy>**](WebhookPolicy.md)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_webhook_policy_of_project

> update_webhook_policy_of_project(project_name_or_id, webhook_policy_id, policy, x_request_id, x_is_resource_name)
Update webhook policy of a project.

This endpoint is aimed to update the webhook policy of a project. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_name_or_id** | **String** | The name or id of the project | [required] |
**webhook_policy_id** | **i64** | The ID of the webhook policy | [required] |
**policy** | [**WebhookPolicy**](WebhookPolicy.md) | All properties needed except \"id\", \"project_id\", \"creation_time\", \"update_time\". | [required] |
**x_request_id** | Option<**String**> | An unique ID for the request |  |
**x_is_resource_name** | Option<**bool**> | The flag to indicate whether the parameter which supports both name and id in the path is the name of the resource. When the X-Is-Resource-Name is false and the parameter can be converted to an integer, the parameter will be as an id, otherwise, it will be as a name. |  |[default to false]

### Return type

 (empty response body)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

