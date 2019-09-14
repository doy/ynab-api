# BudgetDetail

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** |  | 
**name** | **String** |  | 
**last_modified_on** | **String** | The last time any changes were made to the budget from either a web or mobile client | [optional] 
**first_month** | [***String**](string.md) | The earliest budget month | [optional] 
**last_month** | [***String**](string.md) | The latest budget month | [optional] 
**date_format** | [***crate::models::DateFormat**](DateFormat.md) |  | [optional] 
**currency_format** | [***crate::models::CurrencyFormat**](CurrencyFormat.md) |  | [optional] 
**accounts** | [**Vec<crate::models::Account>**](Account.md) |  | [optional] 
**payees** | [**Vec<crate::models::Payee>**](Payee.md) |  | [optional] 
**payee_locations** | [**Vec<crate::models::PayeeLocation>**](PayeeLocation.md) |  | [optional] 
**category_groups** | [**Vec<crate::models::CategoryGroup>**](CategoryGroup.md) |  | [optional] 
**categories** | [**Vec<crate::models::Category>**](Category.md) |  | [optional] 
**months** | [**Vec<crate::models::MonthDetail>**](MonthDetail.md) |  | [optional] 
**transactions** | [**Vec<crate::models::TransactionSummary>**](TransactionSummary.md) |  | [optional] 
**subtransactions** | [**Vec<crate::models::SubTransaction>**](SubTransaction.md) |  | [optional] 
**scheduled_transactions** | [**Vec<crate::models::ScheduledTransactionSummary>**](ScheduledTransactionSummary.md) |  | [optional] 
**scheduled_subtransactions** | [**Vec<crate::models::ScheduledSubTransaction>**](ScheduledSubTransaction.md) |  | [optional] 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


