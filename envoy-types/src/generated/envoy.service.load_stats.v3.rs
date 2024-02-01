/// A load report Envoy sends to the management server.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LoadStatsRequest {
    /// Node identifier for Envoy instance.
    #[prost(message, optional, tag = "1")]
    pub node: ::core::option::Option<super::super::super::config::core::v3::Node>,
    /// A list of load stats to report.
    #[prost(message, repeated, tag = "2")]
    pub cluster_stats: ::prost::alloc::vec::Vec<
        super::super::super::config::endpoint::v3::ClusterStats,
    >,
}
/// The management server sends envoy a LoadStatsResponse with all clusters it
/// is interested in learning load stats about.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LoadStatsResponse {
    /// Clusters to report stats for.
    /// Not populated if `send_all_clusters` is true.
    #[prost(string, repeated, tag = "1")]
    pub clusters: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// If true, the client should send all clusters it knows about.
    /// Only clients that advertise the "envoy.lrs.supports_send_all_clusters" capability in their
    /// :ref:`client_features<envoy_v3_api_field_config.core.v3.Node.client_features>` field will honor this field.
    #[prost(bool, tag = "4")]
    pub send_all_clusters: bool,
    /// The minimum interval of time to collect stats over. This is only a minimum for two reasons:
    ///
    /// 1. There may be some delay from when the timer fires until stats sampling occurs.
    /// 1. For clusters that were already feature in the previous `LoadStatsResponse`, any traffic
    ///    that is observed in between the corresponding previous `LoadStatsRequest` and this
    ///    `LoadStatsResponse` will also be accumulated and billed to the cluster. This avoids a period
    ///    of inobservability that might otherwise exists between the messages. New clusters are not
    ///    subject to this consideration.
    #[prost(message, optional, tag = "2")]
    pub load_reporting_interval: ::core::option::Option<
        super::super::super::super::google::protobuf::Duration,
    >,
    /// Set to `true` if the management server supports endpoint granularity
    /// report.
    #[prost(bool, tag = "3")]
    pub report_endpoint_granularity: bool,
}
/// Generated client implementations.
pub mod load_reporting_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct LoadReportingServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> LoadReportingServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
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
        ) -> LoadReportingServiceClient<InterceptedService<T, F>>
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
            >>::Error: Into<StdError> + Send + Sync,
        {
            LoadReportingServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Advanced API to allow for multi-dimensional load balancing by remote
        /// server. For receiving LB assignments, the steps are:
        /// 1, The management server is configured with per cluster/zone/load metric
        /// capacity configuration. The capacity configuration definition is
        /// outside of the scope of this document.
        /// 2. Envoy issues a standard {Stream,Fetch}Endpoints request for the clusters
        /// to balance.
        ///
        /// Independently, Envoy will initiate a StreamLoadStats bidi stream with a
        /// management server:
        ///
        /// 1. Once a connection establishes, the management server publishes a
        ///   LoadStatsResponse for all clusters it is interested in learning load
        ///   stats about.
        /// 1. For each cluster, Envoy load balances incoming traffic to upstream hosts
        ///   based on per-zone weights and/or per-instance weights (if specified)
        ///   based on intra-zone LbPolicy. This information comes from the above
        ///   {Stream,Fetch}Endpoints.
        /// 1. When upstream hosts reply, they optionally add header <define header
        ///   name> with ASCII representation of EndpointLoadMetricStats.
        /// 1. Envoy aggregates load reports over the period of time given to it in
        ///   LoadStatsResponse.load_reporting_interval. This includes aggregation
        ///   stats Envoy maintains by itself (total_requests, rpc_errors etc.) as
        ///   well as load metrics from upstream hosts.
        /// 1. When the timer of load_reporting_interval expires, Envoy sends new
        ///   LoadStatsRequest filled with load reports for each cluster.
        /// 1. The management server uses the load reports from all reported Envoys
        ///   from around the world, computes global assignment and prepares traffic
        ///   assignment destined for each zone Envoys are located in. Goto 2.
        pub async fn stream_load_stats(
            &mut self,
            request: impl tonic::IntoStreamingRequest<Message = super::LoadStatsRequest>,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::LoadStatsResponse>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/envoy.service.load_stats.v3.LoadReportingService/StreamLoadStats",
            );
            let mut req = request.into_streaming_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "envoy.service.load_stats.v3.LoadReportingService",
                        "StreamLoadStats",
                    ),
                );
            self.inner.streaming(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod load_reporting_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with LoadReportingServiceServer.
    #[async_trait]
    pub trait LoadReportingService: Send + Sync + 'static {
        /// Server streaming response type for the StreamLoadStats method.
        type StreamLoadStatsStream: tonic::codegen::tokio_stream::Stream<
                Item = std::result::Result<super::LoadStatsResponse, tonic::Status>,
            >
            + Send
            + 'static;
        /// Advanced API to allow for multi-dimensional load balancing by remote
        /// server. For receiving LB assignments, the steps are:
        /// 1, The management server is configured with per cluster/zone/load metric
        /// capacity configuration. The capacity configuration definition is
        /// outside of the scope of this document.
        /// 2. Envoy issues a standard {Stream,Fetch}Endpoints request for the clusters
        /// to balance.
        ///
        /// Independently, Envoy will initiate a StreamLoadStats bidi stream with a
        /// management server:
        ///
        /// 1. Once a connection establishes, the management server publishes a
        ///   LoadStatsResponse for all clusters it is interested in learning load
        ///   stats about.
        /// 1. For each cluster, Envoy load balances incoming traffic to upstream hosts
        ///   based on per-zone weights and/or per-instance weights (if specified)
        ///   based on intra-zone LbPolicy. This information comes from the above
        ///   {Stream,Fetch}Endpoints.
        /// 1. When upstream hosts reply, they optionally add header <define header
        ///   name> with ASCII representation of EndpointLoadMetricStats.
        /// 1. Envoy aggregates load reports over the period of time given to it in
        ///   LoadStatsResponse.load_reporting_interval. This includes aggregation
        ///   stats Envoy maintains by itself (total_requests, rpc_errors etc.) as
        ///   well as load metrics from upstream hosts.
        /// 1. When the timer of load_reporting_interval expires, Envoy sends new
        ///   LoadStatsRequest filled with load reports for each cluster.
        /// 1. The management server uses the load reports from all reported Envoys
        ///   from around the world, computes global assignment and prepares traffic
        ///   assignment destined for each zone Envoys are located in. Goto 2.
        async fn stream_load_stats(
            &self,
            request: tonic::Request<tonic::Streaming<super::LoadStatsRequest>>,
        ) -> std::result::Result<
            tonic::Response<Self::StreamLoadStatsStream>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct LoadReportingServiceServer<T: LoadReportingService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: LoadReportingService> LoadReportingServiceServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
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
    impl<T, B> tonic::codegen::Service<http::Request<B>>
    for LoadReportingServiceServer<T>
    where
        T: LoadReportingService,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
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
            let inner = self.inner.clone();
            match req.uri().path() {
                "/envoy.service.load_stats.v3.LoadReportingService/StreamLoadStats" => {
                    #[allow(non_camel_case_types)]
                    struct StreamLoadStatsSvc<T: LoadReportingService>(pub Arc<T>);
                    impl<
                        T: LoadReportingService,
                    > tonic::server::StreamingService<super::LoadStatsRequest>
                    for StreamLoadStatsSvc<T> {
                        type Response = super::LoadStatsResponse;
                        type ResponseStream = T::StreamLoadStatsStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                tonic::Streaming<super::LoadStatsRequest>,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as LoadReportingService>::stream_load_stats(
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
                        let inner = inner.0;
                        let method = StreamLoadStatsSvc(inner);
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
                        let res = grpc.streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: LoadReportingService> Clone for LoadReportingServiceServer<T> {
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
    impl<T: LoadReportingService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: LoadReportingService> tonic::server::NamedService
    for LoadReportingServiceServer<T> {
        const NAME: &'static str = "envoy.service.load_stats.v3.LoadReportingService";
    }
}
