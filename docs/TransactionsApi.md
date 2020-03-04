# \TransactionsApi

All URIs are relative to *https://api.sbanken.no*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_transactions**](TransactionsApi.md#get_transactions) | **GET** /exec.bank/api/v1/Transactions/{accountId} | This operation returns the latest transactions of the given account within the time span set by the start and end date parameters.     Note that dateTime type parameters are relative to Central European Time (GMT+1); only the date part is relevant.



## get_transactions

> crate::models::ListResultTransactionV1 get_transactions(customer_id, account_id, start_date, end_date, index, length)
This operation returns the latest transactions of the given account within the time span set by the start and end date parameters.     Note that dateTime type parameters are relative to Central European Time (GMT+1); only the date part is relevant.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**customer_id** | **String** | The customerId of the customer. | [required] |
**account_id** | **String** | The accountId of the account. | [required] |
**start_date** | Option<**String**> | Optional. The start of the query time span. Must be less than or equal to endDate, and less than or equal to the current date + 1 day. Default value is endDate -30 days. Minimum value is 2000-01-01 |  |
**end_date** | Option<**String**> | Optional. The end of the query time span. Must be greater than or equal to startDate, and less than or equal to the current date +1 day. Query cannot span more than 366 days. Default value is the current date. |  |
**index** | Option<**i32**> | Optional. The index of the first item to be retrieved. Minimum value is 0, which is the first item within the query time span. Default value is 0 |  |
**length** | Option<**i32**> | Optional. The number of items to be retrieved. Minimum value is 1. Maximum value is 1000. Default value is 100 |  |

### Return type

[**crate::models::ListResultTransactionV1**](ListResult.Transaction.v1.md)

### Authorization

[sbanken](../README.md#sbanken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

