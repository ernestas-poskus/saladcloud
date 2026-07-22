# ContainerGroupPatch

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**container** | Option<[**models::UpdateContainer**](UpdateContainer.md)> |  | [optional]
**country_codes** | Option<[**Vec<models::CountryCode>**](CountryCode.md)> | List of countries nodes must be located in. Remove this field to permit nodes from any country. | [optional]
**display_name** | Option<**String**> | The display name for the container group. If null is provided, the display name will be set to the container group name. | [optional]
**liveness_probe** | Option<[**models::ContainerGroupLivenessProbe**](ContainerGroupLivenessProbe.md)> |  | [optional]
**networking** | Option<[**models::UpdateContainerGroupNetworking**](UpdateContainerGroupNetworking.md)> |  | [optional]
**queue_autoscaler** | Option<[**models::ContainerGroupQueueAutoscaler**](ContainerGroupQueueAutoscaler.md)> |  | [optional]
**readiness_probe** | Option<[**models::ContainerGroupReadinessProbe**](ContainerGroupReadinessProbe.md)> |  | [optional]
**replicas** | Option<**i32**> | The desired number of instances for your container group deployment. | [optional]
**scaling_actions** | Option<[**Vec<models::ContainerGroupScalingAction>**](ContainerGroupScalingAction.md)> | List of scaling actions configurations | [optional]
**scheduled_scaling_enabled** | Option<**bool**> | Indicates if scheduled scaling is enabled | [optional]
**startup_probe** | Option<[**models::ContainerGroupStartupProbe**](ContainerGroupStartupProbe.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


