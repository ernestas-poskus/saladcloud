# \WebhookSecretKeyApi

All URIs are relative to *https://api.salad.com/api/public*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_webhook_secret_key**](WebhookSecretKeyApi.md#get_webhook_secret_key) | **GET** /organizations/{organization_name}/webhook-secret-key | Gets the webhook secret key
[**update_webhook_secret_key**](WebhookSecretKeyApi.md#update_webhook_secret_key) | **POST** /organizations/{organization_name}/webhook-secret-key | Updates the webhook secret key



## get_webhook_secret_key

> models::WebhookSecretKey get_webhook_secret_key(organization_name)
Gets the webhook secret key

Gets the webhook secret key

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_name** | **String** | Your organization name. This identifies the billing context for the API operation and represents a security boundary for SaladCloud resources. The organization must be created before using the API, and you must be a member of the organization. | [required] |

### Return type

[**models::WebhookSecretKey**](WebhookSecretKey.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_webhook_secret_key

> models::WebhookSecretKey update_webhook_secret_key(organization_name)
Updates the webhook secret key

Updates the webhook secret key

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_name** | **String** | Your organization name. This identifies the billing context for the API operation and represents a security boundary for SaladCloud resources. The organization must be created before using the API, and you must be a member of the organization. | [required] |

### Return type

[**models::WebhookSecretKey**](WebhookSecretKey.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

