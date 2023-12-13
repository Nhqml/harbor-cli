# \ImmutableApi

All URIs are relative to *http://localhost/api/v2.0*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_immu_rule**](ImmutableApi.md#create_immu_rule) | **POST** /projects/{project_name_or_id}/immutabletagrules | Add an immutable tag rule to current project
[**delete_immu_rule**](ImmutableApi.md#delete_immu_rule) | **DELETE** /projects/{project_name_or_id}/immutabletagrules/{immutable_rule_id} | Delete the immutable tag rule.
[**list_immu_rules**](ImmutableApi.md#list_immu_rules) | **GET** /projects/{project_name_or_id}/immutabletagrules | List all immutable tag rules of current project
[**update_immu_rule**](ImmutableApi.md#update_immu_rule) | **PUT** /projects/{project_name_or_id}/immutabletagrules/{immutable_rule_id} | Update the immutable tag rule or enable or disable the rule



## create_immu_rule

> create_immu_rule(project_name_or_id, immutable_rule, x_request_id, x_is_resource_name)
Add an immutable tag rule to current project

This endpoint add an immutable tag rule to the project 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_name_or_id** | **String** | The name or id of the project | [required] |
**immutable_rule** | [**ImmutableRule**](ImmutableRule.md) |  | [required] |
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


## delete_immu_rule

> delete_immu_rule(project_name_or_id, immutable_rule_id, x_request_id, x_is_resource_name)
Delete the immutable tag rule.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_name_or_id** | **String** | The name or id of the project | [required] |
**immutable_rule_id** | **i64** | The ID of the immutable rule | [required] |
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


## list_immu_rules

> Vec<crate::models::ImmutableRule> list_immu_rules(project_name_or_id, x_request_id, x_is_resource_name, page, page_size, q, sort)
List all immutable tag rules of current project

This endpoint returns the immutable tag rules of a project 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_name_or_id** | **String** | The name or id of the project | [required] |
**x_request_id** | Option<**String**> | An unique ID for the request |  |
**x_is_resource_name** | Option<**bool**> | The flag to indicate whether the parameter which supports both name and id in the path is the name of the resource. When the X-Is-Resource-Name is false and the parameter can be converted to an integer, the parameter will be as an id, otherwise, it will be as a name. |  |[default to false]
**page** | Option<**i64**> | The page number |  |[default to 1]
**page_size** | Option<**i64**> | The size of per page |  |[default to 10]
**q** | Option<**String**> | Query string to query resources. Supported query patterns are \"exact match(k=v)\", \"fuzzy match(k=~v)\", \"range(k=[min~max])\", \"list with union releationship(k={v1 v2 v3})\" and \"list with intersetion relationship(k=(v1 v2 v3))\". The value of range and list can be string(enclosed by \" or '), integer or time(in format \"2020-04-09 02:36:00\"). All of these query patterns should be put in the query string \"q=xxx\" and splitted by \",\". e.g. q=k1=v1,k2=~v2,k3=[min~max] |  |
**sort** | Option<**String**> | Sort the resource list in ascending or descending order. e.g. sort by field1 in ascending order and field2 in descending order with \"sort=field1,-field2\" |  |

### Return type

[**Vec<crate::models::ImmutableRule>**](ImmutableRule.md)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_immu_rule

> update_immu_rule(project_name_or_id, immutable_rule_id, immutable_rule, x_request_id, x_is_resource_name)
Update the immutable tag rule or enable or disable the rule

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_name_or_id** | **String** | The name or id of the project | [required] |
**immutable_rule_id** | **i64** | The ID of the immutable rule | [required] |
**immutable_rule** | [**ImmutableRule**](ImmutableRule.md) |  | [required] |
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

