/// \\[\#not-implemented-hide:\\]
/// An events envoy sends to the management server.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamEventsRequest {
    /// Identifier data that will only be sent in the first message on the stream. This is effectively
    /// structured metadata and is a performance optimization.
    #[prost(message, optional, tag = "1")]
    pub identifier: ::core::option::Option<stream_events_request::Identifier>,
    /// Batch of events. When the stream is already active, it will be the events occurred
    /// since the last message had been sent. If the server receives unknown event type, it should
    /// silently ignore it.
    ///
    /// The following events are supported:
    ///
    /// * :ref:`HealthCheckEvent <envoy_v3_api_msg_data.core.v3.HealthCheckEvent>`
    /// * :ref:`OutlierDetectionEvent <envoy_v3_api_msg_data.cluster.v3.OutlierDetectionEvent>`
    #[prost(message, repeated, tag = "2")]
    pub events: ::prost::alloc::vec::Vec<
        super::super::super::super::google::protobuf::Any,
    >,
}
/// Nested message and enum types in `StreamEventsRequest`.
pub mod stream_events_request {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Identifier {
        /// The node sending the event messages over the stream.
        #[prost(message, optional, tag = "1")]
        pub node: ::core::option::Option<
            super::super::super::super::config::core::v3::Node,
        >,
    }
}
/// \\[\#not-implemented-hide:\\]
/// The management server may send envoy a StreamEventsResponse to tell which events the server
/// is interested in. In future, with aggregated event reporting service, this message will
/// contain, for example, clusters the envoy should send events for, or event types the server
/// wants to process.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamEventsResponse {}
/// Generated client implementations.
pub mod event_reporting_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// \[\#not-implemented-hide:\]
    /// Service for streaming different types of events from Envoy to a server. The examples of
    /// such events may be health check or outlier detection events.
    #[derive(Debug, Clone)]
    pub struct EventReportingServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> EventReportingServiceClient<T>
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
        ) -> EventReportingServiceClient<InterceptedService<T, F>>
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
            EventReportingServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Envoy will connect and send StreamEventsRequest messages forever.
        /// The management server may send StreamEventsResponse to configure event stream. See below.
        /// This API is designed for high throughput with the expectation that it might be lossy.
        pub async fn stream_events(
            &mut self,
            request: impl tonic::IntoStreamingRequest<
                Message = super::StreamEventsRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::StreamEventsResponse>>,
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
                "/envoy.service.event_reporting.v3.EventReportingService/StreamEvents",
            );
            let mut req = request.into_streaming_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "envoy.service.event_reporting.v3.EventReportingService",
                        "StreamEvents",
                    ),
                );
            self.inner.streaming(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod event_reporting_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with EventReportingServiceServer.
    #[async_trait]
    pub trait EventReportingService: Send + Sync + 'static {
        /// Server streaming response type for the StreamEvents method.
        type StreamEventsStream: futures_core::Stream<
                Item = std::result::Result<super::StreamEventsResponse, tonic::Status>,
            >
            + Send
            + 'static;
        /// Envoy will connect and send StreamEventsRequest messages forever.
        /// The management server may send StreamEventsResponse to configure event stream. See below.
        /// This API is designed for high throughput with the expectation that it might be lossy.
        async fn stream_events(
            &self,
            request: tonic::Request<tonic::Streaming<super::StreamEventsRequest>>,
        ) -> std::result::Result<
            tonic::Response<Self::StreamEventsStream>,
            tonic::Status,
        >;
    }
    /// \[\#not-implemented-hide:\]
    /// Service for streaming different types of events from Envoy to a server. The examples of
    /// such events may be health check or outlier detection events.
    #[derive(Debug)]
    pub struct EventReportingServiceServer<T: EventReportingService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: EventReportingService> EventReportingServiceServer<T> {
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
    for EventReportingServiceServer<T>
    where
        T: EventReportingService,
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
                "/envoy.service.event_reporting.v3.EventReportingService/StreamEvents" => {
                    #[allow(non_camel_case_types)]
                    struct StreamEventsSvc<T: EventReportingService>(pub Arc<T>);
                    impl<
                        T: EventReportingService,
                    > tonic::server::StreamingService<super::StreamEventsRequest>
                    for StreamEventsSvc<T> {
                        type Response = super::StreamEventsResponse;
                        type ResponseStream = T::StreamEventsStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                tonic::Streaming<super::StreamEventsRequest>,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).stream_events(request).await
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
                        let method = StreamEventsSvc(inner);
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
    impl<T: EventReportingService> Clone for EventReportingServiceServer<T> {
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
    impl<T: EventReportingService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: EventReportingService> tonic::server::NamedService
    for EventReportingServiceServer<T> {
        const NAME: &'static str = "envoy.service.event_reporting.v3.EventReportingService";
    }
}
