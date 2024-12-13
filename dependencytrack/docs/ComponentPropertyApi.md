# \ComponentPropertyApi

All URIs are relative to */api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_property**](ComponentPropertyApi.md#create_property) | **PUT** /v1/component/{uuid}/property | Creates a new component property
[**delete_property**](ComponentPropertyApi.md#delete_property) | **DELETE** /v1/component/{uuid}/property/{propertyUuid} | Deletes a config property
[**get_properties**](ComponentPropertyApi.md#get_properties) | **GET** /v1/component/{uuid}/property | Returns a list of all ComponentProperties for the specified component



## create_property

> models::ComponentProperty create_property(uuid, body)
Creates a new component property

<p>Requires permission <strong>PORTFOLIO_MANAGEMENT</strong></p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **uuid::Uuid** | The UUID of the component to create a property for | [required] |
**body** | Option<[**ComponentProperty**](ComponentProperty.md)> |  |  |

### Return type

[**models::ComponentProperty**](ComponentProperty.md)

### Authorization

[X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_property

> models::ComponentProperty delete_property(uuid, property_uuid)
Deletes a config property

<p>Requires permission <strong>PORTFOLIO_MANAGEMENT</strong></p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **uuid::Uuid** | The UUID of the component to delete a property from | [required] |
**property_uuid** | **uuid::Uuid** | The UUID of the component property to delete | [required] |

### Return type

[**models::ComponentProperty**](ComponentProperty.md)

### Authorization

[X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_properties

> Vec<models::ComponentProperty> get_properties(uuid)
Returns a list of all ComponentProperties for the specified component

<p>Requires permission <strong>VIEW_PORTFOLIO</strong></p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **uuid::Uuid** | The UUID of the component to retrieve properties for | [required] |

### Return type

[**Vec<models::ComponentProperty>**](ComponentProperty.md)

### Authorization

[X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

