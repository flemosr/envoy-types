// This file is @generated by prost-build.
/// This represents the different types of messages that Envoy can send
/// to an external processing server.
/// \[\#next-free-field: 11\]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProcessingRequest {
    /// Dynamic metadata associated with the request.
    #[prost(message, optional, tag = "8")]
    pub metadata_context: ::core::option::Option<
        super::super::super::config::core::v3::Metadata,
    >,
    ///
    /// The values of properties selected by the `request_attributes`
    /// or `response_attributes` list in the configuration. Each entry
    /// in the list is populated from the standard
    /// : ref:`attributes <arch_overview_attributes>` supported across Envoy.
    #[prost(map = "string, message", tag = "9")]
    pub attributes: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        super::super::super::super::google::protobuf::Struct,
    >,
    /// Specify whether the filter that sent this request is running in :ref:`observability_mode  <envoy_v3_api_field_extensions.filters.http.ext_proc.v3.ExternalProcessor.observability_mode>`
    /// and defaults to false.
    ///
    /// * A value of `false` indicates that the server must respond
    ///   to this message by either sending back a matching ProcessingResponse message,
    ///   or by closing the stream.
    /// * A value of `true` indicates that the server should not respond to this message, as any
    ///   responses will be ignored. However, it may still close the stream to indicate that no more messages
    ///   are needed.
    #[prost(bool, tag = "10")]
    pub observability_mode: bool,
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
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Request {
        /// Information about the HTTP request headers, as well as peer info and additional
        /// properties. Unless `observability_mode` is `true`, the server must send back a
        /// HeaderResponse message, an ImmediateResponse message, or close the stream.
        #[prost(message, tag = "2")]
        RequestHeaders(super::HttpHeaders),
        /// Information about the HTTP response headers, as well as peer info and additional
        /// properties. Unless `observability_mode` is `true`, the server must send back a
        /// HeaderResponse message or close the stream.
        #[prost(message, tag = "3")]
        ResponseHeaders(super::HttpHeaders),
        /// A chunk of the HTTP request body. Unless `observability_mode` is true, the server must send back
        /// a BodyResponse message, an ImmediateResponse message, or close the stream.
        #[prost(message, tag = "4")]
        RequestBody(super::HttpBody),
        /// A chunk of the HTTP response body. Unless `observability_mode` is `true`, the server must send back
        /// a BodyResponse message or close the stream.
        #[prost(message, tag = "5")]
        ResponseBody(super::HttpBody),
        /// The HTTP trailers for the request path. Unless `observability_mode` is `true`, the server
        /// must send back a TrailerResponse message or close the stream.
        ///
        /// This message is only sent if the trailers processing mode is set to `SEND` and
        /// the original downstream request has trailers.
        #[prost(message, tag = "6")]
        RequestTrailers(super::HttpTrailers),
        /// The HTTP trailers for the response path. Unless `observability_mode` is `true`, the server
        /// must send back a TrailerResponse message or close the stream.
        ///
        /// This message is only sent if the trailers processing mode is set to `SEND` and
        /// the original upstream response has trailers.
        #[prost(message, tag = "7")]
        ResponseTrailers(super::HttpTrailers),
    }
}
/// For every ProcessingRequest received by the server with the `observability_mode` field
/// set to false, the server must send back exactly one ProcessingResponse message.
/// \[\#next-free-field: 11\]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProcessingResponse {
    /// Optional metadata that will be emitted as dynamic metadata to be consumed by
    /// following filters. This metadata will be placed in the namespace(s) specified by the top-level
    /// field name(s) of the struct.
    #[prost(message, optional, tag = "8")]
    pub dynamic_metadata: ::core::option::Option<
        super::super::super::super::google::protobuf::Struct,
    >,
    ///
    /// Override how parts of the HTTP request and response are processed
    /// for the duration of this particular request/response only. Servers
    /// may use this to intelligently control how requests are processed
    /// based on the headers and other metadata that they see.
    /// This field is only applicable when servers responding to the header requests.
    /// If it is set in the response to the body or trailer requests, it will be ignored by Envoy.
    /// It is also ignored by Envoy when the ext_proc filter config
    /// : ref:`allow_mode_override  <envoy_v3_api_field_extensions.filters.http.ext_proc.v3.ExternalProcessor.allow_mode_override>`
    /// is set to false, or
    /// : ref:`send_body_without_waiting_for_header_response  <envoy_v3_api_field_extensions.filters.http.ext_proc.v3.ExternalProcessor.send_body_without_waiting_for_header_response>`
    /// is set to true.
    #[prost(message, optional, tag = "9")]
    pub mode_override: ::core::option::Option<
        super::super::super::extensions::filters::http::ext_proc::v3::ProcessingMode,
    >,
    ///
    /// When ext_proc server receives a request message, in case it needs more
    /// time to process the message, it sends back a ProcessingResponse message
    /// with a new timeout value. When Envoy receives this response message,
    /// it ignores other fields in the response, just stop the original timer,
    /// which has the timeout value specified in
    /// : ref:`message_timeout  <envoy_v3_api_field_extensions.filters.http.ext_proc.v3.ExternalProcessor.message_timeout>`
    /// and start a new timer with this `override_message_timeout` value and keep the
    /// Envoy ext_proc filter state machine intact.
    /// Has to be >= 1ms and \<=
    /// : ref:`max_message_timeout <envoy_v3_api_field_extensions.filters.http.ext_proc.v3.ExternalProcessor.max_message_timeout>`
    /// Such message can be sent at most once in a particular Envoy ext_proc filter processing state.
    /// To enable this API, one has to set `max_message_timeout` to a number >= 1ms.
    #[prost(message, optional, tag = "10")]
    pub override_message_timeout: ::core::option::Option<
        super::super::super::super::google::protobuf::Duration,
    >,
    /// The response type that is sent by the server.
    #[prost(oneof = "processing_response::Response", tags = "1, 2, 3, 4, 5, 6, 7")]
    pub response: ::core::option::Option<processing_response::Response>,
}
/// Nested message and enum types in `ProcessingResponse`.
pub mod processing_response {
    /// The response type that is sent by the server.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Response {
        /// The server must send back this message in response to a message with the
        /// `request_headers` field set.
        #[prost(message, tag = "1")]
        RequestHeaders(super::HeadersResponse),
        /// The server must send back this message in response to a message with the
        /// `response_headers` field set.
        #[prost(message, tag = "2")]
        ResponseHeaders(super::HeadersResponse),
        /// The server must send back this message in response to a message with
        /// the `request_body` field set.
        #[prost(message, tag = "3")]
        RequestBody(super::BodyResponse),
        /// The server must send back this message in response to a message with
        /// the `response_body` field set.
        #[prost(message, tag = "4")]
        ResponseBody(super::BodyResponse),
        /// The server must send back this message in response to a message with
        /// the `request_trailers` field set.
        #[prost(message, tag = "5")]
        RequestTrailers(super::TrailersResponse),
        /// The server must send back this message in response to a message with
        /// the `response_trailers` field set.
        #[prost(message, tag = "6")]
        ResponseTrailers(super::TrailersResponse),
        /// If specified, attempt to create a locally generated response, send it
        /// downstream, and stop processing additional filters and ignore any
        /// additional messages received from the remote server for this request or
        /// response. If a response has already started -- for example, if this
        /// message is sent response to a `response_body` message -- then
        /// this will either ship the reply directly to the downstream codec,
        /// or reset the stream.
        #[prost(message, tag = "7")]
        ImmediateResponse(super::ImmediateResponse),
    }
}
/// This message is sent to the external server when the HTTP request and responses
/// are first received.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HttpHeaders {
    ///
    /// The HTTP request headers. All header keys will be
    /// lower-cased, because HTTP header keys are case-insensitive.
    /// The header value is encoded in the
    /// : ref:`raw_value <envoy_v3_api_field_config.core.v3.HeaderValue.raw_value>` field.
    #[prost(message, optional, tag = "1")]
    pub headers: ::core::option::Option<
        super::super::super::config::core::v3::HeaderMap,
    >,
    /// \[\#not-implemented-hide:\]
    /// This field is deprecated and not implemented. Attributes will be sent in
    /// the  top-level :ref:`attributes <envoy_v3_api_field_service.ext_proc.v3.ProcessingRequest.attributes`
    /// field.
    #[prost(map = "string, message", tag = "2")]
    pub attributes: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        super::super::super::super::google::protobuf::Struct,
    >,
    /// If `true`, then there is no message body associated with this
    /// request or response.
    #[prost(bool, tag = "3")]
    pub end_of_stream: bool,
}
/// This message is sent to the external server when the HTTP request and
/// response bodies are received.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HttpBody {
    /// The contents of the body in the HTTP request/response. Note that in
    /// streaming mode multiple `HttpBody` messages may be sent.
    #[prost(bytes = "vec", tag = "1")]
    pub body: ::prost::alloc::vec::Vec<u8>,
    /// If `true`, this will be the last `HttpBody` message that will be sent and no
    /// trailers will be sent for the current request/response.
    #[prost(bool, tag = "2")]
    pub end_of_stream: bool,
}
/// This message is sent to the external server when the HTTP request and
/// response trailers are received.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HttpTrailers {
    ///
    /// The header value is encoded in the
    /// : ref:`raw_value <envoy_v3_api_field_config.core.v3.HeaderValue.raw_value>` field.
    #[prost(message, optional, tag = "1")]
    pub trailers: ::core::option::Option<
        super::super::super::config::core::v3::HeaderMap,
    >,
}
/// This message is sent by the external server to Envoy after `HttpHeaders` was
/// sent to it.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HeadersResponse {
    /// Details the modifications (if any) to be made by Envoy to the current
    /// request/response.
    #[prost(message, optional, tag = "1")]
    pub response: ::core::option::Option<CommonResponse>,
}
/// This message is sent by the external server to Envoy after `HttpBody` was
/// sent to it.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BodyResponse {
    /// Details the modifications (if any) to be made by Envoy to the current
    /// request/response.
    #[prost(message, optional, tag = "1")]
    pub response: ::core::option::Option<CommonResponse>,
}
/// This message is sent by the external server to Envoy after `HttpTrailers` was
/// sent to it.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TrailersResponse {
    /// Details the modifications (if any) to be made by Envoy to the current
    /// request/response trailers.
    #[prost(message, optional, tag = "1")]
    pub header_mutation: ::core::option::Option<HeaderMutation>,
}
/// This message contains common fields between header and body responses.
/// \[\#next-free-field: 6\]
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
    ///
    /// Replace the body of the last message sent to the remote server on this
    /// stream. If responding to an HttpBody request, simply replace or clear
    /// the body chunk that was sent with that request. Body mutations may take
    /// effect in response either to `header` or `body` messages. When it is
    /// in response to `header` messages, it only take effect if the
    /// : ref:`status <envoy_v3_api_field_service.ext_proc.v3.CommonResponse.status>`
    /// is set to CONTINUE_AND_REPLACE.
    #[prost(message, optional, tag = "3")]
    pub body_mutation: ::core::option::Option<BodyMutation>,
    ///
    /// \[\#not-implemented-hide:\]
    /// Add new trailers to the message. This may be used when responding to either a
    /// HttpHeaders or HttpBody message, but only if this message is returned
    /// along with the CONTINUE_AND_REPLACE status.
    /// The header value is encoded in the
    /// : ref:`raw_value <envoy_v3_api_field_config.core.v3.HeaderValue.raw_value>` field.
    #[prost(message, optional, tag = "4")]
    pub trailers: ::core::option::Option<
        super::super::super::config::core::v3::HeaderMap,
    >,
    /// Clear the route cache for the current client request. This is necessary
    /// if the remote server modified headers that are used to calculate the route.
    /// This field is ignored in the response direction. This field is also ignored
    /// if the Envoy ext_proc filter is in the upstream filter chain.
    #[prost(bool, tag = "5")]
    pub clear_route_cache: bool,
}
/// Nested message and enum types in `CommonResponse`.
pub mod common_response {
    /// The status of the response.
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
                Self::Continue => "CONTINUE",
                Self::ContinueAndReplace => "CONTINUE_AND_REPLACE",
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
/// \[\#next-free-field: 6\]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImmediateResponse {
    /// The response code to return.
    #[prost(message, optional, tag = "1")]
    pub status: ::core::option::Option<super::super::super::r#type::v3::HttpStatus>,
    /// Apply changes to the default headers, which will include content-type.
    #[prost(message, optional, tag = "2")]
    pub headers: ::core::option::Option<HeaderMutation>,
    /// The message body to return with the response which is sent using the
    /// text/plain content type, or encoded in the grpc-message header.
    #[prost(bytes = "vec", tag = "3")]
    pub body: ::prost::alloc::vec::Vec<u8>,
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
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct GrpcStatus {
    /// The actual gRPC status.
    #[prost(uint32, tag = "1")]
    pub status: u32,
}
/// Change HTTP headers or trailers by appending, replacing, or removing
/// headers.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HeaderMutation {
    ///
    /// Add or replace HTTP headers. Attempts to set the value of
    /// any `x-envoy` header, and attempts to set the `:method`,
    /// `:authority`, `:scheme`, or `host` headers will be ignored.
    /// The header value is encoded in the
    /// : ref:`raw_value <envoy_v3_api_field_config.core.v3.HeaderValue.raw_value>` field.
    #[prost(message, repeated, tag = "1")]
    pub set_headers: ::prost::alloc::vec::Vec<
        super::super::super::config::core::v3::HeaderValueOption,
    >,
    /// Remove these HTTP headers. Attempts to remove system headers --
    /// any header starting with `:`, plus `host` -- will be ignored.
    #[prost(string, repeated, tag = "2")]
    pub remove_headers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// The body response message corresponding to FULL_DUPLEX_STREAMED body mode.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamedBodyResponse {
    /// The body response chunk that will be passed to the upstream/downstream by Envoy.
    #[prost(bytes = "vec", tag = "1")]
    pub body: ::prost::alloc::vec::Vec<u8>,
    ///
    /// The server sets this flag to true if it has received a body request with
    /// : ref:`end_of_stream <envoy_v3_api_field_service.ext_proc.v3.HttpBody.end_of_stream>` set to true,
    /// and this is the last chunk of body responses.
    #[prost(bool, tag = "2")]
    pub end_of_stream: bool,
}
/// This message specifies the body mutation the server sends to Envoy.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BodyMutation {
    /// The type of mutation for the body.
    #[prost(oneof = "body_mutation::Mutation", tags = "1, 2, 3")]
    pub mutation: ::core::option::Option<body_mutation::Mutation>,
}
/// Nested message and enum types in `BodyMutation`.
pub mod body_mutation {
    /// The type of mutation for the body.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Mutation {
        ///
        /// The entire body to replace.
        /// Should only be used when the corresponding `BodySendMode` in the
        /// : ref:`processing_mode <envoy_v3_api_field_extensions.filters.http.ext_proc.v3.ExternalProcessor.processing_mode>`
        /// is not set to `FULL_DUPLEX_STREAMED`.
        #[prost(bytes, tag = "1")]
        Body(::prost::alloc::vec::Vec<u8>),
        ///
        /// Clear the corresponding body chunk.
        /// Should only be used when the corresponding `BodySendMode` in the
        /// : ref:`processing_mode <envoy_v3_api_field_extensions.filters.http.ext_proc.v3.ExternalProcessor.processing_mode>`
        /// is not set to `FULL_DUPLEX_STREAMED`.
        /// Clear the corresponding body chunk.
        #[prost(bool, tag = "2")]
        ClearBody(bool),
        ///
        /// Must be used when the corresponding `BodySendMode` in the
        /// : ref:`processing_mode <envoy_v3_api_field_extensions.filters.http.ext_proc.v3.ExternalProcessor.processing_mode>`
        /// is set to `FULL_DUPLEX_STREAMED`.
        #[prost(message, tag = "3")]
        StreamedResponse(super::StreamedBodyResponse),
    }
}
/// Generated client implementations.
pub mod external_processor_client {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// A service that can access and modify HTTP requests and responses
    /// as part of a filter chain.
    /// The overall external processing protocol works like this:
    ///
    /// 1. Envoy sends to the service information about the HTTP request.
    /// 1. The service sends back a ProcessingResponse message that directs Envoy
    ///   to either stop processing, continue without it, or send it the
    ///   next chunk of the message body.
    /// 1. If so requested, Envoy sends the server chunks of the message body,
    ///   or the entire body at once. In either case, the server sends back
    ///   a ProcessingResponse after each message it receives.
    /// 1. If so requested, Envoy sends the server the HTTP trailers,
    ///   and the server sends back a ProcessingResponse.
    /// 1. At this point, request processing is done, and we pick up again
    ///   at step 1 when Envoy receives a response from the upstream server.
    /// 1. At any point above, if the server closes the gRPC stream cleanly,
    ///   then Envoy proceeds without consulting the server.
    /// 1. At any point above, if the server closes the gRPC stream with an error,
    ///   then Envoy returns a 500 error to the client, unless the filter
    ///   was configured to ignore errors.
    ///
    /// In other words, the process is a request/response conversation, but
    /// using a gRPC stream to make it easier for the server to
    /// maintain state.
    #[derive(Debug, Clone)]
    pub struct ExternalProcessorClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> ExternalProcessorClient<T>
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
            >>::Error: Into<StdError> + std::marker::Send + std::marker::Sync,
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
                    tonic::Status::unknown(
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
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with ExternalProcessorServer.
    #[async_trait]
    pub trait ExternalProcessor: std::marker::Send + std::marker::Sync + 'static {
        /// Server streaming response type for the Process method.
        type ProcessStream: tonic::codegen::tokio_stream::Stream<
                Item = std::result::Result<super::ProcessingResponse, tonic::Status>,
            >
            + std::marker::Send
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
    /// A service that can access and modify HTTP requests and responses
    /// as part of a filter chain.
    /// The overall external processing protocol works like this:
    ///
    /// 1. Envoy sends to the service information about the HTTP request.
    /// 1. The service sends back a ProcessingResponse message that directs Envoy
    ///   to either stop processing, continue without it, or send it the
    ///   next chunk of the message body.
    /// 1. If so requested, Envoy sends the server chunks of the message body,
    ///   or the entire body at once. In either case, the server sends back
    ///   a ProcessingResponse after each message it receives.
    /// 1. If so requested, Envoy sends the server the HTTP trailers,
    ///   and the server sends back a ProcessingResponse.
    /// 1. At this point, request processing is done, and we pick up again
    ///   at step 1 when Envoy receives a response from the upstream server.
    /// 1. At any point above, if the server closes the gRPC stream cleanly,
    ///   then Envoy proceeds without consulting the server.
    /// 1. At any point above, if the server closes the gRPC stream with an error,
    ///   then Envoy returns a 500 error to the client, unless the filter
    ///   was configured to ignore errors.
    ///
    /// In other words, the process is a request/response conversation, but
    /// using a gRPC stream to make it easier for the server to
    /// maintain state.
    #[derive(Debug)]
    pub struct ExternalProcessorServer<T> {
        inner: Arc<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    impl<T> ExternalProcessorServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for ExternalProcessorServer<T>
    where
        T: ExternalProcessor,
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
                            let fut = async move {
                                <T as ExternalProcessor>::process(&inner, request).await
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
    impl<T> Clone for ExternalProcessorServer<T> {
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
    pub const SERVICE_NAME: &str = "envoy.service.ext_proc.v3.ExternalProcessor";
    impl<T> tonic::server::NamedService for ExternalProcessorServer<T> {
        const NAME: &'static str = SERVICE_NAME;
    }
}
