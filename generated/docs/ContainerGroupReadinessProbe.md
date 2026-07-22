# ContainerGroupReadinessProbe

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**exec** | Option<[**models::ContainerGroupProbeExec**](ContainerGroupProbeExec.md)> |  | [optional]
**failure_threshold** | **i32** | The number of consecutive failures required to consider the probe failed. After this many consecutive failures, the container is marked as not ready. | [default to 3]
**grpc** | Option<[**models::ContainerGroupProbeGrpc**](ContainerGroupProbeGrpc.md)> |  | [optional]
**http** | Option<[**models::ContainerGroupProbeHttp**](ContainerGroupProbeHttp.md)> |  | [optional]
**initial_delay_seconds** | **i32** | The time in seconds to wait after the container starts before initiating the first probe. This allows time for the application to initialize before being tested. | [default to 0]
**period_seconds** | **i32** | How frequently (in seconds) the probe should be executed during the container's lifetime. Specifies the interval between consecutive probe executions. | [default to 1]
**success_threshold** | **i32** | The minimum consecutive successes required to consider the probe successful after it has failed. Defines how many successful probe results are needed to transition from failure to success. | [default to 1]
**tcp** | Option<[**models::ContainerGroupProbeTcp**](ContainerGroupProbeTcp.md)> |  | [optional]
**timeout_seconds** | **i32** | The maximum time in seconds that the probe has to complete. If the probe doesn't return a result before the timeout, it's considered failed. | [default to 1]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


