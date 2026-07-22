//! Example of listing available GPU classes using the SaladCloud API.

use saladcloud::apis::configuration::Configuration;
use saladcloud::apis::organization_data_api;

#[tokio::main]
/// Main entry point for the list_gpu_classes example.
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

    // Call the API
    match organization_data_api::list_gpu_classes(&config, "your-organization-name").await {
        Ok(gpu_classes) => println!("Retrieved GPU classes: {:?}", gpu_classes),
        Err(e) => println!("API call finished (expected placeholder failure): {:?}", e),
    }
}
