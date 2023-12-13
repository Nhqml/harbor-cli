# \StatisticApi

All URIs are relative to *http://localhost/api/v2.0*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_statistic**](StatisticApi.md#get_statistic) | **GET** /statistics | Get the statistic information about the projects and repositories



## get_statistic

> crate::models::Statistic get_statistic(x_request_id)
Get the statistic information about the projects and repositories

Get the statistic information about the projects and repositories

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_request_id** | Option<**String**> | An unique ID for the request |  |

### Return type

[**crate::models::Statistic**](Statistic.md)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

