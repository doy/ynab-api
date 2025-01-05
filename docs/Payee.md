# Payee

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**name** | **String** |  | 
**transfer_account_id** | Option<**String**> | If a transfer payee, the `account_id` to which this payee transfers to | [optional]
**deleted** | **bool** | Whether or not the payee has been deleted.  Deleted payees will only be included in delta requests. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


