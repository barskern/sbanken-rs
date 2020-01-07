# \EFakturasApi

All URIs are relative to *https://api.sbanken.no*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_e_faktura**](EFakturasApi.md#get_e_faktura) | **GET** /exec.bank/api/v1/EFakturas/{eFakturaId} | Read an eFaktura.
[**list_e_fakturas**](EFakturasApi.md#list_e_fakturas) | **GET** /exec.bank/api/v1/EFakturas | List eFakturas.
[**list_new_e_fakturas**](EFakturasApi.md#list_new_e_fakturas) | **GET** /exec.bank/api/v1/EFakturas/new | List eFakturas that have not yet been processed by the customer. These are considered \"new\".
[**pay_e_faktura**](EFakturasApi.md#pay_e_faktura) | **POST** /exec.bank/api/v1/EFakturas | Pay an eFaktura.



## get_e_faktura

> crate::models::ItemResultEFakturaV1 get_e_faktura(e_faktura_id, customer_id)
Read an eFaktura.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**e_faktura_id** | **String** | The eFakturaId. | [required] |
**customer_id** | Option<**String**> | The customerId of the customer. |  |

### Return type

[**crate::models::ItemResultEFakturaV1**](ItemResult.EFaktura.v1.md)

### Authorization

[sbanken](../README.md#sbanken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_e_fakturas

> crate::models::ListResultEFakturaV1 list_e_fakturas(customer_id, status, start_date, end_date, index, length)
List eFakturas.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**customer_id** | Option<**String**> | The customerId of the customer. |  |
**status** | Option<**String**> | Optional. Filter the result to include items with this status: . Default value is ALL. |  |
**start_date** | Option<**String**> | Optional. Filter the result to include items with due date on or after this date. Default value is 30 days before today's date. |  |
**end_date** | Option<**String**> | Optional. Filter the result to include items with due date on or before this date. Default value is today's date. |  |
**index** | Option<**i32**> | Optional. Return items with this index or greater. |  |
**length** | Option<**i32**> | Optional. Return items up to this number. |  |

### Return type

[**crate::models::ListResultEFakturaV1**](ListResult.EFaktura.v1.md)

### Authorization

[sbanken](../README.md#sbanken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_new_e_fakturas

> crate::models::ListResultEFakturaSimpleV1 list_new_e_fakturas(customer_id, index, length)
List eFakturas that have not yet been processed by the customer. These are considered \"new\".

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**customer_id** | Option<**String**> | The customerId of the customer. |  |
**index** | Option<**i32**> | Optional. Return items with this index or greater. |  |
**length** | Option<**i32**> | Optional. Return items up to this number. |  |

### Return type

[**crate::models::ListResultEFakturaSimpleV1**](ListResult.EFakturaSimple.v1.md)

### Authorization

[sbanken](../README.md#sbanken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## pay_e_faktura

> crate::models::NoResult pay_e_faktura(customer_id, e_faktura_pay_request_v1)
Pay an eFaktura.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**customer_id** | Option<**String**> | The customerId of the customer. |  |
**e_faktura_pay_request_v1** | Option<[**EFakturaPayRequestV1**](EFakturaPayRequestV1.md)> | The eFaktura payment details. |  |

### Return type

[**crate::models::NoResult**](NoResult..md)

### Authorization

[sbanken](../README.md#sbanken)

### HTTP request headers

- **Content-Type**: application/json-patch+json, application/json, text/json, application/_*+json
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

