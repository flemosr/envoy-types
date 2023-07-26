/// [#next-free-field: 6]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RouteConfiguration {
    /// The name of the route configuration. Reserved for future use in asynchronous route discovery.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The interface name of the service. Wildcard interface are supported in the suffix or prefix form.
    /// e.g. ``*.methods.add`` will match ``com.dev.methods.add``, ``com.prod.methods.add``, etc.
    /// ``com.dev.methods.*`` will match ``com.dev.methods.add``, ``com.dev.methods.update``, etc.
    /// Special wildcard ``*`` matching any interface.
    ///
    /// .. note::
    ///
    ///   The wildcard will not match the empty string.
    ///   e.g. ``*.methods.add`` will match ``com.dev.methods.add`` but not ``.methods.add``.
    #[prost(string, tag = "2")]
    pub interface: ::prost::alloc::string::String,
    /// Which group does the interface belong to.
    #[prost(string, tag = "3")]
    pub group: ::prost::alloc::string::String,
    /// The version number of the interface.
    #[prost(string, tag = "4")]
    pub version: ::prost::alloc::string::String,
    /// The list of routes that will be matched, in order, against incoming requests. The first route
    /// that matches will be used.
    #[prost(message, repeated, tag = "5")]
    pub routes: ::prost::alloc::vec::Vec<Route>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Route {
    /// Route matching parameters.
    #[prost(message, optional, tag = "1")]
    pub r#match: ::core::option::Option<RouteMatch>,
    /// Route request to some upstream cluster.
    #[prost(message, optional, tag = "2")]
    pub route: ::core::option::Option<RouteAction>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RouteMatch {
    /// Method level routing matching.
    #[prost(message, optional, tag = "1")]
    pub method: ::core::option::Option<MethodMatch>,
    /// Specifies a set of headers that the route should match on. The router will check the request’s
    /// headers against all the specified headers in the route config. A match will happen if all the
    /// headers in the route are present in the request with the same values (or based on presence if
    /// the value field is not in the config).
    #[prost(message, repeated, tag = "2")]
    pub headers: ::prost::alloc::vec::Vec<
        super::super::super::super::super::config::route::v3::HeaderMatcher,
    >,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RouteAction {
    /// Optional endpoint metadata match criteria used by the subset load balancer. Only endpoints in
    /// the upstream cluster with metadata matching what is set in this field will be considered for
    /// load balancing. The filter name should be specified as ``envoy.lb``.
    #[prost(message, optional, tag = "3")]
    pub metadata_match: ::core::option::Option<
        super::super::super::super::super::config::core::v3::Metadata,
    >,
    #[prost(oneof = "route_action::ClusterSpecifier", tags = "1, 2")]
    pub cluster_specifier: ::core::option::Option<route_action::ClusterSpecifier>,
}
/// Nested message and enum types in `RouteAction`.
pub mod route_action {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ClusterSpecifier {
        /// Indicates the upstream cluster to which the request should be routed.
        #[prost(string, tag = "1")]
        Cluster(::prost::alloc::string::String),
        /// Multiple upstream clusters can be specified for a given route. The
        /// request is routed to one of the upstream clusters based on weights
        /// assigned to each cluster.
        /// Currently ClusterWeight only supports the name and weight fields.
        #[prost(message, tag = "2")]
        WeightedClusters(
            super::super::super::super::super::super::config::route::v3::WeightedCluster,
        ),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MethodMatch {
    /// The name of the method.
    #[prost(message, optional, tag = "1")]
    pub name: ::core::option::Option<
        super::super::super::super::super::r#type::matcher::v3::StringMatcher,
    >,
    /// Method parameter definition.
    /// The key is the parameter index, starting from 0.
    /// The value is the parameter matching type.
    #[prost(map = "uint32, message", tag = "2")]
    pub params_match: ::std::collections::HashMap<
        u32,
        method_match::ParameterMatchSpecifier,
    >,
}
/// Nested message and enum types in `MethodMatch`.
pub mod method_match {
    /// The parameter matching type.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ParameterMatchSpecifier {
        #[prost(
            oneof = "parameter_match_specifier::ParameterMatchSpecifier",
            tags = "3, 4"
        )]
        pub parameter_match_specifier: ::core::option::Option<
            parameter_match_specifier::ParameterMatchSpecifier,
        >,
    }
    /// Nested message and enum types in `ParameterMatchSpecifier`.
    pub mod parameter_match_specifier {
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum ParameterMatchSpecifier {
            /// If specified, header match will be performed based on the value of the header.
            #[prost(string, tag = "3")]
            ExactMatch(::prost::alloc::string::String),
            /// If specified, header match will be performed based on range.
            /// The rule will match if the request header value is within this range.
            /// The entire request header value must represent an integer in base 10 notation: consisting
            /// of an optional plus or minus sign followed by a sequence of digits. The rule will not match
            /// if the header value does not represent an integer. Match will fail for empty values,
            /// floating point numbers or if only a subsequence of the header value is an integer.
            ///
            /// Examples:
            ///
            /// * For range [-10,0), route will match for header value -1, but not for 0,
            ///    "somestring", 10.9, "-1somestring"
            #[prost(message, tag = "4")]
            RangeMatch(
                super::super::super::super::super::super::super::r#type::v3::Int64Range,
            ),
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MultipleRouteConfiguration {
    /// The name of the named route configurations. This name is used in asynchronous route discovery.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The route table of the dubbo connection manager.
    #[prost(message, repeated, tag = "4")]
    pub route_config: ::prost::alloc::vec::Vec<RouteConfiguration>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Drds {
    /// Configuration source specifier.
    /// In case of ``api_config_source`` only aggregated ``api_type`` is supported.
    #[prost(message, optional, tag = "1")]
    pub config_source: ::core::option::Option<
        super::super::super::super::super::config::core::v3::ConfigSource,
    >,
    /// The name of the multiple route configuration. This allows to use different multiple route
    /// configurations. Tells which multiple route configuration should be fetched from the configuration
    /// source. Leave unspecified is also valid and means the unnamed multiple route configuration.
    #[prost(string, tag = "2")]
    pub route_config_name: ::prost::alloc::string::String,
}
/// [#next-free-field: 8]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DubboProxy {
    /// The human readable prefix to use when emitting statistics.
    #[prost(string, tag = "1")]
    pub stat_prefix: ::prost::alloc::string::String,
    /// Configure the protocol used.
    #[prost(enumeration = "ProtocolType", tag = "2")]
    pub protocol_type: i32,
    /// Configure the serialization protocol used.
    #[prost(enumeration = "SerializationType", tag = "3")]
    pub serialization_type: i32,
    /// The route table for the connection manager is static and is specified in this property.
    ///
    /// .. note::
    ///
    ///    This field is deprecated. Please use ``drds`` or ``multiple_route_config`` first.
    #[deprecated]
    #[prost(message, repeated, tag = "4")]
    pub route_config: ::prost::alloc::vec::Vec<RouteConfiguration>,
    /// A list of individual Dubbo filters that make up the filter chain for requests made to the
    /// Dubbo proxy. Order matters as the filters are processed sequentially. For backwards
    /// compatibility, if no dubbo_filters are specified, a default Dubbo router filter
    /// (``envoy.filters.dubbo.router``) is used.
    #[prost(message, repeated, tag = "5")]
    pub dubbo_filters: ::prost::alloc::vec::Vec<DubboFilter>,
    #[prost(oneof = "dubbo_proxy::RouteSpecifier", tags = "6, 7")]
    pub route_specifier: ::core::option::Option<dubbo_proxy::RouteSpecifier>,
}
/// Nested message and enum types in `DubboProxy`.
pub mod dubbo_proxy {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum RouteSpecifier {
        /// Use xDS to fetch the route configuration. It is invalid to define both ``route_config`` and ``drds``.
        #[prost(message, tag = "6")]
        Drds(super::Drds),
        #[prost(message, tag = "7")]
        MultipleRouteConfig(super::MultipleRouteConfiguration),
    }
}
/// DubboFilter configures a Dubbo filter.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DubboFilter {
    /// The name of the filter to instantiate. The name must match a supported
    /// filter.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Filter specific configuration which depends on the filter being
    /// instantiated. See the supported filters for further documentation.
    #[prost(message, optional, tag = "2")]
    pub config: ::core::option::Option<
        super::super::super::super::super::super::google::protobuf::Any,
    >,
}
/// Dubbo Protocol types supported by Envoy.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ProtocolType {
    /// the default protocol.
    Dubbo = 0,
}
impl ProtocolType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ProtocolType::Dubbo => "Dubbo",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "Dubbo" => Some(Self::Dubbo),
            _ => None,
        }
    }
}
/// Dubbo Serialization types supported by Envoy.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SerializationType {
    /// the default serialization protocol.
    Hessian2 = 0,
}
impl SerializationType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            SerializationType::Hessian2 => "Hessian2",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "Hessian2" => Some(Self::Hessian2),
            _ => None,
        }
    }
}
