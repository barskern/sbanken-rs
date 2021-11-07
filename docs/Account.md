# Account

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**account_id** | Option<**String**> | The account identifier for use with other APIs that require AccountID. | [optional]
**account_number** | Option<**String**> | The account's public account number | [optional]
**owner_customer_id** | Option<**String**> | The customerId of the person that owns the account, may be different from the queried customerId. | [optional]
**name** | Option<**String**> | The name of the account. Not unique. | [optional]
**account_type** | Option<**String**> | The type of the account. Descriptive. | [optional]
**available** | Option<**f32**> | The amount currently available on the account. This is the money available to spend. It will differ from the account balance, typically by credit limit and by pending transactions. | [optional]
**balance** | Option<**f32**> | The current account balance. This is the money owned. | [optional]
**credit_limit** | Option<**f32**> | The current credit limit on the account. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


