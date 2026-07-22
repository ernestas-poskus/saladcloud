# ContainerGroupLivenessProbe

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**exec** | Option<[**models::ContainerGroupProbeExec**](ContainerGroupProbeExec.md)> |  | [optional]
**failure_threshold** | **i32** | Number of consecutive failures required to consider the probe as failed | [default to 3]
**grpc** | Option<[**models::ContainerGroupProbeGrpc**](ContainerGroupProbeGrpc.md)> |  | [optional]
**http** | Option<[**models::ContainerGroupProbeHttp**](ContainerGroupProbeHttp.md)> |  | [optional]
**initial_delay_seconds** | **i32** | Number of seconds to wait after container start before initiating liveness probes | [default to 0]
**period_seconds** | **i32** | Frequency in seconds at which the probe should be executed | [default to 10]
**success_threshold** | **i32** | Number of consecutive successes required to consider the probe successful | [default to 1]
**tcp** | Option<[**models::ContainerGroupProbeTcp**](ContainerGroupProbeTcp.md)> |  | [optional]
**timeout_seconds** | **i32** | Number of seconds after which the probe times out if no response is received | [default to 30]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


