# NativeReportSummary

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**report_id** | Option<**String**> | id of the native scan report | [optional]
**scan_status** | Option<**String**> | The status of the report generating process | [optional]
**severity** | Option<**String**> | The overall severity | [optional]
**duration** | Option<**i64**> | The seconds spent for generating the report | [optional]
**summary** | Option<[**crate::models::VulnerabilitySummary**](VulnerabilitySummary.md)> |  | [optional]
**start_time** | Option<**String**> | The start time of the scan process that generating report | [optional]
**end_time** | Option<**String**> | The end time of the scan process that generating report | [optional]
**complete_percent** | Option<**i32**> | The complete percent of the scanning which value is between 0 and 100 | [optional]
**scanner** | Option<[**crate::models::Scanner**](Scanner.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


