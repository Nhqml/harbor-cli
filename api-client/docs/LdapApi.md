# \LdapApi

All URIs are relative to *http://localhost/api/v2.0*

Method | HTTP request | Description
------------- | ------------- | -------------
[**import_ldap_user**](LdapApi.md#import_ldap_user) | **POST** /ldap/users/import | Import selected available ldap users.
[**ping_ldap**](LdapApi.md#ping_ldap) | **POST** /ldap/ping | Ping available ldap service.
[**search_ldap_group**](LdapApi.md#search_ldap_group) | **GET** /ldap/groups/search | Search available ldap groups.
[**search_ldap_user**](LdapApi.md#search_ldap_user) | **GET** /ldap/users/search | Search available ldap users.



## import_ldap_user

> import_ldap_user(uid_list, x_request_id)
Import selected available ldap users.

This endpoint adds the selected available ldap users to harbor based on related configuration parameters from the system. System will try to guess the user email address and realname, add to harbor user information. If have errors when import user, will return the list of importing failed uid and the failed reason. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uid_list** | [**LdapImportUsers**](LdapImportUsers.md) | The uid listed for importing. This list will check users validity of ldap service based on configuration from the system. | [required] |
**x_request_id** | Option<**String**> | An unique ID for the request |  |

### Return type

 (empty response body)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ping_ldap

> crate::models::LdapPingResult ping_ldap(x_request_id, ldapconf)
Ping available ldap service.

This endpoint ping the available ldap service for test related configuration parameters. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_request_id** | Option<**String**> | An unique ID for the request |  |
**ldapconf** | Option<[**LdapConf**](LdapConf.md)> | ldap configuration. support input ldap service configuration. If it is a empty request, will load current configuration from the system. |  |

### Return type

[**crate::models::LdapPingResult**](LdapPingResult.md)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_ldap_group

> Vec<crate::models::UserGroup> search_ldap_group(x_request_id, groupname, groupdn)
Search available ldap groups.

This endpoint searches the available ldap groups based on related configuration parameters. support to search by groupname or groupdn. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_request_id** | Option<**String**> | An unique ID for the request |  |
**groupname** | Option<**String**> | Ldap group name |  |
**groupdn** | Option<**String**> | The LDAP group DN |  |

### Return type

[**Vec<crate::models::UserGroup>**](UserGroup.md)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_ldap_user

> Vec<crate::models::LdapUser> search_ldap_user(x_request_id, username)
Search available ldap users.

This endpoint searches the available ldap users based on related configuration parameters. Support searched by input ladp configuration, load configuration from the system and specific filter. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_request_id** | Option<**String**> | An unique ID for the request |  |
**username** | Option<**String**> | Registered user ID |  |

### Return type

[**Vec<crate::models::LdapUser>**](LdapUser.md)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

