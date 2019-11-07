# BudgetDetail

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** |  | 
**name** | **String** |  | 
**last_modified_on** | Option<**String**> | The last time any changes were made to the budget from either a web or mobile client | [optional]
**first_month** | Option<[**String**](string.md)> | The earliest budget month | [optional]
**last_month** | Option<[**String**](string.md)> | The latest budget month | [optional]
**date_format** | Option<[**crate::models::DateFormat**](DateFormat.md)> |  | [optional]
**currency_format** | Option<[**crate::models::CurrencyFormat**](CurrencyFormat.md)> |  | [optional]
**accounts** | Option<[**Vec<crate::models::Account>**](Account.md)> |  | [optional]
**payees** | Option<[**Vec<crate::models::Payee>**](Payee.md)> |  | [optional]
**payee_locations** | Option<[**Vec<crate::models::PayeeLocation>**](PayeeLocation.md)> |  | [optional]
**category_groups** | Option<[**Vec<crate::models::CategoryGroup>**](CategoryGroup.md)> |  | [optional]
**categories** | Option<[**Vec<crate::models::Category>**](Category.md)> |  | [optional]
**months** | Option<[**Vec<crate::models::MonthDetail>**](MonthDetail.md)> |  | [optional]
**transactions** | Option<[**Vec<crate::models::TransactionSummary>**](TransactionSummary.md)> |  | [optional]
**subtransactions** | Option<[**Vec<crate::models::SubTransaction>**](SubTransaction.md)> |  | [optional]
**scheduled_transactions** | Option<[**Vec<crate::models::ScheduledTransactionSummary>**](ScheduledTransactionSummary.md)> |  | [optional]
**scheduled_subtransactions** | Option<[**Vec<crate::models::ScheduledSubTransaction>**](ScheduledSubTransaction.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


