use saladcloud::worker::{Job, WorkerConfig};

#[test]
fn worker_config_defaults_match_salad_worker() {
    let config = WorkerConfig::default();

    assert_eq!(config.metadata_uri.as_str(), "http://169.254.169.254:80");
    assert_eq!(
        config.service_endpoint.as_str(),
        "job-queue-worker-api.salad.com:443"
    );
    assert!(config.use_tls);
}

#[test]
fn service_endpoint_uri_adds_scheme_from_tls_setting() {
    let tls_config = WorkerConfig {
        service_endpoint: "job-queue-worker-api.salad.com:443".to_string(),
        use_tls: true,
        ..WorkerConfig::default()
    };
    let plaintext_config = WorkerConfig {
        service_endpoint: "127.0.0.1:50051".to_string(),
        use_tls: false,
        ..WorkerConfig::default()
    };

    assert_eq!(
        tls_config.service_endpoint_uri(),
        "https://job-queue-worker-api.salad.com:443"
    );
    assert_eq!(
        plaintext_config.service_endpoint_uri(),
        "http://127.0.0.1:50051"
    );
}

#[test]
fn worker_config_reads_current_tls_env_name() {
    let config = WorkerConfig::from_env_values(|key| match key {
        "SALAD_METADATA_URI" => Err(std::env::VarError::NotPresent),
        "SALAD_SERVICE_ENDPOINT" => Err(std::env::VarError::NotPresent),
        "SALAD_SERVICE_USE_TLS" => Ok("false".to_string()),
        _ => panic!("unexpected env key: {key}"),
    });

    assert!(!config.use_tls);
}

#[test]
fn job_input_json_decodes_queue_body() {
    let job = Job {
        id: "job-123".to_string(),
        port: 8080,
        path: "/salad".to_string(),
        input: br#"{"input":{"text":"hello"}}"#.to_vec(),
    };

    let value: serde_json::Value = job.input_json().expect("valid json");

    assert_eq!(value["input"]["text"], "hello");
}
