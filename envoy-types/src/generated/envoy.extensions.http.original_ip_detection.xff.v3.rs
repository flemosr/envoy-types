// This file is @generated by prost-build.
/// This extension allows for the original downstream remote IP to be detected
/// by reading the :ref:`config_http_conn_man_headers_x-forwarded-for` header.
///
/// \[\#extension: envoy.http.original_ip_detection.xff\]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct XffConfig {
    ///
    /// The number of additional ingress proxy hops from the right side of the
    /// : ref:`config_http_conn_man_headers_x-forwarded-for` HTTP header to trust when
    ///   determining the origin client's IP address. The default is zero if this option
    ///   is not specified. See the documentation for
    /// : ref:`config_http_conn_man_headers_x-forwarded-for` for more information.
    ///
    ///
    /// Only one of `xff_num_trusted_hops` and `xff_trusted_cidrs` can be set.
    #[prost(uint32, tag = "1")]
    pub xff_num_trusted_hops: u32,
    ///
    /// The `CIDR <<https://tools.ietf.org/html/rfc4632>`\_> ranges to trust when
    /// evaluating the remote IP address to determine the original client's IP address.
    /// This is used instead of
    /// : ref:`use_remote_address <envoy_v3_api_field_extensions.filters.network.http_connection_manager.v3.HttpConnectionManager.use_remote_address>`.
    ///   When the remote IP address matches a trusted CIDR and the
    /// : ref:`config_http_conn_man_headers_x-forwarded-for` header was sent, each entry
    ///   in the `x-forwarded-for` header is evaluated from right to left and the first
    ///   public non-trusted address is used as the original client address. If all
    ///   addresses in `x-forwarded-for` are within the trusted list, the first (leftmost)
    ///   entry is used.
    ///
    ///
    /// This is typically used when requests are proxied by a
    /// `CDN <<https://en.wikipedia.org/wiki/Content_delivery_network>`\_.>
    ///
    /// Only one of `xff_num_trusted_hops` and `xff_trusted_cidrs` can be set.
    #[prost(message, optional, tag = "2")]
    pub xff_trusted_cidrs: ::core::option::Option<XffTrustedCidrs>,
    ///
    /// If set, Envoy will not append the remote address to the
    /// : ref:`config_http_conn_man_headers_x-forwarded-for` HTTP header.
    ///
    ///
    /// .. attention::
    ///
    /// For proper proxy behaviour it is not recommended to set this option.
    /// For backwards compatibility, if this option is unset it defaults to true.
    ///
    /// This only applies when :ref:`use_remote_address  <envoy_v3_api_field_extensions.filters.network.http_connection_manager.v3.HttpConnectionManager.use_remote_address>`
    /// is false, otherwise :ref:`skip_xff_append  <envoy_v3_api_field_extensions.filters.network.http_connection_manager.v3.HttpConnectionManager.skip_xff_append>`
    /// applies.
    #[prost(message, optional, tag = "3")]
    pub skip_xff_append: ::core::option::Option<
        super::super::super::super::super::super::google::protobuf::BoolValue,
    >,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct XffTrustedCidrs {
    /// The list of `CIDRs <<https://tools.ietf.org/html/rfc4632>`\_> from which remote
    /// connections are considered trusted.
    #[prost(message, repeated, tag = "1")]
    pub cidrs: ::prost::alloc::vec::Vec<
        super::super::super::super::super::config::core::v3::CidrRange,
    >,
}
