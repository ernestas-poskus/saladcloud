# GpuAvailabilityPrototype

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**country_codes** | Option<[**Vec<models::CountryCode>**](CountryCode.md)> | A list of country codes where the resources are available | [optional]
**cpu** | Option<**i32**> | The number of available CPU cores | [optional]
**gpu_classes** | **Vec<uuid::Uuid>** | A list of available GPU class names | 
**memory** | Option<**i64**> | The amount of available memory in MB | [optional]
**storage_amount** | Option<**i64**> | The amount of available storage in bytes | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


