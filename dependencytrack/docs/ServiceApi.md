# \ServiceApi

All URIs are relative to */api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_service**](ServiceApi.md#create_service) | **PUT** /v1/service/project/{uuid} | Creates a new service
[**delete_service**](ServiceApi.md#delete_service) | **DELETE** /v1/service/{uuid} | Deletes a service
[**get_all_services**](ServiceApi.md#get_all_services) | **GET** /v1/service/project/{uuid} | Returns a list of all services for a given project
[**get_service_by_uuid**](ServiceApi.md#get_service_by_uuid) | **GET** /v1/service/{uuid} | Returns a specific service
[**update_service**](ServiceApi.md#update_service) | **POST** /v1/service | Updates a service



## create_service

> models::ServiceComponent create_service(uuid, body)
Creates a new service

<p>Requires permission <strong>PORTFOLIO_MANAGEMENT</strong></p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **uuid::Uuid** | The UUID of the project | [required] |
**body** | Option<[**ServiceComponent**](ServiceComponent.md)> |  |  |

### Return type

[**models::ServiceComponent**](ServiceComponent.md)

### Authorization

[X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_service

> delete_service(uuid)
Deletes a service

<p>Requires permission <strong>PORTFOLIO_MANAGEMENT</strong></p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **uuid::Uuid** | The UUID of the service to delete | [required] |

### Return type

 (empty response body)

### Authorization

[X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_all_services

> Vec<models::ServiceComponent> get_all_services(uuid, page_number, page_size, offset, limit, sort_name, sort_order)
Returns a list of all services for a given project

<p>Requires permission <strong>VIEW_PORTFOLIO</strong></p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **uuid::Uuid** | The UUID of the project | [required] |
**page_number** | Option<[**serde_json::Value**](.md)> | The page to return. To be used in conjunction with <code>pageSize</code>. |  |[default to 1]
**page_size** | Option<[**serde_json::Value**](.md)> | Number of elements to return per page. To be used in conjunction with <code>pageNumber</code>. |  |[default to 100]
**offset** | Option<[**serde_json::Value**](.md)> | Offset to start returning elements from. To be used in conjunction with <code>limit</code>. |  |
**limit** | Option<[**serde_json::Value**](.md)> | Number of elements to return per page. To be used in conjunction with <code>offset</code>. |  |
**sort_name** | Option<**String**> | Name of the resource field to sort on. |  |
**sort_order** | Option<**String**> | Ordering of items when sorting with <code>sortName</code>. |  |

### Return type

[**Vec<models::ServiceComponent>**](ServiceComponent.md)

### Authorization

[X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_service_by_uuid

> models::ServiceComponent get_service_by_uuid(uuid)
Returns a specific service

<p>Requires permission <strong>VIEW_PORTFOLIO</strong></p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **uuid::Uuid** | The UUID of the service to retrieve | [required] |

### Return type

[**models::ServiceComponent**](ServiceComponent.md)

### Authorization

[X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_service

> models::ServiceComponent update_service(body)
Updates a service

<p>Requires permission <strong>PORTFOLIO_MANAGEMENT</strong></p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**ServiceComponent**](ServiceComponent.md)> |  |  |

### Return type

[**models::ServiceComponent**](ServiceComponent.md)

### Authorization

[X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

