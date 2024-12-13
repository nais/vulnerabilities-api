# \BomApi

All URIs are relative to */api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**export_component_as_cyclone_dx**](BomApi.md#export_component_as_cyclone_dx) | **GET** /v1/bom/cyclonedx/component/{uuid} | Returns dependency metadata for a specific component in CycloneDX format
[**export_project_as_cyclone_dx**](BomApi.md#export_project_as_cyclone_dx) | **GET** /v1/bom/cyclonedx/project/{uuid} | Returns dependency metadata for a project in CycloneDX format
[**is_token_being_processed**](BomApi.md#is_token_being_processed) | **GET** /v1/bom/token/{uuid} | Determines if there are any tasks associated with the token that are being processed, or in the queue to be processed.
[**upload_bom**](BomApi.md#upload_bom) | **POST** /v1/bom | Upload a supported bill of material format document
[**upload_bom_base64_encoded**](BomApi.md#upload_bom_base64_encoded) | **PUT** /v1/bom | Upload a supported bill of material format document



## export_component_as_cyclone_dx

> String export_component_as_cyclone_dx(uuid, format)
Returns dependency metadata for a specific component in CycloneDX format

<p>Requires permission <strong>VIEW_PORTFOLIO</strong></p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **uuid::Uuid** | The UUID of the component to export | [required] |
**format** | Option<**String**> | The format to output (defaults to JSON) |  |

### Return type

**String**

### Authorization

[X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.cyclonedx+xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## export_project_as_cyclone_dx

> String export_project_as_cyclone_dx(uuid, format, variant, download)
Returns dependency metadata for a project in CycloneDX format

<p>Requires permission <strong>VIEW_PORTFOLIO</strong></p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **uuid::Uuid** | The UUID of the project to export | [required] |
**format** | Option<**String**> | The format to output (defaults to JSON) |  |
**variant** | Option<**String**> | Specifies the CycloneDX variant to export. Value options are 'inventory' and 'withVulnerabilities'. (defaults to 'inventory') |  |
**download** | Option<**bool**> | Force the resulting BOM to be downloaded as a file (defaults to 'false') |  |

### Return type

**String**

### Authorization

[X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.cyclonedx+xml, application/vnd.cyclonedx+json, application/octet-stream

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## is_token_being_processed

> models::IsTokenBeingProcessedResponse is_token_being_processed(uuid)
Determines if there are any tasks associated with the token that are being processed, or in the queue to be processed.

<p>   This endpoint is intended to be used in conjunction with uploading a supported BOM document.   Upon upload, a token will be returned. The token can then be queried using this endpoint to   determine if any tasks (such as vulnerability analysis) is being performed on the BOM:   <ul>     <li>A value of <code>true</code> indicates processing is occurring.</li>     <li>A value of <code>false</code> indicates that no processing is occurring for the specified token.</li>   </ul>   However, a value of <code>false</code> also does not confirm the token is valid,   only that no processing is associated with the specified token. </p> <p>Requires permission <strong>BOM_UPLOAD</strong></p> <p><strong>Deprecated</strong>. Use <code>/v1/event/token/{uuid}</code> instead.</p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **uuid::Uuid** | The UUID of the token to query | [required] |

### Return type

[**models::IsTokenBeingProcessedResponse**](IsTokenBeingProcessedResponse.md)

### Authorization

[X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## upload_bom

> models::BomUploadResponse upload_bom(project, auto_create, project_name, project_version, parent_name, parent_version, parent_uuid, bom)
Upload a supported bill of material format document

<p>    Expects CycloneDX and a valid project UUID. If a UUID is not specified,    then the <code>projectName</code> and <code>projectVersion</code> must be specified.    Optionally, if <code>autoCreate</code> is specified and <code>true</code> and the project does not exist,    the project will be created. In this scenario, the principal making the request will    additionally need the <strong>PORTFOLIO_MANAGEMENT</strong> or    <strong>PROJECT_CREATION_UPLOAD</strong> permission.  </p>  <p>    The BOM will be validated against the CycloneDX schema. If schema validation fails,    a response with problem details in RFC 9457 format will be returned. In this case,    the response's content type will be <code>application/problem+json</code>.  </p>  <p>Requires permission <strong>BOM_UPLOAD</strong></p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | Option<**String**> |  |  |
**auto_create** | Option<**bool**> |  |  |[default to false]
**project_name** | Option<**String**> |  |  |
**project_version** | Option<**String**> |  |  |
**parent_name** | Option<**String**> |  |  |
**parent_version** | Option<**String**> |  |  |
**parent_uuid** | Option<**String**> |  |  |
**bom** | Option<**String**> |  |  |

### Return type

[**models::BomUploadResponse**](BomUploadResponse.md)

### Authorization

[X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## upload_bom_base64_encoded

> models::BomUploadResponse upload_bom_base64_encoded(body)
Upload a supported bill of material format document

<p>   Expects CycloneDX and a valid project UUID. If a UUID is not specified,   then the <code>projectName</code> and <code>projectVersion</code> must be specified.   Optionally, if <code>autoCreate</code> is specified and <code>true</code> and the project does not exist,   the project will be created. In this scenario, the principal making the request will   additionally need the <strong>PORTFOLIO_MANAGEMENT</strong> or   <strong>PROJECT_CREATION_UPLOAD</strong> permission. </p> <p>   The BOM will be validated against the CycloneDX schema. If schema validation fails,   a response with problem details in RFC 9457 format will be returned. In this case,   the response's content type will be <code>application/problem+json</code>. </p> <p>   The maximum allowed length of the <code>bom</code> value is 20'000'000 characters.   When uploading large BOMs, the <code>POST</code> endpoint is preferred,   as it does not have this limit. </p> <p>Requires permission <strong>BOM_UPLOAD</strong></p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**BomSubmitRequest**](BomSubmitRequest.md) |  | [required] |

### Return type

[**models::BomUploadResponse**](BomUploadResponse.md)

### Authorization

[X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

