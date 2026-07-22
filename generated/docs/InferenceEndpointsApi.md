# \InferenceEndpointsApi

All URIs are relative to *https://api.salad.com/api/public*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_inference_endpoint_job**](InferenceEndpointsApi.md#create_inference_endpoint_job) | **POST** /organizations/{organization_name}/inference-endpoints/{inference_endpoint_name}/jobs | Create a New Inference Endpoint Job
[**delete_inference_endpoint_job**](InferenceEndpointsApi.md#delete_inference_endpoint_job) | **DELETE** /organizations/{organization_name}/inference-endpoints/{inference_endpoint_name}/jobs/{inference_endpoint_job_id} | Cancel an Inference Endpoint Job
[**get_inference_endpoint**](InferenceEndpointsApi.md#get_inference_endpoint) | **GET** /organizations/{organization_name}/inference-endpoints/{inference_endpoint_name} | Get an Inference Endpoint
[**get_inference_endpoint_job**](InferenceEndpointsApi.md#get_inference_endpoint_job) | **GET** /organizations/{organization_name}/inference-endpoints/{inference_endpoint_name}/jobs/{inference_endpoint_job_id} | Get an Inference Endpoint Job
[**list_inference_endpoint_jobs**](InferenceEndpointsApi.md#list_inference_endpoint_jobs) | **GET** /organizations/{organization_name}/inference-endpoints/{inference_endpoint_name}/jobs | List Inference Endpoint Jobs
[**list_inference_endpoints**](InferenceEndpointsApi.md#list_inference_endpoints) | **GET** /organizations/{organization_name}/inference-endpoints | List Inference Endpoints



## create_inference_endpoint_job

> models::InferenceEndpointJob create_inference_endpoint_job(organization_name, inference_endpoint_name, inference_endpoint_job_prototype)
Create a New Inference Endpoint Job

Creates a new inference endpoint job.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_name** | **String** | Your organization name. This identifies the billing context for the API operation and represents a security boundary for SaladCloud resources. The organization must be created before using the API, and you must be a member of the organization. | [required] |
**inference_endpoint_name** | **String** | The inference endpoint name. | [required] |
**inference_endpoint_job_prototype** | [**InferenceEndpointJobPrototype**](InferenceEndpointJobPrototype.md) |  | [required] |

### Return type

[**models::InferenceEndpointJob**](InferenceEndpointJob.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_inference_endpoint_job

> delete_inference_endpoint_job(organization_name, inference_endpoint_name, inference_endpoint_job_id)
Cancel an Inference Endpoint Job

Cancels an inference endpoint job.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_name** | **String** | Your organization name. This identifies the billing context for the API operation and represents a security boundary for SaladCloud resources. The organization must be created before using the API, and you must be a member of the organization. | [required] |
**inference_endpoint_name** | **String** | The inference endpoint name. | [required] |
**inference_endpoint_job_id** | **uuid::Uuid** | The inference endpoint job identifier. | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_inference_endpoint

> models::InferenceEndpoint get_inference_endpoint(organization_name, inference_endpoint_name)
Get an Inference Endpoint

Gets an inference endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_name** | **String** | Your organization name. This identifies the billing context for the API operation and represents a security boundary for SaladCloud resources. The organization must be created before using the API, and you must be a member of the organization. | [required] |
**inference_endpoint_name** | **String** | The inference endpoint name. | [required] |

### Return type

[**models::InferenceEndpoint**](InferenceEndpoint.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_inference_endpoint_job

> models::InferenceEndpointJob get_inference_endpoint_job(organization_name, inference_endpoint_name, inference_endpoint_job_id)
Get an Inference Endpoint Job

Gets an inference endpoint job.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_name** | **String** | Your organization name. This identifies the billing context for the API operation and represents a security boundary for SaladCloud resources. The organization must be created before using the API, and you must be a member of the organization. | [required] |
**inference_endpoint_name** | **String** | The inference endpoint name. | [required] |
**inference_endpoint_job_id** | **uuid::Uuid** | The inference endpoint job identifier. | [required] |

### Return type

[**models::InferenceEndpointJob**](InferenceEndpointJob.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_inference_endpoint_jobs

> models::InferenceEndpointJobCollection list_inference_endpoint_jobs(organization_name, inference_endpoint_name, page, page_size)
List Inference Endpoint Jobs

Lists inference endpoint jobs.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_name** | **String** | Your organization name. This identifies the billing context for the API operation and represents a security boundary for SaladCloud resources. The organization must be created before using the API, and you must be a member of the organization. | [required] |
**inference_endpoint_name** | **String** | The inference endpoint name. | [required] |
**page** | Option<**i32**> | The page number. |  |
**page_size** | Option<**i32**> | The maximum number of items per page. |  |

### Return type

[**models::InferenceEndpointJobCollection**](InferenceEndpointJobCollection.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_inference_endpoints

> models::InferenceEndpointCollection list_inference_endpoints(organization_name, page, page_size)
List Inference Endpoints

Lists inference endpoints.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_name** | **String** | Your organization name. This identifies the billing context for the API operation and represents a security boundary for SaladCloud resources. The organization must be created before using the API, and you must be a member of the organization. | [required] |
**page** | Option<**i32**> | The page number. |  |
**page_size** | Option<**i32**> | The maximum number of items per page. |  |

### Return type

[**models::InferenceEndpointCollection**](InferenceEndpointCollection.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

