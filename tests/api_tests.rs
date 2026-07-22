//! Integration tests for the SaladCloud API Client.

use saladcloud::apis::configuration::Configuration;
use saladcloud::models::webhook_secret_key::WebhookSecretKey;

#[test]
/// Test configuration initialization and defaults.
fn test_configuration_defaults() {
    let config = Configuration::new();
    assert_eq!(config.base_path, "https://api.salad.com/api/public");
    assert!(config.user_agent.is_some());
    assert!(config.api_key.is_none());
}

#[test]
/// Test serialization and deserialization of the WebhookSecretKey model.
fn test_webhook_secret_key_serde() {
    let secret = WebhookSecretKey::new("test_secret_key_12345".to_string());
    assert_eq!(secret.secret_key, "test_secret_key_12345");

    let serialized = serde_json::to_string(&secret).expect("Failed to serialize WebhookSecretKey");
    assert!(serialized.contains("\"secret_key\":\"test_secret_key_12345\""));

    let deserialized: WebhookSecretKey =
        serde_json::from_str(&serialized).expect("Failed to deserialize WebhookSecretKey");
    assert_eq!(deserialized.secret_key, "test_secret_key_12345");
}
