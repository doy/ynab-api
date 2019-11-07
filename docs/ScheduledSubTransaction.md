# ScheduledSubTransaction

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** |  | 
**scheduled_transaction_id** | **String** |  | 
**amount** | **i64** | The scheduled subtransaction amount in milliunits format | 
**memo** | Option<**String**> |  | [optional]
**payee_id** | Option<**String**> |  | [optional]
**category_id** | Option<**String**> |  | [optional]
**transfer_account_id** | Option<**String**> | If a transfer, the account_id which the scheduled subtransaction transfers to | [optional]
**deleted** | **bool** | Whether or not the scheduled subtransaction has been deleted.  Deleted scheduled subtransactions will only be included in delta requests. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


