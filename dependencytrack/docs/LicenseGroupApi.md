# \LicenseGroupApi

All URIs are relative to */api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_license_to_license_group**](LicenseGroupApi.md#add_license_to_license_group) | **POST** /v1/licenseGroup/{uuid}/license/{licenseUuid} | Adds the license to the specified license group.
[**create_license_group**](LicenseGroupApi.md#create_license_group) | **PUT** /v1/licenseGroup | Creates a new license group
[**delete_license_group**](LicenseGroupApi.md#delete_license_group) | **DELETE** /v1/licenseGroup/{uuid} | Deletes a license group
[**get_license_group**](LicenseGroupApi.md#get_license_group) | **GET** /v1/licenseGroup/{uuid} | Returns a specific license group
[**get_license_groups**](LicenseGroupApi.md#get_license_groups) | **GET** /v1/licenseGroup | Returns a list of all license groups
[**remove_license_from_license_group**](LicenseGroupApi.md#remove_license_from_license_group) | **DELETE** /v1/licenseGroup/{uuid}/license/{licenseUuid} | Removes the license from the license group.
[**update_license_group**](LicenseGroupApi.md#update_license_group) | **POST** /v1/licenseGroup | Updates a license group



## add_license_to_license_group

> models::LicenseGroup add_license_to_license_group(uuid, license_uuid)
Adds the license to the specified license group.

<p>Requires permission <strong>POLICY_MANAGEMENT</strong></p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **uuid::Uuid** | A valid license group | [required] |
**license_uuid** | **uuid::Uuid** | A valid license | [required] |

### Return type

[**models::LicenseGroup**](LicenseGroup.md)

### Authorization

[X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_license_group

> models::LicenseGroup create_license_group(body)
Creates a new license group

<p>Requires permission <strong>POLICY_MANAGEMENT</strong></p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**LicenseGroup**](LicenseGroup.md)> |  |  |

### Return type

[**models::LicenseGroup**](LicenseGroup.md)

### Authorization

[X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_license_group

> delete_license_group(uuid)
Deletes a license group

<p>Requires permission <strong>POLICY_MANAGEMENT</strong></p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **uuid::Uuid** | The UUID of the license group to delete | [required] |

### Return type

 (empty response body)

### Authorization

[X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_license_group

> models::License get_license_group(uuid)
Returns a specific license group

<p>Requires permission <strong>POLICY_MANAGEMENT</strong></p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **uuid::Uuid** | The UUID of the license group to retrieve | [required] |

### Return type

[**models::License**](License.md)

### Authorization

[X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_license_groups

> Vec<models::LicenseGroup> get_license_groups(page_number, page_size, offset, limit, sort_name, sort_order)
Returns a list of all license groups

<p>Requires permission <strong>POLICY_MANAGEMENT</strong></p>

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

[**Vec<models::LicenseGroup>**](LicenseGroup.md)

### Authorization

[X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_license_from_license_group

> models::LicenseGroup remove_license_from_license_group(uuid, license_uuid)
Removes the license from the license group.

<p>Requires permission <strong>POLICY_MANAGEMENT</strong></p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **uuid::Uuid** | A valid license group | [required] |
**license_uuid** | **uuid::Uuid** | A valid license | [required] |

### Return type

[**models::LicenseGroup**](LicenseGroup.md)

### Authorization

[X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_license_group

> models::LicenseGroup update_license_group(body)
Updates a license group

<p>Requires permission <strong>POLICY_MANAGEMENT</strong></p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**LicenseGroup**](LicenseGroup.md)> |  |  |

### Return type

[**models::LicenseGroup**](LicenseGroup.md)

### Authorization

[X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

