/// Configuration for the route match action.
/// \[\#next-free-field: 8\]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RouteAction {
    /// The name of the route action. This should be unique across all route actions.
    #[prost(string, tag = "5")]
    pub name: ::prost::alloc::string::String,
    /// Route metadata.
    #[prost(message, optional, tag = "3")]
    pub metadata: ::core::option::Option<
        super::super::super::super::super::super::config::core::v3::Metadata,
    >,
    /// Route level config for L7 generic filters. The key should be the related :ref:`extension name <envoy_v3_api_field_config.core.v3.TypedExtensionConfig.name>` in the :ref:`generic filters <envoy_v3_api_field_extensions.filters.network.generic_proxy.v3.GenericProxy.filters>`.
    #[prost(map = "string, message", tag = "4")]
    pub per_filter_config: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        super::super::super::super::super::super::super::google::protobuf::Any,
    >,
    /// Specifies the upstream timeout for the route. If not specified, the default is 15s. This
    /// spans between the point at which the entire downstream request (i.e. end-of-stream) has been
    /// processed and when the upstream response has been completely processed. A value of 0 will
    /// disable the route's timeout.
    #[prost(message, optional, tag = "6")]
    pub timeout: ::core::option::Option<
        super::super::super::super::super::super::super::google::protobuf::Duration,
    >,
    /// Specifies the retry policy for the route. If not specified, then no retries will be performed.
    ///
    /// .. note::
    /// Only simplest retry policy is supported and only `num_retries` field is used for generic
    /// proxy. The default value for `num_retries` is 1 means that the request will be tried once
    /// and no additional retries will be performed.
    #[prost(message, optional, tag = "7")]
    pub retry_policy: ::core::option::Option<
        super::super::super::super::super::super::config::core::v3::RetryPolicy,
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
        /// \[\#not-implemented-hide:\]
        /// Multiple upstream clusters can be specified for a given route. The request is routed to one
        /// of the upstream clusters based on weights assigned to each cluster.
        /// Currently ClusterWeight only supports the name and weight fields.
        #[prost(message, tag = "2")]
        WeightedClusters(
            super::super::super::super::super::super::super::config::route::v3::WeightedCluster,
        ),
    }
}
