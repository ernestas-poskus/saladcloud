# SystemLog

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**event_name** | **String** | The name of the event | 
**event_time** | [**time::OffsetDateTime**](Time__OffsetDateTime.md) | The UTC date & time when the log item was created | 
**instance_id** | Option<**uuid::Uuid**> | The container group instance identifier. | [optional]
**machine_id** | Option<**uuid::Uuid**> | The container group machine identifier. | [optional]
**resource_cpu** | Option<**i32**> | The number of CPUs | 
**resource_gpu_class** | **String** | The GPU class name | 
**resource_memory** | Option<**i32**> | The memory amount in MB | 
**resource_storage_amount** | Option<**i64**> | The storage amount in bytes | 
**version** | **String** | The version instance ID | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


