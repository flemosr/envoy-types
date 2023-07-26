/// This configuration allows the built-in LEAST_REQUEST LB policy to be configured via the LB policy
/// extension point. See the :ref:`load balancing architecture overview
/// <arch_overview_load_balancing_types>` for more information.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LeastRequest {
    /// The number of random healthy hosts from which the host with the fewest active requests will
    /// be chosen. Defaults to 2 so that we perform two-choice selection if the field is not set.
    #[prost(message, optional, tag = "1")]
    pub choice_count: ::core::option::Option<
        super::super::super::super::super::google::protobuf::UInt32Value,
    >,
    /// The following formula is used to calculate the dynamic weights when hosts have different load
    /// balancing weights:
    ///
    /// `weight = load_balancing_weight / (active_requests + 1)^active_request_bias`
    ///
    /// The larger the active request bias is, the more aggressively active requests will lower the
    /// effective weight when all host weights are not equal.
    ///
    /// `active_request_bias` must be greater than or equal to 0.0.
    ///
    /// When `active_request_bias == 0.0` the Least Request Load Balancer doesn't consider the number
    /// of active requests at the time it picks a host and behaves like the Round Robin Load
    /// Balancer.
    ///
    /// When `active_request_bias > 0.0` the Least Request Load Balancer scales the load balancing
    /// weight by the number of active requests at the time it does a pick.
    ///
    /// The value is cached for performance reasons and refreshed whenever one of the Load Balancer's
    /// host sets changes, e.g., whenever there is a host membership update or a host load balancing
    /// weight change.
    ///
    /// .. note::
    ///    This setting only takes effect if all host weights are not equal.
    #[prost(message, optional, tag = "2")]
    pub active_request_bias: ::core::option::Option<
        super::super::super::super::config::core::v3::RuntimeDouble,
    >,
    /// Configuration for slow start mode.
    /// If this configuration is not set, slow start will not be not enabled.
    #[prost(message, optional, tag = "3")]
    pub slow_start_config: ::core::option::Option<
        super::super::common::v3::SlowStartConfig,
    >,
    /// Configuration for local zone aware load balancing or locality weighted load balancing.
    #[prost(message, optional, tag = "4")]
    pub locality_lb_config: ::core::option::Option<
        super::super::common::v3::LocalityLbConfig,
    >,
}
