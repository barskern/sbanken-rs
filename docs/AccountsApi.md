# \AccountsApi

All URIs are relative to *https://https://publicapi.sbanken.no/apibeta*

Method | HTTP request | Description
------------- | ------------- | -------------
[**accounts_list**](AccountsApi.md#accounts_list) | **GET** /api/v2/Accounts | Lists the accounts.
[**accounts_read**](AccountsApi.md#accounts_read) | **GET** /api/v2/Accounts/{accountId} | Reads an account



## accounts_list

> crate::models::ListResultOfAccount accounts_list()
Lists the accounts.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::ListResultOfAccount**](ListResultOfAccount.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## accounts_read

> crate::models::Account accounts_read(account_id)
Reads an account

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | Option<**String**> | The `accountId` of the account. The account must be one owned by the customer, or an account the customer has been granted access to. An account's `accountId` can be retrieved with the list or read operation on this service. | [required] |

### Return type

[**crate::models::Account**](Account.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

