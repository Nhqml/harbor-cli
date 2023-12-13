# AuthproxySetting

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**endpoint** | Option<**String**> | The fully qualified URI of login endpoint of authproxy, such as 'https://192.168.1.2:8443/login' | [optional]
**tokenreivew_endpoint** | Option<**String**> | The fully qualified URI of token review endpoint of authproxy, such as 'https://192.168.1.2:8443/tokenreview' | [optional]
**skip_search** | Option<**bool**> | The flag to determine whether Harbor can skip search the user/group when adding him as a member. | [optional]
**verify_cert** | Option<**bool**> | The flag to determine whether Harbor should verify the certificate when connecting to the auth proxy. | [optional]
**server_certificate** | Option<**String**> | The certificate to be pinned when connecting auth proxy. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


