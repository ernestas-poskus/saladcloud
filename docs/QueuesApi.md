# \QueuesApi

All URIs are relative to *https://api.salad.com/api/public*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_queue**](QueuesApi.md#create_queue) | **POST** /organizations/{organization_name}/projects/{project_name}/queues | Create Queue
[**create_queue_job**](QueuesApi.md#create_queue_job) | **POST** /organizations/{organization_name}/projects/{project_name}/queues/{queue_name}/jobs | Create Job
[**delete_queue**](QueuesApi.md#delete_queue) | **DELETE** /organizations/{organization_name}/projects/{project_name}/queues/{queue_name} | Delete Queue
[**delete_queue_job**](QueuesApi.md#delete_queue_job) | **DELETE** /organizations/{organization_name}/projects/{project_name}/queues/{queue_name}/jobs/{queue_job_id} | Delete Job
[**get_queue**](QueuesApi.md#get_queue) | **GET** /organizations/{organization_name}/projects/{project_name}/queues/{queue_name} | Get Queue
[**get_queue_job**](QueuesApi.md#get_queue_job) | **GET** /organizations/{organization_name}/projects/{project_name}/queues/{queue_name}/jobs/{queue_job_id} | Get Job
[**list_queue_jobs**](QueuesApi.md#list_queue_jobs) | **GET** /organizations/{organization_name}/projects/{project_name}/queues/{queue_name}/jobs | List Jobs
[**list_queues**](QueuesApi.md#list_queues) | **GET** /organizations/{organization_name}/projects/{project_name}/queues | List Queues
[**update_queue**](QueuesApi.md#update_queue) | **PATCH** /organizations/{organization_name}/projects/{project_name}/queues/{queue_name} | Update Queue



## create_queue

> models::Queue create_queue(organization_name, project_name, queue_prototype)
Create Queue

Creates a new queue in the given project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_name** | **String** | Your organization name. This identifies the billing context for the API operation and represents a security boundary for SaladCloud resources. The organization must be created before using the API, and you must be a member of the organization. | [required] |
**project_name** | **String** | Your project name. This represents a collection of related SaladCloud resources. The project must be created before using the API. | [required] |
**queue_prototype** | [**QueuePrototype**](QueuePrototype.md) |  | [required] |

### Return type

[**models::Queue**](Queue.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_queue_job

> models::QueueJob create_queue_job(organization_name, project_name, queue_name, queue_job_prototype)
Create Job

Creates a new job

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_name** | **String** | Your organization name. This identifies the billing context for the API operation and represents a security boundary for SaladCloud resources. The organization must be created before using the API, and you must be a member of the organization. | [required] |
**project_name** | **String** | Your project name. This represents a collection of related SaladCloud resources. The project must be created before using the API. | [required] |
**queue_name** | **String** | The queue name. | [required] |
**queue_job_prototype** | [**QueueJobPrototype**](QueueJobPrototype.md) |  | [required] |

### Return type

[**models::QueueJob**](QueueJob.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_queue

> delete_queue(organization_name, project_name, queue_name)
Delete Queue

Deletes an existing queue in the given project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_name** | **String** | Your organization name. This identifies the billing context for the API operation and represents a security boundary for SaladCloud resources. The organization must be created before using the API, and you must be a member of the organization. | [required] |
**project_name** | **String** | Your project name. This represents a collection of related SaladCloud resources. The project must be created before using the API. | [required] |
**queue_name** | **String** | The queue name. | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_queue_job

> delete_queue_job(organization_name, project_name, queue_name, queue_job_id)
Delete Job

Cancels a job in a queue

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_name** | **String** | Your organization name. This identifies the billing context for the API operation and represents a security boundary for SaladCloud resources. The organization must be created before using the API, and you must be a member of the organization. | [required] |
**project_name** | **String** | Your project name. This represents a collection of related SaladCloud resources. The project must be created before using the API. | [required] |
**queue_name** | **String** | The queue name. | [required] |
**queue_job_id** | **uuid::Uuid** | The job identifier. This is automatically generated and assigned when the job is created. | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_queue

> models::Queue get_queue(organization_name, project_name, queue_name)
Get Queue

Gets an existing queue in the given project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_name** | **String** | Your organization name. This identifies the billing context for the API operation and represents a security boundary for SaladCloud resources. The organization must be created before using the API, and you must be a member of the organization. | [required] |
**project_name** | **String** | Your project name. This represents a collection of related SaladCloud resources. The project must be created before using the API. | [required] |
**queue_name** | **String** | The queue name. | [required] |

### Return type

[**models::Queue**](Queue.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_queue_job

> models::QueueJob get_queue_job(organization_name, project_name, queue_name, queue_job_id)
Get Job

Gets a job in a queue

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_name** | **String** | Your organization name. This identifies the billing context for the API operation and represents a security boundary for SaladCloud resources. The organization must be created before using the API, and you must be a member of the organization. | [required] |
**project_name** | **String** | Your project name. This represents a collection of related SaladCloud resources. The project must be created before using the API. | [required] |
**queue_name** | **String** | The queue name. | [required] |
**queue_job_id** | **uuid::Uuid** | The job identifier. This is automatically generated and assigned when the job is created. | [required] |

### Return type

[**models::QueueJob**](QueueJob.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_queue_jobs

> models::QueueJobCollection list_queue_jobs(organization_name, project_name, queue_name, page, page_size)
List Jobs

Gets the list of jobs in a queue

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_name** | **String** | Your organization name. This identifies the billing context for the API operation and represents a security boundary for SaladCloud resources. The organization must be created before using the API, and you must be a member of the organization. | [required] |
**project_name** | **String** | Your project name. This represents a collection of related SaladCloud resources. The project must be created before using the API. | [required] |
**queue_name** | **String** | The queue name. | [required] |
**page** | Option<**i32**> | The page number. |  |
**page_size** | Option<**i32**> | The maximum number of items per page. |  |

### Return type

[**models::QueueJobCollection**](QueueJobCollection.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_queues

> models::QueueCollection list_queues(organization_name, project_name)
List Queues

Gets the list of queues in the given project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_name** | **String** | Your organization name. This identifies the billing context for the API operation and represents a security boundary for SaladCloud resources. The organization must be created before using the API, and you must be a member of the organization. | [required] |
**project_name** | **String** | Your project name. This represents a collection of related SaladCloud resources. The project must be created before using the API. | [required] |

### Return type

[**models::QueueCollection**](QueueCollection.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_queue

> models::Queue update_queue(organization_name, project_name, queue_name, queue_patch)
Update Queue

Updates an existing queue in the given project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_name** | **String** | Your organization name. This identifies the billing context for the API operation and represents a security boundary for SaladCloud resources. The organization must be created before using the API, and you must be a member of the organization. | [required] |
**project_name** | **String** | Your project name. This represents a collection of related SaladCloud resources. The project must be created before using the API. | [required] |
**queue_name** | **String** | The queue name. | [required] |
**queue_patch** | [**QueuePatch**](QueuePatch.md) |  | [required] |

### Return type

[**models::Queue**](Queue.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/merge-patch+json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

