# \ConfigPropertyApi

All URIs are relative to */api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_config_properties**](ConfigPropertyApi.md#get_config_properties) | **GET** /v1/configProperty | Returns a list of all ConfigProperties for the specified groupName
[**update_config_property**](ConfigPropertyApi.md#update_config_property) | **POST** /v1/configProperty/aggregate | Updates an array of config properties
[**update_config_property1**](ConfigPropertyApi.md#update_config_property1) | **POST** /v1/configProperty | Updates a config property



## get_config_properties

> Vec<models::ConfigProperty> get_config_properties()
Returns a list of all ConfigProperties for the specified groupName

<p>Requires permission <strong>SYSTEM_CONFIGURATION</strong></p>

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::ConfigProperty>**](ConfigProperty.md)

### Authorization

[X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_config_property

> Vec<models::ConfigProperty> update_config_property(body)
Updates an array of config properties

<p>Requires permission <strong>SYSTEM_CONFIGURATION</strong></p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**Vec<models::ConfigProperty>**](ConfigProperty.md)> |  |  |

### Return type

[**Vec<models::ConfigProperty>**](ConfigProperty.md)

### Authorization

[X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_config_property1

> models::ConfigProperty update_config_property1(body)
Updates a config property

<p>Requires permission <strong>SYSTEM_CONFIGURATION</strong></p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**ConfigProperty**](ConfigProperty.md)> |  |  |

### Return type

[**models::ConfigProperty**](ConfigProperty.md)

### Authorization

[X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

