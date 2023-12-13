# \Robotv1Api

All URIs are relative to *http://localhost/api/v2.0*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_robot_v1**](Robotv1Api.md#create_robot_v1) | **POST** /projects/{project_name_or_id}/robots | Create a robot account
[**delete_robot_v1**](Robotv1Api.md#delete_robot_v1) | **DELETE** /projects/{project_name_or_id}/robots/{robot_id} | Delete a robot account
[**get_robot_by_idv1**](Robotv1Api.md#get_robot_by_idv1) | **GET** /projects/{project_name_or_id}/robots/{robot_id} | Get a robot account
[**list_robot_v1**](Robotv1Api.md#list_robot_v1) | **GET** /projects/{project_name_or_id}/robots | Get all robot accounts of specified project
[**update_robot_v1**](Robotv1Api.md#update_robot_v1) | **PUT** /projects/{project_name_or_id}/robots/{robot_id} | Update status of robot account.



## create_robot_v1

> crate::models::RobotCreated create_robot_v1(project_name_or_id, robot, x_request_id, x_is_resource_name)
Create a robot account

Create a robot account

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_name_or_id** | **String** | The name or id of the project | [required] |
**robot** | [**RobotCreateV1**](RobotCreateV1.md) | The JSON object of a robot account. | [required] |
**x_request_id** | Option<**String**> | An unique ID for the request |  |
**x_is_resource_name** | Option<**bool**> | The flag to indicate whether the parameter which supports both name and id in the path is the name of the resource. When the X-Is-Resource-Name is false and the parameter can be converted to an integer, the parameter will be as an id, otherwise, it will be as a name. |  |[default to false]

### Return type

[**crate::models::RobotCreated**](RobotCreated.md)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_robot_v1

> delete_robot_v1(project_name_or_id, robot_id, x_request_id, x_is_resource_name)
Delete a robot account

This endpoint deletes specific robot account information by robot ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_name_or_id** | **String** | The name or id of the project | [required] |
**robot_id** | **i32** | Robot ID | [required] |
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


## get_robot_by_idv1

> crate::models::Robot get_robot_by_idv1(project_name_or_id, robot_id, x_request_id, x_is_resource_name)
Get a robot account

This endpoint returns specific robot account information by robot ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_name_or_id** | **String** | The name or id of the project | [required] |
**robot_id** | **i32** | Robot ID | [required] |
**x_request_id** | Option<**String**> | An unique ID for the request |  |
**x_is_resource_name** | Option<**bool**> | The flag to indicate whether the parameter which supports both name and id in the path is the name of the resource. When the X-Is-Resource-Name is false and the parameter can be converted to an integer, the parameter will be as an id, otherwise, it will be as a name. |  |[default to false]

### Return type

[**crate::models::Robot**](Robot.md)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_robot_v1

> Vec<crate::models::Robot> list_robot_v1(project_name_or_id, x_request_id, x_is_resource_name, page, page_size, q, sort)
Get all robot accounts of specified project

Get all robot accounts of specified project

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

[**Vec<crate::models::Robot>**](Robot.md)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_robot_v1

> update_robot_v1(project_name_or_id, robot_id, robot, x_request_id, x_is_resource_name)
Update status of robot account.

Used to disable/enable a specified robot account.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_name_or_id** | **String** | The name or id of the project | [required] |
**robot_id** | **i32** | Robot ID | [required] |
**robot** | [**Robot**](Robot.md) | The JSON object of a robot account. | [required] |
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

