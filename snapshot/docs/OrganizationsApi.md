# \OrganizationsApi

All URIs are relative to *https://api.salad.com/api/public*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_cpu_availability**](OrganizationsApi.md#get_cpu_availability) | **POST** /organizations/{organization_name}/availability/sce-cpu-availability | Get CPU Availability
[**get_gpu_availability**](OrganizationsApi.md#get_gpu_availability) | **POST** /organizations/{organization_name}/availability/sce-gpu-availability | Get GPU Availability



## get_cpu_availability

> models::CpuAvailability get_cpu_availability(organization_name, cpu_availability_prototype)
Get CPU Availability

Gets the CPU availability for the given organization

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_name** | **String** | Your organization name. This identifies the billing context for the API operation and represents a security boundary for SaladCloud resources. The organization must be created before using the API, and you must be a member of the organization. | [required] |
**cpu_availability_prototype** | [**CpuAvailabilityPrototype**](CpuAvailabilityPrototype.md) | Represents a request to check CPU availability | [required] |

### Return type

[**models::CpuAvailability**](CpuAvailability.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_gpu_availability

> models::GpuAvailability get_gpu_availability(organization_name, gpu_availability_prototype)
Get GPU Availability

Gets the GPU availability for the given organization

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_name** | **String** | Your organization name. This identifies the billing context for the API operation and represents a security boundary for SaladCloud resources. The organization must be created before using the API, and you must be a member of the organization. | [required] |
**gpu_availability_prototype** | [**GpuAvailabilityPrototype**](GpuAvailabilityPrototype.md) | Represents a request to check GPU availability | [required] |

### Return type

[**models::GpuAvailability**](GpuAvailability.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

