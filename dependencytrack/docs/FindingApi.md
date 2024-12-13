# \FindingApi

All URIs are relative to */api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**analyze_project**](FindingApi.md#analyze_project) | **POST** /v1/finding/project/{uuid}/analyze | Triggers Vulnerability Analysis on a specific project
[**export_findings_by_project**](FindingApi.md#export_findings_by_project) | **GET** /v1/finding/project/{uuid}/export | Returns the findings for the specified project as FPF
[**get_all_findings**](FindingApi.md#get_all_findings) | **GET** /v1/finding | Returns a list of all findings
[**get_all_findings1**](FindingApi.md#get_all_findings1) | **GET** /v1/finding/grouped | Returns a list of all findings grouped by vulnerability
[**get_findings_by_project**](FindingApi.md#get_findings_by_project) | **GET** /v1/finding/project/{uuid} | Returns a list of all findings for a specific project or generates SARIF file if Accept: application/sarif+json header is provided



## analyze_project

> models::Project analyze_project(uuid)
Triggers Vulnerability Analysis on a specific project

<p>Requires permission <strong>VIEW_VULNERABILITY</strong></p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **uuid::Uuid** | The UUID of the project to analyze | [required] |

### Return type

[**models::Project**](Project.md)

### Authorization

[X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## export_findings_by_project

> export_findings_by_project(uuid)
Returns the findings for the specified project as FPF

<p>Requires permission <strong>VIEW_VULNERABILITY</strong></p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **uuid::Uuid** | The UUID of the project | [required] |

### Return type

 (empty response body)

### Authorization

[X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_all_findings

> Vec<models::Finding> get_all_findings(show_inactive, show_suppressed, severity, analysis_status, vendor_response, publish_date_from, publish_date_to, attributed_on_date_from, attributed_on_date_to, text_search_field, text_search_input, cvssv2_from, cvssv2_to, cvssv3_from, cvssv3_to)
Returns a list of all findings

<p>Requires permission <strong>VIEW_VULNERABILITY</strong></p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**show_inactive** | Option<**bool**> | Show inactive projects |  |
**show_suppressed** | Option<**bool**> | Show suppressed findings |  |
**severity** | Option<**String**> | Filter by severity |  |
**analysis_status** | Option<**String**> | Filter by analysis status |  |
**vendor_response** | Option<**String**> | Filter by vendor response |  |
**publish_date_from** | Option<**String**> | Filter published from this date |  |
**publish_date_to** | Option<**String**> | Filter published to this date |  |
**attributed_on_date_from** | Option<**String**> | Filter attributed on from this date |  |
**attributed_on_date_to** | Option<**String**> | Filter attributed on to this date |  |
**text_search_field** | Option<**String**> | Filter the text input in these fields |  |
**text_search_input** | Option<**String**> | Filter by this text input |  |
**cvssv2_from** | Option<**String**> | Filter CVSSv2 from this value |  |
**cvssv2_to** | Option<**String**> | Filter CVSSv2 from this Value |  |
**cvssv3_from** | Option<**String**> | Filter CVSSv3 from this value |  |
**cvssv3_to** | Option<**String**> | Filter CVSSv3 from this Value |  |

### Return type

[**Vec<models::Finding>**](Finding.md)

### Authorization

[X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_all_findings1

> Vec<models::GroupedFinding> get_all_findings1(show_inactive, severity, publish_date_from, publish_date_to, text_search_field, text_search_input, cvssv2_from, cvssv2_to, cvssv3_from, cvssv3_to, occurrences_from, occurrences_to)
Returns a list of all findings grouped by vulnerability

<p>Requires permission <strong>VIEW_VULNERABILITY</strong></p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**show_inactive** | Option<**bool**> | Show inactive projects |  |
**severity** | Option<**String**> | Filter by severity |  |
**publish_date_from** | Option<**String**> | Filter published from this date |  |
**publish_date_to** | Option<**String**> | Filter published to this date |  |
**text_search_field** | Option<**String**> | Filter the text input in these fields |  |
**text_search_input** | Option<**String**> | Filter by this text input |  |
**cvssv2_from** | Option<**String**> | Filter CVSSv2 from this value |  |
**cvssv2_to** | Option<**String**> | Filter CVSSv2 to this value |  |
**cvssv3_from** | Option<**String**> | Filter CVSSv3 from this value |  |
**cvssv3_to** | Option<**String**> | Filter CVSSv3 to this value |  |
**occurrences_from** | Option<**String**> | Filter occurrences in projects from this value |  |
**occurrences_to** | Option<**String**> | Filter occurrences in projects to this value |  |

### Return type

[**Vec<models::GroupedFinding>**](GroupedFinding.md)

### Authorization

[X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_findings_by_project

> Vec<models::Finding> get_findings_by_project(uuid, suppressed, source, accept)
Returns a list of all findings for a specific project or generates SARIF file if Accept: application/sarif+json header is provided

<p>Requires permission <strong>VIEW_VULNERABILITY</strong></p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **uuid::Uuid** | The UUID of the project | [required] |
**suppressed** | Option<**bool**> | Optionally includes suppressed findings |  |
**source** | Option<**String**> | Optionally limit findings to specific sources of vulnerability intelligence |  |
**accept** | Option<**String**> |  |  |

### Return type

[**Vec<models::Finding>**](Finding.md)

### Authorization

[X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/sarif+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

