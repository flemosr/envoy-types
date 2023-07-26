/// This represents the different types of messages that Envoy can send
/// to an external processing server.
/// [#next-free-field: 8]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProcessingRequest {
    /// Specify whether the filter that sent this request is running in synchronous
    /// or asynchronous mode. The choice of synchronous or asynchronous mode
    /// can be set in the filter configuration, and defaults to false.
    ///
    /// * A value of ``false`` indicates that the server must respond
    ///    to this message by either sending back a matching ProcessingResponse message,
    ///    or by closing the stream.
    /// * A value of ``true`` indicates that the server must not respond to this
    ///    message, although it may still close the stream to indicate that no more messages
    ///    are needed.
    ///
    #[prost(bool, tag = "1")]
    pub async_mode: bool,
    /// Each request message will include one of the following sub-messages. Which
    /// ones are set for a particular HTTP request/response depend on the
    /// processing mode.
    #[prost(oneof = "processing_request::Request", tags = "2, 3, 4, 5, 6, 7")]
    pub request: ::core::option::Option<processing_request::Request>,
}
/// Nested message and enum types in `ProcessingRequest`.
pub mod processing_request {
    /// Each request message will include one of the following sub-messages. Which
    /// ones are set for a particular HTTP request/response depend on the
    /// processing mode.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Request {
        /// Information about the HTTP request headers, as well as peer info and additional
        /// properties. Unless ``async_mode`` is ``true``, the server must send back a
        /// HeaderResponse message, an ImmediateResponse message, or close the stream.
        #[prost(message, tag = "2")]
        RequestHeaders(super::HttpHeaders),
        /// Information about the HTTP response headers, as well as peer info and additional
        /// properties. Unless ``async_mode`` is ``true``, the server must send back a
        /// HeaderResponse message or close the stream.
        #[prost(message, tag = "3")]
        ResponseHeaders(super::HttpHeaders),
        /// A chunk of the HTTP request body. Unless ``async_mode`` is true, the server must send back
        /// a BodyResponse message, an ImmediateResponse message, or close the stream.
        #[prost(message, tag = "4")]
        RequestBody(super::HttpBody),
        /// A chunk of the HTTP request body. Unless ``async_mode`` is ``true``, the server must send back
        /// a BodyResponse message or close the stream.
        #[prost(message, tag = "5")]
        ResponseBody(super::HttpBody),
        /// The HTTP trailers for the request path. Unless ``async_mode`` is ``true``, the server
        /// must send back a TrailerResponse message or close the stream.
        ///
        /// This message is only sent if the trailers processing mode is set to ``SEND``.
        /// If there are no trailers on the original downstream request, then this message
        /// will only be sent (with empty trailers waiting to be populated) if the
        /// processing mode is set before the request headers are sent, such as
        /// in the filter configuration.
        #[prost(message, tag = "6")]
        RequestTrailers(super::HttpTrailers),
        /// The HTTP trailers for the response path. Unless ``async_mode`` is ``true``, the server
        /// must send back a TrailerResponse message or close the stream.
        ///
        /// This message is only sent if the trailers processing mode is set to ``SEND``.
        /// If there are no trailers on the original downstream request, then this message
        /// will only be sent (with empty trailers waiting to be populated) if the
        /// processing mode is set before the request headers are sent, such as
        /// in the filter configuration.
        #[prost(message, tag = "7")]
        ResponseTrailers(super::HttpTrailers),
    }
}
/// For every ProcessingRequest received by the server with the ``async_mode`` field
/// set to false, the server must send back exactly one ProcessingResponse message.
/// [#next-free-field: 11]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProcessingResponse {
    /// \[#not-implemented-hide:\]
    /// Optional metadata that will be emitted as dynamic metadata to be consumed by the next
    /// filter. This metadata will be placed in the namespace ``envoy.filters.http.ext_proc``.
    #[prost(message, optional, tag = "8")]
    pub dynamic_metadata: ::core::option::Option<
        super::super::super::super::google::protobuf::Struct,
    >,
    /// Override how parts of the HTTP request and response are processed
    /// for the duration of this particular request/response only. Servers
    /// may use this to intelligently control how requests are processed
    /// based on the headers and other metadata that they see.
    #[prost(message, optional, tag = "9")]
    pub mode_override: ::core::option::Option<
        super::super::super::extensions::filters::http::ext_proc::v3::ProcessingMode,
    >,
    /// When ext_proc server receives a request message, in case it needs more
    /// time to process the message, it sends back a ProcessingResponse message
    /// with a new timeout value. When Envoy receives this response message,
    /// it ignores other fields in the response, just stop the original timer,
    /// which has the timeout value specified in
    /// :ref:`message_timeout
    /// <envoy_v3_api_field_extensions.filters.http.ext_proc.v3.ExternalProcessor.message_timeout>`
    /// and start a new timer with this ``override_message_timeout`` value and keep the
    /// Envoy ext_proc filter state machine intact.
    /// Has to be >= 1ms and <=
    /// :ref:`max_message_timeout <envoy_v3_api_field_extensions.filters.http.ext_proc.v3.ExternalProcessor.max_message_timeout>`
    /// Such message can be sent at most once in a particular Envoy ext_proc filter processing state.
    /// To enable this API, one has to set ``max_message_timeout`` to a number >= 1ms.
    #[prost(message, optional, tag = "10")]
    pub override_message_timeout: ::core::option::Option<
        super::super::super::super::google::protobuf::Duration,
    >,
    #[prost(oneof = "processing_response::Response", tags = "1, 2, 3, 4, 5, 6, 7")]
    pub response: ::core::option::Option<processing_response::Response>,
}
/// Nested message and enum types in `ProcessingResponse`.
pub mod processing_response {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Response {
        /// The server must send back this message in response to a message with the
        /// ``request_headers`` field set.
        #[prost(message, tag = "1")]
        RequestHeaders(super::HeadersResponse),
        /// The server must send back this message in response to a message with the
        /// ``response_headers`` field set.
        #[prost(message, tag = "2")]
        ResponseHeaders(super::HeadersResponse),
        /// The server must send back this message in response to a message with
        /// the ``request_body`` field set.
        #[prost(message, tag = "3")]
        RequestBody(super::BodyResponse),
        /// The server must send back this message in response to a message with
        /// the ``response_body`` field set.
        #[prost(message, tag = "4")]
        ResponseBody(super::BodyResponse),
        /// The server must send back this message in response to a message with
        /// the ``request_trailers`` field set.
        #[prost(message, tag = "5")]
        RequestTrailers(super::TrailersResponse),
        /// The server must send back this message in response to a message with
        /// the ``response_trailers`` field set.
        #[prost(message, tag = "6")]
        ResponseTrailers(super::TrailersResponse),
        /// If specified, attempt to create a locally generated response, send it
        /// downstream, and stop processing additional filters and ignore any
        /// additional messages received from the remote server for this request or
        /// response. If a response has already started -- for example, if this
        /// message is sent response to a ``response_body`` message -- then
        /// this will either ship the reply directly to the downstream codec,
        /// or reset the stream.
        #[prost(message, tag = "7")]
        ImmediateResponse(super::ImmediateResponse),
    }
}
/// This message is sent to the external server when the HTTP request and responses
/// are first received.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HttpHeaders {
    /// The HTTP request headers. All header keys will be
    /// lower-cased, because HTTP header keys are case-insensitive.
    #[prost(message, optional, tag = "1")]
    pub headers: ::core::option::Option<
        super::super::super::config::core::v3::HeaderMap,
    >,
    /// \[#not-implemented-hide:\]
    /// The values of properties selected by the ``request_attributes``
    /// or ``response_attributes`` list in the configuration. Each entry
    /// in the list is populated
    /// from the standard :ref:`attributes <arch_overview_attributes>`
    /// supported across Envoy.
    #[prost(map = "string, message", tag = "2")]
    pub attributes: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        super::super::super::super::google::protobuf::Struct,
    >,
    /// If true, then there is no message body associated with this
    /// request or response.
    #[prost(bool, tag = "3")]
    pub end_of_stream: bool,
}
/// This message contains the message body that Envoy sends to the external server.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HttpBody {
    #[prost(bytes = "vec", tag = "1")]
    pub body: ::prost::alloc::vec::Vec<u8>,
    #[prost(bool, tag = "2")]
    pub end_of_stream: bool,
}
/// This message contains the trailers.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HttpTrailers {
    #[prost(message, optional, tag = "1")]
    pub trailers: ::core::option::Option<
        super::super::super::config::core::v3::HeaderMap,
    >,
}
/// This message must be sent in response to an HttpHeaders message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HeadersResponse {
    #[prost(message, optional, tag = "1")]
    pub response: ::core::option::Option<CommonResponse>,
}
/// This message must be sent in response to an HttpTrailers message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TrailersResponse {
    /// Instructions on how to manipulate the trailers
    #[prost(message, optional, tag = "1")]
    pub header_mutation: ::core::option::Option<HeaderMutation>,
}
/// This message must be sent in response to an HttpBody message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BodyResponse {
    #[prost(message, optional, tag = "1")]
    pub response: ::core::option::Option<CommonResponse>,
}
/// This message contains common fields between header and body responses.
/// [#next-free-field: 6]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommonResponse {
    /// If set, provide additional direction on how the Envoy proxy should
    /// handle the rest of the HTTP filter chain.
    #[prost(enumeration = "common_response::ResponseStatus", tag = "1")]
    pub status: i32,
    /// Instructions on how to manipulate the headers. When responding to an
    /// HttpBody request, header mutations will only take effect if
    /// the current processing mode for the body is BUFFERED.
    #[prost(message, optional, tag = "2")]
    pub header_mutation: ::core::option::Option<HeaderMutation>,
    /// Replace the body of the last message sent to the remote server on this
    /// stream. If responding to an HttpBody request, simply replace or clear
    /// the body chunk that was sent with that request. Body mutations only take
    /// effect in response to ``body`` messages and are ignored otherwise.
    #[prost(message, optional, tag = "3")]
    pub body_mutation: ::core::option::Option<BodyMutation>,
    /// \[#not-implemented-hide:\]
    /// Add new trailers to the message. This may be used when responding to either a
    /// HttpHeaders or HttpBody message, but only if this message is returned
    /// along with the CONTINUE_AND_REPLACE status.
    #[prost(message, optional, tag = "4")]
    pub trailers: ::core::option::Option<
        super::super::super::config::core::v3::HeaderMap,
    >,
    /// Clear the route cache for the current request.
    /// This is necessary if the remote server
    /// modified headers that are used to calculate the route.
    #[prost(bool, tag = "5")]
    pub clear_route_cache: bool,
}
/// Nested message and enum types in `CommonResponse`.
pub mod common_response {
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum ResponseStatus {
        /// Apply the mutation instructions in this message to the
        /// request or response, and then continue processing the filter
        /// stream as normal. This is the default.
        Continue = 0,
        /// Apply the specified header mutation, replace the body with the body
        /// specified in the body mutation (if present), and do not send any
        /// further messages for this request or response even if the processing
        /// mode is configured to do so.
        ///
        /// When used in response to a request_headers or response_headers message,
        /// this status makes it possible to either completely replace the body
        /// while discarding the original body, or to add a body to a message that
        /// formerly did not have one.
        ///
        /// In other words, this response makes it possible to turn an HTTP GET
        /// into a POST, PUT, or PATCH.
        ContinueAndReplace = 1,
    }
    impl ResponseStatus {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ResponseStatus::Continue => "CONTINUE",
                ResponseStatus::ContinueAndReplace => "CONTINUE_AND_REPLACE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "CONTINUE" => Some(Self::Continue),
                "CONTINUE_AND_REPLACE" => Some(Self::ContinueAndReplace),
                _ => None,
            }
        }
    }
}
/// This message causes the filter to attempt to create a locally
/// generated response, send it  downstream, stop processing
/// additional filters, and ignore any additional messages received
/// from the remote server for this request or response. If a response
/// has already started, then  this will either ship the reply directly
/// to the downstream codec, or reset the stream.
/// [#next-free-field: 6]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImmediateResponse {
    /// The response code to return
    #[prost(message, optional, tag = "1")]
    pub status: ::core::option::Option<super::super::super::r#type::v3::HttpStatus>,
    /// Apply changes to the default headers, which will include content-type.
    #[prost(message, optional, tag = "2")]
    pub headers: ::core::option::Option<HeaderMutation>,
    /// The message body to return with the response which is sent using the
    /// text/plain content type, or encoded in the grpc-message header.
    #[prost(string, tag = "3")]
    pub body: ::prost::alloc::string::String,
    /// If set, then include a gRPC status trailer.
    #[prost(message, optional, tag = "4")]
    pub grpc_status: ::core::option::Option<GrpcStatus>,
    /// A string detailing why this local reply was sent, which may be included
    /// in log and debug output (e.g. this populates the %RESPONSE_CODE_DETAILS%
    /// command operator field for use in access logging).
    #[prost(string, tag = "5")]
    pub details: ::prost::alloc::string::String,
}
/// This message specifies a gRPC status for an ImmediateResponse message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GrpcStatus {
    /// The actual gRPC status
    #[prost(uint32, tag = "1")]
    pub status: u32,
}
/// Change HTTP headers or trailers by appending, replacing, or removing
/// headers.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HeaderMutation {
    /// Add or replace HTTP headers. Attempts to set the value of
    /// any ``x-envoy`` header, and attempts to set the ``:method``,
    /// ``:authority``, ``:scheme``, or ``host`` headers will be ignored.
    #[prost(message, repeated, tag = "1")]
    pub set_headers: ::prost::alloc::vec::Vec<
        super::super::super::config::core::v3::HeaderValueOption,
    >,
    /// Remove these HTTP headers. Attempts to remove system headers --
    /// any header starting with ``:``, plus ``host`` -- will be ignored.
    #[prost(string, repeated, tag = "2")]
    pub remove_headers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Replace the entire message body chunk received in the corresponding
