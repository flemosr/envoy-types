#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Thrift {
    /// Specifies the method name that will be set on each health check request dispatched to an upstream host.
    /// Note that method name is case sensitive.
    #[prost(string, tag = "1")]
    pub method_name: ::prost::alloc::string::String,
    /// Configures the transport type to be used with the health checks. Note that
    /// :ref:`AUTO_TRANSPORT<envoy_v3_api_enum_value_extensions.filters.network.thrift_proxy.v3.TransportType.AUTO_TRANSPORT>`
    /// is not supported, and we don't honor :ref:`ThriftProtocolOptions<envoy_v3_api_msg_extensions.filters.network.thrift_proxy.v3.ThriftProtocolOptions>`
    /// since it's possible to set to :ref:`AUTO_TRANSPORT<envoy_v3_api_enum_value_extensions.filters.network.thrift_proxy.v3.TransportType.AUTO_TRANSPORT>`.
    /// [#extension-category: envoy.filters.network]
    #[prost(
        enumeration = "super::super::super::filters::network::thrift_proxy::v3::TransportType",
        tag = "2"
    )]
    pub transport: i32,
    /// Configures the protocol type to be used with the health checks. Note that
    /// :ref:`AUTO_PROTOCOL<envoy_v3_api_enum_value_extensions.filters.network.thrift_proxy.v3.ProtocolType.AUTO_PROTOCOL>`
    /// and :ref:`TWITTER<envoy_v3_api_enum_value_extensions.filters.network.thrift_proxy.v3.ProtocolType.TWITTER>`
    /// are not supported, and we don't honor :ref:`ThriftProtocolOptions<envoy_v3_api_msg_extensions.filters.network.thrift_proxy.v3.ThriftProtocolOptions>`
    /// since it's possible to set to :ref:`AUTO_PROTOCOL<envoy_v3_api_enum_value_extensions.filters.network.thrift_proxy.v3.ProtocolType.AUTO_PROTOCOL>`
    /// or :ref:`TWITTER<envoy_v3_api_enum_value_extensions.filters.network.thrift_proxy.v3.ProtocolType.TWITTER>`.
    #[prost(
        enumeration = "super::super::super::filters::network::thrift_proxy::v3::ProtocolType",
        tag = "3"
    )]
    pub protocol: i32,
}
