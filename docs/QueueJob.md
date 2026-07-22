# QueueJob

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**create_time** | [**time::OffsetDateTime**](Time__OffsetDateTime.md) | The job creation time | 
**events** | [**Vec<models::QueueJobEvent>**](QueueJobEvent.md) | The job events | 
**id** | **uuid::Uuid** | The job identifier | 
**input** | Option<**serde_json::Value**> |  | 
**metadata** | Option<**serde_json::Value**> | Additional metadata for the job | [optional]
**output** | Option<**serde_json::Value**> |  | [optional]
**status** | **Status** | The job status (enum: pending, running, succeeded, cancelled, failed) | 
**update_time** | [**time::OffsetDateTime**](Time__OffsetDateTime.md) | The job update time | 
**webhook** | Option<**String**> | The webhook URL to notify when the job completes | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


