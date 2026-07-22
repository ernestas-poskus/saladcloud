# CreateContainerLoggingHttp

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**compression** | [**models::ContainerLoggingHttpCompression**](ContainerLoggingHttpCompression.md) |  | 
**format** | [**models::ContainerLoggingHttpFormat**](ContainerLoggingHttpFormat.md) |  | 
**headers** | Option<[**Vec<models::ContainerLoggingHttpHeader>**](ContainerLoggingHttpHeader.md)> | Optional HTTP headers to include in log transmission requests | [optional]
**host** | **String** | The hostname or IP address of the HTTP logging endpoint | 
**password** | Option<**String**> | Optional password for HTTP authentication | [optional]
**path** | Option<**String**> | Optional URL path for the HTTP endpoint | [optional]
**port** | **i32** | The port number of the HTTP logging endpoint (1-65535) | 
**user** | Option<**String**> | Optional username for HTTP authentication | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


