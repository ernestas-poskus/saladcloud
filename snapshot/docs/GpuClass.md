# GpuClass

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**gpu_class_type** | Option<**GpuClassType**> | The type of GPU class (enum: community, secure) | [optional]
**gpu_count** | Option<**i32**> | The number of GPUs in the cluster | [optional]
**id** | **uuid::Uuid** | The unique identifier | 
**is_high_demand** | Option<**bool**> | Whether the GPU class is in high demand | [optional]
**max_ram** | Option<**i32**> | The maximum RAM amount in MB | [optional]
**max_storage** | Option<**i64**> | The maximum storage amount in bytes | [optional]
**max_vcpu** | Option<**i32**> | The maximum vCPU count | [optional]
**min_ram** | Option<**i32**> | The minimum RAM amount in MB | [optional]
**min_storage** | Option<**i64**> | The minimum storage amount in bytes | [optional]
**min_vcpu** | Option<**i32**> | The minimum vCPU count | [optional]
**name** | **String** | The GPU class name | 
**prices** | [**Vec<models::GpuClassPrice>**](GpuClassPrice.md) | The list of prices for each container group priority | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


