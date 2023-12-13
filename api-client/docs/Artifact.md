# Artifact

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i64**> | The ID of the artifact | [optional]
**r#type** | Option<**String**> | The type of the artifact, e.g. image, chart, etc | [optional]
**media_type** | Option<**String**> | The media type of the artifact | [optional]
**manifest_media_type** | Option<**String**> | The manifest media type of the artifact | [optional]
**project_id** | Option<**i64**> | The ID of the project that the artifact belongs to | [optional]
**repository_id** | Option<**i64**> | The ID of the repository that the artifact belongs to | [optional]
**digest** | Option<**String**> | The digest of the artifact | [optional]
**size** | Option<**i64**> | The size of the artifact | [optional]
**icon** | Option<**String**> | The digest of the icon | [optional]
**push_time** | Option<**String**> | The push time of the artifact | [optional]
**pull_time** | Option<**String**> | The latest pull time of the artifact | [optional]
**extra_attrs** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**annotations** | Option<**::std::collections::HashMap<String, String>**> |  | [optional]
**references** | Option<[**Vec<crate::models::Reference>**](Reference.md)> |  | [optional]
**tags** | Option<[**Vec<crate::models::Tag>**](Tag.md)> |  | [optional]
**addition_links** | Option<[**::std::collections::HashMap<String, crate::models::AdditionLink>**](AdditionLink.md)> |  | [optional]
**labels** | Option<[**Vec<crate::models::Label>**](Label.md)> |  | [optional]
**scan_overview** | Option<[**::std::collections::HashMap<String, crate::models::NativeReportSummary>**](NativeReportSummary.md)> | The scan overview attached in the metadata of tag | [optional]
**accessories** | Option<[**Vec<crate::models::Accessory>**](Accessory.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


