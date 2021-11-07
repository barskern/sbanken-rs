# \StandingOrdersApi

All URIs are relative to *https://https://publicapi.sbanken.no/apibeta*

Method | HTTP request | Description
------------- | ------------- | -------------
[**standing_orders_list_standing_orders**](StandingOrdersApi.md#standing_orders_list_standing_orders) | **GET** /api/v2/StandingOrders/{accountId} | Lists the standing orders for repeated transfers and payments.



## standing_orders_list_standing_orders

> crate::models::ListResultOfStandingOrder standing_orders_list_standing_orders(account_id)
Lists the standing orders for repeated transfers and payments.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | Option<**String**> | The `accountId` of the account. The account must be one owned by the customer, or an account the customer has been granted access to. An account's `accountId` can be retrieved with the list or read operation on the Accounts service. | [required] |

### Return type

[**crate::models::ListResultOfStandingOrder**](ListResultOfStandingOrder.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

