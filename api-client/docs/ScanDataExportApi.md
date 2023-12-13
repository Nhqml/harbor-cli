# \ScanDataExportApi

All URIs are relative to *http://localhost/api/v2.0*

Method | HTTP request | Description
------------- | ------------- | -------------
[**download_scan_data**](ScanDataExportApi.md#download_scan_data) | **GET** /export/cve/download/{execution_id} | Download the scan data export file
[**export_scan_data**](ScanDataExportApi.md#export_scan_data) | **POST** /export/cve | Export scan data for selected projects
[**get_scan_data_export_execution**](ScanDataExportApi.md#get_scan_data_export_execution) | **GET** /export/cve/execution/{execution_id} | Get the specific scan data export execution
[**get_scan_data_export_execution_list**](ScanDataExportApi.md#get_scan_data_export_execution_list) | **GET** /export/cve/executions | Get a list of specific scan data export execution jobs for a specified user



## download_scan_data

> std::path::PathBuf download_scan_data(execution_id, x_request_id, format)
Download the scan data export file

Download the scan data report. Default format is CSV

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**execution_id** | **i32** | Execution ID | [required] |
**x_request_id** | Option<**String**> | An unique ID for the request |  |
**format** | Option<**String**> | The format of the data to be exported. e.g. CSV or PDF |  |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/csv

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## export_scan_data

> crate::models::ScanDataExportJob export_scan_data(x_scan_data_type, criteria, x_request_id)
Export scan data for selected projects

Export scan data for selected projects

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_scan_data_type** | **String** | The type of scan data to export | [required] |
**criteria** | [**ScanDataExportRequest**](ScanDataExportRequest.md) | The criteria for the export | [required] |
**x_request_id** | Option<**String**> | An unique ID for the request |  |

### Return type

[**crate::models::ScanDataExportJob**](ScanDataExportJob.md)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_scan_data_export_execution

> crate::models::ScanDataExportExecution get_scan_data_export_execution(execution_id, x_request_id)
Get the specific scan data export execution

Get the scan data export execution specified by ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**execution_id** | **i32** | Execution ID | [required] |
**x_request_id** | Option<**String**> | An unique ID for the request |  |

### Return type

[**crate::models::ScanDataExportExecution**](ScanDataExportExecution.md)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_scan_data_export_execution_list

> crate::models::ScanDataExportExecutionList get_scan_data_export_execution_list(x_request_id)
Get a list of specific scan data export execution jobs for a specified user

Get a list of specific scan data export execution jobs for a specified user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_request_id** | Option<**String**> | An unique ID for the request |  |

### Return type

[**crate::models::ScanDataExportExecutionList**](ScanDataExportExecutionList.md)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

