/// External Authorization filter calls out to an external service over the
/// gRPC Authorization API defined by
/// :ref:`CheckRequest <envoy_v3_api_msg_service.auth.v3.CheckRequest>`.
/// A failed check will cause this filter to close the TCP connection.
/// [#next-free-field: 8]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExtAuthz {
    /// The prefix to use when emitting statistics.
    #[prost(string, tag = "1")]
    pub stat_prefix: ::prost::alloc::string::String,
    /// The external authorization gRPC service configuration.
    /// The default timeout is set to 200ms by this filter.
    #[prost(message, optional, tag = "2")]
    pub grpc_service: ::core::option::Option<
        super::super::super::super::super::config::core::v3::GrpcService,
    >,
    /// The filter's behaviour in case the external authorization service does
    /// not respond back. When it is set to true, Envoy will also allow traffic in case of
    /// communication failure between authorization service and the proxy.
    /// Defaults to false.
    #[prost(bool, tag = "3")]
    pub failure_mode_allow: bool,
    /// Specifies if the peer certificate is sent to the external service.
    ///
    /// When this field is true, Envoy will include the peer X.509 certificate, if available, in the
    /// :ref:`certificate<envoy_v3_api_field_service.auth.v3.AttributeContext.Peer.certificate>`.
    #[prost(bool, tag = "4")]
    pub include_peer_certificate: bool,
    /// API version for ext_authz transport protocol. This describes the ext_authz gRPC endpoint and
    /// version of Check{Request,Response} used on the wire.
    #[prost(
        enumeration = "super::super::super::super::super::config::core::v3::ApiVersion",
        tag = "5"
    )]
    pub transport_api_version: i32,
    /// Specifies if the filter is enabled with metadata matcher.
    /// If this field is not specified, the filter will be enabled for all requests.
    #[prost(message, optional, tag = "6")]
    pub filter_enabled_metadata: ::core::option::Option<
        super::super::super::super::super::r#type::matcher::v3::MetadataMatcher,
    >,
    /// Optional labels that will be passed to :ref:`labels<envoy_v3_api_field_service.auth.v3.AttributeContext.Peer.labels>` in
    /// :ref:`destination<envoy_v3_api_field_service.auth.v3.AttributeContext.destination>`.
    /// The labels will be read from :ref:`metadata<envoy_v3_api_msg_config.core.v3.Node>` with the specified key.
    #[prost(string, tag = "7")]
    pub bootstrap_metadata_labels_key: ::prost::alloc::string::String,
}
