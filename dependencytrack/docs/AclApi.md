# \AclApi

All URIs are relative to */api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_mapping**](AclApi.md#add_mapping) | **PUT** /v1/acl/mapping | Adds an ACL mapping
[**delete_mapping**](AclApi.md#delete_mapping) | **DELETE** /v1/acl/mapping/team/{teamUuid}/project/{projectUuid} | Removes an ACL mapping
[**retrieve_projects**](AclApi.md#retrieve_projects) | **GET** /v1/acl/team/{uuid} | Returns the projects assigned to the specified team



## add_mapping

> models::AclMappingRequest add_mapping(body)
Adds an ACL mapping

<p>Requires permission <strong>ACCESS_MANAGEMENT</strong></p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**AclMappingRequest**](AclMappingRequest.md)> |  |  |

### Return type

[**models::AclMappingRequest**](AclMappingRequest.md)

### Authorization

[X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_mapping

> delete_mapping(team_uuid, project_uuid)
Removes an ACL mapping

<p>Requires permission <strong>ACCESS_MANAGEMENT</strong></p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_uuid** | **uuid::Uuid** | The UUID of the team to delete the mapping for | [required] |
**project_uuid** | **uuid::Uuid** | The UUID of the project to delete the mapping for | [required] |

### Return type

 (empty response body)

### Authorization

[X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_projects

> Vec<String> retrieve_projects(uuid, exclude_inactive, only_root, page_number, page_size, offset, limit, sort_name, sort_order)
Returns the projects assigned to the specified team

<p>Requires permission <strong>ACCESS_MANAGEMENT</strong></p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **uuid::Uuid** | The UUID of the team to retrieve mappings for | [required] |
**exclude_inactive** | Option<**bool**> | Optionally excludes inactive projects from being returned |  |
**only_root** | Option<**bool**> | Optionally excludes children projects from being returned |  |
**page_number** | Option<[**serde_json::Value**](.md)> | The page to return. To be used in conjunction with <code>pageSize</code>. |  |[default to 1]
**page_size** | Option<[**serde_json::Value**](.md)> | Number of elements to return per page. To be used in conjunction with <code>pageNumber</code>. |  |[default to 100]
**offset** | Option<[**serde_json::Value**](.md)> | Offset to start returning elements from. To be used in conjunction with <code>limit</code>. |  |
**limit** | Option<[**serde_json::Value**](.md)> | Number of elements to return per page. To be used in conjunction with <code>offset</code>. |  |
**sort_name** | Option<**String**> | Name of the resource field to sort on. |  |
**sort_order** | Option<**String**> | Ordering of items when sorting with <code>sortName</code>. |  |

### Return type

**Vec<String>**

### Authorization

[X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

