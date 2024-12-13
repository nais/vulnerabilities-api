# \PolicyConditionApi

All URIs are relative to */api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_policy_condition**](PolicyConditionApi.md#create_policy_condition) | **PUT** /v1/policy/{uuid}/condition | Creates a new policy condition
[**delete_policy_condition**](PolicyConditionApi.md#delete_policy_condition) | **DELETE** /v1/policy/condition/{uuid} | Deletes a policy condition
[**update_policy_condition**](PolicyConditionApi.md#update_policy_condition) | **POST** /v1/policy/condition | Updates a policy condition



## create_policy_condition

> models::PolicyCondition create_policy_condition(uuid, body)
Creates a new policy condition

<p>Requires permission <strong>POLICY_MANAGEMENT</strong></p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **uuid::Uuid** | The UUID of the policy | [required] |
**body** | Option<[**PolicyCondition**](PolicyCondition.md)> |  |  |

### Return type

[**models::PolicyCondition**](PolicyCondition.md)

### Authorization

[X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_policy_condition

> delete_policy_condition(uuid)
Deletes a policy condition

<p>Requires permission <strong>POLICY_MANAGEMENT</strong></p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **uuid::Uuid** | The UUID of the policy condition to delete | [required] |

### Return type

 (empty response body)

### Authorization

[X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_policy_condition

> models::PolicyCondition update_policy_condition(body)
Updates a policy condition

<p>Requires permission <strong>POLICY_MANAGEMENT</strong></p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**PolicyCondition**](PolicyCondition.md)> |  |  |

### Return type

[**models::PolicyCondition**](PolicyCondition.md)

### Authorization

[X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

