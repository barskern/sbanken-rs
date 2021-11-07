# \EfakturaApi

All URIs are relative to *https://https://publicapi.sbanken.no/apibeta*

Method | HTTP request | Description
------------- | ------------- | -------------
[**efaktura_list**](EfakturaApi.md#efaktura_list) | **GET** /api/v2/Efaktura | List eFakturas.
[**efaktura_list_new**](EfakturaApi.md#efaktura_list_new) | **GET** /api/v2/Efaktura/new | List eFakturas that have not yet been processed by the customer. These are considered \"new\".
[**efaktura_pay**](EfakturaApi.md#efaktura_pay) | **POST** /api/v2/Efaktura | Pay an eFaktura.
[**efaktura_read**](EfakturaApi.md#efaktura_read) | **GET** /api/v2/Efaktura/{eFakturaId} | Read an eFaktura.



## efaktura_list

> crate::models::ListResultOfEFaktura efaktura_list(status, start_date, end_date, index, length)
List eFakturas.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**status** | Option<**String**> | *Optional*. Filter the result to include items with this status: . Default value is ALL. |  |
**start_date** | Option<**String**> | *Optional*. Filter the result to include items with due date on or after this date. Default value is 30 days before today's date. |  |
**end_date** | Option<**String**> | *Optional*. Filter the result to include items with due date on or before this date. Default value is today's date. |  |
**index** | Option<**i32**> | *Optional*. Return items with this `index` or greater. Minimum value is `0`, which is the first item within the query time span. Default value is `0`. |  |
**length** | Option<**i32**> | *Optional*. Return a number of items items up to this value. Minimum value is `1`, maximum value is `1000`. The default value is `100`. |  |

### Return type

[**crate::models::ListResultOfEFaktura**](ListResultOfEFaktura.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## efaktura_list_new

> crate::models::ListResultOfEFakturaSimple efaktura_list_new(index, length)
List eFakturas that have not yet been processed by the customer. These are considered \"new\".

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**index** | Option<**i32**> | *Optional*. Return items with this `index` or greater. Minimum value is `0`, which is the first item within the query time span. Default value is `0`. |  |
**length** | Option<**i32**> | *Optional*. Return a number of items items up to this value. Minimum value is `1`, maximum value is `1000`. The default value is `100`. |  |

### Return type

[**crate::models::ListResultOfEFakturaSimple**](ListResultOfEFakturaSimple.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## efaktura_pay

> std::path::PathBuf efaktura_pay(value)
Pay an eFaktura.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**value** | [**EFakturaPayRequest**](EFakturaPayRequest.md) | The eFaktura payment details. The fields are as follows:\\             \\             **efakturaId:** The `efakturaId` of the eFaktura item to be processed. The `efakturaId` can be retrieved with the list operation on this service.\\             \\             **accountId:** The `accountId` of the account from which the eFaktura is to be paid, i.e. the debit account. The account must be one owned by the customer, or an account the customer has been granted access to. An account's `accountId` can be retrieved with the list or read operation on the Accounts service.\\             \\             **payOnlyMinimumAmount:** Set to `true` if only the minimum amount of the eFaktura is to be paid. Set to `false` if the full amount is to be paid.              | [required] |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, text/json, application/_*+json
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## efaktura_read

> crate::models::EFaktura efaktura_read(e_faktura_id)
Read an eFaktura.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**e_faktura_id** | Option<**String**> | The `eFakturaId`. The `efakturaId` can be retrieved with the list operation on this service. | [required] |

### Return type

[**crate::models::EFaktura**](EFaktura.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

