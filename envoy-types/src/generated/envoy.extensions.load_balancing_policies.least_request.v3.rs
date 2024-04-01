/// This configuration allows the built-in LEAST_REQUEST LB policy to be configured via the LB policy
/// extension point. See the :ref:`load balancing architecture overview <arch_overview_load_balancing_types>` for more information.
/// \[\#next-free-field: 7\]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LeastRequest {
    /// The number of random healthy hosts from which the host with the fewest active requests will
    /// be chosen. Defaults to 2 so that we perform two-choice selection if the field is not set.
    /// Only applies to the `N_CHOICES` selection method.
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
    /// This setting only takes effect if all host weights are not equal.
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
    /// \[\#not-implemented-hide:\]
    /// Unused. Replaced by the `selection_method` enum for better extensibility.
    #[deprecated]
    #[prost(message, optional, tag = "5")]
    pub enable_full_scan: ::core::option::Option<
        super::super::super::super::super::google::protobuf::BoolValue,
    >,
    /// Method for selecting the host set from which to return the host with the fewest active requests.
    ///
    /// Defaults to `N_CHOICES`.
    #[prost(enumeration = "least_request::SelectionMethod", tag = "6")]
    pub selection_method: i32,
}
/// Nested message and enum types in `LeastRequest`.
pub mod least_request {
    /// Available methods for selecting the host set from which to return the host with the
    /// fewest active requests.
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
    pub enum SelectionMethod {
        /// Return host with fewest requests from a set of `choice_count` randomly selected hosts.
        /// Best selection method for most scenarios.
        NChoices = 0,
        /// Return host with fewest requests from all hosts.
        /// Useful in some niche use cases involving low request rates and one of:
        /// (example 1) low request limits on workloads, or (example 2) few hosts.
        ///
        /// Example 1: Consider a workload type that can only accept one connection at a time.
        /// If such workloads are deployed across many hosts, only a small percentage of those
        /// workloads have zero connections at any given time, and the rate of new connections is low,
        /// the `FULL_SCAN` method is more likely to select a suitable host than `N_CHOICES`.
        ///
        /// Example 2: Consider a workload type that is only deployed on 2 hosts. With default settings,
        /// the `N_CHOICES` method will return the host with more active requests 25% of the time.
        /// If the request rate is sufficiently low, the behavior of always selecting the host with least
        /// requests as of the last metrics refresh may be preferable.
        FullScan = 1,
    }
    impl SelectionMethod {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                SelectionMethod::NChoices => "N_CHOICES",
                SelectionMethod::FullScan => "FULL_SCAN",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "N_CHOICES" => Some(Self::NChoices),
                "FULL_SCAN" => Some(Self::FullScan),
                _ => None,
            }
        }
    }
}
