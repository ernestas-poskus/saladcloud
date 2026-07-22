# ContainerGroupInstance

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**cpu_percent** | Option<**f32**> | The percentage of CPU used by this container group instance. This is updated every minute. | [optional]
**cpu_usage** | Option<**i64**> | The total CPU usage in seconds for this container group instance. This is updated every minute. | [optional]
**cpu_usage_total** | Option<**i64**> | The total CPU usage in seconds for this container group instance since it was started. This is updated every minute. | [optional]
**deletion_cost** | Option<**i32**> | The cost of deleting the container group instance | [optional][default to 0]
**id** | **uuid::Uuid** | The container group instance identifier. | 
**machine_id** | **uuid::Uuid** | The container group machine identifier. | 
**memory_usage_mb** | Option<**f32**> | The memory usage in MB for this container group instance. This is updated every minute. | [optional]
**memory_usage_percent** | Option<**f32**> | The percentage of memory used by this container group instance. This is updated every minute. | [optional]
**pulling_progress** | Option<**f32**> | The progress percentage of pulling the container image. This is only relevant when the instance state is 'downloading'. | [optional]
**ready** | Option<**bool**> | Indicates whether the container group instance is currently passing its readiness checks and is able to receive traffic or perform its intended function. If no readiness probe is defined, this will be true once the instance is fully started. | [optional]
**ssh_host_key_fingerprint** | Option<**String**> | The SSH host key fingerprint of the container group instance | [optional]
**ssh_ip** | Option<**String**> | The SSH IP address of the container group instance | [optional]
**ssh_port** | Option<**i32**> | The SSH port of the container group instance | [optional][default to 22]
**started** | Option<**bool**> | Indicates whether the container group instance has successfully completed its startup sequence and passed any configured startup probes. This will always be true when no startup probe is defined for the container group. | [optional]
**state** | [**models::ContainerGroupInstanceState**](ContainerGroupInstanceState.md) |  | 
**update_time** | [**time::OffsetDateTime**](Time__OffsetDateTime.md) | The UTC timestamp when the container group instance last changed its state. This helps track the lifecycle and state transitions of the instance. | 
**version** | **i32** | The version of the container group definition currently running on this instance. Used to track deployment and update progress across the container group fleet. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


