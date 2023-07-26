/// \\[\#not-implemented-hide:\\] Not configuration. Workaround c++ protobuf issue with importing
/// services: <https://github.com/google/protobuf/issues/4221> and protoxform to upgrade the file.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RdsDummy {}
/// Generated client implementations.
pub mod route_discovery_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// The resource_names field in DiscoveryRequest specifies a route configuration.
    /// This allows an Envoy configuration with multiple HTTP listeners (and
    /// associated HTTP connection manager filters) to use different route
    /// configurations. Each listener will bind its HTTP connection manager filter to
    /// a route table via this identifier.
    #[derive(Debug, Clone)]
    pub struct RouteDiscoveryServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> RouteDiscoveryServiceClient<T>
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
        ) -> RouteDiscoveryServiceClient<InterceptedService<T, F>>
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
            RouteDiscoveryServiceClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn stream_routes(
            &mut self,
            request: impl tonic::IntoStreamingRequest<
                Message = super::super::super::discovery::v3::DiscoveryRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<
                tonic::codec::Streaming<
                    super::super::super::discovery::v3::DiscoveryResponse,
                >,
            >,
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
                "/envoy.service.route.v3.RouteDiscoveryService/StreamRoutes",
            );
            let mut req = request.into_streaming_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "envoy.service.route.v3.RouteDiscoveryService",
                        "StreamRoutes",
                    ),
                );
            self.inner.streaming(req, path, codec).await
        }
        pub async fn delta_routes(
            &mut self,
            request: impl tonic::IntoStreamingRequest<
                Message = super::super::super::discovery::v3::DeltaDiscoveryRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<
                tonic::codec::Streaming<
                    super::super::super::discovery::v3::DeltaDiscoveryResponse,
                >,
            >,
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
                "/envoy.service.route.v3.RouteDiscoveryService/DeltaRoutes",
            );
            let mut req = request.into_streaming_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "envoy.service.route.v3.RouteDiscoveryService",
                        "DeltaRoutes",
                    ),
                );
            self.inner.streaming(req, path, codec).await
        }
        pub async fn fetch_routes(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::discovery::v3::DiscoveryRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::super::discovery::v3::DiscoveryResponse>,
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
                "/envoy.service.route.v3.RouteDiscoveryService/FetchRoutes",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "envoy.service.route.v3.RouteDiscoveryService",
                        "FetchRoutes",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated client implementations.
pub mod virtual_host_discovery_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Virtual Host Discovery Service (VHDS) is used to dynamically update the list of virtual hosts for
    /// a given RouteConfiguration. If VHDS is configured a virtual host list update will be triggered
    /// during the processing of an HTTP request if a route for the request cannot be resolved. The
    /// :ref:`resource_names_subscribe <envoy_v3_api_field_service.discovery.v3.DeltaDiscoveryRequest.resource_names_subscribe>`
    /// field contains a list of virtual host names or aliases to track. The contents of an alias would
    /// be the contents of a `host` or `authority` header used to make an http request. An xDS server
    /// will match an alias to a virtual host based on the content of :ref:`domains' <envoy_v3_api_field_config.route.v3.VirtualHost.domains>` field. The `resource_names_unsubscribe` field
    /// contains a list of virtual host names that have been :ref:`unsubscribed <xds_protocol_unsubscribe>` from the routing table associated with the RouteConfiguration.
    #[derive(Debug, Clone)]
    pub struct VirtualHostDiscoveryServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> VirtualHostDiscoveryServiceClient<T>
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
        ) -> VirtualHostDiscoveryServiceClient<InterceptedService<T, F>>
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
            VirtualHostDiscoveryServiceClient::new(
                InterceptedService::new(inner, interceptor),
            )
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
        pub async fn delta_virtual_hosts(
            &mut self,
            request: impl tonic::IntoStreamingRequest<
                Message = super::super::super::discovery::v3::DeltaDiscoveryRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<
                tonic::codec::Streaming<
                    super::super::super::discovery::v3::DeltaDiscoveryResponse,
                >,
            >,
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
                "/envoy.service.route.v3.VirtualHostDiscoveryService/DeltaVirtualHosts",
            );
            let mut req = request.into_streaming_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "envoy.service.route.v3.VirtualHostDiscoveryService",
                        "DeltaVirtualHosts",
                    ),
                );
            self.inner.streaming(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod route_discovery_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with RouteDiscoveryServiceServer.
    #[async_trait]
    pub trait RouteDiscoveryService: Send + Sync + 'static {
        /// Server streaming response type for the StreamRoutes method.
        type StreamRoutesStream: futures_core::Stream<
                Item = std::result::Result<
                    super::super::super::discovery::v3::DiscoveryResponse,
                    tonic::Status,
                >,
            >
            + Send
            + 'static;
        async fn stream_routes(
            &self,
            request: tonic::Request<
                tonic::Streaming<super::super::super::discovery::v3::DiscoveryRequest>,
            >,
        ) -> std::result::Result<
            tonic::Response<Self::StreamRoutesStream>,
            tonic::Status,
        >;
        /// Server streaming response type for the DeltaRoutes method.
        type DeltaRoutesStream: futures_core::Stream<
                Item = std::result::Result<
                    super::super::super::discovery::v3::DeltaDiscoveryResponse,
                    tonic::Status,
                >,
            >
            + Send
            + 'static;
        async fn delta_routes(
            &self,
            request: tonic::Request<
                tonic::Streaming<
                    super::super::super::discovery::v3::DeltaDiscoveryRequest,
                >,
            >,
        ) -> std::result::Result<
            tonic::Response<Self::DeltaRoutesStream>,
            tonic::Status,
        >;
        async fn fetch_routes(
            &self,
            request: tonic::Request<super::super::super::discovery::v3::DiscoveryRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::discovery::v3::DiscoveryResponse>,
            tonic::Status,
        >;
    }
    /// The resource_names field in DiscoveryRequest specifies a route configuration.
    /// This allows an Envoy configuration with multiple HTTP listeners (and
    /// associated HTTP connection manager filters) to use different route
    /// configurations. Each listener will bind its HTTP connection manager filter to
    /// a route table via this identifier.
    #[derive(Debug)]
    pub struct RouteDiscoveryServiceServer<T: RouteDiscoveryService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: RouteDiscoveryService> RouteDiscoveryServiceServer<T> {
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
    for RouteDiscoveryServiceServer<T>
    where
        T: RouteDiscoveryService,
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
                "/envoy.service.route.v3.RouteDiscoveryService/StreamRoutes" => {
                    #[allow(non_camel_case_types)]
                    struct StreamRoutesSvc<T: RouteDiscoveryService>(pub Arc<T>);
                    impl<
                        T: RouteDiscoveryService,
                    > tonic::server::StreamingService<
                        super::super::super::discovery::v3::DiscoveryRequest,
                    > for StreamRoutesSvc<T> {
                        type Response = super::super::super::discovery::v3::DiscoveryResponse;
                        type ResponseStream = T::StreamRoutesStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                tonic::Streaming<
                                    super::super::super::discovery::v3::DiscoveryRequest,
                                >,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).stream_routes(request).await
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
                        let method = StreamRoutesSvc(inner);
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
                "/envoy.service.route.v3.RouteDiscoveryService/DeltaRoutes" => {
                    #[allow(non_camel_case_types)]
                    struct DeltaRoutesSvc<T: RouteDiscoveryService>(pub Arc<T>);
                    impl<
                        T: RouteDiscoveryService,
                    > tonic::server::StreamingService<
                        super::super::super::discovery::v3::DeltaDiscoveryRequest,
                    > for DeltaRoutesSvc<T> {
                        type Response = super::super::super::discovery::v3::DeltaDiscoveryResponse;
                        type ResponseStream = T::DeltaRoutesStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                tonic::Streaming<
                                    super::super::super::discovery::v3::DeltaDiscoveryRequest,
                                >,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).delta_routes(request).await
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
                        let method = DeltaRoutesSvc(inner);
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
                "/envoy.service.route.v3.RouteDiscoveryService/FetchRoutes" => {
                    #[allow(non_camel_case_types)]
                    struct FetchRoutesSvc<T: RouteDiscoveryService>(pub Arc<T>);
                    impl<
                        T: RouteDiscoveryService,
                    > tonic::server::UnaryService<
                        super::super::super::discovery::v3::DiscoveryRequest,
                    > for FetchRoutesSvc<T> {
                        type Response = super::super::super::discovery::v3::DiscoveryResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::super::discovery::v3::DiscoveryRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).fetch_routes(request).await
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
                        let method = FetchRoutesSvc(inner);
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
    impl<T: RouteDiscoveryService> Clone for RouteDiscoveryServiceServer<T> {
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
    impl<T: RouteDiscoveryService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: RouteDiscoveryService> tonic::server::NamedService
    for RouteDiscoveryServiceServer<T> {
        const NAME: &'static str = "envoy.service.route.v3.RouteDiscoveryService";
    }
}
/// Generated server implementations.
pub mod virtual_host_discovery_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with VirtualHostDiscoveryServiceServer.
    #[async_trait]
    pub trait VirtualHostDiscoveryService: Send + Sync + 'static {
        /// Server streaming response type for the DeltaVirtualHosts method.
        type DeltaVirtualHostsStream: futures_core::Stream<
                Item = std::result::Result<
                    super::super::super::discovery::v3::DeltaDiscoveryResponse,
                    tonic::Status,
                >,
            >
            + Send
            + 'static;
        async fn delta_virtual_hosts(
            &self,
            request: tonic::Request<
                tonic::Streaming<
                    super::super::super::discovery::v3::DeltaDiscoveryRequest,
                >,
            >,
        ) -> std::result::Result<
            tonic::Response<Self::DeltaVirtualHostsStream>,
            tonic::Status,
        >;
    }
    /// Virtual Host Discovery Service (VHDS) is used to dynamically update the list of virtual hosts for
    /// a given RouteConfiguration. If VHDS is configured a virtual host list update will be triggered
    /// during the processing of an HTTP request if a route for the request cannot be resolved. The
    /// :ref:`resource_names_subscribe <envoy_v3_api_field_service.discovery.v3.DeltaDiscoveryRequest.resource_names_subscribe>`
    /// field contains a list of virtual host names or aliases to track. The contents of an alias would
    /// be the contents of a `host` or `authority` header used to make an http request. An xDS server
    /// will match an alias to a virtual host based on the content of :ref:`domains' <envoy_v3_api_field_config.route.v3.VirtualHost.domains>` field. The `resource_names_unsubscribe` field
    /// contains a list of virtual host names that have been :ref:`unsubscribed <xds_protocol_unsubscribe>` from the routing table associated with the RouteConfiguration.
    #[derive(Debug)]
    pub struct VirtualHostDiscoveryServiceServer<T: VirtualHostDiscoveryService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: VirtualHostDiscoveryService> VirtualHostDiscoveryServiceServer<T> {
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
    for VirtualHostDiscoveryServiceServer<T>
    where
        T: VirtualHostDiscoveryService,
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
                "/envoy.service.route.v3.VirtualHostDiscoveryService/DeltaVirtualHosts" => {
                    #[allow(non_camel_case_types)]
                    struct DeltaVirtualHostsSvc<T: VirtualHostDiscoveryService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: VirtualHostDiscoveryService,
                    > tonic::server::StreamingService<
                        super::super::super::discovery::v3::DeltaDiscoveryRequest,
                    > for DeltaVirtualHostsSvc<T> {
                        type Response = super::super::super::discovery::v3::DeltaDiscoveryResponse;
                        type ResponseStream = T::DeltaVirtualHostsStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                tonic::Streaming<
                                    super::super::super::discovery::v3::DeltaDiscoveryRequest,
                                >,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).delta_virtual_hosts(request).await
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
                        let method = DeltaVirtualHostsSvc(inner);
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
    impl<T: VirtualHostDiscoveryService> Clone for VirtualHostDiscoveryServiceServer<T> {
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
    impl<T: VirtualHostDiscoveryService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: VirtualHostDiscoveryService> tonic::server::NamedService
    for VirtualHostDiscoveryServiceServer<T> {
        const NAME: &'static str = "envoy.service.route.v3.VirtualHostDiscoveryService";
    }
}
/// \\[\#not-implemented-hide:\\] Not configuration. Workaround c++ protobuf issue with importing
/// services: <https://github.com/google/protobuf/issues/4221> and protoxform to upgrade the file.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SrdsDummy {}
/// Generated client implementations.
pub mod scoped_routes_discovery_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// The Scoped Routes Discovery Service (SRDS) API distributes
    /// :ref:`ScopedRouteConfiguration<envoy_v3_api_msg.ScopedRouteConfiguration>`
    /// resources. Each ScopedRouteConfiguration resource represents a "routing
    /// scope" containing a mapping that allows the HTTP connection manager to
    /// dynamically assign a routing table (specified via a
    /// :ref:`RouteConfiguration<envoy_v3_api_msg_config.route.v3.RouteConfiguration>` message) to each
    /// HTTP request.
    #[derive(Debug, Clone)]
    pub struct ScopedRoutesDiscoveryServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> ScopedRoutesDiscoveryServiceClient<T>
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
        ) -> ScopedRoutesDiscoveryServiceClient<InterceptedService<T, F>>
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
            ScopedRoutesDiscoveryServiceClient::new(
                InterceptedService::new(inner, interceptor),
            )
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
        pub async fn stream_scoped_routes(
            &mut self,
            request: impl tonic::IntoStreamingRequest<
                Message = super::super::super::discovery::v3::DiscoveryRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<
                tonic::codec::Streaming<
                    super::super::super::discovery::v3::DiscoveryResponse,
                >,
            >,
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
                "/envoy.service.route.v3.ScopedRoutesDiscoveryService/StreamScopedRoutes",
            );
            let mut req = request.into_streaming_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "envoy.service.route.v3.ScopedRoutesDiscoveryService",
                        "StreamScopedRoutes",
                    ),
                );
            self.inner.streaming(req, path, codec).await
        }
        pub async fn delta_scoped_routes(
            &mut self,
            request: impl tonic::IntoStreamingRequest<
                Message = super::super::super::discovery::v3::DeltaDiscoveryRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<
                tonic::codec::Streaming<
                    super::super::super::discovery::v3::DeltaDiscoveryResponse,
                >,
            >,
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
                "/envoy.service.route.v3.ScopedRoutesDiscoveryService/DeltaScopedRoutes",
            );
            let mut req = request.into_streaming_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "envoy.service.route.v3.ScopedRoutesDiscoveryService",
                        "DeltaScopedRoutes",
                    ),
                );
            self.inner.streaming(req, path, codec).await
        }
        pub async fn fetch_scoped_routes(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::discovery::v3::DiscoveryRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::super::discovery::v3::DiscoveryResponse>,
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
                "/envoy.service.route.v3.ScopedRoutesDiscoveryService/FetchScopedRoutes",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "envoy.service.route.v3.ScopedRoutesDiscoveryService",
                        "FetchScopedRoutes",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod scoped_routes_discovery_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with ScopedRoutesDiscoveryServiceServer.
    #[async_trait]
    pub trait ScopedRoutesDiscoveryService: Send + Sync + 'static {
        /// Server streaming response type for the StreamScopedRoutes method.
        type StreamScopedRoutesStream: futures_core::Stream<
                Item = std::result::Result<
                    super::super::super::discovery::v3::DiscoveryResponse,
                    tonic::Status,
                >,
            >
            + Send
            + 'static;
        async fn stream_scoped_routes(
            &self,
            request: tonic::Request<
                tonic::Streaming<super::super::super::discovery::v3::DiscoveryRequest>,
            >,
        ) -> std::result::Result<
            tonic::Response<Self::StreamScopedRoutesStream>,
            tonic::Status,
        >;
        /// Server streaming response type for the DeltaScopedRoutes method.
        type DeltaScopedRoutesStream: futures_core::Stream<
                Item = std::result::Result<
                    super::super::super::discovery::v3::DeltaDiscoveryResponse,
                    tonic::Status,
                >,
            >
            + Send
            + 'static;
        async fn delta_scoped_routes(
            &self,
            request: tonic::Request<
                tonic::Streaming<
                    super::super::super::discovery::v3::DeltaDiscoveryRequest,
                >,
            >,
        ) -> std::result::Result<
            tonic::Response<Self::DeltaScopedRoutesStream>,
            tonic::Status,
        >;
        async fn fetch_scoped_routes(
            &self,
            request: tonic::Request<super::super::super::discovery::v3::DiscoveryRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::discovery::v3::DiscoveryResponse>,
            tonic::Status,
        >;
    }
    /// The Scoped Routes Discovery Service (SRDS) API distributes
    /// :ref:`ScopedRouteConfiguration<envoy_v3_api_msg.ScopedRouteConfiguration>`
    /// resources. Each ScopedRouteConfiguration resource represents a "routing
    /// scope" containing a mapping that allows the HTTP connection manager to
    /// dynamically assign a routing table (specified via a
    /// :ref:`RouteConfiguration<envoy_v3_api_msg_config.route.v3.RouteConfiguration>` message) to each
    /// HTTP request.
    #[derive(Debug)]
    pub struct ScopedRoutesDiscoveryServiceServer<T: ScopedRoutesDiscoveryService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: ScopedRoutesDiscoveryService> ScopedRoutesDiscoveryServiceServer<T> {
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
    for ScopedRoutesDiscoveryServiceServer<T>
    where
        T: ScopedRoutesDiscoveryService,
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
                "/envoy.service.route.v3.ScopedRoutesDiscoveryService/StreamScopedRoutes" => {
                    #[allow(non_camel_case_types)]
                    struct StreamScopedRoutesSvc<T: ScopedRoutesDiscoveryService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: ScopedRoutesDiscoveryService,
                    > tonic::server::StreamingService<
                        super::super::super::discovery::v3::DiscoveryRequest,
                    > for StreamScopedRoutesSvc<T> {
                        type Response = super::super::super::discovery::v3::DiscoveryResponse;
                        type ResponseStream = T::StreamScopedRoutesStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                tonic::Streaming<
                                    super::super::super::discovery::v3::DiscoveryRequest,
                                >,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).stream_scoped_routes(request).await
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
                        let method = StreamScopedRoutesSvc(inner);
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
                "/envoy.service.route.v3.ScopedRoutesDiscoveryService/DeltaScopedRoutes" => {
                    #[allow(non_camel_case_types)]
                    struct DeltaScopedRoutesSvc<T: ScopedRoutesDiscoveryService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: ScopedRoutesDiscoveryService,
                    > tonic::server::StreamingService<
                        super::super::super::discovery::v3::DeltaDiscoveryRequest,
                    > for DeltaScopedRoutesSvc<T> {
                        type Response = super::super::super::discovery::v3::DeltaDiscoveryResponse;
                        type ResponseStream = T::DeltaScopedRoutesStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                tonic::Streaming<
                                    super::super::super::discovery::v3::DeltaDiscoveryRequest,
                                >,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).delta_scoped_routes(request).await
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
                        let method = DeltaScopedRoutesSvc(inner);
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
                "/envoy.service.route.v3.ScopedRoutesDiscoveryService/FetchScopedRoutes" => {
                    #[allow(non_camel_case_types)]
                    struct FetchScopedRoutesSvc<T: ScopedRoutesDiscoveryService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: ScopedRoutesDiscoveryService,
                    > tonic::server::UnaryService<
                        super::super::super::discovery::v3::DiscoveryRequest,
                    > for FetchScopedRoutesSvc<T> {
                        type Response = super::super::super::discovery::v3::DiscoveryResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::super::discovery::v3::DiscoveryRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).fetch_scoped_routes(request).await
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
                        let method = FetchScopedRoutesSvc(inner);
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
    impl<T: ScopedRoutesDiscoveryService> Clone
    for ScopedRoutesDiscoveryServiceServer<T> {
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
    impl<T: ScopedRoutesDiscoveryService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: ScopedRoutesDiscoveryService> tonic::server::NamedService
    for ScopedRoutesDiscoveryServiceServer<T> {
        const NAME: &'static str = "envoy.service.route.v3.ScopedRoutesDiscoveryService";
    }
}
