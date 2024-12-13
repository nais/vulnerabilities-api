# \RepositoryApi

All URIs are relative to */api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_repository**](RepositoryApi.md#create_repository) | **PUT** /v1/repository | Creates a new repository
[**delete_repository**](RepositoryApi.md#delete_repository) | **DELETE** /v1/repository/{uuid} | Deletes a repository
[**get_repositories**](RepositoryApi.md#get_repositories) | **GET** /v1/repository | Returns a list of all repositories
[**get_repositories_by_type**](RepositoryApi.md#get_repositories_by_type) | **GET** /v1/repository/{type} | Returns repositories that support the specific type
[**get_repository_meta_component**](RepositoryApi.md#get_repository_meta_component) | **GET** /v1/repository/latest | Attempts to resolve the latest version of the component available in the configured repositories
[**update_repository**](RepositoryApi.md#update_repository) | **POST** /v1/repository | Updates a repository



## create_repository

> models::Repository create_repository(body)
Creates a new repository

<p>Requires permission <strong>SYSTEM_CONFIGURATION</strong></p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**Repository**](Repository.md)> |  |  |

### Return type

[**models::Repository**](Repository.md)

### Authorization

[X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_repository

> delete_repository(uuid)
Deletes a repository

<p>Requires permission <strong>SYSTEM_CONFIGURATION</strong></p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **uuid::Uuid** | The UUID of the repository to delete | [required] |

### Return type

 (empty response body)

### Authorization

[X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_repositories

> Vec<models::Repository> get_repositories(page_number, page_size, offset, limit, sort_name, sort_order)
Returns a list of all repositories

<p>Requires permission <strong>SYSTEM_CONFIGURATION</strong></p>

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

[**Vec<models::Repository>**](Repository.md)

### Authorization

[X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_repositories_by_type

> Vec<models::Repository> get_repositories_by_type(r#type, page_number, page_size, offset, limit, sort_name, sort_order)
Returns repositories that support the specific type

<p>Requires permission <strong>SYSTEM_CONFIGURATION</strong></p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**r#type** | **String** | The type of repositories to retrieve | [required] |
**page_number** | Option<[**serde_json::Value**](.md)> | The page to return. To be used in conjunction with <code>pageSize</code>. |  |[default to 1]
**page_size** | Option<[**serde_json::Value**](.md)> | Number of elements to return per page. To be used in conjunction with <code>pageNumber</code>. |  |[default to 100]
**offset** | Option<[**serde_json::Value**](.md)> | Offset to start returning elements from. To be used in conjunction with <code>limit</code>. |  |
**limit** | Option<[**serde_json::Value**](.md)> | Number of elements to return per page. To be used in conjunction with <code>offset</code>. |  |
**sort_name** | Option<**String**> | Name of the resource field to sort on. |  |
**sort_order** | Option<**String**> | Ordering of items when sorting with <code>sortName</code>. |  |

### Return type

[**Vec<models::Repository>**](Repository.md)

### Authorization

[X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_repository_meta_component

> models::RepositoryMetaComponent get_repository_meta_component(purl)
Attempts to resolve the latest version of the component available in the configured repositories

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**purl** | **String** | The Package URL for the component to query | [required] |

### Return type

[**models::RepositoryMetaComponent**](RepositoryMetaComponent.md)

### Authorization

[X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_repository

> models::Repository update_repository(body)
Updates a repository

<p>Requires permission <strong>SYSTEM_CONFIGURATION</strong></p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**Repository**](Repository.md)> |  |  |

### Return type

[**models::Repository**](Repository.md)

### Authorization

[X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

