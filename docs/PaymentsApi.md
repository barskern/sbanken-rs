# \PaymentsApi

All URIs are relative to *https://api.sbanken.no*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_payment**](PaymentsApi.md#get_payment) | **GET** /exec.bank/api/v1/Payments/{accountId}/{paymentId} | Read a payment.
[**list_payments**](PaymentsApi.md#list_payments) | **GET** /exec.bank/api/v1/Payments/{accountId} | List the payments. These payments are awaiting processing. Payments are processed on the due date.



## get_payment

> crate::models::ItemResultPaymentV1 get_payment(customer_id, account_id, payment_id)
Read a payment.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**customer_id** | **String** | The customerId of the customer. | [required] |
**account_id** | **String** | The accountId of the account. | [required] |
**payment_id** | **String** | The paymentId of the payment. | [required] |

### Return type

[**crate::models::ItemResultPaymentV1**](ItemResult.Payment.v1.md)

### Authorization

[sbanken](../README.md#sbanken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_payments

> crate::models::ListResultPaymentV1 list_payments(customer_id, account_id, index, length)
List the payments. These payments are awaiting processing. Payments are processed on the due date.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**customer_id** | **String** | The customerId of the customer. | [required] |
**account_id** | **String** | The accountId of the account. | [required] |
**index** | Option<**i32**> | Optional. Return items with this index or greater. |  |
**length** | Option<**i32**> | Optional. Return items up to this number. |  |

### Return type

[**crate::models::ListResultPaymentV1**](ListResult.Payment.v1.md)

### Authorization

[sbanken](../README.md#sbanken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

