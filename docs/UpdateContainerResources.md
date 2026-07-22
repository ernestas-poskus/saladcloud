# UpdateContainerResources

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**cpu** | Option<**i32**> | The number of CPU cores to allocate to the container (between 1 and 1024). | [optional]
**gpu_classes** | Option<**Vec<uuid::Uuid>**> | List of GPU class identifiers that the container can use, specified as UUIDs. | [optional]
**memory** | Option<**i32**> | The amount of memory to allocate to the container in megabytes (between 1024 and 1073741824). | [optional]
**shm_size** | Option<**i32**> | The amount of shared memory to allocate to the container via `/dev/shm` in megabytes (between 64 and 1073741824). If not specified, defaults to 64 MB. | [optional][default to 64]
**storage_amount** | Option<**i64**> | The amount of storage to allocate to the container in bytes (between 1 GB and 1 PB). | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


