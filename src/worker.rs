//! SaladCloud Job Queue worker support.

use std::error::Error as StdError;
use std::fmt;
use std::future::Future;
use std::time::Duration;

use serde::de::DeserializeOwned;
use tonic::metadata::MetadataValue;
use tonic::transport::{Channel, Endpoint};
use tonic::{Request, Status};

/// Result type used by the SaladCloud worker client.
pub type Result<T> = std::result::Result<T, Error>;

/// Errors returned by the SaladCloud worker client.
#[derive(Debug)]
pub enum Error {
    /// The SaladCloud metadata service returned an invalid or empty token response.
    MissingToken,
    /// The SaladCloud metadata service returned a non-success HTTP response.
    TokenStatus(reqwest::StatusCode),
    /// HTTP client error while talking to IMDS.
    Reqwest(reqwest::Error),
    /// JSON decode error.
    Json(serde_json::Error),
    /// gRPC transport setup error.
    Transport(tonic::transport::Error),
    /// gRPC status returned by the queue worker service.
    Status(Status),
    /// Invalid gRPC authorization metadata.
    Metadata(tonic::metadata::errors::InvalidMetadataValue),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingToken => write!(f, "missing SaladCloud workload token"),
            Self::TokenStatus(status) => {
                write!(f, "failed to fetch SaladCloud workload token: {status}")
            }
            Self::Reqwest(error) => write!(f, "SaladCloud metadata request failed: {error}"),
            Self::Json(error) => write!(f, "JSON error: {error}"),
            Self::Transport(error) => write!(f, "SaladCloud worker transport failed: {error}"),
            Self::Status(status) => write!(f, "SaladCloud worker gRPC failed: {status}"),
            Self::Metadata(error) => write!(f, "invalid SaladCloud worker metadata: {error}"),
        }
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Self::Reqwest(error) => Some(error),
            Self::Json(error) => Some(error),
            Self::Transport(error) => Some(error),
            Self::Status(error) => Some(error),
            Self::Metadata(error) => Some(error),
            Self::MissingToken | Self::TokenStatus(_) => None,
        }
    }
}

impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Self {
        Self::Reqwest(error)
    }
}

impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Self {
        Self::Json(error)
    }
}

impl From<tonic::transport::Error> for Error {
    fn from(error: tonic::transport::Error) -> Self {
        Self::Transport(error)
    }
}

impl From<Status> for Error {
    fn from(error: Status) -> Self {
        Self::Status(error)
    }
}

impl From<tonic::metadata::errors::InvalidMetadataValue> for Error {
    fn from(error: tonic::metadata::errors::InvalidMetadataValue) -> Self {
        Self::Metadata(error)
    }
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
}

impl Default for WorkerConfig {
    fn default() -> Self {
        Self {
            metadata_uri: "http://169.254.169.254:80".to_string(),
            service_endpoint: "job-queue-worker-api.salad.com:443".to_string(),
            use_tls: true,
        }
    }
}

impl WorkerConfig {
    /// Builds worker configuration from SaladCloud worker environment variables.
    pub fn from_env() -> Self {
        let mut config = Self::default();
        if let Ok(metadata_uri) = std::env::var("SALAD_METADATA_URI") {
            config.metadata_uri = metadata_uri;
        }
        if let Ok(service_endpoint) = std::env::var("SALAD_SERVICE_ENDPOINT") {
            config.service_endpoint = service_endpoint;
        }
        if let Ok(use_tls) = std::env::var("SALAD_SERVICE_ENDPOINT_TLS") {
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
        let endpoint = Endpoint::from_shared(config.service_endpoint_uri())?
            .user_agent("saladcloud-rust-worker/0.1 tonic")?
            .timeout(Duration::from_secs(30));
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
        let mut request = Request::new(proto::AcceptJobsRequest {
            current_job_id: String::new(),
        });
        authorize(&mut request, &token)?;

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
                Err(_) => self.reject_job(&token, job_id).await?,
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
        let mut request = Request::new(proto::CompleteJobRequest { job_id, output });
        authorize(&mut request, token)?;
        let _response = self.grpc.complete_job(request).await?;
        Ok(())
    }

    async fn reject_job(&mut self, token: &str, job_id: String) -> Result<()> {
        let mut request = Request::new(proto::RejectJobRequest { job_id });
        authorize(&mut request, token)?;
        let _response = self.grpc.reject_job(request).await?;
        Ok(())
    }
}

fn authorize<T>(request: &mut Request<T>, token: &str) -> Result<()> {
    let value = MetadataValue::try_from(format!("Bearer {token}"))?;
    let _previous = request.metadata_mut().insert("authorization", value);
    Ok(())
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
