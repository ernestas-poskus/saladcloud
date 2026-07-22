# InferenceEndpointJob

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**create_time** | [**time::OffsetDateTime**](Time__OffsetDateTime.md) | The time the job was created. | 
**events** | [**Vec<models::InferenceEndpointJobEvent>**](InferenceEndpointJobEvent.md) | The list of events. | 
**id** | **uuid::Uuid** | The inference endpoint job identifier. | 
**inference_endpoint_name** | **String** | The inference endpoint name. | 
**input** | Option<**serde_json::Value**> |  | 
**metadata** | Option<**serde_json::Value**> | The job metadata. May be any valid JSON. | [optional]
**organization_name** | **String** | The organization name. | 
**output** | Option<**serde_json::Value**> |  | [optional]
**status** | [**models::InferenceEndpointJobStatus**](InferenceEndpointJobStatus.md) |  | 
**update_time** | [**time::OffsetDateTime**](Time__OffsetDateTime.md) | The time the job was last updated. | 
**webhook** | Option<**String**> | The webhook URL called when the job completes. | [optional]
**webhook_url** | Option<**String**> | The webhook URL called when the job completes. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


