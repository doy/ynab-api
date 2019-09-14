# \PayeesApi

All URIs are relative to *https://api.youneedabudget.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_payee_by_id**](PayeesApi.md#get_payee_by_id) | **get** /budgets/{budget_id}/payees/{payee_id} | Single payee
[**get_payees**](PayeesApi.md#get_payees) | **get** /budgets/{budget_id}/payees | List payees



## get_payee_by_id

> crate::models::PayeeResponse get_payee_by_id(budget_id, payee_id)
Single payee

Returns single payee

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**budget_id** | **String** | The id of the budget (\"last-used\" can be used to specify the last used budget and \"default\" can be used if default budget selection is enabled (see: https://api.youneedabudget.com/#oauth-default-budget) | Required | 
**payee_id** | **String** | The id of the payee | Required | 

### Return type

[**crate::models::PayeeResponse**](PayeeResponse.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_payees

> crate::models::PayeesResponse get_payees(budget_id, last_knowledge_of_server)
List payees

Returns all payees

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**budget_id** | **String** | The id of the budget (\"last-used\" can be used to specify the last used budget and \"default\" can be used if default budget selection is enabled (see: https://api.youneedabudget.com/#oauth-default-budget) | Required | 
**last_knowledge_of_server** | **i64** | The starting server knowledge.  If provided, only entities that have changed since last_knowledge_of_server will be included. |  | 

### Return type

[**crate::models::PayeesResponse**](PayeesResponse.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

