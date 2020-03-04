# \AccountsApi

All URIs are relative to *https://api.sbanken.no*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_account**](AccountsApi.md#get_account) | **GET** /exec.bank/api/v1/Accounts/{accountId} | Reads an account
[**list_accounts**](AccountsApi.md#list_accounts) | **GET** /exec.bank/api/v1/Accounts | Lists the accounts.



## get_account

> crate::models::ItemResultAccountV1 get_account(customer_id, account_id)
Reads an account

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**customer_id** | **String** | The customerId of the customer. | [required] |
**account_id** | **String** | The accountId of the account. | [required] |

### Return type

[**crate::models::ItemResultAccountV1**](ItemResult.Account.v1.md)

### Authorization

[sbanken](../README.md#sbanken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_accounts

> crate::models::ListResultAccountV1 list_accounts(customer_id)
Lists the accounts.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**customer_id** | **String** | The customerId of the customer. | [required] |

### Return type

[**crate::models::ListResultAccountV1**](ListResult.Account.v1.md)

### Authorization

[sbanken](../README.md#sbanken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

