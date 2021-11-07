# \TransactionsApi

All URIs are relative to *https://https://publicapi.sbanken.no/apibeta*

Method | HTTP request | Description
------------- | ------------- | -------------
[**transactions_list**](TransactionsApi.md#transactions_list) | **GET** /api/v2/Transactions/{accountId} | Get Transactions
[**transactions_list_archive**](TransactionsApi.md#transactions_list_archive) | **GET** /api/v2/Transactions/archive/{accountId} | Get Archived Transactions (Norwegian: bokført) with transactionId



## transactions_list

> crate::models::ListResultOfTransaction transactions_list(account_id, start_date, end_date, index, length)
Get Transactions

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | Option<**String**> | The `accountId` of the account. The account must be one owned by the customer, or an account the customer has been granted access to. An account's accountId can be retrieved with the list or read operation on the Accounts service. | [required] |
**start_date** | Option<**String**> | *Optional*. The start of the query time span. Must be less than or equal to `endDate`, and less than or equal to the current date + 1 day. Default value is `endDate` -30 days. Minimum value is `2000-01-01` |  |
**end_date** | Option<**String**> | *Optional*. The end of the query time span. Must be greater than or equal to `startDate`, and less than or equal to the current date +1 day. Query cannot span more than 366 days. Default value is the current date. |  |
**index** | Option<**i32**> | *Optional*. The `index` of the first item to be retrieved. Minimum value is `0`, which is the first item within the query time span. Default value is `0`. |  |
**length** | Option<**i32**> | *Optional*. Return a number of items items up to this value. Minimum value is `1`, maximum value is `1000`. The default value is `100`. |  |

### Return type

[**crate::models::ListResultOfTransaction**](ListResultOfTransaction.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## transactions_list_archive

> crate::models::ListResultOfArchiveTransaction transactions_list_archive(account_id, start_date, end_date, index, length)
Get Archived Transactions (Norwegian: bokført) with transactionId

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | Option<**String**> | The `accountId` of the account. The account must be one owned by the customer, or an account the customer has been granted access to. An account's accountId can be retrieved with the list or read operation on the Accounts service. | [required] |
**start_date** | Option<**String**> | *Optional*. The start of the query time span. Must be less than or equal to `endDate`, and less than or equal to the current date + 1 day. Default value is `endDate` -30 days. Minimum value is `2000-01-01` |  |
**end_date** | Option<**String**> | *Optional*. The end of the query time span. Must be greater than or equal to `startDate`, and less than or equal to the current date +1 day. Query cannot span more than 366 days. Default value is the current date. |  |
**index** | Option<**i32**> | *Optional*. The `index` of the first item to be retrieved. Minimum value is `0`, which is the first item within the query time span. Default value is `0`. |  |
**length** | Option<**i32**> | *Optional*. Return a number of items items up to this value. Minimum value is `1`, maximum value is `1000`. The default value is `100`. |  |

### Return type

[**crate::models::ListResultOfArchiveTransaction**](ListResultOfArchiveTransaction.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

