# \PolicyApi

All URIs are relative to */api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_project_to_policy**](PolicyApi.md#add_project_to_policy) | **POST** /v1/policy/{policyUuid}/project/{projectUuid} | Adds a project to a policy
[**add_tag_to_policy**](PolicyApi.md#add_tag_to_policy) | **POST** /v1/policy/{policyUuid}/tag/{tagName} | Adds a tag to a policy
[**create_policy**](PolicyApi.md#create_policy) | **PUT** /v1/policy | Creates a new policy
[**delete_policy**](PolicyApi.md#delete_policy) | **DELETE** /v1/policy/{uuid} | Deletes a policy
[**get_policies**](PolicyApi.md#get_policies) | **GET** /v1/policy | Returns a list of all policies
[**get_policy**](PolicyApi.md#get_policy) | **GET** /v1/policy/{uuid} | Returns a specific policy
[**remove_project_from_policy**](PolicyApi.md#remove_project_from_policy) | **DELETE** /v1/policy/{policyUuid}/project/{projectUuid} | Removes a project from a policy
[**remove_tag_from_policy**](PolicyApi.md#remove_tag_from_policy) | **DELETE** /v1/policy/{policyUuid}/tag/{tagName} | Removes a tag from a policy
[**update_policy**](PolicyApi.md#update_policy) | **POST** /v1/policy | Updates a policy



## add_project_to_policy

> models::Policy add_project_to_policy(policy_uuid, project_uuid)
Adds a project to a policy

<p>Requires permission <strong>POLICY_MANAGEMENT</strong></p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**policy_uuid** | **uuid::Uuid** | The UUID of the policy to add a project to | [required] |
**project_uuid** | **uuid::Uuid** | The UUID of the project to add to the rule | [required] |

### Return type

[**models::Policy**](Policy.md)

### Authorization

[X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## add_tag_to_policy

> models::Policy add_tag_to_policy(policy_uuid, tag_name)
Adds a tag to a policy

<p>Requires permission <strong>POLICY_MANAGEMENT</strong></p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**policy_uuid** | **uuid::Uuid** | The UUID of the policy to add a project to | [required] |
**tag_name** | **String** | The name of the tag to add to the rule | [required] |

### Return type

[**models::Policy**](Policy.md)

### Authorization

[X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_policy

> models::Policy create_policy(body)
Creates a new policy

<p>Requires permission <strong>POLICY_MANAGEMENT</strong></p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**Policy**](Policy.md)> |  |  |

### Return type

[**models::Policy**](Policy.md)

### Authorization

[X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_policy

> delete_policy(uuid)
Deletes a policy

<p>Requires permission <strong>POLICY_MANAGEMENT</strong></p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **uuid::Uuid** | The UUID of the policy to delete | [required] |

### Return type

 (empty response body)

### Authorization

[X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_policies

> Vec<models::Policy> get_policies(page_number, page_size, offset, limit, sort_name, sort_order)
Returns a list of all policies

<p>Requires permission <strong>POLICY_MANAGEMENT</strong></p>

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

[**Vec<models::Policy>**](Policy.md)

### Authorization

[X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_policy

> models::Policy get_policy(uuid)
Returns a specific policy

<p>Requires permission <strong>POLICY_MANAGEMENT</strong></p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **uuid::Uuid** | The UUID of the policy to retrieve | [required] |

### Return type

[**models::Policy**](Policy.md)

### Authorization

[X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_project_from_policy

> models::Policy remove_project_from_policy(policy_uuid, project_uuid)
Removes a project from a policy

<p>Requires permission <strong>POLICY_MANAGEMENT</strong></p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**policy_uuid** | **uuid::Uuid** | The UUID of the policy to remove the project from | [required] |
**project_uuid** | **uuid::Uuid** | The UUID of the project to remove from the policy | [required] |

### Return type

[**models::Policy**](Policy.md)

### Authorization

[X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_tag_from_policy

> models::Policy remove_tag_from_policy(policy_uuid, tag_name)
Removes a tag from a policy

<p>Requires permission <strong>POLICY_MANAGEMENT</strong></p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**policy_uuid** | **uuid::Uuid** | The UUID of the policy to remove the tag from | [required] |
**tag_name** | **String** | The name of the tag to remove from the policy | [required] |

### Return type

[**models::Policy**](Policy.md)

### Authorization

[X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_policy

> models::Policy update_policy(body)
Updates a policy

<p>Requires permission <strong>POLICY_MANAGEMENT</strong></p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**Policy**](Policy.md)> |  |  |

### Return type

[**models::Policy**](Policy.md)

### Authorization

[X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

