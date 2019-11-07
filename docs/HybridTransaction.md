# HybridTransaction

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** |  | 
**date** | [**String**](string.md) | The transaction date in ISO format (e.g. 2016-12-01) | 
**amount** | **i64** | The transaction amount in milliunits format | 
**memo** | Option<**String**> |  | [optional]
**cleared** | **String** | The cleared status of the transaction | 
**approved** | **bool** | Whether or not the transaction is approved | 
**flag_color** | Option<**String**> | The transaction flag | [optional]
**account_id** | **String** |  | 
**payee_id** | Option<**String**> |  | [optional]
**category_id** | Option<**String**> |  | [optional]
**transfer_account_id** | Option<**String**> | If a transfer transaction, the account to which it transfers | [optional]
**transfer_transaction_id** | Option<**String**> | If a transfer transaction, the id of transaction on the other side of the transfer | [optional]
**matched_transaction_id** | Option<**String**> | If transaction is matched, the id of the matched transaction | [optional]
**import_id** | Option<**String**> | If the Transaction was imported, this field is a unique (by account) import identifier.  If this transaction was imported through File Based Import or Direct Import and not through the API, the import_id will have the format: 'YNAB:[milliunit_amount]:[iso_date]:[occurrence]'.  For example, a transaction dated 2015-12-30 in the amount of -$294.23 USD would have an import_id of 'YNAB:-294230:2015-12-30:1'.  If a second transaction on the same account was imported and had the same date and same amount, its import_id would be 'YNAB:-294230:2015-12-30:2'. | [optional]
**deleted** | **bool** | Whether or not the transaction has been deleted.  Deleted transactions will only be included in delta requests. | 
**_type** | **String** | Whether the hybrid transaction represents a regular transaction or a subtransaction | 
**parent_transaction_id** | Option<**String**> | For subtransaction types, this is the id of the parent transaction.  For transaction types, this id will be always be null. | [optional]
**account_name** | **String** |  | 
**payee_name** | Option<**String**> |  | [optional]
**category_name** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