/// HttpBody message with this new body, or clear the body.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BodyMutation {
    #[prost(oneof = "body_mutation::Mutation", tags = "1, 2")]
    pub mutation: ::core::option::Option<body_mutation::Mutation>,
}
/// Nested message and enum types in `BodyMutation`.
pub mod body_mutation {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Mutation {
        /// The entire body to replace
        #[prost(bytes, tag = "1")]
        Body(::prost::alloc::vec::Vec<u8>),
        /// Clear the corresponding body chunk
        #[prost(bool, tag = "2")]
        ClearBody(bool),
    }
}
/// Generated client implementations.
pub mod external_processor_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct ExternalProcessorClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> ExternalProcessorClient<T>
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
        ) -> ExternalProcessorClient<InterceptedService<T, F>>
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
            ExternalProcessorClient::new(InterceptedService::new(inner, interceptor))
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
        /// This begins the bidirectional stream that Envoy will use to
        /// give the server control over what the filter does. The actual
        /// protocol is described by the ProcessingRequest and ProcessingResponse
        /// messages below.
        pub async fn process(
            &mut self,
            request: impl tonic::IntoStreamingRequest<Message = super::ProcessingRequest>,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::ProcessingResponse>>,
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
                "/envoy.service.ext_proc.v3.ExternalProcessor/Process",
            );
            let mut req = request.into_streaming_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "envoy.service.ext_proc.v3.ExternalProcessor",
                        "Process",
                    ),
                );
            self.inner.streaming(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod external_processor_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with ExternalProcessorServer.
    #[async_trait]
    pub trait ExternalProcessor: Send + Sync + 'static {
        /// Server streaming response type for the Process method.
        type ProcessStream: futures_core::Stream<
                Item = std::result::Result<super::ProcessingResponse, tonic::Status>,
            >
            + Send
            + 'static;
        /// This begins the bidirectional stream that Envoy will use to
        /// give the server control over what the filter does. The actual
        /// protocol is described by the ProcessingRequest and ProcessingResponse
        /// messages below.
        async fn process(
            &self,
            request: tonic::Request<tonic::Streaming<super::ProcessingRequest>>,
        ) -> std::result::Result<tonic::Response<Self::ProcessStream>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct ExternalProcessorServer<T: ExternalProcessor> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: ExternalProcessor> ExternalProcessorServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for ExternalProcessorServer<T>
    where
        T: ExternalProcessor,
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
                "/envoy.service.ext_proc.v3.ExternalProcessor/Process" => {
                    #[allow(non_camel_case_types)]
                    struct ProcessSvc<T: ExternalProcessor>(pub Arc<T>);
                    impl<
                        T: ExternalProcessor,
                    > tonic::server::StreamingService<super::ProcessingRequest>
                    for ProcessSvc<T> {
                        type Response = super::ProcessingResponse;
                        type ResponseStream = T::ProcessStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                tonic::Streaming<super::ProcessingRequest>,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).process(request).await };
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
                        let method = ProcessSvc(inner);
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
    impl<T: ExternalProcessor> Clone for ExternalProcessorServer<T> {
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
    impl<T: ExternalProcessor> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: ExternalProcessor> tonic::server::NamedService
    for ExternalProcessorServer<T> {
        const NAME: &'static str = "envoy.service.ext_proc.v3.ExternalProcessor";
    }
}
