// This file is @generated by prost-build.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WorkloadMetricRequest {
    #[prost(string, tag = "1")]
    pub namespace: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub cluster: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WorkloadMetricReply {
    #[prost(message, repeated, tag = "1")]
    pub workload: ::prost::alloc::vec::Vec<Workload>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Workload {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub workload_type: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub cluster: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "6")]
    pub vulnerabilities: ::prost::alloc::vec::Vec<VulnerabilityMetrics>,
}
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct VulnerabilityMetrics {
    #[prost(int32, tag = "1")]
    pub critical: i32,
    #[prost(int32, tag = "2")]
    pub high: i32,
    #[prost(int32, tag = "3")]
    pub medium: i32,
    #[prost(int32, tag = "4")]
    pub low: i32,
    #[prost(int32, tag = "5")]
    pub unassigned: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WorkloadVulnerabilityDetailsRequest {
    #[prost(string, tag = "1")]
    pub namespace: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub cluster: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub workload: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub workload_type: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WorkloadVulnerabilityDetailsReply {
    #[prost(message, repeated, tag = "1")]
    pub vulnerability_details: ::prost::alloc::vec::Vec<VulnerabilityDetails>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VulnerabilityDetails {
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub vuln_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub source: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub description: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub detail: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub recommendation: ::prost::alloc::string::String,
    #[prost(string, tag = "7")]
    pub created: ::prost::alloc::string::String,
    #[prost(string, tag = "8")]
    pub published: ::prost::alloc::string::String,
    #[prost(string, tag = "9")]
    pub updated: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "10")]
    pub cwes: ::prost::alloc::vec::Vec<Cwe>,
    #[prost(enumeration = "Severity", tag = "11")]
    pub severity: i32,
    #[prost(message, repeated, tag = "13")]
    pub aliases: ::prost::alloc::vec::Vec<VulnerabilityAlias>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Cwe {
    #[prost(int32, tag = "1")]
    pub cwe_id: i32,
    #[prost(string, tag = "2")]
    pub cwe_name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VulnerabilityAlias {
    #[prost(string, tag = "1")]
    pub cve_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub ghsa_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub osv_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub gsd_id: ::prost::alloc::string::String,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Severity {
    Unknown = 0,
    Critical = 1,
    High = 2,
    Medium = 3,
    Low = 4,
    Unassigned = 5,
}
impl Severity {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::Unknown => "UNKNOWN",
            Self::Critical => "CRITICAL",
            Self::High => "HIGH",
            Self::Medium => "MEDIUM",
            Self::Low => "LOW",
            Self::Unassigned => "UNASSIGNED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "UNKNOWN" => Some(Self::Unknown),
            "CRITICAL" => Some(Self::Critical),
            "HIGH" => Some(Self::High),
            "MEDIUM" => Some(Self::Medium),
            "LOW" => Some(Self::Low),
            "UNASSIGNED" => Some(Self::Unassigned),
            _ => None,
        }
    }
}
/// Generated client implementations.
pub mod vulnerabilities_client {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct VulnerabilitiesClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl VulnerabilitiesClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> VulnerabilitiesClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + std::marker::Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + std::marker::Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> VulnerabilitiesClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + std::marker::Send + std::marker::Sync,
        {
            VulnerabilitiesClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        pub async fn get_workloads_vulnerability_metrics(
            &mut self,
            request: impl tonic::IntoRequest<super::WorkloadMetricRequest>,
        ) -> std::result::Result<
            tonic::Response<super::WorkloadMetricReply>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/vulnerabilities.Vulnerabilities/GetWorkloadsVulnerabilityMetrics",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "vulnerabilities.Vulnerabilities",
                        "GetWorkloadsVulnerabilityMetrics",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_workload_vulnerability_details(
            &mut self,
            request: impl tonic::IntoRequest<super::WorkloadVulnerabilityDetailsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::WorkloadVulnerabilityDetailsReply>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/vulnerabilities.Vulnerabilities/GetWorkloadVulnerabilityDetails",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "vulnerabilities.Vulnerabilities",
                        "GetWorkloadVulnerabilityDetails",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod vulnerabilities_server {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with VulnerabilitiesServer.
    #[async_trait]
    pub trait Vulnerabilities: std::marker::Send + std::marker::Sync + 'static {
        async fn get_workloads_vulnerability_metrics(
            &self,
            request: tonic::Request<super::WorkloadMetricRequest>,
        ) -> std::result::Result<
            tonic::Response<super::WorkloadMetricReply>,
            tonic::Status,
        >;
        async fn get_workload_vulnerability_details(
            &self,
            request: tonic::Request<super::WorkloadVulnerabilityDetailsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::WorkloadVulnerabilityDetailsReply>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct VulnerabilitiesServer<T> {
        inner: Arc<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    impl<T> VulnerabilitiesServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None,
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.max_decoding_message_size = Some(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.max_encoding_message_size = Some(limit);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for VulnerabilitiesServer<T>
    where
        T: Vulnerabilities,
        B: Body + std::marker::Send + 'static,
        B::Error: Into<StdError> + std::marker::Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            match req.uri().path() {
                "/vulnerabilities.Vulnerabilities/GetWorkloadsVulnerabilityMetrics" => {
                    #[allow(non_camel_case_types)]
                    struct GetWorkloadsVulnerabilityMetricsSvc<T: Vulnerabilities>(
                        pub Arc<T>,
                    );
                    impl<
                        T: Vulnerabilities,
                    > tonic::server::UnaryService<super::WorkloadMetricRequest>
                    for GetWorkloadsVulnerabilityMetricsSvc<T> {
                        type Response = super::WorkloadMetricReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::WorkloadMetricRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Vulnerabilities>::get_workloads_vulnerability_metrics(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = GetWorkloadsVulnerabilityMetricsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/vulnerabilities.Vulnerabilities/GetWorkloadVulnerabilityDetails" => {
                    #[allow(non_camel_case_types)]
                    struct GetWorkloadVulnerabilityDetailsSvc<T: Vulnerabilities>(
                        pub Arc<T>,
                    );
                    impl<
                        T: Vulnerabilities,
                    > tonic::server::UnaryService<
                        super::WorkloadVulnerabilityDetailsRequest,
                    > for GetWorkloadVulnerabilityDetailsSvc<T> {
                        type Response = super::WorkloadVulnerabilityDetailsReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::WorkloadVulnerabilityDetailsRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Vulnerabilities>::get_workload_vulnerability_details(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = GetWorkloadVulnerabilityDetailsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        let mut response = http::Response::new(empty_body());
                        let headers = response.headers_mut();
                        headers
                            .insert(
                                tonic::Status::GRPC_STATUS,
                                (tonic::Code::Unimplemented as i32).into(),
                            );
                        headers
                            .insert(
                                http::header::CONTENT_TYPE,
                                tonic::metadata::GRPC_CONTENT_TYPE,
                            );
                        Ok(response)
                    })
                }
            }
        }
    }
    impl<T> Clone for VulnerabilitiesServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
                max_decoding_message_size: self.max_decoding_message_size,
                max_encoding_message_size: self.max_encoding_message_size,
            }
        }
    }
    /// Generated gRPC service name
    pub const SERVICE_NAME: &str = "vulnerabilities.Vulnerabilities";
    impl<T> tonic::server::NamedService for VulnerabilitiesServer<T> {
        const NAME: &'static str = SERVICE_NAME;
    }
}
