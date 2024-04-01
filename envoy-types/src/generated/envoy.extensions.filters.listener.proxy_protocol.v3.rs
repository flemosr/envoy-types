#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProxyProtocol {
    /// The list of rules to apply to requests.
    #[prost(message, repeated, tag = "1")]
    pub rules: ::prost::alloc::vec::Vec<proxy_protocol::Rule>,
    /// Allow requests through that don't use proxy protocol. Defaults to false.
    ///
    /// .. attention::
    ///
    /// This breaks conformance with the specification.
    /// Only enable if ALL traffic to the listener comes from a trusted source.
    /// For more information on the security implications of this feature, see
    /// <https://www.haproxy.org/download/2.1/doc/proxy-protocol.txt>
    ///
    /// .. attention::
    ///
    /// Requests of 12 or fewer bytes that match the proxy protocol v2 signature
    /// and requests of 6 or fewer bytes that match the proxy protocol v1
    /// signature will timeout (Envoy is unable to differentiate these requests
    /// from incomplete proxy protocol requests).
    #[prost(bool, tag = "2")]
    pub allow_requests_without_proxy_protocol: bool,
    /// This config controls which TLVs can be passed to filter state if it is Proxy Protocol
    /// V2 header. If there is no setting for this field, no TLVs will be passed through.
    ///
    /// .. note::
    ///
    /// If this is configured, you likely also want to set
    /// :ref:`core.v3.ProxyProtocolConfig.pass_through_tlvs <envoy_v3_api_field_config.core.v3.ProxyProtocolConfig.pass_through_tlvs>`,
    /// which controls pass-through for the upstream.
    #[prost(message, optional, tag = "3")]
    pub pass_through_tlvs: ::core::option::Option<
        super::super::super::super::super::config::core::v3::ProxyProtocolPassThroughTlVs,
    >,
    /// The PROXY protocol versions that won't be matched. Useful to limit the scope and attack surface of the filter.
    ///
    /// When the filter receives PROXY protocol data that is disallowed, it will reject the connection.
    /// By default, the filter will match all PROXY protocol versions.
    /// See <https://www.haproxy.org/download/2.1/doc/proxy-protocol.txt> for details.
    ///
    /// .. attention::
    ///
    /// When used in conjunction with the :ref:`allow_requests_without_proxy_protocol <envoy_v3_api_field_extensions.filters.listener.proxy_protocol.v3.ProxyProtocol.allow_requests_without_proxy_protocol>`,
    /// the filter will not attempt to match signatures for the disallowed versions.
    /// For example, when `disallowed_versions=V2`, `allow_requests_without_proxy_protocol=true`,
    /// and an incoming request matches the V2 signature, the filter will allow the request through without any modification.
    /// The filter treats this request as if it did not have any PROXY protocol information.
    #[prost(
        enumeration = "super::super::super::super::super::config::core::v3::proxy_protocol_config::Version",
        repeated,
        tag = "4"
    )]
    pub disallowed_versions: ::prost::alloc::vec::Vec<i32>,
}
/// Nested message and enum types in `ProxyProtocol`.
pub mod proxy_protocol {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct KeyValuePair {
        /// The namespace — if this is empty, the filter's namespace will be used.
        #[prost(string, tag = "1")]
        pub metadata_namespace: ::prost::alloc::string::String,
        /// The key to use within the namespace.
        #[prost(string, tag = "2")]
        pub key: ::prost::alloc::string::String,
    }
    /// A Rule defines what metadata to apply when a header is present or missing.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Rule {
        /// The type that triggers the rule - required
        /// TLV type is defined as uint8_t in proxy protocol. See `the spec <<https://www.haproxy.org/download/2.1/doc/proxy-protocol.txt>`\_> for details.
        #[prost(uint32, tag = "1")]
        pub tlv_type: u32,
        /// If the TLV type is present, apply this metadata KeyValuePair.
        #[prost(message, optional, tag = "2")]
        pub on_tlv_present: ::core::option::Option<KeyValuePair>,
    }
}
