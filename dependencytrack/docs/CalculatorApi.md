# \CalculatorApi

All URIs are relative to */api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_cvss_scores**](CalculatorApi.md#get_cvss_scores) | **GET** /v1/calculator/cvss | Returns the CVSS base score, impact sub-score and exploitability sub-score
[**get_owasp_rr_scores**](CalculatorApi.md#get_owasp_rr_scores) | **GET** /v1/calculator/owasp | Returns the OWASP Risk Rating likelihood score, technical impact score and business impact score



## get_cvss_scores

> models::Score get_cvss_scores(vector)
Returns the CVSS base score, impact sub-score and exploitability sub-score

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**vector** | **String** | A valid CVSSv2 or CVSSv3 vector | [required] |

### Return type

[**models::Score**](Score.md)

### Authorization

[X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_owasp_rr_scores

> models::Score get_owasp_rr_scores(vector)
Returns the OWASP Risk Rating likelihood score, technical impact score and business impact score

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**vector** | **String** | A valid OWASP Risk Rating vector | [required] |

### Return type

[**models::Score**](Score.md)

### Authorization

[X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

