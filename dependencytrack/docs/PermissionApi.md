# \PermissionApi

All URIs are relative to */api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_permission_to_team**](PermissionApi.md#add_permission_to_team) | **POST** /v1/permission/{permission}/team/{uuid} | Adds the permission to the specified team.
[**add_permission_to_user**](PermissionApi.md#add_permission_to_user) | **POST** /v1/permission/{permission}/user/{username} | Adds the permission to the specified username.
[**get_all_permissions**](PermissionApi.md#get_all_permissions) | **GET** /v1/permission | Returns a list of all permissions
[**remove_permission_from_team**](PermissionApi.md#remove_permission_from_team) | **DELETE** /v1/permission/{permission}/team/{uuid} | Removes the permission from the team.
[**remove_permission_from_user**](PermissionApi.md#remove_permission_from_user) | **DELETE** /v1/permission/{permission}/user/{username} | Removes the permission from the user.



## add_permission_to_team

> models::Team add_permission_to_team(uuid, permission)
Adds the permission to the specified team.

<p>Requires permission <strong>ACCESS_MANAGEMENT</strong></p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **uuid::Uuid** | A valid team uuid | [required] |
**permission** | **String** | A valid permission | [required] |

### Return type

[**models::Team**](Team.md)

### Authorization

[X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## add_permission_to_user

> models::UserPrincipal add_permission_to_user(username, permission)
Adds the permission to the specified username.

<p>Requires permission <strong>ACCESS_MANAGEMENT</strong></p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | **String** | A valid username | [required] |
**permission** | **String** | A valid permission | [required] |

### Return type

[**models::UserPrincipal**](UserPrincipal.md)

### Authorization

[X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_all_permissions

> Vec<models::Permission> get_all_permissions()
Returns a list of all permissions

<p>Requires permission <strong>ACCESS_MANAGEMENT</strong></p>

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::Permission>**](Permission.md)

### Authorization

[X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_permission_from_team

> models::Team remove_permission_from_team(uuid, permission)
Removes the permission from the team.

<p>Requires permission <strong>ACCESS_MANAGEMENT</strong></p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **uuid::Uuid** | A valid team uuid | [required] |
**permission** | **String** | A valid permission | [required] |

### Return type

[**models::Team**](Team.md)

### Authorization

[X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_permission_from_user

> models::UserPrincipal remove_permission_from_user(username, permission)
Removes the permission from the user.

<p>Requires permission <strong>ACCESS_MANAGEMENT</strong></p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | **String** | A valid username | [required] |
**permission** | **String** | A valid permission | [required] |

### Return type

[**models::UserPrincipal**](UserPrincipal.md)

### Authorization

[X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

