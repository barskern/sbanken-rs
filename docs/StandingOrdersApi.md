# \StandingOrdersApi

All URIs are relative to *https://api.sbanken.no*

Method | HTTP request | Description
------------- | ------------- | -------------
[**list_standing_orders**](StandingOrdersApi.md#list_standing_orders) | **GET** /exec.bank/api/v1/StandingOrders/{accountId} | Lists the standing orders for repeated transfers and payments.



## list_standing_orders

> crate::models::ListResultStandingOrderV1 list_standing_orders(customer_id, account_id)
Lists the standing orders for repeated transfers and payments.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**customer_id** | **String** | The customerId of the customer. | [required] |
**account_id** | **String** | The accountId of the account. | [required] |

### Return type

[**crate::models::ListResultStandingOrderV1**](ListResult.StandingOrder.v1.md)

### Authorization

[sbanken](../README.md#sbanken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

