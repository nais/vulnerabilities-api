# \LdapApi

All URIs are relative to */api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_mapping1**](LdapApi.md#add_mapping1) | **PUT** /v1/ldap/mapping | Adds a mapping
[**delete_mapping1**](LdapApi.md#delete_mapping1) | **DELETE** /v1/ldap/mapping/{uuid} | Removes a mapping
[**retrieve_ldap_groups**](LdapApi.md#retrieve_ldap_groups) | **GET** /v1/ldap/team/{uuid} | Returns the DNs of all groups mapped to the specified team
[**retrieve_ldap_groups1**](LdapApi.md#retrieve_ldap_groups1) | **GET** /v1/ldap/groups | Returns the DNs of all accessible groups within the directory



## add_mapping1

> models::MappedLdapGroup add_mapping1(body)
Adds a mapping

<p>Requires permission <strong>ACCESS_MANAGEMENT</strong></p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**MappedLdapGroupRequest**](MappedLdapGroupRequest.md)> |  |  |

### Return type

[**models::MappedLdapGroup**](MappedLdapGroup.md)

### Authorization

[X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_mapping1

> models::MappedLdapGroup delete_mapping1(uuid)
Removes a mapping

<p>Requires permission <strong>ACCESS_MANAGEMENT</strong></p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **uuid::Uuid** | The UUID of the mapping to delete | [required] |

### Return type

[**models::MappedLdapGroup**](MappedLdapGroup.md)

### Authorization

[X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_ldap_groups

> Vec<String> retrieve_ldap_groups(uuid)
Returns the DNs of all groups mapped to the specified team

<p>Requires permission <strong>ACCESS_MANAGEMENT</strong></p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **uuid::Uuid** | The UUID of the team to retrieve mappings for | [required] |

### Return type

**Vec<String>**

### Authorization

[X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_ldap_groups1

> Vec<String> retrieve_ldap_groups1()
Returns the DNs of all accessible groups within the directory

<p>   This API performs a pass-through query to the configured LDAP server.   Search criteria results are cached using default Alpine CacheManager policy. <p> <p>Requires permission <strong>ACCESS_MANAGEMENT</strong></p>

### Parameters

This endpoint does not need any parameter.

### Return type

**Vec<String>**

### Authorization

[X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

