# \RobotApi

All URIs are relative to *http://localhost/api/v2.0*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_robot**](RobotApi.md#create_robot) | **POST** /robots | Create a robot account
[**delete_robot**](RobotApi.md#delete_robot) | **DELETE** /robots/{robot_id} | Delete a robot account
[**get_robot_by_id**](RobotApi.md#get_robot_by_id) | **GET** /robots/{robot_id} | Get a robot account
[**list_robot**](RobotApi.md#list_robot) | **GET** /robots | Get robot account
[**refresh_sec**](RobotApi.md#refresh_sec) | **PATCH** /robots/{robot_id} | Refresh the robot secret
[**update_robot**](RobotApi.md#update_robot) | **PUT** /robots/{robot_id} | Update a robot account



## create_robot

> crate::models::RobotCreated create_robot(robot, x_request_id)
Create a robot account

Create a robot account

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**robot** | [**RobotCreate**](RobotCreate.md) | The JSON object of a robot account. | [required] |
**x_request_id** | Option<**String**> | An unique ID for the request |  |

### Return type

[**crate::models::RobotCreated**](RobotCreated.md)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_robot

> delete_robot(robot_id, x_request_id)
Delete a robot account

This endpoint deletes specific robot account information by robot ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**robot_id** | **i32** | Robot ID | [required] |
**x_request_id** | Option<**String**> | An unique ID for the request |  |

### Return type

 (empty response body)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_robot_by_id

> crate::models::Robot get_robot_by_id(robot_id, x_request_id)
Get a robot account

This endpoint returns specific robot account information by robot ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**robot_id** | **i32** | Robot ID | [required] |
**x_request_id** | Option<**String**> | An unique ID for the request |  |

### Return type

[**crate::models::Robot**](Robot.md)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_robot

> Vec<crate::models::Robot> list_robot(x_request_id, q, sort, page, page_size)
Get robot account

List the robot accounts with the specified level and project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_request_id** | Option<**String**> | An unique ID for the request |  |
**q** | Option<**String**> | Query string to query resources. Supported query patterns are \"exact match(k=v)\", \"fuzzy match(k=~v)\", \"range(k=[min~max])\", \"list with union releationship(k={v1 v2 v3})\" and \"list with intersetion relationship(k=(v1 v2 v3))\". The value of range and list can be string(enclosed by \" or '), integer or time(in format \"2020-04-09 02:36:00\"). All of these query patterns should be put in the query string \"q=xxx\" and splitted by \",\". e.g. q=k1=v1,k2=~v2,k3=[min~max] |  |
**sort** | Option<**String**> | Sort the resource list in ascending or descending order. e.g. sort by field1 in ascending order and field2 in descending order with \"sort=field1,-field2\" |  |
**page** | Option<**i64**> | The page number |  |[default to 1]
**page_size** | Option<**i64**> | The size of per page |  |[default to 10]

### Return type

[**Vec<crate::models::Robot>**](Robot.md)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## refresh_sec

> crate::models::RobotSec refresh_sec(robot_id, robot_sec, x_request_id)
Refresh the robot secret

Refresh the robot secret

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**robot_id** | **i32** | Robot ID | [required] |
**robot_sec** | [**RobotSec**](RobotSec.md) | The JSON object of a robot account. | [required] |
**x_request_id** | Option<**String**> | An unique ID for the request |  |

### Return type

[**crate::models::RobotSec**](RobotSec.md)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_robot

> update_robot(robot_id, robot, x_request_id)
Update a robot account

This endpoint updates specific robot account information by robot ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**robot_id** | **i32** | Robot ID | [required] |
**robot** | [**Robot**](Robot.md) | The JSON object of a robot account. | [required] |
**x_request_id** | Option<**String**> | An unique ID for the request |  |

### Return type

 (empty response body)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

