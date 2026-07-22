# CreateContainer

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**command** | Option<**Vec<String>**> | Pass a command (and optional arguments) to override the ENTRYPOINT and CMD of a container image. Each element in the array represents a command segment or argument. | [optional]
**environment_variables** | Option<**std::collections::HashMap<String, String>**> | Key-value pairs of environment variables to set within the container. These variables will be available to processes running inside the container. | [optional]
**image** | **String** | The container image. | 
**image_caching** | Option<**bool**> | The container image caching. | [optional]
**logging** | Option<[**models::CreateContainerLogging**](CreateContainerLogging.md)> |  | [optional]
**priority** | Option<[**models::ContainerGroupPriority**](ContainerGroupPriority.md)> |  | [optional]
**registry_authentication** | Option<[**models::ContainerRegistryAuthentication**](ContainerRegistryAuthentication.md)> |  | [optional]
**resources** | [**models::CreateContainerResourceRequirements**](CreateContainerResourceRequirements.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


