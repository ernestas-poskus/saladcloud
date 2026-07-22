# \SystemLogsApi

All URIs are relative to *https://api.salad.com/api/public*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_system_logs**](SystemLogsApi.md#get_system_logs) | **GET** /organizations/{organization_name}/projects/{project_name}/containers/{container_group_name}/system-logs | Get System Logs



## get_system_logs

> models::SystemLogList get_system_logs(organization_name, project_name, container_group_name)
Get System Logs

Gets the System Logs

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_name** | **String** | Your organization name. This identifies the billing context for the API operation and represents a security boundary for SaladCloud resources. The organization must be created before using the API, and you must be a member of the organization. | [required] |
**project_name** | **String** | Your project name. This represents a collection of related SaladCloud resources. The project must be created before using the API. | [required] |
**container_group_name** | **String** | The unique container group name | [required] |

### Return type

[**models::SystemLogList**](SystemLogList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

