# ContainerGroupQueueAutoscaler

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**desired_queue_length** | **i32** | The target number of items in the queue that the autoscaler attempts to maintain by scaling the containers up or down | 
**max_downscale_per_minute** | Option<**i32**> | The maximum number of instances that can be removed per minute to prevent rapid downscaling | [optional]
**max_replicas** | **i32** | The maximum number of instances the container can scale up to | 
**max_upscale_per_minute** | Option<**i32**> | The maximum number of instances that can be added per minute to prevent rapid upscaling | [optional]
**min_replicas** | **i32** | The minimum number of instances the container can scale down to, ensuring baseline availability | 
**polling_period** | Option<**i32**> | The period (in seconds) in which the autoscaler checks the queue length and applies the scaling formula | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


