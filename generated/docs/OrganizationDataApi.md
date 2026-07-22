# \OrganizationDataApi

All URIs are relative to *https://api.salad.com/api/public*

Method | HTTP request | Description
------------- | ------------- | -------------
[**list_gpu_classes**](OrganizationDataApi.md#list_gpu_classes) | **GET** /organizations/{organization_name}/gpu-classes | List the GPU Classes



## list_gpu_classes

> models::GpuClassesList list_gpu_classes(organization_name)
List the GPU Classes

List the GPU Classes

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_name** | **String** | Your organization name. This identifies the billing context for the API operation and represents a security boundary for SaladCloud resources. The organization must be created before using the API, and you must be a member of the organization. | [required] |

### Return type

[**models::GpuClassesList**](GpuClassesList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

