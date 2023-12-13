# \SecurityhubApi

All URIs are relative to *http://localhost/api/v2.0*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_security_summary**](SecurityhubApi.md#get_security_summary) | **GET** /security/summary | Get vulnerability system summary
[**list_vulnerabilities**](SecurityhubApi.md#list_vulnerabilities) | **GET** /security/vul | Get the vulnerability list.



## get_security_summary

> crate::models::SecuritySummary get_security_summary(x_request_id, with_dangerous_cve, with_dangerous_artifact)
Get vulnerability system summary

Retrieve the vulnerability summary of the system

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_request_id** | Option<**String**> | An unique ID for the request |  |
**with_dangerous_cve** | Option<**bool**> | Specify whether the dangerous CVEs are included inside summary information |  |[default to false]
**with_dangerous_artifact** | Option<**bool**> | Specify whether the dangerous Artifact are included inside summary information |  |[default to false]

### Return type

[**crate::models::SecuritySummary**](SecuritySummary.md)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_vulnerabilities

> Vec<crate::models::VulnerabilityItem> list_vulnerabilities(x_request_id, q, page, page_size, tune_count, with_tag)
Get the vulnerability list.

Get the vulnerability list. use q to pass the query condition,  supported conditions: cve_id(exact match) cvss_score_v3(range condition) severity(exact match) repository_name(exact match)  project_id(exact match)  package(exact match) tag(exact match)  digest(exact match) 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_request_id** | Option<**String**> | An unique ID for the request |  |
**q** | Option<**String**> | Query string to query resources. Supported query patterns are \"exact match(k=v)\", \"fuzzy match(k=~v)\", \"range(k=[min~max])\", \"list with union releationship(k={v1 v2 v3})\" and \"list with intersetion relationship(k=(v1 v2 v3))\". The value of range and list can be string(enclosed by \" or '), integer or time(in format \"2020-04-09 02:36:00\"). All of these query patterns should be put in the query string \"q=xxx\" and splitted by \",\". e.g. q=k1=v1,k2=~v2,k3=[min~max] |  |
**page** | Option<**i64**> | The page number |  |[default to 1]
**page_size** | Option<**i64**> | The size of per page |  |[default to 10]
**tune_count** | Option<**bool**> | Enable to ignore X-Total-Count when the total count > 1000, if the total count is less than 1000, the real total count is returned, else -1. |  |[default to false]
**with_tag** | Option<**bool**> | Specify whether the tag information is included inside vulnerability information |  |[default to false]

### Return type

[**Vec<crate::models::VulnerabilityItem>**](VulnerabilityItem.md)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

