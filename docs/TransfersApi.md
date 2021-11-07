# \TransfersApi

All URIs are relative to *https://https://publicapi.sbanken.no/apibeta*

Method | HTTP request | Description
------------- | ------------- | -------------
[**transfers_create**](TransfersApi.md#transfers_create) | **POST** /api/v2/Transfers | This operation executes a transfer between two accounts.



## transfers_create

> std::path::PathBuf transfers_create(value)
This operation executes a transfer between two accounts.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**value** | [**TransferCreateRequest**](TransferCreateRequest.md) | The details of the transfer to be executed. The fields are as follows:\\             \\             **FromAccountId:** The `accountId` of the account that the amount is to be transferred from, i.e. the debit account. The account must be one owned by the customer, or an account the customer has been granted access to. An account's `accountId` can be retrieved with the list or read operation on the Accounts service.\\             \\             **ToAccountId:** The `accountId` of the account that the amount is to be transferred to, i.e. the credit account. The account must be one owned by the customer, or an account the customer has been granted access to. An account's `accountId` can be retrieved with the list or read operation on the Accounts service.\\             \\             **Amount:** A decimal number representing the amount to be transferred. Must be equal to or greater than `1.00` and less than `100000000000000000.00` (sic!). Transfers with amounts in excess of the current debit account available amount will fail. Transfer currency is NOK.\\             \\             **Message:** A description of the transfer. Must be between 1 and 30 characters. The following characters are allowed: `1234567890aAbBcCdDeEfFgGhHi IjJkKlLmMnNoOpPqQrRsStTuUvVwW xXyYzZæÆøØåÅäÄëËïÏöÖüÜÿâÂêÊîÎ ôÔûÛãÃñÑõÕàÀèÈìÌòÒùÙáÁéÉí ÍóÓýÝ,;.:!-/()?`, and `space`.              | [required] |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, text/json, application/_*+json
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

