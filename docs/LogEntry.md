# LogEntry

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**json_log** | Option<**std::collections::HashMap<String, serde_json::Value>**> | The log message in JSON format. | [optional]
**parent_span_id** | Option<**String**> | The parent span ID of the log entry | [optional]
**receive_time** | [**time::OffsetDateTime**](Time__OffsetDateTime.md) | The time when the log entry was received | 
**resource** | [**models::LogEntryResource**](LogEntryResource.md) |  | 
**severity** | [**models::LogEntrySeverity**](LogEntrySeverity.md) |  | 
**span_id** | Option<**String**> | The span ID of the log entry | [optional]
**text_log** | Option<**String**> | The log message in text format. | [optional]
**time** | [**time::OffsetDateTime**](Time__OffsetDateTime.md) | The timestamp of the log entry | 
**trace_id** | Option<**String**> | The trace ID of the log entry | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


