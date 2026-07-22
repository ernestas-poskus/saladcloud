# ContainerGroup

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**autostart_policy** | **bool** | Defines whether containers in this group should automatically start when deployed (true) or require manual starting (false) | 
**container** | [**models::Container**](Container.md) |  | 
**country_codes** | [**Vec<models::CountryCode>**](CountryCode.md) | List of country codes where container instances are permitted to run. When not specified or empty, containers may run in any available region. | 
**create_time** | [**time::OffsetDateTime**](Time__OffsetDateTime.md) | ISO 8601 timestamp when this container group was initially created | 
**current_state** | [**models::ContainerGroupState**](ContainerGroupState.md) |  | 
**display_name** | **String** | The display-friendly name of the resource. | 
**id** | **uuid::Uuid** | The container group identifier. | 
**liveness_probe** | Option<[**models::ContainerGroupLivenessProbe**](ContainerGroupLivenessProbe.md)> |  | [optional]
**name** | **String** | The container group name. | 
**networking** | Option<[**models::ContainerGroupNetworking**](ContainerGroupNetworking.md)> |  | [optional]
**organization_name** | **String** | The organization name. | 
**pending_change** | **bool** | Indicates whether a configuration change has been requested but not yet applied to all containers in the group | 
**priority** | Option<[**models::ContainerGroupPriority**](ContainerGroupPriority.md)> |  | 
**project_name** | **String** | The project name. | 
**queue_autoscaler** | Option<[**models::ContainerGroupQueueAutoscaler**](ContainerGroupQueueAutoscaler.md)> |  | [optional]
**queue_connection** | Option<[**models::ContainerGroupQueueConnection**](ContainerGroupQueueConnection.md)> |  | [optional]
**readiness_probe** | Option<[**models::ContainerGroupReadinessProbe**](ContainerGroupReadinessProbe.md)> |  | [optional]
**readme** | Option<**String**> |  | [optional]
**replicas** | **i32** | The container group replicas. | 
**restart_policy** | [**models::ContainerRestartPolicy**](ContainerRestartPolicy.md) |  | 
**scaling_actions** | [**Vec<models::ContainerGroupScalingAction>**](ContainerGroupScalingAction.md) | List of scaling actions configurations | 
**scheduled_scaling_enabled** | **bool** | Indicates if scheduled scaling is enabled | 
**startup_probe** | Option<[**models::ContainerGroupStartupProbe**](ContainerGroupStartupProbe.md)> |  | [optional]
**update_time** | [**time::OffsetDateTime**](Time__OffsetDateTime.md) | ISO 8601 timestamp when this container group was last updated | 
**version** | **i32** | Incremental version number that increases with each configuration change to the container group | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


