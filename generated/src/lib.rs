//! # SaladCloud API Client
//!
//! A Rust client library for the SaladCloud REST API.
//!
//! For more details, refer to the [SaladCloud API Documentation](https://docs.salad.com/api-reference).
//!
//! ## Example Usage
//!
//! ```rust,no_run
//! use saladcloud::apis::configuration::Configuration;
//! use saladcloud::apis::quotas_api;
//!
//! #[tokio::main]
//! async fn main() {
//!     let mut config = Configuration::new();
//!     config.api_key = Some(saladcloud::apis::configuration::ApiKey {
//!         prefix: None,
//!         key: "your-salad-api-key".to_string(),
//!     });
//!
//!     // Fetch quotas for your organization
//!     if let Ok(quotas) = quotas_api::get_quotas(&config, "your-organization-name").await {
//!         println!("Quotas retrieved successfully: {:?}", quotas);
//!     }
//! }
//! ```

#![deny(
    warnings,
    bad_style,
    dead_code,
    improper_ctypes,
    non_shorthand_field_patterns,
    no_mangle_generic_items,
    overflowing_literals,
    path_statements,
    patterns_in_fns_without_body,
    unconditional_recursion,
    unused,
    unused_allocation,
    unused_comparisons,
    unused_parens,
    while_true,
    missing_debug_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unused_extern_crates,
    unused_import_braces,
    unused_results,
    unreachable_pub,
    deprecated,
    unknown_lints,
    unreachable_code,
    unused_mut,
    non_camel_case_types,
    missing_docs,
    unused_qualifications
)]

/// Module.
pub mod apis;
/// Module.
pub mod models;
