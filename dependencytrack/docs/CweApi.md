# \CweApi

All URIs are relative to */api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_cwe**](CweApi.md#get_cwe) | **GET** /v1/cwe/{cweId} | Returns a specific CWE
[**get_cwes**](CweApi.md#get_cwes) | **GET** /v1/cwe | Returns a list of all CWEs



## get_cwe

> models::Cwe get_cwe(cwe_id)
Returns a specific CWE

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cwe_id** | **i32** | The CWE ID of the CWE to retrieve | [required] |

### Return type

[**models::Cwe**](Cwe.md)

### Authorization

[X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_cwes

> Vec<models::Cwe> get_cwes(page_number, page_size, offset, limit, sort_name, sort_order)
Returns a list of all CWEs

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_number** | Option<[**serde_json::Value**](.md)> | The page to return. To be used in conjunction with <code>pageSize</code>. |  |[default to 1]
**page_size** | Option<[**serde_json::Value**](.md)> | Number of elements to return per page. To be used in conjunction with <code>pageNumber</code>. |  |[default to 100]
**offset** | Option<[**serde_json::Value**](.md)> | Offset to start returning elements from. To be used in conjunction with <code>limit</code>. |  |
**limit** | Option<[**serde_json::Value**](.md)> | Number of elements to return per page. To be used in conjunction with <code>offset</code>. |  |
**sort_name** | Option<**String**> | Name of the resource field to sort on. |  |
**sort_order** | Option<**String**> | Ordering of items when sorting with <code>sortName</code>. |  |

### Return type

[**Vec<models::Cwe>**](Cwe.md)

### Authorization

[X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

