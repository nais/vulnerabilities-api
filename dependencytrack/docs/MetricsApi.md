# \MetricsApi

All URIs are relative to */api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_component_current_metrics**](MetricsApi.md#get_component_current_metrics) | **GET** /v1/metrics/component/{uuid}/current | Returns current metrics for a specific component
[**get_component_metrics_since**](MetricsApi.md#get_component_metrics_since) | **GET** /v1/metrics/component/{uuid}/since/{date} | Returns historical metrics for a specific component from a specific date
[**get_component_metrics_x_days**](MetricsApi.md#get_component_metrics_x_days) | **GET** /v1/metrics/component/{uuid}/days/{days} | Returns X days of historical metrics for a specific component
[**get_portfolio_current_metrics**](MetricsApi.md#get_portfolio_current_metrics) | **GET** /v1/metrics/portfolio/current | Returns current metrics for the entire portfolio
[**get_portfolio_metrics_since**](MetricsApi.md#get_portfolio_metrics_since) | **GET** /v1/metrics/portfolio/since/{date} | Returns historical metrics for the entire portfolio from a specific date
[**get_portfolio_metrics_x_days**](MetricsApi.md#get_portfolio_metrics_x_days) | **GET** /v1/metrics/portfolio/{days}/days | Returns X days of historical metrics for the entire portfolio
[**get_project_current_metrics**](MetricsApi.md#get_project_current_metrics) | **GET** /v1/metrics/project/{uuid}/current | Returns current metrics for a specific project
[**get_project_metrics_since**](MetricsApi.md#get_project_metrics_since) | **GET** /v1/metrics/project/{uuid}/since/{date} | Returns historical metrics for a specific project from a specific date
[**get_project_metrics_x_days**](MetricsApi.md#get_project_metrics_x_days) | **GET** /v1/metrics/project/{uuid}/days/{days} | Returns X days of historical metrics for a specific project
[**get_vulnerability_metrics**](MetricsApi.md#get_vulnerability_metrics) | **GET** /v1/metrics/vulnerability | Returns the sum of all vulnerabilities in the database by year and month
[**refresh_component_metrics**](MetricsApi.md#refresh_component_metrics) | **GET** /v1/metrics/component/{uuid}/refresh | Requests a refresh of a specific components metrics
[**refresh_portfolio_metrics**](MetricsApi.md#refresh_portfolio_metrics) | **GET** /v1/metrics/portfolio/refresh | Requests a refresh of the portfolio metrics
[**refresh_project_metrics**](MetricsApi.md#refresh_project_metrics) | **GET** /v1/metrics/project/{uuid}/refresh | Requests a refresh of a specific projects metrics



## get_component_current_metrics

> models::DependencyMetrics get_component_current_metrics(uuid)
Returns current metrics for a specific component

<p>Requires permission <strong>VIEW_PORTFOLIO</strong></p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **uuid::Uuid** | The UUID of the component to retrieve metrics for | [required] |

### Return type

[**models::DependencyMetrics**](DependencyMetrics.md)

### Authorization

[X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_component_metrics_since

> Vec<models::DependencyMetrics> get_component_metrics_since(uuid, date)
Returns historical metrics for a specific component from a specific date

<p>Date format must be <code>YYYYMMDD</code></p> <p>Requires permission <strong>VIEW_PORTFOLIO</strong></p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **uuid::Uuid** | The UUID of the component to retrieve metrics for | [required] |
**date** | **String** | The start date to retrieve metrics for | [required] |

### Return type

[**Vec<models::DependencyMetrics>**](DependencyMetrics.md)

### Authorization

[X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_component_metrics_x_days

> Vec<models::DependencyMetrics> get_component_metrics_x_days(uuid, days)
Returns X days of historical metrics for a specific component

<p>Requires permission <strong>VIEW_PORTFOLIO</strong></p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **uuid::Uuid** | The UUID of the component to retrieve metrics for | [required] |
**days** | **i32** | The number of days back to retrieve metrics for | [required] |

### Return type

[**Vec<models::DependencyMetrics>**](DependencyMetrics.md)

### Authorization

[X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_portfolio_current_metrics

> models::PortfolioMetrics get_portfolio_current_metrics()
Returns current metrics for the entire portfolio

<p>Requires permission <strong>VIEW_PORTFOLIO</strong></p>

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::PortfolioMetrics**](PortfolioMetrics.md)

### Authorization

[X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_portfolio_metrics_since

> Vec<models::PortfolioMetrics> get_portfolio_metrics_since(date)
Returns historical metrics for the entire portfolio from a specific date

<p>Date format must be <code>YYYYMMDD</code></p> <p>Requires permission <strong>VIEW_PORTFOLIO</strong></p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**date** | **String** | The start date to retrieve metrics for | [required] |

### Return type

[**Vec<models::PortfolioMetrics>**](PortfolioMetrics.md)

### Authorization

[X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_portfolio_metrics_x_days

> Vec<models::PortfolioMetrics> get_portfolio_metrics_x_days(days)
Returns X days of historical metrics for the entire portfolio

<p>Requires permission <strong>VIEW_PORTFOLIO</strong></p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**days** | **i32** | The number of days back to retrieve metrics for | [required] |

### Return type

[**Vec<models::PortfolioMetrics>**](PortfolioMetrics.md)

### Authorization

[X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_project_current_metrics

> models::ProjectMetrics get_project_current_metrics(uuid)
Returns current metrics for a specific project

<p>Requires permission <strong>VIEW_PORTFOLIO</strong></p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **uuid::Uuid** | The UUID of the project to retrieve metrics for | [required] |

### Return type

[**models::ProjectMetrics**](ProjectMetrics.md)

### Authorization

[X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_project_metrics_since

> Vec<models::ProjectMetrics> get_project_metrics_since(uuid, date)
Returns historical metrics for a specific project from a specific date

<p>Date format must be <code>YYYYMMDD</code></p> <p>Requires permission <strong>VIEW_PORTFOLIO</strong></p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **uuid::Uuid** | The UUID of the project to retrieve metrics for | [required] |
**date** | **String** | The start date to retrieve metrics for | [required] |

### Return type

[**Vec<models::ProjectMetrics>**](ProjectMetrics.md)

### Authorization

[X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_project_metrics_x_days

> Vec<models::ProjectMetrics> get_project_metrics_x_days(uuid, days)
Returns X days of historical metrics for a specific project

<p>Requires permission <strong>VIEW_PORTFOLIO</strong></p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **uuid::Uuid** | The UUID of the project to retrieve metrics for | [required] |
**days** | **i32** | The number of days back to retrieve metrics for | [required] |

### Return type

[**Vec<models::ProjectMetrics>**](ProjectMetrics.md)

### Authorization

[X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_vulnerability_metrics

> Vec<models::VulnerabilityMetrics> get_vulnerability_metrics()
Returns the sum of all vulnerabilities in the database by year and month

<p>Requires permission <strong>VIEW_PORTFOLIO</strong></p>

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::VulnerabilityMetrics>**](VulnerabilityMetrics.md)

### Authorization

[X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## refresh_component_metrics

> refresh_component_metrics(uuid)
Requests a refresh of a specific components metrics

<p>Requires permission <strong>PORTFOLIO_MANAGEMENT</strong></p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **uuid::Uuid** | The UUID of the component to refresh metrics on | [required] |

### Return type

 (empty response body)

### Authorization

[X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## refresh_portfolio_metrics

> models::PortfolioMetrics refresh_portfolio_metrics()
Requests a refresh of the portfolio metrics

<p>Requires permission <strong>PORTFOLIO_MANAGEMENT</strong></p>

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::PortfolioMetrics**](PortfolioMetrics.md)

### Authorization

[X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## refresh_project_metrics

> refresh_project_metrics(uuid)
Requests a refresh of a specific projects metrics

<p>Requires permission <strong>PORTFOLIO_MANAGEMENT</strong></p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **uuid::Uuid** | The UUID of the project to refresh metrics on | [required] |

### Return type

 (empty response body)

### Authorization

[X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

