# \SearchApi

All URIs are relative to */api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**aggregate_search**](SearchApi.md#aggregate_search) | **GET** /v1/search | Processes and returns search results
[**component_search**](SearchApi.md#component_search) | **GET** /v1/search/component | Processes and returns search results
[**license_search**](SearchApi.md#license_search) | **GET** /v1/search/license | Processes and returns search results
[**project_search**](SearchApi.md#project_search) | **GET** /v1/search/project | Processes and returns search results
[**reindex**](SearchApi.md#reindex) | **POST** /v1/search/reindex | Rebuild lucene indexes for search operations
[**service_search**](SearchApi.md#service_search) | **GET** /v1/search/service | Processes and returns search results
[**vulnerability_search**](SearchApi.md#vulnerability_search) | **GET** /v1/search/vulnerability | Processes and returns search results
[**vulnerable_software_search**](SearchApi.md#vulnerable_software_search) | **GET** /v1/search/vulnerablesoftware | Processes and returns search results



## aggregate_search

> models::SearchResult aggregate_search(query)
Processes and returns search results

<p>Requires permission <strong>VIEW_PORTFOLIO</strong></p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**query** | Option<**String**> |  |  |

### Return type

[**models::SearchResult**](SearchResult.md)

### Authorization

[X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## component_search

> models::SearchResult component_search(query)
Processes and returns search results

<p>Requires permission <strong>VIEW_PORTFOLIO</strong></p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**query** | Option<**String**> |  |  |

### Return type

[**models::SearchResult**](SearchResult.md)

### Authorization

[X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## license_search

> models::SearchResult license_search(query)
Processes and returns search results

<p>Requires permission <strong>VIEW_PORTFOLIO</strong></p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**query** | Option<**String**> |  |  |

### Return type

[**models::SearchResult**](SearchResult.md)

### Authorization

[X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## project_search

> models::SearchResult project_search(query)
Processes and returns search results

<p>Requires permission <strong>VIEW_PORTFOLIO</strong></p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**query** | Option<**String**> |  |  |

### Return type

[**models::SearchResult**](SearchResult.md)

### Authorization

[X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reindex

> reindex(r#type)
Rebuild lucene indexes for search operations

<p>Requires permission <strong>SYSTEM_CONFIGURATION</strong></p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**r#type** | Option<[**Vec<String>**](String.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## service_search

> models::SearchResult service_search(query)
Processes and returns search results

<p>Requires permission <strong>VIEW_PORTFOLIO</strong></p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**query** | Option<**String**> |  |  |

### Return type

[**models::SearchResult**](SearchResult.md)

### Authorization

[X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## vulnerability_search

> models::SearchResult vulnerability_search(query)
Processes and returns search results

<p>Requires permission <strong>VIEW_PORTFOLIO</strong></p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**query** | Option<**String**> |  |  |

### Return type

[**models::SearchResult**](SearchResult.md)

### Authorization

[X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## vulnerable_software_search

> models::SearchResult vulnerable_software_search(query, cpe)
Processes and returns search results

<p>Requires permission <strong>VIEW_PORTFOLIO</strong></p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**query** | Option<**String**> |  |  |
**cpe** | Option<**String**> |  |  |

### Return type

[**models::SearchResult**](SearchResult.md)

### Authorization

[X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

