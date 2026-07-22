# ContainerGroupNetworking

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**auth** | **bool** | Whether authentication is required for network access to the container group | 
**client_request_timeout** | Option<**i32**> | The container group networking client request timeout. | [optional][default to 100000]
**dns** | **String** | Domain name or URL endpoint for the container group's network interface | 
**load_balancer** | [**models::ContainerGroupNetworkingLoadBalancer**](ContainerGroupNetworkingLoadBalancer.md) |  | 
**port** | **i32** | The container group networking port. | 
**protocol** | [**models::ContainerNetworkingProtocol**](ContainerNetworkingProtocol.md) |  | 
**server_response_timeout** | Option<**i32**> | The container group networking server response timeout. | [optional][default to 100000]
**single_connection_limit** | Option<**bool**> | The container group networking single connection limit flag. | [optional][default to false]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


