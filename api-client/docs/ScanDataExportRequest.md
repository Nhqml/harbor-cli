# ScanDataExportRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**job_name** | Option<**String**> | Name of the scan data export job | [optional]
**projects** | Option<**Vec<i64>**> | A list of one or more projects for which to export the scan data, currently only one project is supported due to performance concerns, but define as array for extension in the future. | [optional]
**labels** | Option<**Vec<i64>**> | A list of one or more labels for which to export the scan data, defaults to all if empty | [optional]
**repositories** | Option<**String**> | A list of repositories for which to export the scan data, defaults to all if empty | [optional]
**cve_ids** | Option<**String**> | CVE-IDs for which to export data. Multiple CVE-IDs can be specified by separating using ',' and enclosed between '{}'. Defaults to all if empty | [optional]
**tags** | Option<**String**> | A list of tags enclosed within '{}'. Defaults to all if empty | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


