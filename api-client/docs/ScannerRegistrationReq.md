# ScannerRegistrationReq

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | The name of this registration | 
**description** | Option<**String**> | An optional description of this registration. | [optional]
**url** | **String** | A base URL of the scanner adapter. | 
**auth** | Option<**String**> | Specify what authentication approach is adopted for the HTTP communications. Supported types Basic\", \"Bearer\" and api key header \"X-ScannerAdapter-API-Key\"  | [optional]
**access_credential** | Option<**String**> | An optional value of the HTTP Authorization header sent with each request to the Scanner Adapter API.  | [optional]
**skip_cert_verify** | Option<**bool**> | Indicate if skip the certificate verification when sending HTTP requests | [optional][default to false]
**use_internal_addr** | Option<**bool**> | Indicate whether use internal registry addr for the scanner to pull content or not | [optional][default to false]
**disabled** | Option<**bool**> | Indicate whether the registration is enabled or not | [optional][default to false]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


