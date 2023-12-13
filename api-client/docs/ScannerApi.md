# \ScannerApi

All URIs are relative to *http://localhost/api/v2.0*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_scanner**](ScannerApi.md#create_scanner) | **POST** /scanners | Create a scanner registration
[**delete_scanner**](ScannerApi.md#delete_scanner) | **DELETE** /scanners/{registration_id} | Delete a scanner registration
[**get_scanner**](ScannerApi.md#get_scanner) | **GET** /scanners/{registration_id} | Get a scanner registration details
[**get_scanner_metadata**](ScannerApi.md#get_scanner_metadata) | **GET** /scanners/{registration_id}/metadata | Get the metadata of the specified scanner registration
[**list_scanners**](ScannerApi.md#list_scanners) | **GET** /scanners | List scanner registrations
[**ping_scanner**](ScannerApi.md#ping_scanner) | **POST** /scanners/ping | Tests scanner registration settings
[**set_scanner_as_default**](ScannerApi.md#set_scanner_as_default) | **PATCH** /scanners/{registration_id} | Set system default scanner registration
[**update_scanner**](ScannerApi.md#update_scanner) | **PUT** /scanners/{registration_id} | Update a scanner registration



## create_scanner

> create_scanner(registration, x_request_id)
Create a scanner registration

Creats a new scanner registration with the given data. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**registration** | [**ScannerRegistrationReq**](ScannerRegistrationReq.md) | A scanner registration to be created. | [required] |
**x_request_id** | Option<**String**> | An unique ID for the request |  |

### Return type

 (empty response body)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_scanner

> crate::models::ScannerRegistration delete_scanner(registration_id, x_request_id)
Delete a scanner registration

Deletes the specified scanner registration. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**registration_id** | **String** | The scanner registration identifier. | [required] |
**x_request_id** | Option<**String**> | An unique ID for the request |  |

### Return type

[**crate::models::ScannerRegistration**](ScannerRegistration.md)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_scanner

> crate::models::ScannerRegistration get_scanner(registration_id, x_request_id)
Get a scanner registration details

Retruns the details of the specified scanner registration. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**registration_id** | **String** | The scanner registration identifer. | [required] |
**x_request_id** | Option<**String**> | An unique ID for the request |  |

### Return type

[**crate::models::ScannerRegistration**](ScannerRegistration.md)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_scanner_metadata

> crate::models::ScannerAdapterMetadata get_scanner_metadata(registration_id, x_request_id)
Get the metadata of the specified scanner registration

Get the metadata of the specified scanner registration, including the capabilities and customized properties. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**registration_id** | **String** | The scanner registration identifier. | [required] |
**x_request_id** | Option<**String**> | An unique ID for the request |  |

### Return type

[**crate::models::ScannerAdapterMetadata**](ScannerAdapterMetadata.md)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_scanners

> Vec<crate::models::ScannerRegistration> list_scanners(x_request_id, q, sort, page, page_size)
List scanner registrations

Returns a list of currently configured scanner registrations. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_request_id** | Option<**String**> | An unique ID for the request |  |
**q** | Option<**String**> | Query string to query resources. Supported query patterns are \"exact match(k=v)\", \"fuzzy match(k=~v)\", \"range(k=[min~max])\", \"list with union releationship(k={v1 v2 v3})\" and \"list with intersetion relationship(k=(v1 v2 v3))\". The value of range and list can be string(enclosed by \" or '), integer or time(in format \"2020-04-09 02:36:00\"). All of these query patterns should be put in the query string \"q=xxx\" and splitted by \",\". e.g. q=k1=v1,k2=~v2,k3=[min~max] |  |
**sort** | Option<**String**> | Sort the resource list in ascending or descending order. e.g. sort by field1 in ascending order and field2 in descending order with \"sort=field1,-field2\" |  |
**page** | Option<**i64**> | The page number |  |[default to 1]
**page_size** | Option<**i64**> | The size of per page |  |[default to 10]

### Return type

[**Vec<crate::models::ScannerRegistration>**](ScannerRegistration.md)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ping_scanner

> ping_scanner(settings, x_request_id)
Tests scanner registration settings

Pings scanner adapter to test endpoint URL and authorization settings. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**settings** | [**ScannerRegistrationSettings**](ScannerRegistrationSettings.md) | A scanner registration settings to be tested. | [required] |
**x_request_id** | Option<**String**> | An unique ID for the request |  |

### Return type

 (empty response body)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_scanner_as_default

> set_scanner_as_default(registration_id, payload, x_request_id)
Set system default scanner registration

Set the specified scanner registration as the system default one. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**registration_id** | **String** | The scanner registration identifier. | [required] |
**payload** | [**IsDefault**](IsDefault.md) |  | [required] |
**x_request_id** | Option<**String**> | An unique ID for the request |  |

### Return type

 (empty response body)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_scanner

> update_scanner(registration_id, registration, x_request_id)
Update a scanner registration

Updates the specified scanner registration. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**registration_id** | **String** | The scanner registration identifier. | [required] |
**registration** | [**ScannerRegistrationReq**](ScannerRegistrationReq.md) | A scanner registraiton to be updated. | [required] |
**x_request_id** | Option<**String**> | An unique ID for the request |  |

### Return type

 (empty response body)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

