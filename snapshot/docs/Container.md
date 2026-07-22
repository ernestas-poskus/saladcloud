# Container

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**command** | Option<**Vec<String>**> | List of commands to run inside the container. Each command is a string representing a command-line instruction. | 
**environment_variables** | Option<**std::collections::HashMap<String, String>**> | Environment variables to set in the container. | [optional]
**hash** | Option<**String**> | SHA-256 hash (64-character hexadecimal string) | [optional]
**image** | **String** | The container image. | 
**image_caching** | Option<**bool**> | The container image caching. | [optional]
**logging** | Option<[**models::ContainerLogging**](ContainerLogging.md)> |  | [optional]
**resources** | [**models::ContainerResourceRequirements**](ContainerResourceRequirements.md) |  | 
**size** | Option<**i64**> | Size of the container in bytes. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


