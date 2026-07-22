# ContainerResourceRequirements

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**cpu** | **i32** | The number of CPU cores required by the container. Must be between 1 and 16. | 
**gpu_classes** | **Vec<uuid::Uuid>** | A list of GPU class UUIDs required by the container. Can be null if no GPU is required. | 
**memory** | **i32** | The amount of memory (in MB) required by the container. Must be between 1024 MB and 61440 MB. | 
**shm_size** | Option<**i32**> | The size of the shared memory (/dev/shm) in MB. If not specified, defaults to 1024MB. | [optional][default to 64]
**storage_amount** | Option<**i64**> | The amount of storage (in bytes) required by the container. Must be between 1 GB (1073741824 bytes) and 250 GB (268435456000 bytes). | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


