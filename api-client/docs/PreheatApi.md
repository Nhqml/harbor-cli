# \PreheatApi

All URIs are relative to *http://localhost/api/v2.0*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_instance**](PreheatApi.md#create_instance) | **POST** /p2p/preheat/instances | Create p2p provider instances
[**create_policy**](PreheatApi.md#create_policy) | **POST** /projects/{project_name}/preheat/policies | Create a preheat policy under a project
[**delete_instance**](PreheatApi.md#delete_instance) | **DELETE** /p2p/preheat/instances/{preheat_instance_name} | Delete the specified P2P provider instance
[**delete_policy**](PreheatApi.md#delete_policy) | **DELETE** /projects/{project_name}/preheat/policies/{preheat_policy_name} | Delete a preheat policy
[**get_execution**](PreheatApi.md#get_execution) | **GET** /projects/{project_name}/preheat/policies/{preheat_policy_name}/executions/{execution_id} | Get a execution detail by id
[**get_instance**](PreheatApi.md#get_instance) | **GET** /p2p/preheat/instances/{preheat_instance_name} | Get a P2P provider instance
[**get_policy**](PreheatApi.md#get_policy) | **GET** /projects/{project_name}/preheat/policies/{preheat_policy_name} | Get a preheat policy
[**get_preheat_log**](PreheatApi.md#get_preheat_log) | **GET** /projects/{project_name}/preheat/policies/{preheat_policy_name}/executions/{execution_id}/tasks/{task_id}/logs | Get the log text stream of the specified task for the given execution
[**list_executions**](PreheatApi.md#list_executions) | **GET** /projects/{project_name}/preheat/policies/{preheat_policy_name}/executions | List executions for the given policy
[**list_instances**](PreheatApi.md#list_instances) | **GET** /p2p/preheat/instances | List P2P provider instances
[**list_policies**](PreheatApi.md#list_policies) | **GET** /projects/{project_name}/preheat/policies | List preheat policies
[**list_providers**](PreheatApi.md#list_providers) | **GET** /p2p/preheat/providers | List P2P providers
[**list_providers_under_project**](PreheatApi.md#list_providers_under_project) | **GET** /projects/{project_name}/preheat/providers | Get all providers at project level
[**list_tasks**](PreheatApi.md#list_tasks) | **GET** /projects/{project_name}/preheat/policies/{preheat_policy_name}/executions/{execution_id}/tasks | List all the related tasks for the given execution
[**manual_preheat**](PreheatApi.md#manual_preheat) | **POST** /projects/{project_name}/preheat/policies/{preheat_policy_name} | Manual preheat
[**ping_instances**](PreheatApi.md#ping_instances) | **POST** /p2p/preheat/instances/ping | Ping status of a instance.
[**stop_execution**](PreheatApi.md#stop_execution) | **PATCH** /projects/{project_name}/preheat/policies/{preheat_policy_name}/executions/{execution_id} | Stop a execution
[**update_instance**](PreheatApi.md#update_instance) | **PUT** /p2p/preheat/instances/{preheat_instance_name} | Update the specified P2P provider instance
[**update_policy**](PreheatApi.md#update_policy) | **PUT** /projects/{project_name}/preheat/policies/{preheat_policy_name} | Update preheat policy



## create_instance

> create_instance(instance, x_request_id)
Create p2p provider instances

Create p2p provider instances

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**instance** | [**Instance**](Instance.md) | The JSON object of instance. | [required] |
**x_request_id** | Option<**String**> | An unique ID for the request |  |

### Return type

 (empty response body)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_policy

> create_policy(project_name, policy, x_request_id)
Create a preheat policy under a project

Create a preheat policy under a project

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_name** | **String** | The name of the project | [required] |
**policy** | [**PreheatPolicy**](PreheatPolicy.md) | The policy schema info | [required] |
**x_request_id** | Option<**String**> | An unique ID for the request |  |

### Return type

 (empty response body)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_instance

> delete_instance(preheat_instance_name, x_request_id)
Delete the specified P2P provider instance

Delete the specified P2P provider instance

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**preheat_instance_name** | **String** | Instance Name | [required] |
**x_request_id** | Option<**String**> | An unique ID for the request |  |

### Return type

 (empty response body)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_policy

> delete_policy(project_name, preheat_policy_name, x_request_id)
Delete a preheat policy

Delete a preheat policy

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_name** | **String** | The name of the project | [required] |
**preheat_policy_name** | **String** | Preheat Policy Name | [required] |
**x_request_id** | Option<**String**> | An unique ID for the request |  |

### Return type

 (empty response body)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_execution

> crate::models::Execution get_execution(project_name, preheat_policy_name, execution_id, x_request_id)
Get a execution detail by id

Get a execution detail by id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_name** | **String** | The name of the project | [required] |
**preheat_policy_name** | **String** | Preheat Policy Name | [required] |
**execution_id** | **i32** | Execution ID | [required] |
**x_request_id** | Option<**String**> | An unique ID for the request |  |

### Return type

[**crate::models::Execution**](Execution.md)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_instance

> crate::models::Instance get_instance(preheat_instance_name, x_request_id)
Get a P2P provider instance

Get a P2P provider instance

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**preheat_instance_name** | **String** | Instance Name | [required] |
**x_request_id** | Option<**String**> | An unique ID for the request |  |

### Return type

[**crate::models::Instance**](Instance.md)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_policy

> crate::models::PreheatPolicy get_policy(project_name, preheat_policy_name, x_request_id)
Get a preheat policy

Get a preheat policy

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_name** | **String** | The name of the project | [required] |
**preheat_policy_name** | **String** | Preheat Policy Name | [required] |
**x_request_id** | Option<**String**> | An unique ID for the request |  |

### Return type

[**crate::models::PreheatPolicy**](PreheatPolicy.md)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_preheat_log

> String get_preheat_log(project_name, preheat_policy_name, execution_id, task_id, x_request_id)
Get the log text stream of the specified task for the given execution

Get the log text stream of the specified task for the given execution

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_name** | **String** | The name of the project | [required] |
**preheat_policy_name** | **String** | Preheat Policy Name | [required] |
**execution_id** | **i32** | Execution ID | [required] |
**task_id** | **i32** | Task ID | [required] |
**x_request_id** | Option<**String**> | An unique ID for the request |  |

### Return type

**String**

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_executions

> Vec<crate::models::Execution> list_executions(project_name, preheat_policy_name, x_request_id, page, page_size, q, sort)
List executions for the given policy

List executions for the given policy

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_name** | **String** | The name of the project | [required] |
**preheat_policy_name** | **String** | Preheat Policy Name | [required] |
**x_request_id** | Option<**String**> | An unique ID for the request |  |
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


## list_instances

> Vec<crate::models::Instance> list_instances(x_request_id, page, page_size, q, sort)
List P2P provider instances

List P2P provider instances

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_request_id** | Option<**String**> | An unique ID for the request |  |
**page** | Option<**i64**> | The page number |  |[default to 1]
**page_size** | Option<**i64**> | The size of per page |  |[default to 10]
**q** | Option<**String**> | Query string to query resources. Supported query patterns are \"exact match(k=v)\", \"fuzzy match(k=~v)\", \"range(k=[min~max])\", \"list with union releationship(k={v1 v2 v3})\" and \"list with intersetion relationship(k=(v1 v2 v3))\". The value of range and list can be string(enclosed by \" or '), integer or time(in format \"2020-04-09 02:36:00\"). All of these query patterns should be put in the query string \"q=xxx\" and splitted by \",\". e.g. q=k1=v1,k2=~v2,k3=[min~max] |  |
**sort** | Option<**String**> | Sort the resource list in ascending or descending order. e.g. sort by field1 in ascending order and field2 in descending order with \"sort=field1,-field2\" |  |

### Return type

[**Vec<crate::models::Instance>**](Instance.md)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_policies

> Vec<crate::models::PreheatPolicy> list_policies(project_name, x_request_id, page, page_size, q, sort)
List preheat policies

List preheat policies

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_name** | **String** | The name of the project | [required] |
**x_request_id** | Option<**String**> | An unique ID for the request |  |
**page** | Option<**i64**> | The page number |  |[default to 1]
**page_size** | Option<**i64**> | The size of per page |  |[default to 10]
**q** | Option<**String**> | Query string to query resources. Supported query patterns are \"exact match(k=v)\", \"fuzzy match(k=~v)\", \"range(k=[min~max])\", \"list with union releationship(k={v1 v2 v3})\" and \"list with intersetion relationship(k=(v1 v2 v3))\". The value of range and list can be string(enclosed by \" or '), integer or time(in format \"2020-04-09 02:36:00\"). All of these query patterns should be put in the query string \"q=xxx\" and splitted by \",\". e.g. q=k1=v1,k2=~v2,k3=[min~max] |  |
**sort** | Option<**String**> | Sort the resource list in ascending or descending order. e.g. sort by field1 in ascending order and field2 in descending order with \"sort=field1,-field2\" |  |

### Return type

[**Vec<crate::models::PreheatPolicy>**](PreheatPolicy.md)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_providers

> Vec<crate::models::Metadata> list_providers(x_request_id)
List P2P providers

List P2P providers

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_request_id** | Option<**String**> | An unique ID for the request |  |

### Return type

[**Vec<crate::models::Metadata>**](Metadata.md)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_providers_under_project

> Vec<crate::models::ProviderUnderProject> list_providers_under_project(project_name, x_request_id)
Get all providers at project level

Get all providers at project level

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_name** | **String** | The name of the project | [required] |
**x_request_id** | Option<**String**> | An unique ID for the request |  |

### Return type

[**Vec<crate::models::ProviderUnderProject>**](ProviderUnderProject.md)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_tasks

> Vec<crate::models::Task> list_tasks(project_name, preheat_policy_name, execution_id, x_request_id, page, page_size, q, sort)
List all the related tasks for the given execution

List all the related tasks for the given execution

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_name** | **String** | The name of the project | [required] |
**preheat_policy_name** | **String** | Preheat Policy Name | [required] |
**execution_id** | **i32** | Execution ID | [required] |
**x_request_id** | Option<**String**> | An unique ID for the request |  |
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


## manual_preheat

> manual_preheat(project_name, preheat_policy_name, policy, x_request_id)
Manual preheat

Manual preheat

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_name** | **String** | The name of the project | [required] |
**preheat_policy_name** | **String** | Preheat Policy Name | [required] |
**policy** | [**PreheatPolicy**](PreheatPolicy.md) | The policy schema info | [required] |
**x_request_id** | Option<**String**> | An unique ID for the request |  |

### Return type

 (empty response body)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ping_instances

> ping_instances(instance, x_request_id)
Ping status of a instance.

This endpoint checks status of a instance, the instance can be given by ID or Endpoint URL (together with credential) 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**instance** | [**Instance**](Instance.md) | The JSON object of instance. | [required] |
**x_request_id** | Option<**String**> | An unique ID for the request |  |

### Return type

 (empty response body)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stop_execution

> stop_execution(project_name, preheat_policy_name, execution_id, execution, x_request_id)
Stop a execution

Stop a execution

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_name** | **String** | The name of the project | [required] |
**preheat_policy_name** | **String** | Preheat Policy Name | [required] |
**execution_id** | **i32** | Execution ID | [required] |
**execution** | [**Execution**](Execution.md) | The data of execution | [required] |
**x_request_id** | Option<**String**> | An unique ID for the request |  |

### Return type

 (empty response body)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_instance

> update_instance(preheat_instance_name, instance, x_request_id)
Update the specified P2P provider instance

Update the specified P2P provider instance

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**preheat_instance_name** | **String** | Instance Name | [required] |
**instance** | [**Instance**](Instance.md) | The instance to update | [required] |
**x_request_id** | Option<**String**> | An unique ID for the request |  |

### Return type

 (empty response body)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_policy

> update_policy(project_name, preheat_policy_name, policy, x_request_id)
Update preheat policy

Update preheat policy

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_name** | **String** | The name of the project | [required] |
**preheat_policy_name** | **String** | Preheat Policy Name | [required] |
**policy** | [**PreheatPolicy**](PreheatPolicy.md) | The policy schema info | [required] |
**x_request_id** | Option<**String**> | An unique ID for the request |  |

### Return type

 (empty response body)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

