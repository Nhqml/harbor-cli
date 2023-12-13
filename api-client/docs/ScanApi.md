# \ScanApi

All URIs are relative to *http://localhost/api/v2.0*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_report_log**](ScanApi.md#get_report_log) | **GET** /projects/{project_name}/repositories/{repository_name}/artifacts/{reference}/scan/{report_id}/log | Get the log of the scan report
[**scan_artifact**](ScanApi.md#scan_artifact) | **POST** /projects/{project_name}/repositories/{repository_name}/artifacts/{reference}/scan | Scan the artifact
[**stop_scan_artifact**](ScanApi.md#stop_scan_artifact) | **POST** /projects/{project_name}/repositories/{repository_name}/artifacts/{reference}/scan/stop | Cancelling a scan job for a particular artifact



## get_report_log

> String get_report_log(project_name, repository_name, reference, report_id, x_request_id)
Get the log of the scan report

Get the log of the scan report

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_name** | **String** | The name of the project | [required] |
**repository_name** | **String** | The name of the repository. If it contains slash, encode it twice over with URL encoding. e.g. a/b -> a%2Fb -> a%252Fb | [required] |
**reference** | **String** | The reference of the artifact, can be digest or tag | [required] |
**report_id** | **String** | The report id to get the log | [required] |
**x_request_id** | Option<**String**> | An unique ID for the request |  |

### Return type

**String**

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## scan_artifact

> scan_artifact(project_name, repository_name, reference, x_request_id)
Scan the artifact

Scan the specified artifact

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_name** | **String** | The name of the project | [required] |
**repository_name** | **String** | The name of the repository. If it contains slash, encode it twice over with URL encoding. e.g. a/b -> a%2Fb -> a%252Fb | [required] |
**reference** | **String** | The reference of the artifact, can be digest or tag | [required] |
**x_request_id** | Option<**String**> | An unique ID for the request |  |

### Return type

 (empty response body)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stop_scan_artifact

> stop_scan_artifact(project_name, repository_name, reference, x_request_id)
Cancelling a scan job for a particular artifact

Cancelling a scan job for a particular artifact

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_name** | **String** | The name of the project | [required] |
**repository_name** | **String** | The name of the repository. If it contains slash, encode it twice over with URL encoding. e.g. a/b -> a%2Fb -> a%252Fb | [required] |
**reference** | **String** | The reference of the artifact, can be digest or tag | [required] |
**x_request_id** | Option<**String**> | An unique ID for the request |  |

### Return type

 (empty response body)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

