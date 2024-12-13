# \AnalysisApi

All URIs are relative to */api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**retrieve_analysis**](AnalysisApi.md#retrieve_analysis) | **GET** /v1/analysis | Retrieves an analysis trail
[**update_analysis**](AnalysisApi.md#update_analysis) | **PUT** /v1/analysis | Records an analysis decision



## retrieve_analysis

> models::Analysis retrieve_analysis(component, vulnerability, project)
Retrieves an analysis trail

<p>Requires permission <strong>VIEW_VULNERABILITY</strong></p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**component** | **uuid::Uuid** | The UUID of the component | [required] |
**vulnerability** | **uuid::Uuid** | The UUID of the vulnerability | [required] |
**project** | Option<**uuid::Uuid**> | The UUID of the project |  |

### Return type

[**models::Analysis**](Analysis.md)

### Authorization

[X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_analysis

> models::Analysis update_analysis(body)
Records an analysis decision

<p>Requires permission <strong>VULNERABILITY_ANALYSIS</strong></p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**AnalysisRequest**](AnalysisRequest.md)> |  |  |

### Return type

[**models::Analysis**](Analysis.md)

### Authorization

[X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

