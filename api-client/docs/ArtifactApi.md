# \ArtifactApi

All URIs are relative to *http://localhost/api/v2.0*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_label**](ArtifactApi.md#add_label) | **POST** /projects/{project_name}/repositories/{repository_name}/artifacts/{reference}/labels | Add label to artifact
[**copy_artifact**](ArtifactApi.md#copy_artifact) | **POST** /projects/{project_name}/repositories/{repository_name}/artifacts | Copy artifact
[**create_tag**](ArtifactApi.md#create_tag) | **POST** /projects/{project_name}/repositories/{repository_name}/artifacts/{reference}/tags | Create tag
[**delete_artifact**](ArtifactApi.md#delete_artifact) | **DELETE** /projects/{project_name}/repositories/{repository_name}/artifacts/{reference} | Delete the specific artifact
[**delete_tag**](ArtifactApi.md#delete_tag) | **DELETE** /projects/{project_name}/repositories/{repository_name}/artifacts/{reference}/tags/{tag_name} | Delete tag
[**get_addition**](ArtifactApi.md#get_addition) | **GET** /projects/{project_name}/repositories/{repository_name}/artifacts/{reference}/additions/{addition} | Get the addition of the specific artifact
[**get_artifact**](ArtifactApi.md#get_artifact) | **GET** /projects/{project_name}/repositories/{repository_name}/artifacts/{reference} | Get the specific artifact
[**get_vulnerabilities_addition**](ArtifactApi.md#get_vulnerabilities_addition) | **GET** /projects/{project_name}/repositories/{repository_name}/artifacts/{reference}/additions/vulnerabilities | Get the vulnerabilities addition of the specific artifact
[**list_accessories**](ArtifactApi.md#list_accessories) | **GET** /projects/{project_name}/repositories/{repository_name}/artifacts/{reference}/accessories | List accessories
[**list_artifacts**](ArtifactApi.md#list_artifacts) | **GET** /projects/{project_name}/repositories/{repository_name}/artifacts | List artifacts
[**list_tags**](ArtifactApi.md#list_tags) | **GET** /projects/{project_name}/repositories/{repository_name}/artifacts/{reference}/tags | List tags
[**remove_label**](ArtifactApi.md#remove_label) | **DELETE** /projects/{project_name}/repositories/{repository_name}/artifacts/{reference}/labels/{label_id} | Remove label from artifact



## add_label

> add_label(project_name, repository_name, reference, label, x_request_id)
Add label to artifact

Add label to the specified artiact.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_name** | **String** | The name of the project | [required] |
**repository_name** | **String** | The name of the repository. If it contains slash, encode it twice over with URL encoding. e.g. a/b -> a%2Fb -> a%252Fb | [required] |
**reference** | **String** | The reference of the artifact, can be digest or tag | [required] |
**label** | [**Label**](Label.md) | The label that added to the artifact. Only the ID property is needed. | [required] |
**x_request_id** | Option<**String**> | An unique ID for the request |  |

### Return type

 (empty response body)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## copy_artifact

> copy_artifact(project_name, repository_name, from, x_request_id)
Copy artifact

Copy the artifact specified in the \"from\" parameter to the repository.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_name** | **String** | The name of the project | [required] |
**repository_name** | **String** | The name of the repository. If it contains slash, encode it twice over with URL encoding. e.g. a/b -> a%2Fb -> a%252Fb | [required] |
**from** | **String** | The artifact from which the new artifact is copied from, the format should be \"project/repository:tag\" or \"project/repository@digest\". | [required] |
**x_request_id** | Option<**String**> | An unique ID for the request |  |

### Return type

 (empty response body)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_tag

> create_tag(project_name, repository_name, reference, tag, x_request_id)
Create tag

Create a tag for the specified artifact

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_name** | **String** | The name of the project | [required] |
**repository_name** | **String** | The name of the repository. If it contains slash, encode it twice over with URL encoding. e.g. a/b -> a%2Fb -> a%252Fb | [required] |
**reference** | **String** | The reference of the artifact, can be digest or tag | [required] |
**tag** | [**Tag**](Tag.md) | The JSON object of tag. | [required] |
**x_request_id** | Option<**String**> | An unique ID for the request |  |

### Return type

 (empty response body)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_artifact

> delete_artifact(project_name, repository_name, reference, x_request_id)
Delete the specific artifact

Delete the artifact specified by the reference under the project and repository. The reference can be digest or tag

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_name** | **String** | The name of the project | [required] |
**repository_name** | **String** | The name of the repository. If it contains slash, encode it twice over with URL encoding. e.g. a/b -> a%2Fb -> a%252Fb | [required] |
**reference** | **String** | The reference of the artifact, can be digest or tag | [required] |
**x_request_id** | Option<**String**> | An unique ID for the request |  |

### Return type

 (empty response body)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_tag

> delete_tag(project_name, repository_name, reference, tag_name, x_request_id)
Delete tag

Delete the tag of the specified artifact

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_name** | **String** | The name of the project | [required] |
**repository_name** | **String** | The name of the repository. If it contains slash, encode it twice over with URL encoding. e.g. a/b -> a%2Fb -> a%252Fb | [required] |
**reference** | **String** | The reference of the artifact, can be digest or tag | [required] |
**tag_name** | **String** | The name of the tag | [required] |
**x_request_id** | Option<**String**> | An unique ID for the request |  |

### Return type

 (empty response body)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_addition

> String get_addition(project_name, repository_name, reference, addition, x_request_id)
Get the addition of the specific artifact

Get the addition of the artifact specified by the reference under the project and repository.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_name** | **String** | The name of the project | [required] |
**repository_name** | **String** | The name of the repository. If it contains slash, encode it twice over with URL encoding. e.g. a/b -> a%2Fb -> a%252Fb | [required] |
**reference** | **String** | The reference of the artifact, can be digest or tag | [required] |
**addition** | **String** | The type of addition. | [required] |
**x_request_id** | Option<**String**> | An unique ID for the request |  |

### Return type

**String**

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_artifact

> crate::models::Artifact get_artifact(project_name, repository_name, reference, x_request_id, page, page_size, x_accept_vulnerabilities, with_tag, with_label, with_scan_overview, with_accessory, with_signature, with_immutable_status)
Get the specific artifact

Get the artifact specified by the reference under the project and repository. The reference can be digest or tag.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_name** | **String** | The name of the project | [required] |
**repository_name** | **String** | The name of the repository. If it contains slash, encode it twice over with URL encoding. e.g. a/b -> a%2Fb -> a%252Fb | [required] |
**reference** | **String** | The reference of the artifact, can be digest or tag | [required] |
**x_request_id** | Option<**String**> | An unique ID for the request |  |
**page** | Option<**i64**> | The page number |  |[default to 1]
**page_size** | Option<**i64**> | The size of per page |  |[default to 10]
**x_accept_vulnerabilities** | Option<**String**> | A comma-separated lists of MIME types for the scan report or scan summary. The first mime type will be used when the report found for it. Currently the mime type supports 'application/vnd.scanner.adapter.vuln.report.harbor+json; version=1.0' and 'application/vnd.security.vulnerability.report; version=1.1' |  |[default to application/vnd.security.vulnerability.report; version=1.1, application/vnd.scanner.adapter.vuln.report.harbor+json; version=1.0]
**with_tag** | Option<**bool**> | Specify whether the tags are inclued inside the returning artifacts |  |[default to true]
**with_label** | Option<**bool**> | Specify whether the labels are inclued inside the returning artifacts |  |[default to false]
**with_scan_overview** | Option<**bool**> | Specify whether the scan overview is inclued inside the returning artifacts |  |[default to false]
**with_accessory** | Option<**bool**> | Specify whether the accessories are included of the returning artifacts. |  |[default to false]
**with_signature** | Option<**bool**> | Specify whether the signature is inclued inside the returning artifacts |  |[default to false]
**with_immutable_status** | Option<**bool**> | Specify whether the immutable status is inclued inside the tags of the returning artifacts. |  |[default to false]

### Return type

[**crate::models::Artifact**](Artifact.md)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_vulnerabilities_addition

> String get_vulnerabilities_addition(project_name, repository_name, reference, x_request_id, x_accept_vulnerabilities)
Get the vulnerabilities addition of the specific artifact

Get the vulnerabilities addition of the artifact specified by the reference under the project and repository.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_name** | **String** | The name of the project | [required] |
**repository_name** | **String** | The name of the repository. If it contains slash, encode it twice over with URL encoding. e.g. a/b -> a%2Fb -> a%252Fb | [required] |
**reference** | **String** | The reference of the artifact, can be digest or tag | [required] |
**x_request_id** | Option<**String**> | An unique ID for the request |  |
**x_accept_vulnerabilities** | Option<**String**> | A comma-separated lists of MIME types for the scan report or scan summary. The first mime type will be used when the report found for it. Currently the mime type supports 'application/vnd.scanner.adapter.vuln.report.harbor+json; version=1.0' and 'application/vnd.security.vulnerability.report; version=1.1' |  |[default to application/vnd.security.vulnerability.report; version=1.1, application/vnd.scanner.adapter.vuln.report.harbor+json; version=1.0]

### Return type

**String**

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_accessories

> Vec<crate::models::Accessory> list_accessories(project_name, repository_name, reference, x_request_id, q, sort, page, page_size)
List accessories

List accessories of the specific artifact

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_name** | **String** | The name of the project | [required] |
**repository_name** | **String** | The name of the repository. If it contains slash, encode it twice over with URL encoding. e.g. a/b -> a%2Fb -> a%252Fb | [required] |
**reference** | **String** | The reference of the artifact, can be digest or tag | [required] |
**x_request_id** | Option<**String**> | An unique ID for the request |  |
**q** | Option<**String**> | Query string to query resources. Supported query patterns are \"exact match(k=v)\", \"fuzzy match(k=~v)\", \"range(k=[min~max])\", \"list with union releationship(k={v1 v2 v3})\" and \"list with intersetion relationship(k=(v1 v2 v3))\". The value of range and list can be string(enclosed by \" or '), integer or time(in format \"2020-04-09 02:36:00\"). All of these query patterns should be put in the query string \"q=xxx\" and splitted by \",\". e.g. q=k1=v1,k2=~v2,k3=[min~max] |  |
**sort** | Option<**String**> | Sort the resource list in ascending or descending order. e.g. sort by field1 in ascending order and field2 in descending order with \"sort=field1,-field2\" |  |
**page** | Option<**i64**> | The page number |  |[default to 1]
**page_size** | Option<**i64**> | The size of per page |  |[default to 10]

### Return type

[**Vec<crate::models::Accessory>**](Accessory.md)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_artifacts

> Vec<crate::models::Artifact> list_artifacts(project_name, repository_name, x_request_id, q, sort, page, page_size, x_accept_vulnerabilities, with_tag, with_label, with_scan_overview, with_signature, with_immutable_status, with_accessory)
List artifacts

List artifacts under the specific project and repository. Except the basic properties, the other supported queries in \"q\" includes \"tags=*\" to list only tagged artifacts, \"tags=nil\" to list only untagged artifacts, \"tags=~v\" to list artifacts whose tag fuzzy matches \"v\", \"tags=v\" to list artifact whose tag exactly matches \"v\", \"labels=(id1, id2)\" to list artifacts that both labels with id1 and id2 are added to

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_name** | **String** | The name of the project | [required] |
**repository_name** | **String** | The name of the repository. If it contains slash, encode it twice over with URL encoding. e.g. a/b -> a%2Fb -> a%252Fb | [required] |
**x_request_id** | Option<**String**> | An unique ID for the request |  |
**q** | Option<**String**> | Query string to query resources. Supported query patterns are \"exact match(k=v)\", \"fuzzy match(k=~v)\", \"range(k=[min~max])\", \"list with union releationship(k={v1 v2 v3})\" and \"list with intersetion relationship(k=(v1 v2 v3))\". The value of range and list can be string(enclosed by \" or '), integer or time(in format \"2020-04-09 02:36:00\"). All of these query patterns should be put in the query string \"q=xxx\" and splitted by \",\". e.g. q=k1=v1,k2=~v2,k3=[min~max] |  |
**sort** | Option<**String**> | Sort the resource list in ascending or descending order. e.g. sort by field1 in ascending order and field2 in descending order with \"sort=field1,-field2\" |  |
**page** | Option<**i64**> | The page number |  |[default to 1]
**page_size** | Option<**i64**> | The size of per page |  |[default to 10]
**x_accept_vulnerabilities** | Option<**String**> | A comma-separated lists of MIME types for the scan report or scan summary. The first mime type will be used when the report found for it. Currently the mime type supports 'application/vnd.scanner.adapter.vuln.report.harbor+json; version=1.0' and 'application/vnd.security.vulnerability.report; version=1.1' |  |[default to application/vnd.security.vulnerability.report; version=1.1, application/vnd.scanner.adapter.vuln.report.harbor+json; version=1.0]
**with_tag** | Option<**bool**> | Specify whether the tags are included inside the returning artifacts |  |[default to true]
**with_label** | Option<**bool**> | Specify whether the labels are included inside the returning artifacts |  |[default to false]
**with_scan_overview** | Option<**bool**> | Specify whether the scan overview is included inside the returning artifacts |  |[default to false]
**with_signature** | Option<**bool**> | Specify whether the signature is included inside the tags of the returning artifacts. Only works when setting \"with_tag=true\" |  |[default to false]
**with_immutable_status** | Option<**bool**> | Specify whether the immutable status is included inside the tags of the returning artifacts. Only works when setting \"with_immutable_status=true\" |  |[default to false]
**with_accessory** | Option<**bool**> | Specify whether the accessories are included of the returning artifacts. Only works when setting \"with_accessory=true\" |  |[default to false]

### Return type

[**Vec<crate::models::Artifact>**](Artifact.md)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_tags

> Vec<crate::models::Tag> list_tags(project_name, repository_name, reference, x_request_id, q, sort, page, page_size, with_signature, with_immutable_status)
List tags

List tags of the specific artifact

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_name** | **String** | The name of the project | [required] |
**repository_name** | **String** | The name of the repository. If it contains slash, encode it twice over with URL encoding. e.g. a/b -> a%2Fb -> a%252Fb | [required] |
**reference** | **String** | The reference of the artifact, can be digest or tag | [required] |
**x_request_id** | Option<**String**> | An unique ID for the request |  |
**q** | Option<**String**> | Query string to query resources. Supported query patterns are \"exact match(k=v)\", \"fuzzy match(k=~v)\", \"range(k=[min~max])\", \"list with union releationship(k={v1 v2 v3})\" and \"list with intersetion relationship(k=(v1 v2 v3))\". The value of range and list can be string(enclosed by \" or '), integer or time(in format \"2020-04-09 02:36:00\"). All of these query patterns should be put in the query string \"q=xxx\" and splitted by \",\". e.g. q=k1=v1,k2=~v2,k3=[min~max] |  |
**sort** | Option<**String**> | Sort the resource list in ascending or descending order. e.g. sort by field1 in ascending order and field2 in descending order with \"sort=field1,-field2\" |  |
**page** | Option<**i64**> | The page number |  |[default to 1]
**page_size** | Option<**i64**> | The size of per page |  |[default to 10]
**with_signature** | Option<**bool**> | Specify whether the signature is included inside the returning tags |  |[default to false]
**with_immutable_status** | Option<**bool**> | Specify whether the immutable status is included inside the returning tags |  |[default to false]

### Return type

[**Vec<crate::models::Tag>**](Tag.md)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_label

> remove_label(project_name, repository_name, reference, label_id, x_request_id)
Remove label from artifact

Remove the label from the specified artiact.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_name** | **String** | The name of the project | [required] |
**repository_name** | **String** | The name of the repository. If it contains slash, encode it twice over with URL encoding. e.g. a/b -> a%2Fb -> a%252Fb | [required] |
**reference** | **String** | The reference of the artifact, can be digest or tag | [required] |
**label_id** | **i64** | The ID of the label that removed from the artifact. | [required] |
**x_request_id** | Option<**String**> | An unique ID for the request |  |

### Return type

 (empty response body)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

