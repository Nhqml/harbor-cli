# \ProjectMetadataApi

All URIs are relative to *http://localhost/api/v2.0*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_project_metadatas**](ProjectMetadataApi.md#add_project_metadatas) | **POST** /projects/{project_name_or_id}/metadatas/ | Add metadata for the specific project
[**delete_project_metadata**](ProjectMetadataApi.md#delete_project_metadata) | **DELETE** /projects/{project_name_or_id}/metadatas/{meta_name} | Delete the specific metadata for the specific project
[**get_project_metadata**](ProjectMetadataApi.md#get_project_metadata) | **GET** /projects/{project_name_or_id}/metadatas/{meta_name} | Get the specific metadata of the specific project
[**list_project_metadatas**](ProjectMetadataApi.md#list_project_metadatas) | **GET** /projects/{project_name_or_id}/metadatas/ | Get the metadata of the specific project
[**update_project_metadata**](ProjectMetadataApi.md#update_project_metadata) | **PUT** /projects/{project_name_or_id}/metadatas/{meta_name} | Update the specific metadata for the specific project



## add_project_metadatas

> add_project_metadatas(project_name_or_id, x_request_id, x_is_resource_name, metadata)
Add metadata for the specific project

Add metadata for the specific project

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_name_or_id** | **String** | The name or id of the project | [required] |
**x_request_id** | Option<**String**> | An unique ID for the request |  |
**x_is_resource_name** | Option<**bool**> | The flag to indicate whether the parameter which supports both name and id in the path is the name of the resource. When the X-Is-Resource-Name is false and the parameter can be converted to an integer, the parameter will be as an id, otherwise, it will be as a name. |  |[default to false]
**metadata** | Option<[**::std::collections::HashMap<String, String>**](String.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_project_metadata

> delete_project_metadata(project_name_or_id, meta_name, x_request_id, x_is_resource_name)
Delete the specific metadata for the specific project

Delete the specific metadata for the specific project

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_name_or_id** | **String** | The name or id of the project | [required] |
**meta_name** | **String** | The name of metadata. | [required] |
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


## get_project_metadata

> ::std::collections::HashMap<String, String> get_project_metadata(project_name_or_id, meta_name, x_request_id, x_is_resource_name)
Get the specific metadata of the specific project

Get the specific metadata of the specific project

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_name_or_id** | **String** | The name or id of the project | [required] |
**meta_name** | **String** | The name of metadata. | [required] |
**x_request_id** | Option<**String**> | An unique ID for the request |  |
**x_is_resource_name** | Option<**bool**> | The flag to indicate whether the parameter which supports both name and id in the path is the name of the resource. When the X-Is-Resource-Name is false and the parameter can be converted to an integer, the parameter will be as an id, otherwise, it will be as a name. |  |[default to false]

### Return type

**::std::collections::HashMap<String, String>**

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_project_metadatas

> ::std::collections::HashMap<String, String> list_project_metadatas(project_name_or_id, x_request_id, x_is_resource_name)
Get the metadata of the specific project

Get the metadata of the specific project

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_name_or_id** | **String** | The name or id of the project | [required] |
**x_request_id** | Option<**String**> | An unique ID for the request |  |
**x_is_resource_name** | Option<**bool**> | The flag to indicate whether the parameter which supports both name and id in the path is the name of the resource. When the X-Is-Resource-Name is false and the parameter can be converted to an integer, the parameter will be as an id, otherwise, it will be as a name. |  |[default to false]

### Return type

**::std::collections::HashMap<String, String>**

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_project_metadata

> update_project_metadata(project_name_or_id, meta_name, x_request_id, x_is_resource_name, metadata)
Update the specific metadata for the specific project

Update the specific metadata for the specific project

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_name_or_id** | **String** | The name or id of the project | [required] |
**meta_name** | **String** | The name of metadata. | [required] |
**x_request_id** | Option<**String**> | An unique ID for the request |  |
**x_is_resource_name** | Option<**bool**> | The flag to indicate whether the parameter which supports both name and id in the path is the name of the resource. When the X-Is-Resource-Name is false and the parameter can be converted to an integer, the parameter will be as an id, otherwise, it will be as a name. |  |[default to false]
**metadata** | Option<[**::std::collections::HashMap<String, String>**](String.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

