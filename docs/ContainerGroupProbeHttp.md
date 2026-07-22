# ContainerGroupProbeHttp

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**headers** | [**Vec<models::ContainerGroupProbeHttpHeader>**](ContainerGroupProbeHttpHeader.md) | A collection of HTTP header name-value pairs used for configuring requests and responses in container group endpoints. Each header consists of a name and its corresponding value. | 
**path** | **String** | The HTTP path that will be probed to check container health. | 
**port** | **i32** | The TCP port number to which the HTTP request will be sent. | 
**scheme** | Option<[**models::ContainerProbeHttpScheme**](ContainerProbeHttpScheme.md)> |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


