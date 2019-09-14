# \ScheduledTransactionsApi

All URIs are relative to *https://api.youneedabudget.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_scheduled_transaction_by_id**](ScheduledTransactionsApi.md#get_scheduled_transaction_by_id) | **get** /budgets/{budget_id}/scheduled_transactions/{scheduled_transaction_id} | Single scheduled transaction
[**get_scheduled_transactions**](ScheduledTransactionsApi.md#get_scheduled_transactions) | **get** /budgets/{budget_id}/scheduled_transactions | List scheduled transactions



## get_scheduled_transaction_by_id

> crate::models::ScheduledTransactionResponse get_scheduled_transaction_by_id(budget_id, scheduled_transaction_id)
Single scheduled transaction

Returns a single scheduled transaction

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**budget_id** | **String** | The id of the budget (\"last-used\" can be used to specify the last used budget and \"default\" can be used if default budget selection is enabled (see: https://api.youneedabudget.com/#oauth-default-budget) | Required | 
**scheduled_transaction_id** | **String** | The id of the scheduled transaction | Required | 

### Return type

[**crate::models::ScheduledTransactionResponse**](ScheduledTransactionResponse.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_scheduled_transactions

> crate::models::ScheduledTransactionsResponse get_scheduled_transactions(budget_id, last_knowledge_of_server)
List scheduled transactions

Returns all scheduled transactions

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**budget_id** | **String** | The id of the budget (\"last-used\" can be used to specify the last used budget and \"default\" can be used if default budget selection is enabled (see: https://api.youneedabudget.com/#oauth-default-budget) | Required | 
**last_knowledge_of_server** | **i64** | The starting server knowledge.  If provided, only entities that have changed since last_knowledge_of_server will be included. |  | 

### Return type

[**crate::models::ScheduledTransactionsResponse**](ScheduledTransactionsResponse.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

