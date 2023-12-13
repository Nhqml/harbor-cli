# \RepositoryApi

All URIs are relative to *http://localhost/api/v2.0*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_repository**](RepositoryApi.md#delete_repository) | **DELETE** /projects/{project_name}/repositories/{repository_name} | Delete repository
[**get_repository**](RepositoryApi.md#get_repository) | **GET** /projects/{project_name}/repositories/{repository_name} | Get repository
[**list_all_repositories**](RepositoryApi.md#list_all_repositories) | **GET** /repositories | List all authorized repositories
[**list_repositories**](RepositoryApi.md#list_repositories) | **GET** /projects/{project_name}/repositories | List repositories
[**update_repository**](RepositoryApi.md#update_repository) | **PUT** /projects/{project_name}/repositories/{repository_name} | Update repository



## delete_repository

> delete_repository(project_name, repository_name, x_request_id)
Delete repository

Delete the repository specified by name

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_name** | **String** | The name of the project | [required] |
**repository_name** | **String** | The name of the repository. If it contains slash, encode it twice over with URL encoding. e.g. a/b -> a%2Fb -> a%252Fb | [required] |
**x_request_id** | Option<**String**> | An unique ID for the request |  |

### Return type

 (empty response body)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_repository

> crate::models::Repository get_repository(project_name, repository_name, x_request_id)
Get repository

Get the repository specified by name

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_name** | **String** | The name of the project | [required] |
**repository_name** | **String** | The name of the repository. If it contains slash, encode it twice over with URL encoding. e.g. a/b -> a%2Fb -> a%252Fb | [required] |
**x_request_id** | Option<**String**> | An unique ID for the request |  |

### Return type

[**crate::models::Repository**](Repository.md)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_all_repositories

> Vec<crate::models::Repository> list_all_repositories(x_request_id, q, sort, page, page_size)
List all authorized repositories

List all authorized repositories

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_request_id** | Option<**String**> | An unique ID for the request |  |
**q** | Option<**String**> | Query string to query resources. Supported query patterns are \"exact match(k=v)\", \"fuzzy match(k=~v)\", \"range(k=[min~max])\", \"list with union releationship(k={v1 v2 v3})\" and \"list with intersetion relationship(k=(v1 v2 v3))\". The value of range and list can be string(enclosed by \" or '), integer or time(in format \"2020-04-09 02:36:00\"). All of these query patterns should be put in the query string \"q=xxx\" and splitted by \",\". e.g. q=k1=v1,k2=~v2,k3=[min~max] |  |
**sort** | Option<**String**> | Sort the resource list in ascending or descending order. e.g. sort by field1 in ascending order and field2 in descending order with \"sort=field1,-field2\" |  |
**page** | Option<**i64**> | The page number |  |[default to 1]
**page_size** | Option<**i64**> | The size of per page |  |[default to 10]

### Return type

[**Vec<crate::models::Repository>**](Repository.md)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_repositories

> Vec<crate::models::Repository> list_repositories(project_name, x_request_id, q, sort, page, page_size)
List repositories

List repositories of the specified project

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_name** | **String** | The name of the project | [required] |
**x_request_id** | Option<**String**> | An unique ID for the request |  |
**q** | Option<**String**> | Query string to query resources. Supported query patterns are \"exact match(k=v)\", \"fuzzy match(k=~v)\", \"range(k=[min~max])\", \"list with union releationship(k={v1 v2 v3})\" and \"list with intersetion relationship(k=(v1 v2 v3))\". The value of range and list can be string(enclosed by \" or '), integer or time(in format \"2020-04-09 02:36:00\"). All of these query patterns should be put in the query string \"q=xxx\" and splitted by \",\". e.g. q=k1=v1,k2=~v2,k3=[min~max] |  |
**sort** | Option<**String**> | Sort the resource list in ascending or descending order. e.g. sort by field1 in ascending order and field2 in descending order with \"sort=field1,-field2\" |  |
**page** | Option<**i64**> | The page number |  |[default to 1]
**page_size** | Option<**i64**> | The size of per page |  |[default to 10]

### Return type

[**Vec<crate::models::Repository>**](Repository.md)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_repository

> update_repository(project_name, repository_name, repository, x_request_id)
Update repository

Update the repository specified by name

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_name** | **String** | The name of the project | [required] |
**repository_name** | **String** | The name of the repository. If it contains slash, encode it twice over with URL encoding. e.g. a/b -> a%2Fb -> a%252Fb | [required] |
**repository** | [**Repository**](Repository.md) | The JSON object of repository. | [required] |
**x_request_id** | Option<**String**> | An unique ID for the request |  |

### Return type

 (empty response body)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

