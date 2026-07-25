//! SaladCloud Job Queue worker support.

use std::future::Future;
use std::time::Duration;

use serde::de::DeserializeOwned;
use tonic::metadata::MetadataValue;
use tonic::transport::{Channel, Endpoint};
use tonic::{Request, Status};

use thiserror::Error;
use tracing::error;

/// Result type used by the SaladCloud worker client.
pub type Result<T> = std::result::Result<T, Error>;

/// Errors returned by the SaladCloud worker client.
#[derive(Debug, Error)]
pub enum Error {
    /// The SaladCloud metadata service returned an invalid or empty token response.
    #[error("missing SaladCloud workload token")]
    MissingToken,
    /// The SaladCloud metadata service returned a non-success HTTP response.
    #[error("failed to fetch SaladCloud workload token: {0}")]
    TokenStatus(reqwest::StatusCode),
    /// HTTP client error while talking to IMDS.
    #[error("SaladCloud metadata request failed: {0}")]
    Reqwest(#[from] reqwest::Error),
    /// JSON decode error.
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    /// gRPC transport setup error.
    #[error("SaladCloud worker transport failed: {0}")]
    Transport(#[from] tonic::transport::Error),
    /// gRPC status returned by the queue worker service.
    #[error("SaladCloud worker gRPC failed: {0}")]
    Status(#[from] Status),
    /// Invalid gRPC authorization metadata.
    #[error("invalid SaladCloud worker metadata: {0}")]
    Metadata(#[from] tonic::metadata::errors::InvalidMetadataValue),
}

/// Configuration for the SaladCloud Job Queue worker service.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorkerConfig {
    /// Base URI for SaladCloud instance metadata.
    pub metadata_uri: String,
    /// Host and port, or full URI, for the SaladCloud Job Queue worker gRPC service.
    pub service_endpoint: String,
    /// Whether to use TLS for the gRPC connection.
    pub use_tls: bool,
    /// Request timeout for individual gRPC requests.
    pub request_timeout: Option<Duration>,
    /// Connection establishment timeout.
    pub connect_timeout: Option<Duration>,
    /// Duration of inactivity before sending TCP keepalive probes.
    pub tcp_keepalive: Option<Duration>,
    /// Duration between successive TCP keepalive retransmissions.
    pub tcp_keepalive_interval: Option<Duration>,
    /// Number of TCP keepalive retransmissions before considering connection dead.
    pub tcp_keepalive_retries: Option<u32>,
    /// Whether `TCP_NODELAY` is enabled.
    pub tcp_nodelay: bool,
    /// Interval between HTTP/2 PING frames to keep connection alive.
    pub http2_keep_alive_interval: Option<Duration>,
    /// Timeout for HTTP/2 PING response before considering connection dead.
    pub keep_alive_timeout: Option<Duration>,
    /// Whether HTTP/2 PING frames are sent even when there are no active streams.
    pub keep_alive_while_idle: Option<bool>,
    /// Maximum number of concurrent requests permitted on the channel.
    pub concurrency_limit: Option<usize>,
    /// Rate limit tuple `(max_requests, duration)` for client-side rate limiting.
    pub rate_limit: Option<(u64, Duration)>,
    /// Initial HTTP/2 stream-level flow control window size in bytes.
    pub initial_stream_window_size: Option<u32>,
    /// Initial HTTP/2 connection-level flow control window size in bytes.
    pub initial_connection_window_size: Option<u32>,
    /// Maximum HTTP/2 frame size in bytes.
    pub max_frame_size: Option<u32>,
    /// Maximum size of HTTP/2 header table.
    pub http2_header_table_size: Option<u32>,
    /// Maximum size of received HTTP/2 header frames.
    pub http2_max_header_list_size: Option<u32>,
    /// Whether to enable HTTP/2 adaptive flow control window.
    pub http2_adaptive_window: Option<bool>,
    /// Buffer size for internal Tower service channel.
    pub buffer_size: Option<usize>,
}

impl Default for WorkerConfig {
    fn default() -> Self {
        Self {
            metadata_uri: "http://169.254.169.254:80".to_string(),
            service_endpoint: "job-queue-worker-api.salad.com:443".to_string(),
            use_tls: true,
            request_timeout: Some(Duration::from_secs(30)),
            connect_timeout: Some(Duration::from_secs(10)),
            tcp_keepalive: Some(Duration::from_secs(30)),
            tcp_keepalive_interval: Some(Duration::from_secs(10)),
            tcp_keepalive_retries: Some(3),
            tcp_nodelay: true,
            http2_keep_alive_interval: Some(Duration::from_secs(30)),
            keep_alive_timeout: Some(Duration::from_secs(10)),
            keep_alive_while_idle: Some(true),
            concurrency_limit: None,
            rate_limit: None,
            initial_stream_window_size: None,
            initial_connection_window_size: None,
            max_frame_size: None,
            http2_header_table_size: None,
            http2_max_header_list_size: None,
            http2_adaptive_window: None,
            buffer_size: None,
        }
    }
}

impl WorkerConfig {
    /// Builds worker configuration from SaladCloud worker environment variables.
    pub fn from_env() -> Self {
        Self::from_env_values(|key| std::env::var(key))
    }

    /// Builds worker configuration from an injected environment reader.
    pub fn from_env_values(
        mut get_env: impl FnMut(&str) -> std::result::Result<String, std::env::VarError>,
    ) -> Self {
        let mut config = Self::default();
        if let Ok(metadata_uri) = get_env("SALAD_METADATA_URI") {
            config.metadata_uri = metadata_uri;
        }
        if let Ok(service_endpoint) = get_env("SALAD_SERVICE_ENDPOINT") {
            config.service_endpoint = service_endpoint;
        }
        if let Ok(use_tls) = get_env("SALAD_SERVICE_USE_TLS") {
            config.use_tls = !matches!(use_tls.as_str(), "0" | "false" | "FALSE" | "False");
        }
        config
    }

    /// Returns a URI suitable for `tonic` channel creation.
    pub fn service_endpoint_uri(&self) -> String {
        if self.service_endpoint.starts_with("http://")
            || self.service_endpoint.starts_with("https://")
        {
            return self.service_endpoint.clone();
        }

        let scheme = if self.use_tls { "https" } else { "http" };
        format!("{scheme}://{}", self.service_endpoint)
    }
}

/// A job delivered by the SaladCloud Job Queue worker stream.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Job {
    /// SaladCloud queue job identifier.
    pub id: String,
    /// HTTP port configured on the queue connection.
    pub port: i32,
    /// HTTP path configured on the queue connection.
    pub path: String,
    /// Raw JSON request body submitted to the queue.
    pub input: Vec<u8>,
}

impl Job {
    /// Decodes the raw queue body as JSON.
    pub fn input_json<T: DeserializeOwned>(&self) -> Result<T> {
        serde_json::from_slice(&self.input).map_err(Error::from)
    }
}

/// Client for the SaladCloud Job Queue worker gRPC service.
#[derive(Debug, Clone)]
pub struct QueueWorker {
    config: WorkerConfig,
    http: reqwest::Client,
    grpc: proto::JobQueueWorkerServiceClient<Channel>,
}

impl QueueWorker {
    /// Connects to the SaladCloud Job Queue worker service.
    pub async fn connect(config: WorkerConfig) -> Result<Self> {
        let mut endpoint = Endpoint::from_shared(config.service_endpoint_uri())?
            .user_agent("saladcloud-rust-worker/0.1 tonic")?
            .tcp_nodelay(config.tcp_nodelay)
            .tcp_keepalive(config.tcp_keepalive)
            .tcp_keepalive_interval(config.tcp_keepalive_interval)
            .tcp_keepalive_retries(config.tcp_keepalive_retries);

        if let Some(timeout) = config.request_timeout {
            endpoint = endpoint.timeout(timeout);
        }
        if let Some(connect_timeout) = config.connect_timeout {
            endpoint = endpoint.connect_timeout(connect_timeout);
        }
        if let Some(interval) = config.http2_keep_alive_interval {
            endpoint = endpoint.http2_keep_alive_interval(interval);
        }
        if let Some(timeout) = config.keep_alive_timeout {
            endpoint = endpoint.keep_alive_timeout(timeout);
        }
        if let Some(while_idle) = config.keep_alive_while_idle {
            endpoint = endpoint.keep_alive_while_idle(while_idle);
        }
        if let Some(limit) = config.concurrency_limit {
            endpoint = endpoint.concurrency_limit(limit);
        }
        if let Some((limit, duration)) = config.rate_limit {
            endpoint = endpoint.rate_limit(limit, duration);
        }
        if let Some(sz) = config.initial_stream_window_size {
            endpoint = endpoint.initial_stream_window_size(sz);
        }
        if let Some(sz) = config.initial_connection_window_size {
            endpoint = endpoint.initial_connection_window_size(sz);
        }
        if let Some(sz) = config.max_frame_size {
            endpoint = endpoint.max_frame_size(sz);
        }
        if let Some(size) = config.http2_header_table_size {
            endpoint = endpoint.http2_header_table_size(size);
        }
        if let Some(size) = config.http2_max_header_list_size {
            endpoint = endpoint.http2_max_header_list_size(size);
        }
        if let Some(adaptive) = config.http2_adaptive_window {
            endpoint = endpoint.http2_adaptive_window(adaptive);
        }
        if let Some(sz) = config.buffer_size {
            endpoint = endpoint.buffer_size(sz);
        }

        let grpc = proto::JobQueueWorkerServiceClient::connect(endpoint).await?;
        Ok(Self {
            config,
            http: reqwest::Client::new(),
            grpc,
        })
    }

    /// Runs a sequential worker loop until the queue stream ends or an error occurs.
    pub async fn run<F, Fut>(&mut self, mut handler: F) -> Result<()>
    where
        F: FnMut(Job) -> Fut,
        Fut: Future<Output = std::result::Result<Vec<u8>, String>>,
    {
        let token = self.fetch_token().await?;
        let request = authorize(
            proto::AcceptJobsRequest {
                current_job_id: String::new(),
            },
            &token,
        )?;

        let mut stream = self.grpc.accept_jobs(request).await?.into_inner();
        while let Some(response) = stream.message().await? {
            let Some(proto::accept_jobs_response::Message::Job(job)) = response.message else {
                continue;
            };

            let job = Job {
                id: job.job_id,
                port: job.port,
                path: job.path,
                input: job.input,
            };
            let job_id = job.id.clone();
            match handler(job).await {
                Ok(output) => self.complete_job(&token, job_id, output).await?,
                Err(e) => {
                    error!("Job {job_id} failed: {e}");
                    self.reject_job(&token, job_id).await?
                }
            }
        }

        Ok(())
    }

    async fn fetch_token(&self) -> Result<String> {
        #[derive(serde::Deserialize)]
        struct TokenResponse {
            jwt: Option<String>,
        }

        let uri = format!(
            "{}/v1/token",
            self.config.metadata_uri.trim_end_matches('/')
        );
        let response = self.http.get(uri).header("Metadata", "true").send().await?;
        if !response.status().is_success() {
            return Err(Error::TokenStatus(response.status()));
        }
        let token = response.json::<TokenResponse>().await?.jwt;
        token
            .filter(|token| !token.is_empty())
            .ok_or(Error::MissingToken)
    }

    async fn complete_job(&mut self, token: &str, job_id: String, output: Vec<u8>) -> Result<()> {
        let request = authorize(proto::CompleteJobRequest { job_id, output }, token)?;
        let _response = self.grpc.complete_job(request).await?;
        Ok(())
    }

    async fn reject_job(&mut self, token: &str, job_id: String) -> Result<()> {
        let request = authorize(proto::RejectJobRequest { job_id }, token)?;
        let _response = self.grpc.reject_job(request).await?;
        Ok(())
    }
}

fn authorize<T>(message: T, token: &str) -> Result<Request<T>> {
    let value = MetadataValue::try_from(format!("Bearer {token}"))?;
    let mut request = Request::new(message);
    let _ = request.metadata_mut().insert("authorization", value);
    Ok(request)
}

mod proto {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub(crate) struct AcceptJobsRequest {
        #[prost(string, tag = "1")]
        pub(crate) current_job_id: String,
    }

    #[derive(Clone, PartialEq, ::prost::Message)]
    pub(crate) struct AcceptJobsResponse {
        #[prost(oneof = "accept_jobs_response::Message", tags = "1, 2")]
        pub(crate) message: Option<accept_jobs_response::Message>,
    }

    pub(crate) mod accept_jobs_response {
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub(crate) enum Message {
            #[prost(message, tag = "1")]
            Heartbeat(::prost_types::Any),
            #[prost(message, tag = "2")]
            Job(super::Job),
        }
    }

    #[derive(Clone, PartialEq, ::prost::Message)]
    pub(crate) struct Job {
        #[prost(string, tag = "1")]
        pub(crate) job_id: String,
        #[prost(int32, tag = "2")]
        pub(crate) port: i32,
        #[prost(string, tag = "3")]
        pub(crate) path: String,
        #[prost(bytes = "vec", tag = "4")]
        pub(crate) input: Vec<u8>,
    }

    #[derive(Clone, PartialEq, ::prost::Message)]
    pub(crate) struct CompleteJobRequest {
        #[prost(string, tag = "1")]
        pub(crate) job_id: String,
        #[prost(bytes = "vec", tag = "2")]
        pub(crate) output: Vec<u8>,
    }

    #[derive(Clone, PartialEq, ::prost::Message)]
    pub(crate) struct RejectJobRequest {
        #[prost(string, tag = "1")]
        pub(crate) job_id: String,
    }

    #[derive(Debug, Clone)]
    pub(crate) struct JobQueueWorkerServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }

    impl JobQueueWorkerServiceClient<tonic::transport::Channel> {
        pub(crate) async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<tonic::codegen::StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }

    impl<T> JobQueueWorkerServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::Body>,
        T::Error: Into<tonic::codegen::StdError>,
        T::ResponseBody: tonic::codegen::Body<Data = tonic::codegen::Bytes> + Send + 'static,
        <T::ResponseBody as tonic::codegen::Body>::Error: Into<tonic::codegen::StdError> + Send,
    {
        pub(crate) fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }

        pub(crate) async fn accept_jobs(
            &mut self,
            request: impl tonic::IntoRequest<AcceptJobsRequest>,
        ) -> Result<tonic::Response<tonic::codec::Streaming<AcceptJobsResponse>>, tonic::Status>
        {
            self.inner.ready().await.map_err(|error| {
                tonic::Status::unknown(format!("service was not ready: {}", error.into()))
            })?;
            let path = tonic::codegen::http::uri::PathAndQuery::from_static(
                "/salad.grpc.saladcloud_job_queue_worker.v1alpha.JobQueueWorkerService/AcceptJobs",
            );
            self.inner
                .server_streaming(
                    request.into_request(),
                    path,
                    tonic_prost::ProstCodec::default(),
                )
                .await
        }

        pub(crate) async fn complete_job(
            &mut self,
            request: impl tonic::IntoRequest<CompleteJobRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|error| {
                tonic::Status::unknown(format!("service was not ready: {}", error.into()))
            })?;
            let path = tonic::codegen::http::uri::PathAndQuery::from_static(
                "/salad.grpc.saladcloud_job_queue_worker.v1alpha.JobQueueWorkerService/CompleteJob",
            );
            self.inner
                .unary(
                    request.into_request(),
                    path,
                    tonic_prost::ProstCodec::default(),
                )
                .await
        }

        pub(crate) async fn reject_job(
            &mut self,
            request: impl tonic::IntoRequest<RejectJobRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|error| {
                tonic::Status::unknown(format!("service was not ready: {}", error.into()))
            })?;
            let path = tonic::codegen::http::uri::PathAndQuery::from_static(
                "/salad.grpc.saladcloud_job_queue_worker.v1alpha.JobQueueWorkerService/RejectJob",
            );
            self.inner
                .unary(
                    request.into_request(),
                    path,
                    tonic_prost::ProstCodec::default(),
                )
                .await
        }
    }
}
