# \NotificationApi

All URIs are relative to */api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_project_to_rule**](NotificationApi.md#add_project_to_rule) | **POST** /v1/notification/rule/{ruleUuid}/project/{projectUuid} | Adds a project to a notification rule
[**add_team_to_rule**](NotificationApi.md#add_team_to_rule) | **POST** /v1/notification/rule/{ruleUuid}/team/{teamUuid} | Adds a team to a notification rule
[**create_notification_publisher**](NotificationApi.md#create_notification_publisher) | **PUT** /v1/notification/publisher | Creates a new notification publisher
[**create_notification_rule**](NotificationApi.md#create_notification_rule) | **PUT** /v1/notification/rule | Creates a new notification rule
[**delete_notification_publisher**](NotificationApi.md#delete_notification_publisher) | **DELETE** /v1/notification/publisher/{notificationPublisherUuid} | Deletes a notification publisher and all related notification rules
[**delete_notification_rule**](NotificationApi.md#delete_notification_rule) | **DELETE** /v1/notification/rule | Deletes a notification rule
[**get_all_notification_publishers**](NotificationApi.md#get_all_notification_publishers) | **GET** /v1/notification/publisher | Returns a list of all notification publishers
[**get_all_notification_rules**](NotificationApi.md#get_all_notification_rules) | **GET** /v1/notification/rule | Returns a list of all notification rules
[**remove_project_from_rule**](NotificationApi.md#remove_project_from_rule) | **DELETE** /v1/notification/rule/{ruleUuid}/project/{projectUuid} | Removes a project from a notification rule
[**remove_team_from_rule**](NotificationApi.md#remove_team_from_rule) | **DELETE** /v1/notification/rule/{ruleUuid}/team/{teamUuid} | Removes a team from a notification rule
[**restore_default_templates**](NotificationApi.md#restore_default_templates) | **POST** /v1/notification/publisher/restoreDefaultTemplates | Restore the default notification publisher templates using the ones in the solution classpath
[**test_smtp_publisher_config**](NotificationApi.md#test_smtp_publisher_config) | **POST** /v1/notification/publisher/test/smtp | Dispatches a SMTP notification test
[**update_notification_publisher**](NotificationApi.md#update_notification_publisher) | **POST** /v1/notification/publisher | Updates a notification publisher
[**update_notification_rule**](NotificationApi.md#update_notification_rule) | **POST** /v1/notification/rule | Updates a notification rule



## add_project_to_rule

> models::NotificationRule add_project_to_rule(rule_uuid, project_uuid)
Adds a project to a notification rule

<p>Requires permission <strong>SYSTEM_CONFIGURATION</strong></p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**rule_uuid** | **uuid::Uuid** | The UUID of the rule to add a project to | [required] |
**project_uuid** | **uuid::Uuid** | The UUID of the project to add to the rule | [required] |

### Return type

[**models::NotificationRule**](NotificationRule.md)

### Authorization

[X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## add_team_to_rule

> models::NotificationRule add_team_to_rule(rule_uuid, team_uuid)
Adds a team to a notification rule

<p>Requires permission <strong>SYSTEM_CONFIGURATION</strong></p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**rule_uuid** | **uuid::Uuid** | The UUID of the rule to add a team to | [required] |
**team_uuid** | **uuid::Uuid** | The UUID of the team to add to the rule | [required] |

### Return type

[**models::NotificationRule**](NotificationRule.md)

### Authorization

[X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_notification_publisher

> models::NotificationPublisher create_notification_publisher(body)
Creates a new notification publisher

<p>Requires permission <strong>SYSTEM_CONFIGURATION</strong></p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**NotificationPublisher**](NotificationPublisher.md)> |  |  |

### Return type

[**models::NotificationPublisher**](NotificationPublisher.md)

### Authorization

[X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_notification_rule

> models::NotificationRule create_notification_rule(body)
Creates a new notification rule

<p>Requires permission <strong>SYSTEM_CONFIGURATION</strong></p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**NotificationRule**](NotificationRule.md)> |  |  |

### Return type

[**models::NotificationRule**](NotificationRule.md)

### Authorization

[X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_notification_publisher

> delete_notification_publisher(notification_publisher_uuid)
Deletes a notification publisher and all related notification rules

<p>Requires permission <strong>SYSTEM_CONFIGURATION</strong></p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**notification_publisher_uuid** | **uuid::Uuid** | The UUID of the notification publisher to delete | [required] |

### Return type

 (empty response body)

### Authorization

[X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_notification_rule

> delete_notification_rule(body)
Deletes a notification rule

<p>Requires permission <strong>SYSTEM_CONFIGURATION</strong></p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**NotificationRule**](NotificationRule.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_all_notification_publishers

> Vec<models::NotificationPublisher> get_all_notification_publishers()
Returns a list of all notification publishers

<p>Requires permission <strong>SYSTEM_CONFIGURATION</strong></p>

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::NotificationPublisher>**](NotificationPublisher.md)

### Authorization

[X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_all_notification_rules

> Vec<models::NotificationRule> get_all_notification_rules(page_number, page_size, offset, limit, sort_name, sort_order)
Returns a list of all notification rules

<p>Requires permission <strong>SYSTEM_CONFIGURATION</strong></p>

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

[**Vec<models::NotificationRule>**](NotificationRule.md)

### Authorization

[X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_project_from_rule

> models::NotificationRule remove_project_from_rule(rule_uuid, project_uuid)
Removes a project from a notification rule

<p>Requires permission <strong>SYSTEM_CONFIGURATION</strong></p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**rule_uuid** | **uuid::Uuid** | The UUID of the rule to remove the project from | [required] |
**project_uuid** | **uuid::Uuid** | The UUID of the project to remove from the rule | [required] |

### Return type

[**models::NotificationRule**](NotificationRule.md)

### Authorization

[X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_team_from_rule

> models::NotificationRule remove_team_from_rule(rule_uuid, team_uuid)
Removes a team from a notification rule

<p>Requires permission <strong>SYSTEM_CONFIGURATION</strong></p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**rule_uuid** | **uuid::Uuid** | The UUID of the rule to remove the project from | [required] |
**team_uuid** | **uuid::Uuid** | The UUID of the project to remove from the rule | [required] |

### Return type

[**models::NotificationRule**](NotificationRule.md)

### Authorization

[X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## restore_default_templates

> restore_default_templates()
Restore the default notification publisher templates using the ones in the solution classpath

<p>Requires permission <strong>SYSTEM_CONFIGURATION</strong></p>

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## test_smtp_publisher_config

> test_smtp_publisher_config(destination)
Dispatches a SMTP notification test

<p>Requires permission <strong>SYSTEM_CONFIGURATION</strong></p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**destination** | Option<**String**> |  |  |

### Return type

 (empty response body)

### Authorization

[X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_notification_publisher

> models::NotificationRule update_notification_publisher(body)
Updates a notification publisher

<p>Requires permission <strong>SYSTEM_CONFIGURATION</strong></p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**NotificationPublisher**](NotificationPublisher.md)> |  |  |

### Return type

[**models::NotificationRule**](NotificationRule.md)

### Authorization

[X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_notification_rule

> models::NotificationRule update_notification_rule(body)
Updates a notification rule

<p>Requires permission <strong>SYSTEM_CONFIGURATION</strong></p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**NotificationRule**](NotificationRule.md)> |  |  |

### Return type

[**models::NotificationRule**](NotificationRule.md)

### Authorization

[X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

