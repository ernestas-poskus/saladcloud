# ContainerGroupStartupProbe

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**exec** | Option<[**models::ContainerGroupProbeExec**](ContainerGroupProbeExec.md)> |  | [optional]
**failure_threshold** | **i32** | Number of times the probe must fail before considering the container not started | [default to 15]
**grpc** | Option<[**models::ContainerGroupProbeGrpc**](ContainerGroupProbeGrpc.md)> |  | [optional]
**http** | Option<[**models::ContainerGroupProbeHttp**](ContainerGroupProbeHttp.md)> |  | [optional]
**initial_delay_seconds** | **i32** | Number of seconds to wait after container startup before the first probe is executed | [default to 0]
**period_seconds** | **i32** | How frequently (in seconds) to perform the probe | [default to 3]
**success_threshold** | **i32** | Minimum consecutive successes required for the probe to be considered successful | [default to 2]
**tcp** | Option<[**models::ContainerGroupProbeTcp**](ContainerGroupProbeTcp.md)> |  | [optional]
**timeout_seconds** | **i32** | Maximum time (in seconds) to wait for a probe response before considering it failed | [default to 10]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


