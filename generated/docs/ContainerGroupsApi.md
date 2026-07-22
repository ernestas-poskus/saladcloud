# \ContainerGroupsApi

All URIs are relative to *https://api.salad.com/api/public*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_container_group**](ContainerGroupsApi.md#create_container_group) | **POST** /organizations/{organization_name}/projects/{project_name}/containers | Create Container Group
[**delete_container_group**](ContainerGroupsApi.md#delete_container_group) | **DELETE** /organizations/{organization_name}/projects/{project_name}/containers/{container_group_name} | Delete Container Group
[**get_container_group**](ContainerGroupsApi.md#get_container_group) | **GET** /organizations/{organization_name}/projects/{project_name}/containers/{container_group_name} | Get Container Group
[**get_container_group_instance**](ContainerGroupsApi.md#get_container_group_instance) | **GET** /organizations/{organization_name}/projects/{project_name}/containers/{container_group_name}/instances/{container_group_instance_id} | Get Container Group Instance
[**list_container_group_instances**](ContainerGroupsApi.md#list_container_group_instances) | **GET** /organizations/{organization_name}/projects/{project_name}/containers/{container_group_name}/instances | List Container Group Instances
[**list_container_groups**](ContainerGroupsApi.md#list_container_groups) | **GET** /organizations/{organization_name}/projects/{project_name}/containers | List Container Groups
[**reallocate_container_group_instance**](ContainerGroupsApi.md#reallocate_container_group_instance) | **POST** /organizations/{organization_name}/projects/{project_name}/containers/{container_group_name}/instances/{container_group_instance_id}/reallocate | Reallocate Container Group Instance
[**recreate_container_group_instance**](ContainerGroupsApi.md#recreate_container_group_instance) | **POST** /organizations/{organization_name}/projects/{project_name}/containers/{container_group_name}/instances/{container_group_instance_id}/recreate | Recreate Container Group Instance
[**restart_container_group_instance**](ContainerGroupsApi.md#restart_container_group_instance) | **POST** /organizations/{organization_name}/projects/{project_name}/containers/{container_group_name}/instances/{container_group_instance_id}/restart | Restart container Group Instance
[**start_container_group**](ContainerGroupsApi.md#start_container_group) | **POST** /organizations/{organization_name}/projects/{project_name}/containers/{container_group_name}/start | Start Container Group
[**stop_container_group**](ContainerGroupsApi.md#stop_container_group) | **POST** /organizations/{organization_name}/projects/{project_name}/containers/{container_group_name}/stop | Stop Container Group
[**update_container_group**](ContainerGroupsApi.md#update_container_group) | **PATCH** /organizations/{organization_name}/projects/{project_name}/containers/{container_group_name} | Update Container Group
[**update_container_group_instance**](ContainerGroupsApi.md#update_container_group_instance) | **PATCH** /organizations/{organization_name}/projects/{project_name}/containers/{container_group_name}/instances/{container_group_instance_id} | Update Container Group Instance



## create_container_group

> models::ContainerGroup create_container_group(organization_name, project_name, container_group_prototype)
Create Container Group

Creates a new container group

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_name** | **String** | Your organization name. This identifies the billing context for the API operation and represents a security boundary for SaladCloud resources. The organization must be created before using the API, and you must be a member of the organization. | [required] |
**project_name** | **String** | Your project name. This represents a collection of related SaladCloud resources. The project must be created before using the API. | [required] |
**container_group_prototype** | [**ContainerGroupPrototype**](ContainerGroupPrototype.md) |  | [required] |

### Return type

[**models::ContainerGroup**](ContainerGroup.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_container_group

> delete_container_group(organization_name, project_name, container_group_name)
Delete Container Group

Deletes a container group

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_name** | **String** | Your organization name. This identifies the billing context for the API operation and represents a security boundary for SaladCloud resources. The organization must be created before using the API, and you must be a member of the organization. | [required] |
**project_name** | **String** | Your project name. This represents a collection of related SaladCloud resources. The project must be created before using the API. | [required] |
**container_group_name** | **String** | The unique container group name | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_container_group

> models::ContainerGroup get_container_group(organization_name, project_name, container_group_name)
Get Container Group

Gets a container group

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_name** | **String** | Your organization name. This identifies the billing context for the API operation and represents a security boundary for SaladCloud resources. The organization must be created before using the API, and you must be a member of the organization. | [required] |
**project_name** | **String** | Your project name. This represents a collection of related SaladCloud resources. The project must be created before using the API. | [required] |
**container_group_name** | **String** | The unique container group name | [required] |

### Return type

[**models::ContainerGroup**](ContainerGroup.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_container_group_instance

> models::ContainerGroupInstance get_container_group_instance(organization_name, project_name, container_group_name, container_group_instance_id)
Get Container Group Instance

Gets a container group instance

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_name** | **String** | Your organization name. This identifies the billing context for the API operation and represents a security boundary for SaladCloud resources. The organization must be created before using the API, and you must be a member of the organization. | [required] |
**project_name** | **String** | Your project name. This represents a collection of related SaladCloud resources. The project must be created before using the API. | [required] |
**container_group_name** | **String** | The unique container group name | [required] |
**container_group_instance_id** | **uuid::Uuid** | The unique container group instance identifier | [required] |

### Return type

[**models::ContainerGroupInstance**](ContainerGroupInstance.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_container_group_instances

> models::ContainerGroupInstanceCollection list_container_group_instances(organization_name, project_name, container_group_name)
List Container Group Instances

Gets the list of container group instances

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_name** | **String** | Your organization name. This identifies the billing context for the API operation and represents a security boundary for SaladCloud resources. The organization must be created before using the API, and you must be a member of the organization. | [required] |
**project_name** | **String** | Your project name. This represents a collection of related SaladCloud resources. The project must be created before using the API. | [required] |
**container_group_name** | **String** | The unique container group name | [required] |

### Return type

[**models::ContainerGroupInstanceCollection**](ContainerGroupInstanceCollection.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_container_groups

> models::ContainerGroupCollection list_container_groups(organization_name, project_name)
List Container Groups

Gets the list of container groups

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_name** | **String** | Your organization name. This identifies the billing context for the API operation and represents a security boundary for SaladCloud resources. The organization must be created before using the API, and you must be a member of the organization. | [required] |
**project_name** | **String** | Your project name. This represents a collection of related SaladCloud resources. The project must be created before using the API. | [required] |

### Return type

[**models::ContainerGroupCollection**](ContainerGroupCollection.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reallocate_container_group_instance

> reallocate_container_group_instance(organization_name, project_name, container_group_name, container_group_instance_id)
Reallocate Container Group Instance

Reallocates a container group instance to run on a different Salad Node

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_name** | **String** | Your organization name. This identifies the billing context for the API operation and represents a security boundary for SaladCloud resources. The organization must be created before using the API, and you must be a member of the organization. | [required] |
**project_name** | **String** | Your project name. This represents a collection of related SaladCloud resources. The project must be created before using the API. | [required] |
**container_group_name** | **String** | The unique container group name | [required] |
**container_group_instance_id** | **uuid::Uuid** | The unique container group instance identifier | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## recreate_container_group_instance

> recreate_container_group_instance(organization_name, project_name, container_group_name, container_group_instance_id)
Recreate Container Group Instance

Stops a container, destroys it, and starts a new one without requiring the image to be downloaded again on a new Salad Node

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_name** | **String** | Your organization name. This identifies the billing context for the API operation and represents a security boundary for SaladCloud resources. The organization must be created before using the API, and you must be a member of the organization. | [required] |
**project_name** | **String** | Your project name. This represents a collection of related SaladCloud resources. The project must be created before using the API. | [required] |
**container_group_name** | **String** | The unique container group name | [required] |
**container_group_instance_id** | **uuid::Uuid** | The unique container group instance identifier | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## restart_container_group_instance

> restart_container_group_instance(organization_name, project_name, container_group_name, container_group_instance_id)
Restart container Group Instance

Stops a container and restarts it on the same Salad Node

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_name** | **String** | Your organization name. This identifies the billing context for the API operation and represents a security boundary for SaladCloud resources. The organization must be created before using the API, and you must be a member of the organization. | [required] |
**project_name** | **String** | Your project name. This represents a collection of related SaladCloud resources. The project must be created before using the API. | [required] |
**container_group_name** | **String** | The unique container group name | [required] |
**container_group_instance_id** | **uuid::Uuid** | The unique container group instance identifier | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## start_container_group

> start_container_group(organization_name, project_name, container_group_name)
Start Container Group

Starts a container group

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_name** | **String** | Your organization name. This identifies the billing context for the API operation and represents a security boundary for SaladCloud resources. The organization must be created before using the API, and you must be a member of the organization. | [required] |
**project_name** | **String** | Your project name. This represents a collection of related SaladCloud resources. The project must be created before using the API. | [required] |
**container_group_name** | **String** | The unique container group name | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stop_container_group

> stop_container_group(organization_name, project_name, container_group_name)
Stop Container Group

Stops a container group

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_name** | **String** | Your organization name. This identifies the billing context for the API operation and represents a security boundary for SaladCloud resources. The organization must be created before using the API, and you must be a member of the organization. | [required] |
**project_name** | **String** | Your project name. This represents a collection of related SaladCloud resources. The project must be created before using the API. | [required] |
**container_group_name** | **String** | The unique container group name | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_container_group

> models::ContainerGroup update_container_group(organization_name, project_name, container_group_name, container_group_patch)
Update Container Group

Updates a container group

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_name** | **String** | Your organization name. This identifies the billing context for the API operation and represents a security boundary for SaladCloud resources. The organization must be created before using the API, and you must be a member of the organization. | [required] |
**project_name** | **String** | Your project name. This represents a collection of related SaladCloud resources. The project must be created before using the API. | [required] |
**container_group_name** | **String** | The unique container group name | [required] |
**container_group_patch** | [**ContainerGroupPatch**](ContainerGroupPatch.md) |  | [required] |

### Return type

[**models::ContainerGroup**](ContainerGroup.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/merge-patch+json
- **Accept**: application/merge-patch+json, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_container_group_instance

> models::ContainerGroupInstance update_container_group_instance(organization_name, project_name, container_group_name, container_group_instance_id, container_group_instance_patch)
Update Container Group Instance

Updates a container group instance

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_name** | **String** | Your organization name. This identifies the billing context for the API operation and represents a security boundary for SaladCloud resources. The organization must be created before using the API, and you must be a member of the organization. | [required] |
**project_name** | **String** | Your project name. This represents a collection of related SaladCloud resources. The project must be created before using the API. | [required] |
**container_group_name** | **String** | The unique container group name | [required] |
**container_group_instance_id** | **uuid::Uuid** | The unique container group instance identifier | [required] |
**container_group_instance_patch** | [**ContainerGroupInstancePatch**](ContainerGroupInstancePatch.md) |  | [required] |

### Return type

[**models::ContainerGroupInstance**](ContainerGroupInstance.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/merge-patch+json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

