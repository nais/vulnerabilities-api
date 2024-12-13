# \ProjectPropertyApi

All URIs are relative to */api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_property1**](ProjectPropertyApi.md#create_property1) | **PUT** /v1/project/{uuid}/property | Creates a new project property
[**delete_property1**](ProjectPropertyApi.md#delete_property1) | **DELETE** /v1/project/{uuid}/property | Deletes a config property
[**get_properties1**](ProjectPropertyApi.md#get_properties1) | **GET** /v1/project/{uuid}/property | Returns a list of all ProjectProperties for the specified project
[**update_property**](ProjectPropertyApi.md#update_property) | **POST** /v1/project/{uuid}/property | Updates a project property



## create_property1

> models::ProjectProperty create_property1(uuid, body)
Creates a new project property

<p>Requires permission <strong>PORTFOLIO_MANAGEMENT</strong></p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **uuid::Uuid** | The UUID of the project to create a property for | [required] |
**body** | Option<[**ProjectProperty**](ProjectProperty.md)> |  |  |

### Return type

[**models::ProjectProperty**](ProjectProperty.md)

### Authorization

[X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_property1

> models::ProjectProperty delete_property1(uuid, body)
Deletes a config property

<p>Requires permission <strong>PORTFOLIO_MANAGEMENT</strong></p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **uuid::Uuid** | The UUID of the project to delete a property from | [required] |
**body** | Option<[**ProjectProperty**](ProjectProperty.md)> |  |  |

### Return type

[**models::ProjectProperty**](ProjectProperty.md)

### Authorization

[X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_properties1

> Vec<models::ProjectProperty> get_properties1(uuid)
Returns a list of all ProjectProperties for the specified project

<p>Requires permission <strong>PORTFOLIO_MANAGEMENT</strong></p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **uuid::Uuid** | The UUID of the project to retrieve properties for | [required] |

### Return type

[**Vec<models::ProjectProperty>**](ProjectProperty.md)

### Authorization

[X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_property

> models::ProjectProperty update_property(uuid, body)
Updates a project property

<p>Requires permission <strong>PORTFOLIO_MANAGEMENT</strong></p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **uuid::Uuid** | The UUID of the project to create a property for | [required] |
**body** | Option<[**ProjectProperty**](ProjectProperty.md)> |  |  |

### Return type

[**models::ProjectProperty**](ProjectProperty.md)

### Authorization

[X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

