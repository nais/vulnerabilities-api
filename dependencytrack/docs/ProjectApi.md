# \ProjectApi

All URIs are relative to */api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**clone_project**](ProjectApi.md#clone_project) | **PUT** /v1/project/clone | Clones a project
[**create_project**](ProjectApi.md#create_project) | **PUT** /v1/project | Creates a new project
[**delete_project**](ProjectApi.md#delete_project) | **DELETE** /v1/project/{uuid} | Deletes a project
[**get_children_projects**](ProjectApi.md#get_children_projects) | **GET** /v1/project/{uuid}/children | Returns a list of all children for a project
[**get_children_projects_by_classifier**](ProjectApi.md#get_children_projects_by_classifier) | **GET** /v1/project/{uuid}/children/classifier/{classifier} | Returns a list of all children for a project by classifier
[**get_children_projects_by_tag**](ProjectApi.md#get_children_projects_by_tag) | **GET** /v1/project/{uuid}/children/tag/{tag} | Returns a list of all children for a project by tag
[**get_project**](ProjectApi.md#get_project) | **GET** /v1/project/{uuid} | Returns a specific project
[**get_project_by_name_and_version**](ProjectApi.md#get_project_by_name_and_version) | **GET** /v1/project/lookup | Returns a specific project by its name and version
[**get_projects**](ProjectApi.md#get_projects) | **GET** /v1/project | Returns a list of all projects
[**get_projects_by_classifier**](ProjectApi.md#get_projects_by_classifier) | **GET** /v1/project/classifier/{classifier} | Returns a list of all projects by classifier
[**get_projects_by_tag**](ProjectApi.md#get_projects_by_tag) | **GET** /v1/project/tag/{tag} | Returns a list of all projects by tag
[**get_projects_without_descendants_of**](ProjectApi.md#get_projects_without_descendants_of) | **GET** /v1/project/withoutDescendantsOf/{uuid} | Returns a list of all projects without the descendants of the selected project
[**patch_project**](ProjectApi.md#patch_project) | **PATCH** /v1/project/{uuid} | Partially updates a project
[**update_project**](ProjectApi.md#update_project) | **POST** /v1/project | Updates a project



## clone_project

> models::Project clone_project(body)
Clones a project

<p>Requires permission <strong>PORTFOLIO_MANAGEMENT</strong></p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**CloneProjectRequest**](CloneProjectRequest.md)> |  |  |

### Return type

[**models::Project**](Project.md)

### Authorization

[X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_project

> models::Project create_project(body)
Creates a new project

<p>If a parent project exists, <code>parent.uuid</code> is required</p> <p>Requires permission <strong>PORTFOLIO_MANAGEMENT</strong></p> 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**Project**](Project.md)> |  |  |

### Return type

[**models::Project**](Project.md)

### Authorization

[X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_project

> delete_project(uuid)
Deletes a project

<p>Requires permission <strong>PORTFOLIO_MANAGEMENT</strong></p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **uuid::Uuid** | The UUID of the project to delete | [required] |

### Return type

 (empty response body)

### Authorization

[X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_children_projects

> Vec<models::Project> get_children_projects(uuid, exclude_inactive, page_number, page_size, offset, limit, sort_name, sort_order)
Returns a list of all children for a project

<p>Requires permission <strong>VIEW_PORTFOLIO</strong></p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **uuid::Uuid** | The UUID of the project to get the children from | [required] |
**exclude_inactive** | Option<**bool**> | Optionally excludes inactive projects from being returned |  |
**page_number** | Option<[**serde_json::Value**](.md)> | The page to return. To be used in conjunction with <code>pageSize</code>. |  |[default to 1]
**page_size** | Option<[**serde_json::Value**](.md)> | Number of elements to return per page. To be used in conjunction with <code>pageNumber</code>. |  |[default to 100]
**offset** | Option<[**serde_json::Value**](.md)> | Offset to start returning elements from. To be used in conjunction with <code>limit</code>. |  |
**limit** | Option<[**serde_json::Value**](.md)> | Number of elements to return per page. To be used in conjunction with <code>offset</code>. |  |
**sort_name** | Option<**String**> | Name of the resource field to sort on. |  |
**sort_order** | Option<**String**> | Ordering of items when sorting with <code>sortName</code>. |  |

### Return type

[**Vec<models::Project>**](Project.md)

### Authorization

[X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_children_projects_by_classifier

> Vec<models::Project> get_children_projects_by_classifier(classifier, uuid, exclude_inactive, page_number, page_size, offset, limit, sort_name, sort_order)
Returns a list of all children for a project by classifier

<p>Requires permission <strong>VIEW_PORTFOLIO</strong></p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**classifier** | **String** | The classifier to query on | [required] |
**uuid** | **uuid::Uuid** | The UUID of the project to get the children from | [required] |
**exclude_inactive** | Option<**bool**> | Optionally excludes inactive projects from being returned |  |
**page_number** | Option<[**serde_json::Value**](.md)> | The page to return. To be used in conjunction with <code>pageSize</code>. |  |[default to 1]
**page_size** | Option<[**serde_json::Value**](.md)> | Number of elements to return per page. To be used in conjunction with <code>pageNumber</code>. |  |[default to 100]
**offset** | Option<[**serde_json::Value**](.md)> | Offset to start returning elements from. To be used in conjunction with <code>limit</code>. |  |
**limit** | Option<[**serde_json::Value**](.md)> | Number of elements to return per page. To be used in conjunction with <code>offset</code>. |  |
**sort_name** | Option<**String**> | Name of the resource field to sort on. |  |
**sort_order** | Option<**String**> | Ordering of items when sorting with <code>sortName</code>. |  |

### Return type

[**Vec<models::Project>**](Project.md)

### Authorization

[X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_children_projects_by_tag

> Vec<models::Project> get_children_projects_by_tag(tag, uuid, exclude_inactive, page_number, page_size, offset, limit, sort_name, sort_order)
Returns a list of all children for a project by tag

<p>Requires permission <strong>VIEW_PORTFOLIO</strong></p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tag** | **String** | The tag to query on | [required] |
**uuid** | **uuid::Uuid** | The UUID of the project to get the children from | [required] |
**exclude_inactive** | Option<**bool**> | Optionally excludes inactive projects from being returned |  |
**page_number** | Option<[**serde_json::Value**](.md)> | The page to return. To be used in conjunction with <code>pageSize</code>. |  |[default to 1]
**page_size** | Option<[**serde_json::Value**](.md)> | Number of elements to return per page. To be used in conjunction with <code>pageNumber</code>. |  |[default to 100]
**offset** | Option<[**serde_json::Value**](.md)> | Offset to start returning elements from. To be used in conjunction with <code>limit</code>. |  |
**limit** | Option<[**serde_json::Value**](.md)> | Number of elements to return per page. To be used in conjunction with <code>offset</code>. |  |
**sort_name** | Option<**String**> | Name of the resource field to sort on. |  |
**sort_order** | Option<**String**> | Ordering of items when sorting with <code>sortName</code>. |  |

### Return type

[**Vec<models::Project>**](Project.md)

### Authorization

[X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_project

> models::Project get_project(uuid)
Returns a specific project

<p>Requires permission <strong>VIEW_PORTFOLIO</strong></p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **uuid::Uuid** | The UUID of the project to retrieve | [required] |

### Return type

[**models::Project**](Project.md)

### Authorization

[X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_project_by_name_and_version

> models::Project get_project_by_name_and_version(name, version)
Returns a specific project by its name and version

<p>Requires permission <strong>VIEW_PORTFOLIO</strong></p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | The name of the project to query on | [required] |
**version** | **String** | The version of the project to query on | [required] |

### Return type

[**models::Project**](Project.md)

### Authorization

[X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_projects

> Vec<models::Project> get_projects(name, exclude_inactive, only_root, not_assigned_to_team_with_uuid, page_number, page_size, offset, limit, sort_name, sort_order)
Returns a list of all projects

<p>Requires permission <strong>VIEW_PORTFOLIO</strong></p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | Option<**String**> | The optional name of the project to query on |  |
**exclude_inactive** | Option<**bool**> | Optionally excludes inactive projects from being returned |  |
**only_root** | Option<**bool**> | Optionally excludes children projects from being returned |  |
**not_assigned_to_team_with_uuid** | Option<**uuid::Uuid**> | The UUID of the team which projects shall be excluded |  |
**page_number** | Option<[**serde_json::Value**](.md)> | The page to return. To be used in conjunction with <code>pageSize</code>. |  |[default to 1]
**page_size** | Option<[**serde_json::Value**](.md)> | Number of elements to return per page. To be used in conjunction with <code>pageNumber</code>. |  |[default to 100]
**offset** | Option<[**serde_json::Value**](.md)> | Offset to start returning elements from. To be used in conjunction with <code>limit</code>. |  |
**limit** | Option<[**serde_json::Value**](.md)> | Number of elements to return per page. To be used in conjunction with <code>offset</code>. |  |
**sort_name** | Option<**String**> | Name of the resource field to sort on. |  |
**sort_order** | Option<**String**> | Ordering of items when sorting with <code>sortName</code>. |  |

### Return type

[**Vec<models::Project>**](Project.md)

### Authorization

[X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_projects_by_classifier

> Vec<models::Project> get_projects_by_classifier(classifier, exclude_inactive, only_root, page_number, page_size, offset, limit, sort_name, sort_order)
Returns a list of all projects by classifier

<p>Requires permission <strong>VIEW_PORTFOLIO</strong></p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**classifier** | **String** | The classifier to query on | [required] |
**exclude_inactive** | Option<**bool**> | Optionally excludes inactive projects from being returned |  |
**only_root** | Option<**bool**> | Optionally excludes children projects from being returned |  |
**page_number** | Option<[**serde_json::Value**](.md)> | The page to return. To be used in conjunction with <code>pageSize</code>. |  |[default to 1]
**page_size** | Option<[**serde_json::Value**](.md)> | Number of elements to return per page. To be used in conjunction with <code>pageNumber</code>. |  |[default to 100]
**offset** | Option<[**serde_json::Value**](.md)> | Offset to start returning elements from. To be used in conjunction with <code>limit</code>. |  |
**limit** | Option<[**serde_json::Value**](.md)> | Number of elements to return per page. To be used in conjunction with <code>offset</code>. |  |
**sort_name** | Option<**String**> | Name of the resource field to sort on. |  |
**sort_order** | Option<**String**> | Ordering of items when sorting with <code>sortName</code>. |  |

### Return type

[**Vec<models::Project>**](Project.md)

### Authorization

[X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_projects_by_tag

> Vec<models::Project> get_projects_by_tag(tag, exclude_inactive, only_root, page_number, page_size, offset, limit, sort_name, sort_order)
Returns a list of all projects by tag

<p>Requires permission <strong>VIEW_PORTFOLIO</strong></p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tag** | **String** | The tag to query on | [required] |
**exclude_inactive** | Option<**bool**> | Optionally excludes inactive projects from being returned |  |
**only_root** | Option<**bool**> | Optionally excludes children projects from being returned |  |
**page_number** | Option<[**serde_json::Value**](.md)> | The page to return. To be used in conjunction with <code>pageSize</code>. |  |[default to 1]
**page_size** | Option<[**serde_json::Value**](.md)> | Number of elements to return per page. To be used in conjunction with <code>pageNumber</code>. |  |[default to 100]
**offset** | Option<[**serde_json::Value**](.md)> | Offset to start returning elements from. To be used in conjunction with <code>limit</code>. |  |
**limit** | Option<[**serde_json::Value**](.md)> | Number of elements to return per page. To be used in conjunction with <code>offset</code>. |  |
**sort_name** | Option<**String**> | Name of the resource field to sort on. |  |
**sort_order** | Option<**String**> | Ordering of items when sorting with <code>sortName</code>. |  |

### Return type

[**Vec<models::Project>**](Project.md)

### Authorization

[X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_projects_without_descendants_of

> Vec<models::Project> get_projects_without_descendants_of(uuid, name, exclude_inactive, page_number, page_size, offset, limit, sort_name, sort_order)
Returns a list of all projects without the descendants of the selected project

<p>Requires permission <strong>VIEW_PORTFOLIO</strong></p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **uuid::Uuid** | The UUID of the project which descendants will be excluded | [required] |
**name** | Option<**String**> | The optional name of the project to query on |  |
**exclude_inactive** | Option<**bool**> | Optionally excludes inactive projects from being returned |  |
**page_number** | Option<[**serde_json::Value**](.md)> | The page to return. To be used in conjunction with <code>pageSize</code>. |  |[default to 1]
**page_size** | Option<[**serde_json::Value**](.md)> | Number of elements to return per page. To be used in conjunction with <code>pageNumber</code>. |  |[default to 100]
**offset** | Option<[**serde_json::Value**](.md)> | Offset to start returning elements from. To be used in conjunction with <code>limit</code>. |  |
**limit** | Option<[**serde_json::Value**](.md)> | Number of elements to return per page. To be used in conjunction with <code>offset</code>. |  |
**sort_name** | Option<**String**> | Name of the resource field to sort on. |  |
**sort_order** | Option<**String**> | Ordering of items when sorting with <code>sortName</code>. |  |

### Return type

[**Vec<models::Project>**](Project.md)

### Authorization

[X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_project

> models::Project patch_project(uuid, body)
Partially updates a project

<p>Requires permission <strong>PORTFOLIO_MANAGEMENT</strong></p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **uuid::Uuid** | The UUID of the project to modify | [required] |
**body** | Option<[**Project**](Project.md)> |  |  |

### Return type

[**models::Project**](Project.md)

### Authorization

[X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_project

> models::Project update_project(body)
Updates a project

<p>Requires permission <strong>PORTFOLIO_MANAGEMENT</strong></p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**Project**](Project.md)> |  |  |

### Return type

[**models::Project**](Project.md)

### Authorization

[X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

