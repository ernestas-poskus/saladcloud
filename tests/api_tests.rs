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

#[test]
/// Test serialization and deserialization of the Quotas model.
fn test_quotas_serde() {
    use saladcloud::models::container_groups_quotas::ContainerGroupsQuotas;
    use saladcloud::models::quotas::Quotas;

    let cg_quotas = ContainerGroupsQuotas::new(10, 20);
    let quotas = Quotas::new(cg_quotas);

    let serialized = serde_json::to_string(&quotas).expect("Failed to serialize Quotas");
    assert!(serialized.contains("\"container_groups_quotas\""));

    let deserialized: Quotas =
        serde_json::from_str(&serialized).expect("Failed to deserialize Quotas");
    assert_eq!(
        deserialized
            .container_groups_quotas
            .container_replicas_quota,
        10
    );
}

#[test]
/// Test serialization and deserialization of the GpuClassesList model.
fn test_gpu_classes_list_serde() {
    use saladcloud::models::gpu_class::GpuClass;
    use saladcloud::models::gpu_classes_list::GpuClassesList;

    let gpu = GpuClass::new(uuid::Uuid::new_v4(), "NVIDIA RTX 3090".to_string(), vec![]);
    let list = GpuClassesList::new(vec![gpu]);

    let serialized = serde_json::to_string(&list).expect("Failed to serialize GpuClassesList");
    assert!(serialized.contains("\"name\":\"NVIDIA RTX 3090\""));

    let deserialized: GpuClassesList =
        serde_json::from_str(&serialized).expect("Failed to deserialize GpuClassesList");
    assert_eq!(deserialized.items[0].name, "NVIDIA RTX 3090");
}
