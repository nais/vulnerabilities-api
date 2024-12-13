# \EventApi

All URIs are relative to */api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**is_token_being_processed1**](EventApi.md#is_token_being_processed1) | **GET** /v1/event/token/{uuid} | Determines if there are any tasks associated with the token that are being processed, or in the queue to be processed.



## is_token_being_processed1

> models::IsTokenBeingProcessedResponse is_token_being_processed1(uuid)
Determines if there are any tasks associated with the token that are being processed, or in the queue to be processed.

<p>   This endpoint is intended to be used in conjunction with other API calls which return a token for asynchronous tasks.   The token can then be queried using this endpoint to determine if the task is complete:   <ul>     <li>A value of <code>true</code> indicates processing is occurring.</li>     <li>A value of <code>false</code> indicates that no processing is occurring for the specified token.</li>   </ul>   However, a value of <code>false</code> also does not confirm the token is valid,   only that no processing is associated with the specified token. </p>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **uuid::Uuid** | The UUID of the token to query | [required] |

### Return type

[**models::IsTokenBeingProcessedResponse**](IsTokenBeingProcessedResponse.md)

### Authorization

[X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

