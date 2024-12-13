# \DependencyGraphApi

All URIs are relative to */api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_components_and_services_by_component_uuid**](DependencyGraphApi.md#get_components_and_services_by_component_uuid) | **GET** /v1/dependencyGraph/component/{uuid}/directDependencies | Returns a list of specific components and services from component UUID
[**get_components_and_services_by_project_uuid**](DependencyGraphApi.md#get_components_and_services_by_project_uuid) | **GET** /v1/dependencyGraph/project/{uuid}/directDependencies | Returns a list of specific components and services from project UUID



## get_components_and_services_by_component_uuid

> Vec<models::DependencyGraphResponse> get_components_and_services_by_component_uuid(uuid)
Returns a list of specific components and services from component UUID

<p>Requires permission <strong>VIEW_PORTFOLIO</strong></p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **uuid::Uuid** | The UUID of the component | [required] |

### Return type

[**Vec<models::DependencyGraphResponse>**](DependencyGraphResponse.md)

### Authorization

[X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_components_and_services_by_project_uuid

> Vec<models::DependencyGraphResponse> get_components_and_services_by_project_uuid(uuid)
Returns a list of specific components and services from project UUID

<p>Requires permission <strong>VIEW_PORTFOLIO</strong></p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **uuid::Uuid** | The UUID of the project | [required] |

### Return type

[**Vec<models::DependencyGraphResponse>**](DependencyGraphResponse.md)

### Authorization

[X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

