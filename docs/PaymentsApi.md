# \PaymentsApi

All URIs are relative to *https://https://publicapi.sbanken.no/apibeta*

Method | HTTP request | Description
------------- | ------------- | -------------
[**payments_list**](PaymentsApi.md#payments_list) | **GET** /api/v2/Payments/{accountId} | List the payments. These payments are awaiting processing. Payments are processed on the due date.
[**payments_read**](PaymentsApi.md#payments_read) | **GET** /api/v2/Payments/{accountId}/{paymentId} | Read a payment.



## payments_list

> crate::models::ListResultOfPayment payments_list(account_id, index, length)
List the payments. These payments are awaiting processing. Payments are processed on the due date.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | Option<**String**> | The `accountId` of the account. The account must be one owned by the customer, or an account the customer has been granted access to. An account's `accountId` can be retrieved with the list or read operation on the Accounts service. | [required] |
**index** | Option<**i32**> | *Optional*. Return items with this `index` or greater. Minimum value is `0`, which is the first item within the query time span. Default value is `0`. |  |
**length** | Option<**i32**> | *Optional*. Return a number of items items up to this value. Minimum value is `1`, maximum value is `1000`. The default value is `100`. |  |

### Return type

[**crate::models::ListResultOfPayment**](ListResultOfPayment.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## payments_read

> crate::models::Payment payments_read(account_id, payment_id)
Read a payment.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | Option<**String**> | The `accountId` of the account. The account must be one owned by the customer, or an account the customer has been granted access to. An account's `accountId` can be retrieved with the list or read operation on the Accounts service. | [required] |
**payment_id** | Option<**String**> | The `paymentId` of the payment. The `paymentId` can be retrieved with the list operation on this service. | [required] |

### Return type

[**crate::models::Payment**](Payment.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

