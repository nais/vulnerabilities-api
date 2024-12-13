# \VexApi

All URIs are relative to */api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**export_project_as_cyclone_dx1**](VexApi.md#export_project_as_cyclone_dx1) | **GET** /v1/vex/cyclonedx/project/{uuid} | Returns a VEX for a project in CycloneDX format
[**upload_vex**](VexApi.md#upload_vex) | **POST** /v1/vex | Upload a supported VEX document
[**upload_vex1**](VexApi.md#upload_vex1) | **PUT** /v1/vex | Upload a supported VEX document



## export_project_as_cyclone_dx1

> String export_project_as_cyclone_dx1(uuid, download)
Returns a VEX for a project in CycloneDX format

<p>Requires permission <strong>VULNERABILITY_ANALYSIS</strong></p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **uuid::Uuid** | The UUID of the project to export | [required] |
**download** | Option<**bool**> | Force the resulting VEX to be downloaded as a file (defaults to 'false') |  |

### Return type

**String**

### Authorization

[X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.cyclonedx+json, application/octet-stream

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## upload_vex

> upload_vex(project, project_name, project_version, vex)
Upload a supported VEX document

<p>   Expects CycloneDX and a valid project UUID. If a UUID is not specified,   then the <code>projectName</code> and <code>projectVersion</code> must be specified. </p> <p>   The VEX will be validated against the CycloneDX schema. If schema validation fails,   a response with problem details in RFC 9457 format will be returned. In this case,   the response's content type will be <code>application/problem+json</code>. </p> <p>Requires permission <strong>VULNERABILITY_ANALYSIS</strong></p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | Option<**String**> |  |  |
**project_name** | Option<**String**> |  |  |
**project_version** | Option<**String**> |  |  |
**vex** | Option<**String**> |  |  |

### Return type

 (empty response body)

### Authorization

[X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## upload_vex1

> upload_vex1(body)
Upload a supported VEX document

<p>   Expects CycloneDX and a valid project UUID. If a UUID is not specified,   then the <code>projectName</code> and <code>projectVersion</code> must be specified. </p> <p>   The VEX will be validated against the CycloneDX schema. If schema validation fails,   a response with problem details in RFC 9457 format will be returned. In this case,   the response's content type will be <code>application/problem+json</code>. </p> <p>   The maximum allowed length of the <code>vex</code> value is 20'000'000 characters.   When uploading large VEX files, the <code>POST</code> endpoint is preferred,   as it does not have this limit. </p> <p>Requires permission <strong>VULNERABILITY_ANALYSIS</strong></p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**VexSubmitRequest**](VexSubmitRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

