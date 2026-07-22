# \LogsApi

All URIs are relative to *https://api.salad.com/api/public*

Method | HTTP request | Description
------------- | ------------- | -------------
[**query_log_entries**](LogsApi.md#query_log_entries) | **POST** /organizations/{organization_name}/log-entries | Query Log Entries



## query_log_entries

> models::LogEntryCollection query_log_entries(organization_name, log_entry_query)
Query Log Entries

Retrieve a collection of _log entries_ for the _organization_ identified by `{organization_name}` matching the log query.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_name** | **String** | Your organization name. This identifies the billing context for the API operation and represents a security boundary for SaladCloud resources. The organization must be created before using the API, and you must be a member of the organization. | [required] |
**log_entry_query** | [**LogEntryQuery**](LogEntryQuery.md) |  | [required] |

### Return type

[**models::LogEntryCollection**](LogEntryCollection.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

