# \ViolationApi

All URIs are relative to */api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_violations**](ViolationApi.md#get_violations) | **GET** /v1/violation | Returns a list of all policy violations for the entire portfolio
[**get_violations_by_component**](ViolationApi.md#get_violations_by_component) | **GET** /v1/violation/component/{uuid} | Returns a list of all policy violations for a specific component
[**get_violations_by_project**](ViolationApi.md#get_violations_by_project) | **GET** /v1/violation/project/{uuid} | Returns a list of all policy violations for a specific project



## get_violations

> Vec<models::PolicyViolation> get_violations(suppressed, page_number, page_size, offset, limit, sort_name, sort_order)
Returns a list of all policy violations for the entire portfolio

<p>Requires permission <strong>VIEW_POLICY_VIOLATION</strong></p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**suppressed** | Option<**bool**> | Optionally includes suppressed violations |  |
**page_number** | Option<[**serde_json::Value**](.md)> | The page to return. To be used in conjunction with <code>pageSize</code>. |  |[default to 1]
**page_size** | Option<[**serde_json::Value**](.md)> | Number of elements to return per page. To be used in conjunction with <code>pageNumber</code>. |  |[default to 100]
**offset** | Option<[**serde_json::Value**](.md)> | Offset to start returning elements from. To be used in conjunction with <code>limit</code>. |  |
**limit** | Option<[**serde_json::Value**](.md)> | Number of elements to return per page. To be used in conjunction with <code>offset</code>. |  |
**sort_name** | Option<**String**> | Name of the resource field to sort on. |  |
**sort_order** | Option<**String**> | Ordering of items when sorting with <code>sortName</code>. |  |

### Return type

[**Vec<models::PolicyViolation>**](PolicyViolation.md)

### Authorization

[X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_violations_by_component

> Vec<models::PolicyViolation> get_violations_by_component(uuid, suppressed, page_number, page_size, offset, limit, sort_name, sort_order)
Returns a list of all policy violations for a specific component

<p>Requires permission <strong>VIEW_POLICY_VIOLATION</strong></p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **uuid::Uuid** | The UUID of the component | [required] |
**suppressed** | Option<**bool**> | Optionally includes suppressed violations |  |
**page_number** | Option<[**serde_json::Value**](.md)> | The page to return. To be used in conjunction with <code>pageSize</code>. |  |[default to 1]
**page_size** | Option<[**serde_json::Value**](.md)> | Number of elements to return per page. To be used in conjunction with <code>pageNumber</code>. |  |[default to 100]
**offset** | Option<[**serde_json::Value**](.md)> | Offset to start returning elements from. To be used in conjunction with <code>limit</code>. |  |
**limit** | Option<[**serde_json::Value**](.md)> | Number of elements to return per page. To be used in conjunction with <code>offset</code>. |  |
**sort_name** | Option<**String**> | Name of the resource field to sort on. |  |
**sort_order** | Option<**String**> | Ordering of items when sorting with <code>sortName</code>. |  |

### Return type

[**Vec<models::PolicyViolation>**](PolicyViolation.md)

### Authorization

[X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_violations_by_project

> Vec<models::PolicyViolation> get_violations_by_project(uuid, suppressed, page_number, page_size, offset, limit, sort_name, sort_order)
Returns a list of all policy violations for a specific project

<p>Requires permission <strong>VIEW_POLICY_VIOLATION</strong></p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **uuid::Uuid** | The UUID of the project | [required] |
**suppressed** | Option<**bool**> | Optionally includes suppressed violations |  |
**page_number** | Option<[**serde_json::Value**](.md)> | The page to return. To be used in conjunction with <code>pageSize</code>. |  |[default to 1]
**page_size** | Option<[**serde_json::Value**](.md)> | Number of elements to return per page. To be used in conjunction with <code>pageNumber</code>. |  |[default to 100]
**offset** | Option<[**serde_json::Value**](.md)> | Offset to start returning elements from. To be used in conjunction with <code>limit</code>. |  |
**limit** | Option<[**serde_json::Value**](.md)> | Number of elements to return per page. To be used in conjunction with <code>offset</code>. |  |
**sort_name** | Option<**String**> | Name of the resource field to sort on. |  |
**sort_order** | Option<**String**> | Ordering of items when sorting with <code>sortName</code>. |  |

### Return type

[**Vec<models::PolicyViolation>**](PolicyViolation.md)

### Authorization

[X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

