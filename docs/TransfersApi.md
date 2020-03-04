# \TransfersApi

All URIs are relative to *https://api.sbanken.no*

Method | HTTP request | Description
------------- | ------------- | -------------
[**transfer**](TransfersApi.md#transfer) | **POST** /exec.bank/api/v1/Transfers | This operation executes a transfer between two accounts.



## transfer

> crate::models::NoResult transfer(customer_id, transfer_create_request_v1)
This operation executes a transfer between two accounts.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**customer_id** | **String** | The customerId of the customer | [required] |
**transfer_create_request_v1** | Option<[**TransferCreateRequestV1**](TransferCreateRequestV1.md)> | The details of the transfer to be executed. The fields are as follows:               FromAccountId: The account number of the account that the amount is to be transferred from, i.e. the debit account. This is a numerical string 11 characters long. The account number must be one of the accounts owned by the customer, or an account the customer has been granted access to.              ToAccountId: The account number of the account that the amount is to be transferred to, i.e. the credit account. This is a numerical string 11 characters long. The account number must be one of the accounts owned by the customer, or an account the customer has been granted access to.              Amount: A decimal number representing the amount to be transferred. Must be equal to or greater than 1.00 and less than 100000000000000000.00 (sic!). Transfers with amounts in excess of the current debit account available amount will fail. Transfer currency is NOK.              Message: A description of the transfer. Must be between 1 and 30 characters. The following characters are allowed: \"123456789 0aAbBcCdDeEfFgGhHi IjJkKlLmMnNoOpPqQrRsStTuUvVwW xXyYzZæÆøØåÅäÄëËïÏöÖüÜÿâÂêÊîÎ ôÔûÛãÃñÑõÕàÀèÈìÌòÒùÙáÁéÉí ÍóÓýÝ,;.:!-/()?\", and space. |  |

### Return type

[**crate::models::NoResult**](NoResult..md)

### Authorization

[sbanken](../README.md#sbanken)

### HTTP request headers

- **Content-Type**: application/json-patch+json, application/json, text/json, application/_*+json
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

