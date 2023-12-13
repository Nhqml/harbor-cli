# ProjectMetadata

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**public** | Option<**String**> | The public status of the project. The valid values are \"true\", \"false\". | [optional]
**enable_content_trust** | Option<**String**> | Whether content trust is enabled or not. If it is enabled, user can't pull unsigned images from this project. The valid values are \"true\", \"false\". | [optional]
**enable_content_trust_cosign** | Option<**String**> | Whether cosign content trust is enabled or not. If it is enabled, user can't pull images without cosign signature from this project. The valid values are \"true\", \"false\". | [optional]
**prevent_vul** | Option<**String**> | Whether prevent the vulnerable images from running. The valid values are \"true\", \"false\". | [optional]
**severity** | Option<**String**> | If the vulnerability is high than severity defined here, the images can't be pulled. The valid values are \"none\", \"low\", \"medium\", \"high\", \"critical\". | [optional]
**auto_scan** | Option<**String**> | Whether scan images automatically when pushing. The valid values are \"true\", \"false\". | [optional]
**reuse_sys_cve_allowlist** | Option<**String**> | Whether this project reuse the system level CVE allowlist as the allowlist of its own.  The valid values are \"true\", \"false\". If it is set to \"true\" the actual allowlist associate with this project, if any, will be ignored. | [optional]
**retention_id** | Option<**String**> | The ID of the tag retention policy for the project | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


