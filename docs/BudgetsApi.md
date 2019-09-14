# \BudgetsApi

All URIs are relative to *https://api.youneedabudget.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_budget_by_id**](BudgetsApi.md#get_budget_by_id) | **get** /budgets/{budget_id} | Single budget
[**get_budget_settings_by_id**](BudgetsApi.md#get_budget_settings_by_id) | **get** /budgets/{budget_id}/settings | Budget Settings
[**get_budgets**](BudgetsApi.md#get_budgets) | **get** /budgets | List budgets



## get_budget_by_id

> crate::models::BudgetDetailResponse get_budget_by_id(budget_id, last_knowledge_of_server)
Single budget

Returns a single budget with all related entities.  This resource is effectively a full budget export.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**budget_id** | **String** | The id of the budget (\"last-used\" can be used to specify the last used budget and \"default\" can be used if default budget selection is enabled (see: https://api.youneedabudget.com/#oauth-default-budget) | Required | 
**last_knowledge_of_server** | **i64** | The starting server knowledge.  If provided, only entities that have changed since last_knowledge_of_server will be included. |  | 

### Return type

[**crate::models::BudgetDetailResponse**](BudgetDetailResponse.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_budget_settings_by_id

> crate::models::BudgetSettingsResponse get_budget_settings_by_id(budget_id)
Budget Settings

Returns settings for a budget

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**budget_id** | **String** | The id of the budget (\"last-used\" can be used to specify the last used budget and \"default\" can be used if default budget selection is enabled (see: https://api.youneedabudget.com/#oauth-default-budget) | Required | 

### Return type

[**crate::models::BudgetSettingsResponse**](BudgetSettingsResponse.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_budgets

> crate::models::BudgetSummaryResponse get_budgets()
List budgets

Returns budgets list with summary information

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::BudgetSummaryResponse**](BudgetSummaryResponse.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

