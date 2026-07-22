# ContainerGroupPrototype

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**autostart_policy** | **bool** | Determines whether the container group should start automatically when created (true) or remain stopped until manually started (false) | 
**container** | [**models::CreateContainer**](CreateContainer.md) |  | 
**country_codes** | Option<[**Vec<models::CountryCode>**](CountryCode.md)> | List of countries nodes must be located in. Remove this field to permit nodes from any country. | [optional]
**display_name** | Option<**String**> | Human-readable name for the container group that can include spaces and special characters, used for display purposes | [optional]
**liveness_probe** | Option<[**models::ContainerGroupLivenessProbe**](ContainerGroupLivenessProbe.md)> |  | [optional]
**name** | **String** | Unique identifier for the container group that must follow DNS naming conventions (lowercase alphanumeric with hyphens) | 
**networking** | Option<[**models::CreateContainerGroupNetworking**](CreateContainerGroupNetworking.md)> |  | [optional]
**queue_autoscaler** | Option<[**models::ContainerGroupQueueAutoscaler**](ContainerGroupQueueAutoscaler.md)> |  | [optional]
**queue_connection** | Option<[**models::ContainerGroupQueueConnection**](ContainerGroupQueueConnection.md)> |  | [optional]
**readiness_probe** | Option<[**models::ContainerGroupReadinessProbe**](ContainerGroupReadinessProbe.md)> |  | [optional]
**replicas** | **i32** | Number of container instances to deploy and maintain for this container group | 
**restart_policy** | [**models::ContainerRestartPolicy**](ContainerRestartPolicy.md) |  | 
**scaling_actions** | Option<[**Vec<models::ContainerGroupScalingAction>**](ContainerGroupScalingAction.md)> | List of scaling action configurations | [optional]
**scheduled_scaling_enabled** | Option<**bool**> | Indicates if scheduled scaling is enabled | [optional]
**startup_probe** | Option<[**models::ContainerGroupStartupProbe**](ContainerGroupStartupProbe.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


