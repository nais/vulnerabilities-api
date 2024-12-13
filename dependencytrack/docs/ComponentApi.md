# \ComponentApi

All URIs are relative to */api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_component**](ComponentApi.md#create_component) | **PUT** /v1/component/project/{uuid} | Creates a new component
[**delete_component**](ComponentApi.md#delete_component) | **DELETE** /v1/component/{uuid} | Deletes a component
[**get_all_components**](ComponentApi.md#get_all_components) | **GET** /v1/component/project/{uuid} | Returns a list of all components for a given project
[**get_component_by_hash**](ComponentApi.md#get_component_by_hash) | **GET** /v1/component/hash/{hash} | Returns a list of components that have the specified hash value
[**get_component_by_identity**](ComponentApi.md#get_component_by_identity) | **GET** /v1/component/identity | Returns a list of components that have the specified component identity. This resource accepts coordinates (group, name, version) or purl, cpe, or swidTagId
[**get_component_by_uuid**](ComponentApi.md#get_component_by_uuid) | **GET** /v1/component/{uuid} | Returns a specific component
[**get_dependency_graph_for_component**](ComponentApi.md#get_dependency_graph_for_component) | **GET** /v1/component/project/{projectUuid}/dependencyGraph/{componentUuids} | Returns the expanded dependency graph to every occurrence of a component
[**identify_internal_components**](ComponentApi.md#identify_internal_components) | **GET** /v1/component/internal/identify | Requests the identification of internal components in the portfolio
[**update_component**](ComponentApi.md#update_component) | **POST** /v1/component | Updates a component



## create_component

> models::Component create_component(uuid, body)
Creates a new component

<p>Requires permission <strong>PORTFOLIO_MANAGEMENT</strong></p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **uuid::Uuid** | The UUID of the project to create a component for | [required] |
**body** | Option<[**Component**](Component.md)> |  |  |

### Return type

[**models::Component**](Component.md)

### Authorization

[X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_component

> delete_component(uuid)
Deletes a component

<p>Requires permission <strong>PORTFOLIO_MANAGEMENT</strong></p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **uuid::Uuid** | The UUID of the component to delete | [required] |

### Return type

 (empty response body)

### Authorization

[X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_all_components

> Vec<models::Component> get_all_components(uuid, only_outdated, only_direct, page_number, page_size, offset, limit, sort_name, sort_order)
Returns a list of all components for a given project

<p>Requires permission <strong>VIEW_PORTFOLIO</strong></p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **uuid::Uuid** | The UUID of the project to retrieve components for | [required] |
**only_outdated** | Option<**bool**> | Optionally exclude recent components so only outdated components are returned |  |
**only_direct** | Option<**bool**> | Optionally exclude transitive dependencies so only direct dependencies are returned |  |
**page_number** | Option<[**serde_json::Value**](.md)> | The page to return. To be used in conjunction with <code>pageSize</code>. |  |[default to 1]
**page_size** | Option<[**serde_json::Value**](.md)> | Number of elements to return per page. To be used in conjunction with <code>pageNumber</code>. |  |[default to 100]
**offset** | Option<[**serde_json::Value**](.md)> | Offset to start returning elements from. To be used in conjunction with <code>limit</code>. |  |
**limit** | Option<[**serde_json::Value**](.md)> | Number of elements to return per page. To be used in conjunction with <code>offset</code>. |  |
**sort_name** | Option<**String**> | Name of the resource field to sort on. |  |
**sort_order** | Option<**String**> | Ordering of items when sorting with <code>sortName</code>. |  |

### Return type

[**Vec<models::Component>**](Component.md)

### Authorization

[X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_component_by_hash

> Vec<models::Component> get_component_by_hash(hash, page_number, page_size, offset, limit, sort_name, sort_order)
Returns a list of components that have the specified hash value

<p>Requires permission <strong>VIEW_PORTFOLIO</strong></p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**hash** | **String** | The MD5, SHA-1, SHA-256, SHA-384, SHA-512, SHA3-256, SHA3-384, SHA3-512, BLAKE2b-256, BLAKE2b-384, BLAKE2b-512, or BLAKE3 hash of the component to retrieve | [required] |
**page_number** | Option<[**serde_json::Value**](.md)> | The page to return. To be used in conjunction with <code>pageSize</code>. |  |[default to 1]
**page_size** | Option<[**serde_json::Value**](.md)> | Number of elements to return per page. To be used in conjunction with <code>pageNumber</code>. |  |[default to 100]
**offset** | Option<[**serde_json::Value**](.md)> | Offset to start returning elements from. To be used in conjunction with <code>limit</code>. |  |
**limit** | Option<[**serde_json::Value**](.md)> | Number of elements to return per page. To be used in conjunction with <code>offset</code>. |  |
**sort_name** | Option<**String**> | Name of the resource field to sort on. |  |
**sort_order** | Option<**String**> | Ordering of items when sorting with <code>sortName</code>. |  |

### Return type

[**Vec<models::Component>**](Component.md)

### Authorization

[X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_component_by_identity

> Vec<models::Component> get_component_by_identity(group, name, version, purl, cpe, swid_tag_id, project, page_number, page_size, offset, limit, sort_name, sort_order)
Returns a list of components that have the specified component identity. This resource accepts coordinates (group, name, version) or purl, cpe, or swidTagId

<p>Requires permission <strong>VIEW_PORTFOLIO</strong></p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group** | Option<**String**> | The group of the component |  |
**name** | Option<**String**> | The name of the component |  |
**version** | Option<**String**> | The version of the component |  |
**purl** | Option<**String**> | The purl of the component |  |
**cpe** | Option<**String**> | The cpe of the component |  |
**swid_tag_id** | Option<**String**> | The swidTagId of the component |  |
**project** | Option<**uuid::Uuid**> | The project the component belongs to |  |
**page_number** | Option<[**serde_json::Value**](.md)> | The page to return. To be used in conjunction with <code>pageSize</code>. |  |[default to 1]
**page_size** | Option<[**serde_json::Value**](.md)> | Number of elements to return per page. To be used in conjunction with <code>pageNumber</code>. |  |[default to 100]
**offset** | Option<[**serde_json::Value**](.md)> | Offset to start returning elements from. To be used in conjunction with <code>limit</code>. |  |
**limit** | Option<[**serde_json::Value**](.md)> | Number of elements to return per page. To be used in conjunction with <code>offset</code>. |  |
**sort_name** | Option<**String**> | Name of the resource field to sort on. |  |
**sort_order** | Option<**String**> | Ordering of items when sorting with <code>sortName</code>. |  |

### Return type

[**Vec<models::Component>**](Component.md)

### Authorization

[X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_component_by_uuid

> models::Component get_component_by_uuid(uuid, include_repository_meta_data)
Returns a specific component

<p>Requires permission <strong>VIEW_PORTFOLIO</strong></p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **uuid::Uuid** | The UUID of the component to retrieve | [required] |
**include_repository_meta_data** | Option<**bool**> | Optionally includes third-party metadata about the component from external repositories |  |

### Return type

[**models::Component**](Component.md)

### Authorization

[X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_dependency_graph_for_component

> std::collections::HashMap<String, models::Component> get_dependency_graph_for_component(project_uuid, component_uuids)
Returns the expanded dependency graph to every occurrence of a component

<p>Requires permission <strong>VIEW_PORTFOLIO</strong></p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_uuid** | **uuid::Uuid** | The UUID of the project to get the expanded dependency graph for | [required] |
**component_uuids** | **String** | List of UUIDs of the components (separated by |) to get the expanded dependency graph for | [required] |

### Return type

[**std::collections::HashMap<String, models::Component>**](Component.md)

### Authorization

[X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## identify_internal_components

> identify_internal_components()
Requests the identification of internal components in the portfolio

<p>Requires permission <strong>SYSTEM_CONFIGURATION</strong></p>

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_component

> models::Component update_component(body)
Updates a component

<p>Requires permission <strong>PORTFOLIO_MANAGEMENT</strong></p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**Component**](Component.md)> |  |  |

### Return type

[**models::Component**](Component.md)

### Authorization

[X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

