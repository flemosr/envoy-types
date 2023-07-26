/// This extension allows for the original downstream remote IP to be detected
/// by reading the :ref:`config_http_conn_man_headers_x-forwarded-for` header.
///
/// [#extension: envoy.http.original_ip_detection.xff]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct XffConfig {
    /// The number of additional ingress proxy hops from the right side of the
    /// :ref:`config_http_conn_man_headers_x-forwarded-for` HTTP header to trust when
    /// determining the origin client's IP address. The default is zero if this option
    /// is not specified. See the documentation for
    /// :ref:`config_http_conn_man_headers_x-forwarded-for` for more information.
    #[prost(uint32, tag = "1")]
    pub xff_num_trusted_hops: u32,
}
