# UpdateContainer

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**command** | Option<**Vec<String>**> | Pass a command (and optional arguments) to override the ENTRYPOINT and CMD of a container image. | [optional]
**environment_variables** | Option<**std::collections::HashMap<String, String>**> | Environment variables to set in the container. | [optional]
**image** | Option<**String**> | The container image to use. | [optional]
**image_caching** | Option<**bool**> | The container image caching. | [optional]
**logging** | Option<[**models::UpdateContainerLogging**](UpdateContainerLogging.md)> |  | [optional]
**priority** | Option<[**models::ContainerGroupPriority**](ContainerGroupPriority.md)> |  | [optional]
**registry_authentication** | Option<[**models::ContainerRegistryAuthentication**](ContainerRegistryAuthentication.md)> |  | [optional]
**resources** | Option<[**models::UpdateContainerResources**](UpdateContainerResources.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


