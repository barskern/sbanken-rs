# \CardsApi

All URIs are relative to *https://api.sbanken.no*

Method | HTTP request | Description
------------- | ------------- | -------------
[**list_cards**](CardsApi.md#list_cards) | **GET** /exec.bank/api/v1/Cards | Lists the cards.



## list_cards

> crate::models::ListResultCardV1 list_cards(customer_id)
Lists the cards.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**customer_id** | Option<**String**> | The customerId of the customer. |  |

### Return type

[**crate::models::ListResultCardV1**](ListResult.Card.v1.md)

### Authorization

[sbanken](../README.md#sbanken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

