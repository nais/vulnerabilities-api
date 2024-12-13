# \TeamApi

All URIs are relative to */api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_team**](TeamApi.md#create_team) | **PUT** /v1/team | Creates a new team along with an associated API key
[**delete_api_key**](TeamApi.md#delete_api_key) | **DELETE** /v1/team/key/{apikey} | Deletes the specified API key
[**delete_team**](TeamApi.md#delete_team) | **DELETE** /v1/team | Deletes a team
[**generate_api_key**](TeamApi.md#generate_api_key) | **PUT** /v1/team/{uuid}/key | Generates an API key and returns its value
[**get_self**](TeamApi.md#get_self) | **GET** /v1/team/self | Returns information about the current team.
[**get_team**](TeamApi.md#get_team) | **GET** /v1/team/{uuid} | Returns a specific team
[**get_teams**](TeamApi.md#get_teams) | **GET** /v1/team | Returns a list of all teams
[**regenerate_api_key**](TeamApi.md#regenerate_api_key) | **POST** /v1/team/key/{apikey} | Regenerates an API key by removing the specified key, generating a new one and returning its value
[**update_api_key_comment**](TeamApi.md#update_api_key_comment) | **POST** /v1/team/key/{key}/comment | Updates an API key's comment
[**update_team**](TeamApi.md#update_team) | **POST** /v1/team | Updates a team's fields including



## create_team

> models::Team create_team(body)
Creates a new team along with an associated API key

<p>Requires permission <strong>ACCESS_MANAGEMENT</strong></p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**Team**](Team.md)> |  |  |

### Return type

[**models::Team**](Team.md)

### Authorization

[X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_api_key

> delete_api_key(apikey)
Deletes the specified API key

<p>Requires permission <strong>ACCESS_MANAGEMENT</strong></p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**apikey** | **String** | The API key to delete | [required] |

### Return type

 (empty response body)

### Authorization

[X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_team

> delete_team(body)
Deletes a team

<p>Requires permission <strong>ACCESS_MANAGEMENT</strong></p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**Team**](Team.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## generate_api_key

> models::ApiKey generate_api_key(uuid)
Generates an API key and returns its value

<p>Requires permission <strong>ACCESS_MANAGEMENT</strong></p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **uuid::Uuid** | The UUID of the team to generate a key for | [required] |

### Return type

[**models::ApiKey**](ApiKey.md)

### Authorization

[X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_self

> models::TeamSelfResponse get_self()
Returns information about the current team.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::TeamSelfResponse**](TeamSelfResponse.md)

### Authorization

[X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_team

> models::Team get_team(uuid)
Returns a specific team

<p>Requires permission <strong>ACCESS_MANAGEMENT</strong></p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **uuid::Uuid** | The UUID of the team to retrieve | [required] |

### Return type

[**models::Team**](Team.md)

### Authorization

[X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_teams

> Vec<models::Team> get_teams()
Returns a list of all teams

<p>Requires permission <strong>ACCESS_MANAGEMENT</strong></p>

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::Team>**](Team.md)

### Authorization

[X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## regenerate_api_key

> models::ApiKey regenerate_api_key(apikey)
Regenerates an API key by removing the specified key, generating a new one and returning its value

<p>Requires permission <strong>ACCESS_MANAGEMENT</strong></p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**apikey** | **String** | The API key to regenerate | [required] |

### Return type

[**models::ApiKey**](ApiKey.md)

### Authorization

[X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_api_key_comment

> models::ApiKey update_api_key_comment(key, body)
Updates an API key's comment

<p>Requires permission <strong>ACCESS_MANAGEMENT</strong></p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key** | **String** |  | [required] |
**body** | Option<**String**> |  |  |

### Return type

[**models::ApiKey**](ApiKey.md)

### Authorization

[X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: text/plain
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_team

> models::Team update_team(body)
Updates a team's fields including

<p>Requires permission <strong>ACCESS_MANAGEMENT</strong></p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**Team**](Team.md)> |  |  |

### Return type

[**models::Team**](Team.md)

### Authorization

[X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

