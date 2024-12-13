# \ViolationanalysisApi

All URIs are relative to */api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**retrieve_analysis1**](ViolationanalysisApi.md#retrieve_analysis1) | **GET** /v1/violation/analysis | Retrieves a violation analysis trail
[**update_analysis1**](ViolationanalysisApi.md#update_analysis1) | **PUT** /v1/violation/analysis | Records a violation analysis decision



## retrieve_analysis1

> models::ViolationAnalysis retrieve_analysis1(component, policy_violation)
Retrieves a violation analysis trail

<p>Requires permission <strong>VIEW_POLICY_VIOLATION</strong></p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**component** | **uuid::Uuid** | The UUID of the component | [required] |
**policy_violation** | **uuid::Uuid** | The UUID of the policy violation | [required] |

### Return type

[**models::ViolationAnalysis**](ViolationAnalysis.md)

### Authorization

[X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_analysis1

> models::ViolationAnalysis update_analysis1(body)
Records a violation analysis decision

<p>Requires permission <strong>POLICY_VIOLATION_ANALYSIS</strong></p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**ViolationAnalysisRequest**](ViolationAnalysisRequest.md)> |  |  |

### Return type

[**models::ViolationAnalysis**](ViolationAnalysis.md)

### Authorization

[X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

