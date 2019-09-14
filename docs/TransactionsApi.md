# \TransactionsApi

All URIs are relative to *https://api.youneedabudget.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_transaction**](TransactionsApi.md#create_transaction) | **post** /budgets/{budget_id}/transactions | Create a single transaction or multiple transactions
[**get_transaction_by_id**](TransactionsApi.md#get_transaction_by_id) | **get** /budgets/{budget_id}/transactions/{transaction_id} | Single transaction
[**get_transactions**](TransactionsApi.md#get_transactions) | **get** /budgets/{budget_id}/transactions | List transactions
[**get_transactions_by_account**](TransactionsApi.md#get_transactions_by_account) | **get** /budgets/{budget_id}/accounts/{account_id}/transactions | List account transactions
[**get_transactions_by_category**](TransactionsApi.md#get_transactions_by_category) | **get** /budgets/{budget_id}/categories/{category_id}/transactions | List category transactions
[**get_transactions_by_payee**](TransactionsApi.md#get_transactions_by_payee) | **get** /budgets/{budget_id}/payees/{payee_id}/transactions | List payee transactions
[**update_transaction**](TransactionsApi.md#update_transaction) | **put** /budgets/{budget_id}/transactions/{transaction_id} | Updates an existing transaction
[**update_transactions**](TransactionsApi.md#update_transactions) | **patch** /budgets/{budget_id}/transactions | Update multiple transactions



## create_transaction

> crate::models::SaveTransactionsResponse create_transaction(budget_id, data)
Create a single transaction or multiple transactions

Creates a single transaction or multiple transactions.  If you provide a body containing a 'transaction' object, a single transaction will be created and if you provide a body containing a 'transactions' array, multiple transactions will be created.  Scheduled transactions cannot be created on this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**budget_id** | **String** | The id of the budget (\"last-used\" can be used to specify the last used budget and \"default\" can be used if default budget selection is enabled (see: https://api.youneedabudget.com/#oauth-default-budget) | Required | 
**data** | [**SaveTransactionsWrapper**](SaveTransactionsWrapper.md) | The transaction or transactions to create.  To create a single transaction you can specify a value for the 'transaction' object and to create multiple transactions you can specify an array of 'transactions'.  It is expected that you will only provide a value for one of these objects. | Required | 

### Return type

[**crate::models::SaveTransactionsResponse**](SaveTransactionsResponse.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_transaction_by_id

> crate::models::TransactionResponse get_transaction_by_id(budget_id, transaction_id)
Single transaction

Returns a single transaction

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**budget_id** | **String** | The id of the budget (\"last-used\" can be used to specify the last used budget and \"default\" can be used if default budget selection is enabled (see: https://api.youneedabudget.com/#oauth-default-budget) | Required | 
**transaction_id** | **String** | The id of the transaction | Required | 

### Return type

[**crate::models::TransactionResponse**](TransactionResponse.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_transactions

> crate::models::TransactionsResponse get_transactions(budget_id, since_date, _type, last_knowledge_of_server)
List transactions

Returns budget transactions

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**budget_id** | **String** | The id of the budget (\"last-used\" can be used to specify the last used budget and \"default\" can be used if default budget selection is enabled (see: https://api.youneedabudget.com/#oauth-default-budget) | Required | 
**since_date** | **String** | If specified, only transactions on or after this date will be included.  The date should be ISO formatted (e.g. 2016-12-30). |  | 
**_type** | **String** | If specified, only transactions of the specified type will be included. 'uncategorized' and 'unapproved' are currently supported. |  | 
**last_knowledge_of_server** | **i64** | The starting server knowledge.  If provided, only entities that have changed since last_knowledge_of_server will be included. |  | 

### Return type

[**crate::models::TransactionsResponse**](TransactionsResponse.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_transactions_by_account

> crate::models::TransactionsResponse get_transactions_by_account(budget_id, account_id, since_date, _type, last_knowledge_of_server)
List account transactions

Returns all transactions for a specified account

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**budget_id** | **String** | The id of the budget (\"last-used\" can be used to specify the last used budget and \"default\" can be used if default budget selection is enabled (see: https://api.youneedabudget.com/#oauth-default-budget) | Required | 
**account_id** | **String** | The id of the account | Required | 
**since_date** | **String** | If specified, only transactions on or after this date will be included.  The date should be ISO formatted (e.g. 2016-12-30). |  | 
**_type** | **String** | If specified, only transactions of the specified type will be included. 'uncategorized' and 'unapproved' are currently supported. |  | 
**last_knowledge_of_server** | **i64** | The starting server knowledge.  If provided, only entities that have changed since last_knowledge_of_server will be included. |  | 

### Return type

[**crate::models::TransactionsResponse**](TransactionsResponse.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_transactions_by_category

> crate::models::HybridTransactionsResponse get_transactions_by_category(budget_id, category_id, since_date, _type, last_knowledge_of_server)
List category transactions

Returns all transactions for a specified category

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**budget_id** | **String** | The id of the budget (\"last-used\" can be used to specify the last used budget and \"default\" can be used if default budget selection is enabled (see: https://api.youneedabudget.com/#oauth-default-budget) | Required | 
**category_id** | **String** | The id of the category | Required | 
**since_date** | **String** | If specified, only transactions on or after this date will be included.  The date should be ISO formatted (e.g. 2016-12-30). |  | 
**_type** | **String** | If specified, only transactions of the specified type will be included. 'uncategorized' and 'unapproved' are currently supported. |  | 
**last_knowledge_of_server** | **i64** | The starting server knowledge.  If provided, only entities that have changed since last_knowledge_of_server will be included. |  | 

### Return type

[**crate::models::HybridTransactionsResponse**](HybridTransactionsResponse.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_transactions_by_payee

> crate::models::HybridTransactionsResponse get_transactions_by_payee(budget_id, payee_id, since_date, _type, last_knowledge_of_server)
List payee transactions

Returns all transactions for a specified payee

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**budget_id** | **String** | The id of the budget (\"last-used\" can be used to specify the last used budget and \"default\" can be used if default budget selection is enabled (see: https://api.youneedabudget.com/#oauth-default-budget) | Required | 
**payee_id** | **String** | The id of the payee | Required | 
**since_date** | **String** | If specified, only transactions on or after this date will be included.  The date should be ISO formatted (e.g. 2016-12-30). |  | 
**_type** | **String** | If specified, only transactions of the specified type will be included. 'uncategorized' and 'unapproved' are currently supported. |  | 
**last_knowledge_of_server** | **i64** | The starting server knowledge.  If provided, only entities that have changed since last_knowledge_of_server will be included. |  | 

### Return type

[**crate::models::HybridTransactionsResponse**](HybridTransactionsResponse.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_transaction

> crate::models::TransactionResponse update_transaction(budget_id, transaction_id, data)
Updates an existing transaction

Updates a transaction

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**budget_id** | **String** | The id of the budget (\"last-used\" can be used to specify the last used budget and \"default\" can be used if default budget selection is enabled (see: https://api.youneedabudget.com/#oauth-default-budget) | Required | 
**transaction_id** | **String** | The id of the transaction | Required | 
**data** | [**SaveTransactionWrapper**](SaveTransactionWrapper.md) | The transaction to update | Required | 

### Return type

[**crate::models::TransactionResponse**](TransactionResponse.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_transactions

> crate::models::SaveTransactionsResponse update_transactions(budget_id, data)
Update multiple transactions

Updates multiple transactions, by 'id' or 'import_id'.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**budget_id** | **String** | The id of the budget (\"last-used\" can be used to specify the last used budget and \"default\" can be used if default budget selection is enabled (see: https://api.youneedabudget.com/#oauth-default-budget) | Required | 
**data** | [**UpdateTransactionsWrapper**](UpdateTransactionsWrapper.md) | The transactions to update. Each transaction must have either an 'id' or 'import_id' specified. If 'id' is specified as null an 'import_id' value can be provided which will allow transaction(s) to be updated by their import_id. If an id is specified, it will always be used for lookup. | Required | 

### Return type

[**crate::models::SaveTransactionsResponse**](SaveTransactionsResponse.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

