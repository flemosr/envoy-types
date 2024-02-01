/// Upstream host identifier.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Endpoint {
    /// The upstream host address.
    ///
    /// .. attention::
    ///
    /// The form of host address depends on the given cluster type. For STATIC or EDS,
    /// it is expected to be a direct IP address (or something resolvable by the
    /// specified :ref:`resolver <envoy_v3_api_field_config.core.v3.SocketAddress.resolver_name>`
    /// in the Address). For LOGICAL or STRICT DNS, it is expected to be hostname,
    /// and will be resolved via DNS.
    #[prost(message, optional, tag = "1")]
    pub address: ::core::option::Option<super::super::core::v3::Address>,
    /// The optional health check configuration is used as configuration for the
    /// health checker to contact the health checked host.
    ///
    /// .. attention::
    ///
    /// This takes into effect only for upstream clusters with
    /// :ref:`active health checking <arch_overview_health_checking>` enabled.
    #[prost(message, optional, tag = "2")]
    pub health_check_config: ::core::option::Option<endpoint::HealthCheckConfig>,
    /// The hostname associated with this endpoint. This hostname is not used for routing or address
    /// resolution. If provided, it will be associated with the endpoint, and can be used for features
    /// that require a hostname, like
    /// :ref:`auto_host_rewrite <envoy_v3_api_field_config.route.v3.RouteAction.auto_host_rewrite>`.
    #[prost(string, tag = "3")]
    pub hostname: ::prost::alloc::string::String,
    /// An ordered list of addresses that together with `address` comprise the
    /// list of addresses for an endpoint. The address given in the `address` is
    /// prepended to this list. It is assumed that the list must already be
    /// sorted by preference order of the addresses. This will only be supported
    /// for STATIC and EDS clusters.
    #[prost(message, repeated, tag = "4")]
    pub additional_addresses: ::prost::alloc::vec::Vec<endpoint::AdditionalAddress>,
}
/// Nested message and enum types in `Endpoint`.
pub mod endpoint {
    /// The optional health check configuration.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct HealthCheckConfig {
        /// Optional alternative health check port value.
        ///
        /// By default the health check address port of an upstream host is the same
        /// as the host's serving address port. This provides an alternative health
        /// check port. Setting this with a non-zero value allows an upstream host
        /// to have different health check address port.
        #[prost(uint32, tag = "1")]
        pub port_value: u32,
        /// By default, the host header for L7 health checks is controlled by cluster level configuration
        /// (see: :ref:`host <envoy_v3_api_field_config.core.v3.HealthCheck.HttpHealthCheck.host>` and
        /// :ref:`authority <envoy_v3_api_field_config.core.v3.HealthCheck.GrpcHealthCheck.authority>`). Setting this
        /// to a non-empty value allows overriding the cluster level configuration for a specific
        /// endpoint.
        #[prost(string, tag = "2")]
        pub hostname: ::prost::alloc::string::String,
        /// Optional alternative health check host address.
        ///
        /// .. attention::
        ///
        /// The form of the health check host address is expected to be a direct IP address.
        #[prost(message, optional, tag = "3")]
        pub address: ::core::option::Option<super::super::super::core::v3::Address>,
        /// Optional flag to control if perform active health check for this endpoint.
        /// Active health check is enabled by default if there is a health checker.
        #[prost(bool, tag = "4")]
        pub disable_active_health_check: bool,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct AdditionalAddress {
        /// Additional address that is associated with the endpoint.
        #[prost(message, optional, tag = "1")]
        pub address: ::core::option::Option<super::super::super::core::v3::Address>,
    }
}
/// An Endpoint that Envoy can route traffic to.
/// \[\#next-free-field: 6\]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LbEndpoint {
    /// Optional health status when known and supplied by EDS server.
    #[prost(enumeration = "super::super::core::v3::HealthStatus", tag = "2")]
    pub health_status: i32,
    /// The endpoint metadata specifies values that may be used by the load
    /// balancer to select endpoints in a cluster for a given request. The filter
    /// name should be specified as `envoy.lb`. An example boolean key-value pair
    /// is `canary`, providing the optional canary status of the upstream host.
    /// This may be matched against in a route's
    /// :ref:`RouteAction <envoy_v3_api_msg_config.route.v3.RouteAction>` metadata_match field
    /// to subset the endpoints considered in cluster load balancing.
    #[prost(message, optional, tag = "3")]
    pub metadata: ::core::option::Option<super::super::core::v3::Metadata>,
    /// The optional load balancing weight of the upstream host; at least 1.
    /// Envoy uses the load balancing weight in some of the built in load
    /// balancers. The load balancing weight for an endpoint is divided by the sum
    /// of the weights of all endpoints in the endpoint's locality to produce a
    /// percentage of traffic for the endpoint. This percentage is then further
    /// weighted by the endpoint's locality's load balancing weight from
    /// LocalityLbEndpoints. If unspecified, will be treated as 1. The sum
    /// of the weights of all endpoints in the endpoint's locality must not
    /// exceed uint32_t maximal value (4294967295).
    #[prost(message, optional, tag = "4")]
    pub load_balancing_weight: ::core::option::Option<
        super::super::super::super::google::protobuf::UInt32Value,
    >,
    /// Upstream host identifier or a named reference.
    #[prost(oneof = "lb_endpoint::HostIdentifier", tags = "1, 5")]
    pub host_identifier: ::core::option::Option<lb_endpoint::HostIdentifier>,
}
/// Nested message and enum types in `LbEndpoint`.
pub mod lb_endpoint {
    /// Upstream host identifier or a named reference.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum HostIdentifier {
        #[prost(message, tag = "1")]
        Endpoint(super::Endpoint),
        /// \[\#not-implemented-hide:\]
        #[prost(string, tag = "5")]
        EndpointName(::prost::alloc::string::String),
    }
}
/// \[\#not-implemented-hide:\]
/// A configuration for a LEDS collection.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LedsClusterLocalityConfig {
    /// Configuration for the source of LEDS updates for a Locality.
    #[prost(message, optional, tag = "1")]
    pub leds_config: ::core::option::Option<super::super::core::v3::ConfigSource>,
    /// The xDS transport protocol glob collection resource name.
    /// The service is only supported in delta xDS (incremental) mode.
    #[prost(string, tag = "2")]
    pub leds_collection_name: ::prost::alloc::string::String,
}
/// A group of endpoints belonging to a Locality.
/// One can have multiple LocalityLbEndpoints for a locality, but only if
/// they have different priorities.
/// \[\#next-free-field: 9\]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LocalityLbEndpoints {
    /// Identifies location of where the upstream hosts run.
    #[prost(message, optional, tag = "1")]
    pub locality: ::core::option::Option<super::super::core::v3::Locality>,
    /// The group of endpoints belonging to the locality specified.
    /// \[\#comment:TODO(adisuissa): Once LEDS is implemented this field needs to be
    /// deprecated and replaced by `load_balancer_endpoints`.\]
    #[prost(message, repeated, tag = "2")]
    pub lb_endpoints: ::prost::alloc::vec::Vec<LbEndpoint>,
    /// Optional: Per priority/region/zone/sub_zone weight; at least 1. The load
    /// balancing weight for a locality is divided by the sum of the weights of all
    /// localities  at the same priority level to produce the effective percentage
    /// of traffic for the locality. The sum of the weights of all localities at
    /// the same priority level must not exceed uint32_t maximal value (4294967295).
    ///
    /// Locality weights are only considered when :ref:`locality weighted load balancing <arch_overview_load_balancing_locality_weighted_lb>` is
    /// configured. These weights are ignored otherwise. If no weights are
    /// specified when locality weighted load balancing is enabled, the locality is
    /// assigned no load.
    #[prost(message, optional, tag = "3")]
    pub load_balancing_weight: ::core::option::Option<
        super::super::super::super::google::protobuf::UInt32Value,
    >,
    /// Optional: the priority for this LocalityLbEndpoints. If unspecified this will
    /// default to the highest priority (0).
    ///
    /// Under usual circumstances, Envoy will only select endpoints for the highest
    /// priority (0). In the event that enough endpoints for a particular priority are
    /// unavailable/unhealthy, Envoy will fail over to selecting endpoints for the
    /// next highest priority group. Read more at :ref:`priority levels <arch_overview_load_balancing_priority_levels>`.
    ///
    /// Priorities should range from 0 (highest) to N (lowest) without skipping.
    #[prost(uint32, tag = "5")]
    pub priority: u32,
    /// Optional: Per locality proximity value which indicates how close this
    /// locality is from the source locality. This value only provides ordering
    /// information (lower the value, closer it is to the source locality).
    /// This will be consumed by load balancing schemes that need proximity order
    /// to determine where to route the requests.
    /// \[\#not-implemented-hide:\]
    #[prost(message, optional, tag = "6")]
    pub proximity: ::core::option::Option<
        super::super::super::super::google::protobuf::UInt32Value,
    >,
    /// \[\#not-implemented-hide:\]
    #[prost(oneof = "locality_lb_endpoints::LbConfig", tags = "7, 8")]
    pub lb_config: ::core::option::Option<locality_lb_endpoints::LbConfig>,
}
/// Nested message and enum types in `LocalityLbEndpoints`.
pub mod locality_lb_endpoints {
    /// \[\#not-implemented-hide:\]
    /// A list of endpoints of a specific locality.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct LbEndpointList {
        #[prost(message, repeated, tag = "1")]
        pub lb_endpoints: ::prost::alloc::vec::Vec<super::LbEndpoint>,
    }
    /// \[\#not-implemented-hide:\]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum LbConfig {
        /// The group of endpoints belonging to the locality.
        /// \[\#comment:TODO(adisuissa): Once LEDS is implemented the `lb_endpoints` field
        /// needs to be deprecated.\]
        #[prost(message, tag = "7")]
        LoadBalancerEndpoints(LbEndpointList),
        /// LEDS Configuration for the current locality.
        #[prost(message, tag = "8")]
        LedsClusterLocalityConfig(super::LedsClusterLocalityConfig),
    }
}
/// Each route from RDS will map to a single cluster or traffic split across
/// clusters using weights expressed in the RDS WeightedCluster.
///
/// With EDS, each cluster is treated independently from a LB perspective, with
/// LB taking place between the Localities within a cluster and at a finer
/// granularity between the hosts within a locality. The percentage of traffic
/// for each endpoint is determined by both its load_balancing_weight, and the
/// load_balancing_weight of its locality. First, a locality will be selected,
/// then an endpoint within that locality will be chose based on its weight.
/// \[\#next-free-field: 6\]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClusterLoadAssignment {
    /// Name of the cluster. This will be the :ref:`service_name <envoy_v3_api_field_config.cluster.v3.Cluster.EdsClusterConfig.service_name>` value if specified
    /// in the cluster :ref:`EdsClusterConfig <envoy_v3_api_msg_config.cluster.v3.Cluster.EdsClusterConfig>`.
    #[prost(string, tag = "1")]
    pub cluster_name: ::prost::alloc::string::String,
    /// List of endpoints to load balance to.
    #[prost(message, repeated, tag = "2")]
    pub endpoints: ::prost::alloc::vec::Vec<LocalityLbEndpoints>,
    /// Map of named endpoints that can be referenced in LocalityLbEndpoints.
    /// \[\#not-implemented-hide:\]
    #[prost(map = "string, message", tag = "5")]
    pub named_endpoints: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        Endpoint,
    >,
    /// Load balancing policy settings.
    #[prost(message, optional, tag = "4")]
    pub policy: ::core::option::Option<cluster_load_assignment::Policy>,
}
/// Nested message and enum types in `ClusterLoadAssignment`.
pub mod cluster_load_assignment {
    /// Load balancing policy settings.
    /// \[\#next-free-field: 7\]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Policy {
        /// Action to trim the overall incoming traffic to protect the upstream
        /// hosts. This action allows protection in case the hosts are unable to
        /// recover from an outage, or unable to autoscale or unable to handle
        /// incoming traffic volume for any reason.
        ///
        /// At the client each category is applied one after the other to generate
        /// the 'actual' drop percentage on all outgoing traffic. For example:
        ///
        /// .. code-block:: json
        ///
        /// { "drop_overloads": \[
        /// { "category": "throttle", "drop_percentage": 60 }
        /// { "category": "lb", "drop_percentage": 50 }
        /// \]}
        ///
        /// The actual drop percentages applied to the traffic at the clients will be
        /// "throttle"\_drop = 60%
        /// "lb"\_drop = 20%  // 50% of the remaining 'actual' load, which is 40%.
        /// actual_outgoing_load = 20% // remaining after applying all categories.
        ///
        /// Envoy supports only one element and will NACK if more than one element is present.
        /// Other xDS-capable data planes will not necessarily have this limitation.
        #[prost(message, repeated, tag = "2")]
        pub drop_overloads: ::prost::alloc::vec::Vec<policy::DropOverload>,
        /// Priority levels and localities are considered overprovisioned with this
        /// factor (in percentage). This means that we don't consider a priority
        /// level or locality unhealthy until the fraction of healthy hosts
        /// multiplied by the overprovisioning factor drops below 100.
        /// With the default value 140(1.4), Envoy doesn't consider a priority level
        /// or a locality unhealthy until their percentage of healthy hosts drops
        /// below 72%. For example:
        ///
        /// .. code-block:: json
        ///
        /// { "overprovisioning_factor": 100 }
        ///
        /// Read more at :ref:`priority levels <arch_overview_load_balancing_priority_levels>` and
        /// :ref:`localities <arch_overview_load_balancing_locality_weighted_lb>`.
        #[prost(message, optional, tag = "3")]
        pub overprovisioning_factor: ::core::option::Option<
            super::super::super::super::super::google::protobuf::UInt32Value,
        >,
        /// The max time until which the endpoints from this assignment can be used.
        /// If no new assignments are received before this time expires the endpoints
        /// are considered stale and should be marked unhealthy.
        /// Defaults to 0 which means endpoints never go stale.
        #[prost(message, optional, tag = "4")]
        pub endpoint_stale_after: ::core::option::Option<
            super::super::super::super::super::google::protobuf::Duration,
        >,
        /// If true, use the :ref:`load balancing weight <envoy_v3_api_field_config.endpoint.v3.LbEndpoint.load_balancing_weight>` of healthy and unhealthy
        /// hosts to determine the health of the priority level. If false, use the number of healthy and unhealthy hosts
        /// to determine the health of the priority level, or in other words assume each host has a weight of 1 for
        /// this calculation.
        ///
        /// Note: this is not currently implemented for
        /// :ref:`locality weighted load balancing <arch_overview_load_balancing_locality_weighted_lb>`.
        #[prost(bool, tag = "6")]
        pub weighted_priority_health: bool,
    }
    /// Nested message and enum types in `Policy`.
    pub mod policy {
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct DropOverload {
            /// Identifier for the policy specifying the drop.
            #[prost(string, tag = "1")]
            pub category: ::prost::alloc::string::String,
            /// Percentage of traffic that should be dropped for the category.
            #[prost(message, optional, tag = "2")]
            pub drop_percentage: ::core::option::Option<
                super::super::super::super::super::r#type::v3::FractionalPercent,
            >,
        }
    }
}
/// These are stats Envoy reports to the management server at a frequency defined by
/// :ref:`LoadStatsResponse.load_reporting_interval<envoy_v3_api_field_service.load_stats.v3.LoadStatsResponse.load_reporting_interval>`.
/// Stats per upstream region/zone and optionally per subzone.
/// \[\#next-free-field: 9\]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpstreamLocalityStats {
    /// Name of zone, region and optionally endpoint group these metrics were
    /// collected from. Zone and region names could be empty if unknown.
    #[prost(message, optional, tag = "1")]
    pub locality: ::core::option::Option<super::super::core::v3::Locality>,
    /// The total number of requests successfully completed by the endpoints in the
    /// locality.
    #[prost(uint64, tag = "2")]
    pub total_successful_requests: u64,
    /// The total number of unfinished requests
    #[prost(uint64, tag = "3")]
    pub total_requests_in_progress: u64,
    /// The total number of requests that failed due to errors at the endpoint,
    /// aggregated over all endpoints in the locality.
    #[prost(uint64, tag = "4")]
    pub total_error_requests: u64,
    /// The total number of requests that were issued by this Envoy since
    /// the last report. This information is aggregated over all the
    /// upstream endpoints in the locality.
    #[prost(uint64, tag = "8")]
    pub total_issued_requests: u64,
    /// Stats for multi-dimensional load balancing.
    #[prost(message, repeated, tag = "5")]
    pub load_metric_stats: ::prost::alloc::vec::Vec<EndpointLoadMetricStats>,
    /// Endpoint granularity stats information for this locality. This information
    /// is populated if the Server requests it by setting
    /// :ref:`LoadStatsResponse.report_endpoint_granularity<envoy_v3_api_field_service.load_stats.v3.LoadStatsResponse.report_endpoint_granularity>`.
    #[prost(message, repeated, tag = "7")]
    pub upstream_endpoint_stats: ::prost::alloc::vec::Vec<UpstreamEndpointStats>,
    /// \[\#not-implemented-hide:\] The priority of the endpoint group these metrics
    /// were collected from.
    #[prost(uint32, tag = "6")]
    pub priority: u32,
}
/// \[\#next-free-field: 8\]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpstreamEndpointStats {
    /// Upstream host address.
    #[prost(message, optional, tag = "1")]
    pub address: ::core::option::Option<super::super::core::v3::Address>,
    /// Opaque and implementation dependent metadata of the
    /// endpoint. Envoy will pass this directly to the management server.
    #[prost(message, optional, tag = "6")]
    pub metadata: ::core::option::Option<
        super::super::super::super::google::protobuf::Struct,
    >,
    /// The total number of requests successfully completed by the endpoints in the
    /// locality. These include non-5xx responses for HTTP, where errors
    /// originate at the client and the endpoint responded successfully. For gRPC,
    /// the grpc-status values are those not covered by total_error_requests below.
    #[prost(uint64, tag = "2")]
    pub total_successful_requests: u64,
    /// The total number of unfinished requests for this endpoint.
    #[prost(uint64, tag = "3")]
    pub total_requests_in_progress: u64,
    /// The total number of requests that failed due to errors at the endpoint.
    /// For HTTP these are responses with 5xx status codes and for gRPC the
    /// grpc-status values:
    ///
    /// * DeadlineExceeded
    /// * Unimplemented
    /// * Internal
    /// * Unavailable
    /// * Unknown
    /// * DataLoss
    #[prost(uint64, tag = "4")]
    pub total_error_requests: u64,
    /// The total number of requests that were issued to this endpoint
    /// since the last report. A single TCP connection, HTTP or gRPC
    /// request or stream is counted as one request.
    #[prost(uint64, tag = "7")]
    pub total_issued_requests: u64,
    /// Stats for multi-dimensional load balancing.
    #[prost(message, repeated, tag = "5")]
    pub load_metric_stats: ::prost::alloc::vec::Vec<EndpointLoadMetricStats>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EndpointLoadMetricStats {
    /// Name of the metric; may be empty.
    #[prost(string, tag = "1")]
    pub metric_name: ::prost::alloc::string::String,
    /// Number of calls that finished and included this metric.
    #[prost(uint64, tag = "2")]
    pub num_requests_finished_with_metric: u64,
    /// Sum of metric values across all calls that finished with this metric for
    /// load_reporting_interval.
    #[prost(double, tag = "3")]
    pub total_metric_value: f64,
}
/// Per cluster load stats. Envoy reports these stats a management server in a
/// :ref:`LoadStatsRequest<envoy_v3_api_msg_service.load_stats.v3.LoadStatsRequest>`
/// Next ID: 7
/// \[\#next-free-field: 7\]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClusterStats {
    /// The name of the cluster.
    #[prost(string, tag = "1")]
    pub cluster_name: ::prost::alloc::string::String,
    /// The eds_cluster_config service_name of the cluster.
    /// It's possible that two clusters send the same service_name to EDS,
    /// in that case, the management server is supposed to do aggregation on the load reports.
    #[prost(string, tag = "6")]
    pub cluster_service_name: ::prost::alloc::string::String,
    /// Need at least one.
    #[prost(message, repeated, tag = "2")]
    pub upstream_locality_stats: ::prost::alloc::vec::Vec<UpstreamLocalityStats>,
    /// Cluster-level stats such as total_successful_requests may be computed by
    /// summing upstream_locality_stats. In addition, below there are additional
    /// cluster-wide stats.
    ///
    /// The total number of dropped requests. This covers requests
    /// deliberately dropped by the drop_overload policy and circuit breaking.
    #[prost(uint64, tag = "3")]
    pub total_dropped_requests: u64,
    /// Information about deliberately dropped requests for each category specified
    /// in the DropOverload policy.
    #[prost(message, repeated, tag = "5")]
    pub dropped_requests: ::prost::alloc::vec::Vec<cluster_stats::DroppedRequests>,
    /// Period over which the actual load report occurred. This will be guaranteed to include every
    /// request reported. Due to system load and delays between the `LoadStatsRequest` sent from Envoy
    /// and the `LoadStatsResponse` message sent from the management server, this may be longer than
    /// the requested load reporting interval in the `LoadStatsResponse`.
    #[prost(message, optional, tag = "4")]
    pub load_report_interval: ::core::option::Option<
        super::super::super::super::google::protobuf::Duration,
    >,
}
/// Nested message and enum types in `ClusterStats`.
pub mod cluster_stats {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DroppedRequests {
        /// Identifier for the policy specifying the drop.
        #[prost(string, tag = "1")]
        pub category: ::prost::alloc::string::String,
        /// Total number of deliberately dropped requests for the category.
        #[prost(uint64, tag = "2")]
        pub dropped_count: u64,
    }
}
