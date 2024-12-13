# \TagApi

All URIs are relative to */api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_tags**](TagApi.md#get_tags) | **GET** /v1/tag/{policyUuid} | Returns a list of all tags associated with a given policy



## get_tags

> Vec<models::Tag> get_tags(policy_uuid)
Returns a list of all tags associated with a given policy

<p>Requires permission <strong>VIEW_PORTFOLIO</strong></p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**policy_uuid** | **uuid::Uuid** | The UUID of the policy | [required] |

### Return type

[**Vec<models::Tag>**](Tag.md)

### Authorization

[X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

