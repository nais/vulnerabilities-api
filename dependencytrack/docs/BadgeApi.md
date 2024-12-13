# \BadgeApi

All URIs are relative to */api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_project_policy_violations_badge**](BadgeApi.md#get_project_policy_violations_badge) | **GET** /v1/badge/violations/project/{uuid} | Returns a policy violations badge for a specific project
[**get_project_policy_violations_badge1**](BadgeApi.md#get_project_policy_violations_badge1) | **GET** /v1/badge/violations/project/{name}/{version} | Returns a policy violations badge for a specific project
[**get_project_vulnerabilities_badge**](BadgeApi.md#get_project_vulnerabilities_badge) | **GET** /v1/badge/vulns/project/{name}/{version} | Returns current metrics for a specific project
[**get_project_vulnerabilities_badge1**](BadgeApi.md#get_project_vulnerabilities_badge1) | **GET** /v1/badge/vulns/project/{uuid} | Returns current metrics for a specific project



## get_project_policy_violations_badge

> String get_project_policy_violations_badge(uuid)
Returns a policy violations badge for a specific project

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **uuid::Uuid** | The UUID of the project to retrieve a badge for | [required] |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: image/svg+xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_project_policy_violations_badge1

> String get_project_policy_violations_badge1(name, version)
Returns a policy violations badge for a specific project

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | The name of the project to query on | [required] |
**version** | **String** | The version of the project to query on | [required] |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: image/svg+xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_project_vulnerabilities_badge

> models::ProjectMetrics get_project_vulnerabilities_badge(name, version)
Returns current metrics for a specific project

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | The name of the project to query on | [required] |
**version** | **String** | The version of the project to query on | [required] |

### Return type

[**models::ProjectMetrics**](ProjectMetrics.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: image/svg+xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_project_vulnerabilities_badge1

> models::ProjectMetrics get_project_vulnerabilities_badge1(uuid)
Returns current metrics for a specific project

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **uuid::Uuid** | The UUID of the project to retrieve metrics for | [required] |

### Return type

[**models::ProjectMetrics**](ProjectMetrics.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: image/svg+xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

