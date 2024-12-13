# \UserApi

All URIs are relative to */api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_team_to_user**](UserApi.md#add_team_to_user) | **POST** /v1/user/{username}/membership | Adds the username to the specified team.
[**create_ldap_user**](UserApi.md#create_ldap_user) | **PUT** /v1/user/ldap | Creates a new user that references an existing LDAP object.
[**create_managed_user**](UserApi.md#create_managed_user) | **PUT** /v1/user/managed | Creates a new user.
[**create_oidc_user**](UserApi.md#create_oidc_user) | **PUT** /v1/user/oidc | Creates a new user that references an existing OpenID Connect user.
[**delete_ldap_user**](UserApi.md#delete_ldap_user) | **DELETE** /v1/user/ldap | Deletes a user.
[**delete_managed_user**](UserApi.md#delete_managed_user) | **DELETE** /v1/user/managed | Deletes a user.
[**delete_oidc_user**](UserApi.md#delete_oidc_user) | **DELETE** /v1/user/oidc | Deletes an OpenID Connect user.
[**force_change_password**](UserApi.md#force_change_password) | **POST** /v1/user/forceChangePassword | Asserts login credentials and upon successful authentication, verifies passwords match and changes users password
[**get_ldap_users**](UserApi.md#get_ldap_users) | **GET** /v1/user/ldap | Returns a list of all LDAP users
[**get_managed_users**](UserApi.md#get_managed_users) | **GET** /v1/user/managed | Returns a list of all managed users
[**get_oidc_users**](UserApi.md#get_oidc_users) | **GET** /v1/user/oidc | Returns a list of all OIDC users
[**get_self1**](UserApi.md#get_self1) | **GET** /v1/user/self | Returns information about the current logged in user.
[**remove_team_from_user**](UserApi.md#remove_team_from_user) | **DELETE** /v1/user/{username}/membership | Removes the username from the specified team.
[**update_managed_user**](UserApi.md#update_managed_user) | **POST** /v1/user/managed | Updates a managed user.
[**update_self**](UserApi.md#update_self) | **POST** /v1/user/self | Updates information about the current logged in user.
[**validate_credentials**](UserApi.md#validate_credentials) | **POST** /v1/user/login | Assert login credentials
[**validate_oidc_access_token**](UserApi.md#validate_oidc_access_token) | **POST** /v1/user/oidc/login | Login with OpenID Connect



## add_team_to_user

> models::UserPrincipal add_team_to_user(username, body)
Adds the username to the specified team.

<p>Requires permission <strong>ACCESS_MANAGEMENT</strong></p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | **String** | A valid username | [required] |
**body** | [**IdentifiableObject**](IdentifiableObject.md) | The UUID of the team to associate username with | [required] |

### Return type

[**models::UserPrincipal**](UserPrincipal.md)

### Authorization

[X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_ldap_user

> models::LdapUser create_ldap_user(body)
Creates a new user that references an existing LDAP object.

<p>Requires permission <strong>ACCESS_MANAGEMENT</strong></p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**LdapUser**](LdapUser.md)> |  |  |

### Return type

[**models::LdapUser**](LdapUser.md)

### Authorization

[X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_managed_user

> models::ManagedUser create_managed_user(body)
Creates a new user.

<p>Requires permission <strong>ACCESS_MANAGEMENT</strong></p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**ManagedUser**](ManagedUser.md)> |  |  |

### Return type

[**models::ManagedUser**](ManagedUser.md)

### Authorization

[X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_oidc_user

> models::OidcUser create_oidc_user(body)
Creates a new user that references an existing OpenID Connect user.

<p>Requires permission <strong>ACCESS_MANAGEMENT</strong></p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**OidcUser**](OidcUser.md)> |  |  |

### Return type

[**models::OidcUser**](OidcUser.md)

### Authorization

[X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_ldap_user

> delete_ldap_user(body)
Deletes a user.

<p>Requires permission <strong>ACCESS_MANAGEMENT</strong></p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**LdapUser**](LdapUser.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_managed_user

> delete_managed_user(body)
Deletes a user.

<p>Requires permission <strong>ACCESS_MANAGEMENT</strong></p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**ManagedUser**](ManagedUser.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_oidc_user

> delete_oidc_user(body)
Deletes an OpenID Connect user.

<p>Requires permission <strong>ACCESS_MANAGEMENT</strong></p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**OidcUser**](OidcUser.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## force_change_password

> String force_change_password(username, password, new_password, confirm_password)
Asserts login credentials and upon successful authentication, verifies passwords match and changes users password

Upon a successful login, a JSON Web Token will be returned in the response body. This functionality requires authentication to be enabled.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | Option<**String**> |  |  |
**password** | Option<**String**> |  |  |
**new_password** | Option<**String**> |  |  |
**confirm_password** | Option<**String**> |  |  |

### Return type

**String**

### Authorization

[X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_ldap_users

> Vec<models::LdapUser> get_ldap_users()
Returns a list of all LDAP users

<p>Requires permission <strong>ACCESS_MANAGEMENT</strong></p>

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::LdapUser>**](LdapUser.md)

### Authorization

[X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_managed_users

> Vec<models::ManagedUser> get_managed_users()
Returns a list of all managed users

<p>Requires permission <strong>ACCESS_MANAGEMENT</strong></p>

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::ManagedUser>**](ManagedUser.md)

### Authorization

[X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_oidc_users

> Vec<models::OidcUser> get_oidc_users()
Returns a list of all OIDC users

<p>Requires permission <strong>ACCESS_MANAGEMENT</strong></p>

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::OidcUser>**](OidcUser.md)

### Authorization

[X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_self1

> models::UserPrincipal get_self1()
Returns information about the current logged in user.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::UserPrincipal**](UserPrincipal.md)

### Authorization

[X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_team_from_user

> models::UserPrincipal remove_team_from_user(username, body)
Removes the username from the specified team.

<p>Requires permission <strong>ACCESS_MANAGEMENT</strong></p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | **String** | A valid username | [required] |
**body** | [**IdentifiableObject**](IdentifiableObject.md) | The UUID of the team to un-associate username from | [required] |

### Return type

[**models::UserPrincipal**](UserPrincipal.md)

### Authorization

[X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_managed_user

> models::ManagedUser update_managed_user(body)
Updates a managed user.

<p>Requires permission <strong>ACCESS_MANAGEMENT</strong></p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**ManagedUser**](ManagedUser.md)> |  |  |

### Return type

[**models::ManagedUser**](ManagedUser.md)

### Authorization

[X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_self

> models::UserPrincipal update_self(body)
Updates information about the current logged in user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**ManagedUser**](ManagedUser.md)> |  |  |

### Return type

[**models::UserPrincipal**](UserPrincipal.md)

### Authorization

[X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## validate_credentials

> String validate_credentials(username, password)
Assert login credentials

Upon a successful login, a JSON Web Token will be returned in the response body. This functionality requires authentication to be enabled.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | Option<**String**> |  |  |
**password** | Option<**String**> |  |  |

### Return type

**String**

### Authorization

[X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## validate_oidc_access_token

> String validate_oidc_access_token(id_token, access_token)
Login with OpenID Connect

Upon a successful login, a JSON Web Token will be returned in the response body. This functionality requires authentication to be enabled.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id_token** | **String** | An OAuth2 access token | [required] |
**access_token** | Option<**String**> |  |  |

### Return type

**String**

### Authorization

[X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

