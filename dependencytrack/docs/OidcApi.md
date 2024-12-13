# \OidcApi

All URIs are relative to */api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_mapping2**](OidcApi.md#add_mapping2) | **PUT** /v1/oidc/mapping | Adds a mapping
[**create_group**](OidcApi.md#create_group) | **PUT** /v1/oidc/group | Creates group
[**delete_group**](OidcApi.md#delete_group) | **DELETE** /v1/oidc/group/{uuid} | Deletes a group
[**delete_mapping2**](OidcApi.md#delete_mapping2) | **DELETE** /v1/oidc/group/{groupUuid}/team/{teamUuid}/mapping | Deletes a mapping
[**delete_mapping_by_uuid**](OidcApi.md#delete_mapping_by_uuid) | **DELETE** /v1/oidc/mapping/{uuid} | Deletes a mapping
[**is_available**](OidcApi.md#is_available) | **GET** /v1/oidc/available | Indicates if OpenID Connect is available for this application
[**retrieve_groups**](OidcApi.md#retrieve_groups) | **GET** /v1/oidc/group | Returns a list of all groups
[**retrieve_teams_mapped_to_group**](OidcApi.md#retrieve_teams_mapped_to_group) | **GET** /v1/oidc/group/{uuid}/team | Returns a list of teams associated with the specified group
[**update_group**](OidcApi.md#update_group) | **POST** /v1/oidc/group | Updates group



## add_mapping2

> models::MappedOidcGroup add_mapping2(body)
Adds a mapping

<p>Requires permission <strong>ACCESS_MANAGEMENT</strong></p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**MappedOidcGroupRequest**](MappedOidcGroupRequest.md)> |  |  |

### Return type

[**models::MappedOidcGroup**](MappedOidcGroup.md)

### Authorization

[X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_group

> models::OidcGroup create_group(body)
Creates group

<p>Requires permission <strong>ACCESS_MANAGEMENT</strong></p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**OidcGroup**](OidcGroup.md)> |  |  |

### Return type

[**models::OidcGroup**](OidcGroup.md)

### Authorization

[X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_group

> delete_group(uuid)
Deletes a group

<p>Requires permission <strong>ACCESS_MANAGEMENT</strong></p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **uuid::Uuid** | The UUID of the group to delete | [required] |

### Return type

 (empty response body)

### Authorization

[X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_mapping2

> delete_mapping2(group_uuid, team_uuid)
Deletes a mapping

<p>Requires permission <strong>ACCESS_MANAGEMENT</strong></p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_uuid** | **uuid::Uuid** | The UUID of the group to delete a mapping for | [required] |
**team_uuid** | **uuid::Uuid** | The UUID of the team to delete a mapping for | [required] |

### Return type

 (empty response body)

### Authorization

[X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_mapping_by_uuid

> delete_mapping_by_uuid(uuid)
Deletes a mapping

<p>Requires permission <strong>ACCESS_MANAGEMENT</strong></p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **uuid::Uuid** | The UUID of the mapping to delete | [required] |

### Return type

 (empty response body)

### Authorization

[X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## is_available

> bool is_available()
Indicates if OpenID Connect is available for this application

### Parameters

This endpoint does not need any parameter.

### Return type

**bool**

### Authorization

[X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_groups

> Vec<models::OidcGroup> retrieve_groups()
Returns a list of all groups

<p>Requires permission <strong>ACCESS_MANAGEMENT</strong></p>

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::OidcGroup>**](OidcGroup.md)

### Authorization

[X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_teams_mapped_to_group

> Vec<models::Team> retrieve_teams_mapped_to_group(uuid)
Returns a list of teams associated with the specified group

<p>Requires permission <strong>ACCESS_MANAGEMENT</strong></p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **uuid::Uuid** | The UUID of the mapping to retrieve the team for | [required] |

### Return type

[**Vec<models::Team>**](Team.md)

### Authorization

[X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_group

> models::OidcGroup update_group(body)
Updates group

<p>Requires permission <strong>ACCESS_MANAGEMENT</strong></p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**OidcGroup**](OidcGroup.md)> |  |  |

### Return type

[**models::OidcGroup**](OidcGroup.md)

### Authorization

[X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

