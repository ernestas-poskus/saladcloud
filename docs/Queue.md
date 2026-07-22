# Queue

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**container_groups** | [**Vec<models::ContainerGroup>**](ContainerGroup.md) | The container groups that are part of this queue. Each container group represents a scalable set of identical containers running as a distributed service. | 
**create_time** | [**time::OffsetDateTime**](Time__OffsetDateTime.md) | The date and time the queue was created. | 
**current_queue_length** | Option<**i32**> | The current length of the queue | [optional]
**description** | Option<**String**> | The description. This may be used as a space for notes or other information about the queue. | [optional]
**display_name** | **String** | The display name. This may be used as a more human-readable name. | 
**id** | **uuid::Uuid** | The queue identifier. This is automatically generated and assigned when the queue is created. | 
**name** | **String** | The queue name. This must be unique within the project. | 
**update_time** | [**time::OffsetDateTime**](Time__OffsetDateTime.md) | The date and time the queue was last updated. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


