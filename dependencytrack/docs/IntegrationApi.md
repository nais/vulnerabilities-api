# \IntegrationApi

All URIs are relative to */api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_all_ecosystems**](IntegrationApi.md#get_all_ecosystems) | **GET** /v1/integration/osv/ecosystem | Returns a list of all ecosystems in OSV
[**get_inactive_ecosystems**](IntegrationApi.md#get_inactive_ecosystems) | **GET** /v1/integration/osv/ecosystem/inactive | Returns a list of available inactive ecosystems in OSV to be selected by user



## get_all_ecosystems

> Vec<String> get_all_ecosystems()
Returns a list of all ecosystems in OSV

<p>Requires permission <strong>SYSTEM_CONFIGURATION</strong></p>

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


## get_inactive_ecosystems

> Vec<String> get_inactive_ecosystems()
Returns a list of available inactive ecosystems in OSV to be selected by user

<p>Requires permission <strong>SYSTEM_CONFIGURATION</strong></p>

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

