# BudgetDetail

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** |  | 
**name** | **String** |  | 
**last_modified_on** | **String** | The last time any changes were made to the budget from either a web or mobile client | [optional] 
**first_month** | [***String**](string.md) | The earliest budget month | [optional] 
**last_month** | [***String**](string.md) | The latest budget month | [optional] 
**date_format** | [***::models::DateFormat**](DateFormat.md) |  | [optional] 
**currency_format** | [***::models::CurrencyFormat**](CurrencyFormat.md) |  | [optional] 
**accounts** | [**Vec<::models::Account>**](Account.md) |  | [optional] 
**payees** | [**Vec<::models::Payee>**](Payee.md) |  | [optional] 
**payee_locations** | [**Vec<::models::PayeeLocation>**](PayeeLocation.md) |  | [optional] 
**category_groups** | [**Vec<::models::CategoryGroup>**](CategoryGroup.md) |  | [optional] 
**categories** | [**Vec<::models::Category>**](Category.md) |  | [optional] 
**months** | [**Vec<::models::MonthDetail>**](MonthDetail.md) |  | [optional] 
**transactions** | [**Vec<::models::TransactionSummary>**](TransactionSummary.md) |  | [optional] 
**subtransactions** | [**Vec<::models::SubTransaction>**](SubTransaction.md) |  | [optional] 
**scheduled_transactions** | [**Vec<::models::ScheduledTransactionSummary>**](ScheduledTransactionSummary.md) |  | [optional] 
**scheduled_subtransactions** | [**Vec<::models::ScheduledSubTransaction>**](ScheduledSubTransaction.md) |  | [optional] 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


