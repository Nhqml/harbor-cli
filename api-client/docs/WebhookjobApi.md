# \WebhookjobApi

All URIs are relative to *http://localhost/api/v2.0*

Method | HTTP request | Description
------------- | ------------- | -------------
[**list_webhook_jobs**](WebhookjobApi.md#list_webhook_jobs) | **GET** /projects/{project_name_or_id}/webhook/jobs | List project webhook jobs



## list_webhook_jobs

> Vec<crate::models::WebhookJob> list_webhook_jobs(project_name_or_id, policy_id, x_request_id, x_is_resource_name, q, sort, page, page_size, status)
List project webhook jobs

This endpoint returns webhook jobs of a project. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_name_or_id** | **String** | The name or id of the project | [required] |
**policy_id** | **i64** | The policy ID. | [required] |
**x_request_id** | Option<**String**> | An unique ID for the request |  |
**x_is_resource_name** | Option<**bool**> | The flag to indicate whether the parameter which supports both name and id in the path is the name of the resource. When the X-Is-Resource-Name is false and the parameter can be converted to an integer, the parameter will be as an id, otherwise, it will be as a name. |  |[default to false]
**q** | Option<**String**> | Query string to query resources. Supported query patterns are \"exact match(k=v)\", \"fuzzy match(k=~v)\", \"range(k=[min~max])\", \"list with union releationship(k={v1 v2 v3})\" and \"list with intersetion relationship(k=(v1 v2 v3))\". The value of range and list can be string(enclosed by \" or '), integer or time(in format \"2020-04-09 02:36:00\"). All of these query patterns should be put in the query string \"q=xxx\" and splitted by \",\". e.g. q=k1=v1,k2=~v2,k3=[min~max] |  |
**sort** | Option<**String**> | Sort the resource list in ascending or descending order. e.g. sort by field1 in ascending order and field2 in descending order with \"sort=field1,-field2\" |  |
**page** | Option<**i64**> | The page number |  |[default to 1]
**page_size** | Option<**i64**> | The size of per page |  |[default to 10]
**status** | Option<[**Vec<String>**](String.md)> | The status of webhook job. |  |

### Return type

[**Vec<crate::models::WebhookJob>**](WebhookJob.md)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

