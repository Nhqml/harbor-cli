# \RegistryApi

All URIs are relative to *http://localhost/api/v2.0*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_registry**](RegistryApi.md#create_registry) | **POST** /registries | Create a registry
[**delete_registry**](RegistryApi.md#delete_registry) | **DELETE** /registries/{id} | Delete the specific registry
[**get_registry**](RegistryApi.md#get_registry) | **GET** /registries/{id} | Get the specific registry
[**get_registry_info**](RegistryApi.md#get_registry_info) | **GET** /registries/{id}/info | Get the registry info
[**list_registries**](RegistryApi.md#list_registries) | **GET** /registries | List the registries
[**list_registry_provider_infos**](RegistryApi.md#list_registry_provider_infos) | **GET** /replication/adapterinfos | List all registered registry provider information
[**list_registry_provider_types**](RegistryApi.md#list_registry_provider_types) | **GET** /replication/adapters | List registry adapters
[**ping_registry**](RegistryApi.md#ping_registry) | **POST** /registries/ping | Check status of a registry
[**update_registry**](RegistryApi.md#update_registry) | **PUT** /registries/{id} | Update the registry



## create_registry

> create_registry(registry, x_request_id)
Create a registry

Create a registry

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**registry** | [**Registry**](Registry.md) | The registry | [required] |
**x_request_id** | Option<**String**> | An unique ID for the request |  |

### Return type

 (empty response body)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_registry

> delete_registry(id, x_request_id)
Delete the specific registry

Delete the specific registry

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** | Registry ID | [required] |
**x_request_id** | Option<**String**> | An unique ID for the request |  |

### Return type

 (empty response body)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_registry

> crate::models::Registry get_registry(id, x_request_id)
Get the specific registry

Get the specific registry

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** | Registry ID | [required] |
**x_request_id** | Option<**String**> | An unique ID for the request |  |

### Return type

[**crate::models::Registry**](Registry.md)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_registry_info

> crate::models::RegistryInfo get_registry_info(id, x_request_id)
Get the registry info

Get the registry info

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** | Registry ID | [required] |
**x_request_id** | Option<**String**> | An unique ID for the request |  |

### Return type

[**crate::models::RegistryInfo**](RegistryInfo.md)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_registries

> Vec<crate::models::Registry> list_registries(x_request_id, q, sort, page, page_size, name)
List the registries

List the registries

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_request_id** | Option<**String**> | An unique ID for the request |  |
**q** | Option<**String**> | Query string to query resources. Supported query patterns are \"exact match(k=v)\", \"fuzzy match(k=~v)\", \"range(k=[min~max])\", \"list with union releationship(k={v1 v2 v3})\" and \"list with intersetion relationship(k=(v1 v2 v3))\". The value of range and list can be string(enclosed by \" or '), integer or time(in format \"2020-04-09 02:36:00\"). All of these query patterns should be put in the query string \"q=xxx\" and splitted by \",\". e.g. q=k1=v1,k2=~v2,k3=[min~max] |  |
**sort** | Option<**String**> | Sort the resource list in ascending or descending order. e.g. sort by field1 in ascending order and field2 in descending order with \"sort=field1,-field2\" |  |
**page** | Option<**i64**> | The page number |  |[default to 1]
**page_size** | Option<**i64**> | The size of per page |  |[default to 10]
**name** | Option<**String**> | Deprecated, use `q` instead. |  |

### Return type

[**Vec<crate::models::Registry>**](Registry.md)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_registry_provider_infos

> ::std::collections::HashMap<String, crate::models::RegistryProviderInfo> list_registry_provider_infos(x_request_id)
List all registered registry provider information

List all registered registry provider information

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_request_id** | Option<**String**> | An unique ID for the request |  |

### Return type

[**::std::collections::HashMap<String, crate::models::RegistryProviderInfo>**](RegistryProviderInfo.md)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_registry_provider_types

> Vec<String> list_registry_provider_types(x_request_id)
List registry adapters

List registry adapters

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_request_id** | Option<**String**> | An unique ID for the request |  |

### Return type

**Vec<String>**

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ping_registry

> ping_registry(registry, x_request_id)
Check status of a registry

Check status of a registry

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**registry** | [**RegistryPing**](RegistryPing.md) | The registry | [required] |
**x_request_id** | Option<**String**> | An unique ID for the request |  |

### Return type

 (empty response body)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_registry

> update_registry(id, registry, x_request_id)
Update the registry

Update the registry

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** | The registry ID | [required] |
**registry** | [**RegistryUpdate**](RegistryUpdate.md) | The registry | [required] |
**x_request_id** | Option<**String**> | An unique ID for the request |  |

### Return type

 (empty response body)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

