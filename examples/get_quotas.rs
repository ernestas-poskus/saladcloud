//! Example of retrieving organization quotas using the SaladCloud API.

use saladcloud::apis::configuration::Configuration;
use saladcloud::apis::quotas_api;

#[tokio::main]
/// Main entry point for the get_quotas example.
async fn main() {
    let mut config = Configuration::new();
    config.api_key = Some(saladcloud::apis::configuration::ApiKey {
        prefix: None,
        key: "your-api-key-here".to_string(),
    });

    println!(
        "Initializing SaladCloud client using base path: {}",
        config.base_path
    );

    // Call the API (this will fail with a 401 if the API key is invalid/placeholder)
    match quotas_api::get_quotas(&config, "your-organization-name").await {
        Ok(quotas) => println!("Retrieved organization quotas: {:?}", quotas),
        Err(e) => println!("API call finished (expected placeholder failure): {:?}", e),
    }
}
