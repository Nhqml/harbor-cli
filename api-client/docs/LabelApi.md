# \LabelApi

All URIs are relative to *http://localhost/api/v2.0*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_label**](LabelApi.md#create_label) | **POST** /labels | Post creates a label
[**delete_label**](LabelApi.md#delete_label) | **DELETE** /labels/{label_id} | Delete the label specified by ID.
[**get_label_by_id**](LabelApi.md#get_label_by_id) | **GET** /labels/{label_id} | Get the label specified by ID.
[**list_labels**](LabelApi.md#list_labels) | **GET** /labels | List labels according to the query strings.
[**update_label**](LabelApi.md#update_label) | **PUT** /labels/{label_id} | Update the label properties.



## create_label

> create_label(label, x_request_id)
Post creates a label

This endpoint let user creates a label. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**label** | [**Label**](Label.md) | The json object of label. | [required] |
**x_request_id** | Option<**String**> | An unique ID for the request |  |

### Return type

 (empty response body)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_label

> delete_label(label_id, x_request_id)
Delete the label specified by ID.

Delete the label specified by ID. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**label_id** | **i64** | Label ID | [required] |
**x_request_id** | Option<**String**> | An unique ID for the request |  |

### Return type

 (empty response body)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_label_by_id

> crate::models::Label get_label_by_id(label_id, x_request_id)
Get the label specified by ID.

This endpoint let user get the label by specific ID. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**label_id** | **i64** | Label ID | [required] |
**x_request_id** | Option<**String**> | An unique ID for the request |  |

### Return type

[**crate::models::Label**](Label.md)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_labels

> Vec<crate::models::Label> list_labels(x_request_id, q, sort, page, page_size, name, scope, project_id)
List labels according to the query strings.

This endpoint let user list labels by name, scope and project_id 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_request_id** | Option<**String**> | An unique ID for the request |  |
**q** | Option<**String**> | Query string to query resources. Supported query patterns are \"exact match(k=v)\", \"fuzzy match(k=~v)\", \"range(k=[min~max])\", \"list with union releationship(k={v1 v2 v3})\" and \"list with intersetion relationship(k=(v1 v2 v3))\". The value of range and list can be string(enclosed by \" or '), integer or time(in format \"2020-04-09 02:36:00\"). All of these query patterns should be put in the query string \"q=xxx\" and splitted by \",\". e.g. q=k1=v1,k2=~v2,k3=[min~max] |  |
**sort** | Option<**String**> | Sort the resource list in ascending or descending order. e.g. sort by field1 in ascending order and field2 in descending order with \"sort=field1,-field2\" |  |
**page** | Option<**i64**> | The page number |  |[default to 1]
**page_size** | Option<**i64**> | The size of per page |  |[default to 10]
**name** | Option<**String**> | The label name. |  |
**scope** | Option<**String**> | The label scope. Valid values are g and p. g for global labels and p for project labels. |  |
**project_id** | Option<**i64**> | Relevant project ID, required when scope is p. |  |

### Return type

[**Vec<crate::models::Label>**](Label.md)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_label

> update_label(label_id, label, x_request_id)
Update the label properties.

This endpoint let user update label properties. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**label_id** | **i64** | Label ID | [required] |
**label** | [**Label**](Label.md) | The updated label json object. | [required] |
**x_request_id** | Option<**String**> | An unique ID for the request |  |

### Return type

 (empty response body)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

