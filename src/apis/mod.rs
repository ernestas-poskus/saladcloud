use std::fmt;

use thiserror::Error;

#[derive(Debug, Clone)]
/// Struct.
pub struct ResponseContent<T> {
    /// Field.
    pub status: reqwest::StatusCode,
    /// Field.
    pub content: String,
    /// Field.
    pub entity: Option<T>,
}

#[derive(Debug, Error)]
/// Documentation.
pub enum Error<T> {
    /// Variant.
    #[error("error in reqwest: {0}")]
    Reqwest(#[from] reqwest::Error),
    /// Variant.
    #[error("error in serde: {0}")]
    Serde(#[from] serde_json::Error),
    /// Variant.
    #[error("error in IO: {0}")]
    Io(#[from] std::io::Error),
    /// Variant.
    #[error("error in response: status code {0}")]
    ResponseError(ResponseContent<T>),
}

impl<T> fmt::Display for ResponseContent<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.status)
    }
}

/// Documentation.
pub fn urlencode<T: AsRef<str>>(s: T) -> String {
    ::url::form_urlencoded::byte_serialize(s.as_ref().as_bytes()).collect()
}

/// Documentation.
pub fn parse_deep_object(prefix: &str, value: &serde_json::Value) -> Vec<(String, String)> {
    if let serde_json::Value::Object(object) = value {
        let mut params = vec![];

        for (key, value) in object {
            match value {
                serde_json::Value::Object(_) => params.append(&mut parse_deep_object(
                    &format!("{}[{}]", prefix, key),
                    value,
                )),
                serde_json::Value::Array(array) => {
                    for (i, value) in array.iter().enumerate() {
                        params.append(&mut parse_deep_object(
                            &format!("{}[{}][{}]", prefix, key, i),
                            value,
                        ));
                    }
                }
                serde_json::Value::String(s) => {
                    params.push((format!("{}[{}]", prefix, key), s.clone()))
                }
                _ => params.push((format!("{}[{}]", prefix, key), value.to_string())),
            }
        }

        return params;
    }

    unimplemented!("Only objects are supported with style=deepObject")
}

/// Internal use only
/// A content type supported by this client.
#[allow(dead_code)]
enum ContentType {
    Json,
    Text,
    Unsupported(String),
}

impl From<&str> for ContentType {
    fn from(content_type: &str) -> Self {
        if content_type.starts_with("application") && content_type.contains("json") {
            Self::Json
        } else if content_type.starts_with("text/plain") {
            Self::Text
        } else {
            Self::Unsupported(content_type.to_string())
        }
    }
}

/// Module.
pub mod container_groups_api;
/// Module.
pub mod inference_endpoints_api;
/// Module.
pub mod logs_api;
/// Module.
pub mod organization_data_api;
/// Module.
pub mod organizations_api;
/// Module.
pub mod queues_api;
/// Module.
pub mod quotas_api;
/// Module.
pub mod system_logs_api;
/// Module.
pub mod webhook_secret_key_api;

/// Module.
pub mod configuration;
