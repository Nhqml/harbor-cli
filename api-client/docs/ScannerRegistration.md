# ScannerRegistration

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**uuid** | Option<**String**> | The unique identifier of this registration. | [optional]
**name** | Option<**String**> | The name of this registration. | [optional]
**description** | Option<**String**> | An optional description of this registration. | [optional]
**url** | Option<**String**> | A base URL of the scanner adapter | [optional]
**disabled** | Option<**bool**> | Indicate whether the registration is enabled or not | [optional][default to false]
**is_default** | Option<**bool**> | Indicate if the registration is set as the system default one | [optional][default to false]
**auth** | Option<**String**> | Specify what authentication approach is adopted for the HTTP communications. Supported types Basic\", \"Bearer\" and api key header \"X-ScannerAdapter-API-Key\"  | [optional][default to ]
**access_credential** | Option<**String**> | An optional value of the HTTP Authorization header sent with each request to the Scanner Adapter API.  | [optional]
**skip_cert_verify** | Option<**bool**> | Indicate if skip the certificate verification when sending HTTP requests | [optional][default to false]
**use_internal_addr** | Option<**bool**> | Indicate whether use internal registry addr for the scanner to pull content or not | [optional][default to false]
**create_time** | Option<**String**> | The creation time of this registration | [optional]
**update_time** | Option<**String**> | The update time of this registration | [optional]
**adapter** | Option<**String**> | Optional property to describe the name of the scanner registration | [optional]
**vendor** | Option<**String**> | Optional property to describe the vendor of the scanner registration | [optional]
**version** | Option<**String**> | Optional property to describe the version of the scanner registration | [optional]
**health** | Option<**String**> | Indicate the healthy of the registration | [optional][default to ]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


