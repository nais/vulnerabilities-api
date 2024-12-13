# \LicenseApi

All URIs are relative to */api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_license**](LicenseApi.md#create_license) | **PUT** /v1/license | Creates a new custom license
[**delete_license**](LicenseApi.md#delete_license) | **DELETE** /v1/license/{licenseId} | Deletes a custom license
[**get_license**](LicenseApi.md#get_license) | **GET** /v1/license/{licenseId} | Returns a specific license
[**get_license_listing**](LicenseApi.md#get_license_listing) | **GET** /v1/license/concise | Returns a concise listing of all licenses
[**get_licenses**](LicenseApi.md#get_licenses) | **GET** /v1/license | Returns a list of all licenses with complete metadata for each license



## create_license

> models::License create_license(body)
Creates a new custom license

<p>Requires permission <strong>SYSTEM_CONFIGURATION</strong></p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**License**](License.md)> |  |  |

### Return type

[**models::License**](License.md)

### Authorization

[X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_license

> delete_license(license_id)
Deletes a custom license

<p>Requires permission <strong>SYSTEM_CONFIGURATION</strong></p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**license_id** | **String** | The SPDX License ID of the license to delete | [required] |

### Return type

 (empty response body)

### Authorization

[X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_license

> models::License get_license(license_id)
Returns a specific license

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**license_id** | **String** | The SPDX License ID of the license to retrieve | [required] |

### Return type

[**models::License**](License.md)

### Authorization

[X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_license_listing

> Vec<models::License> get_license_listing()
Returns a concise listing of all licenses

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::License>**](License.md)

### Authorization

[X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_licenses

> Vec<models::License> get_licenses(page_number, page_size, offset, limit, sort_name, sort_order)
Returns a list of all licenses with complete metadata for each license

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

[**Vec<models::License>**](License.md)

### Authorization

[X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

