# Transaction

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**accounting_date** | **String** |  | 
**interest_date** | Option<**String**> |  | [optional]
**other_account_number** | Option<**String**> |  | [optional]
**other_account_number_specified** | **bool** |  | 
**amount** | **f32** |  | 
**text** | Option<**String**> |  | [optional]
**transaction_type** | Option<**String**> |  | [optional]
**transaction_type_code** | **i32** |  | 
**transaction_type_text** | Option<**String**> |  | [optional]
**is_reservation** | **bool** |  | 
**reservation_type** | Option<[**crate::models::ReservationType**](ReservationType.md)> |  | [optional]
**source** | [**crate::models::SourceType**](SourceType.md) |  | 
**card_details_specified** | **bool** |  | 
**card_details** | Option<[**crate::models::CardDetails**](CardDetails.md)> |  | [optional]
**transaction_detail_specified** | **bool** |  | 
**transaction_detail** | Option<[**crate::models::TransactionDetail**](TransactionDetail.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


