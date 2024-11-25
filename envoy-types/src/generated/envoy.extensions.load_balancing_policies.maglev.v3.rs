// This file is @generated by prost-build.
/// This configuration allows the built-in Maglev LB policy to be configured via the LB policy
/// extension point. See the :ref:`load balancing architecture overview  <arch_overview_load_balancing_types>` and :ref:`Maglev<arch_overview_load_balancing_types_maglev>` for more information.
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Maglev {
    /// The table size for Maglev hashing. Maglev aims for "minimal disruption" rather than an absolute guarantee.
    /// Minimal disruption means that when the set of upstream hosts change, a connection will likely be sent to the same
    /// upstream as it was before. Increasing the table size reduces the amount of disruption.
    /// The table size must be prime number limited to 5000011. If it is not specified, the default is 65537.
    #[prost(message, optional, tag = "1")]
    pub table_size: ::core::option::Option<
        super::super::super::super::super::google::protobuf::UInt64Value,
    >,
    /// Common configuration for hashing-based load balancing policies.
    #[prost(message, optional, tag = "2")]
    pub consistent_hashing_lb_config: ::core::option::Option<
        super::super::common::v3::ConsistentHashingLbConfig,
    >,
    /// Enable locality weighted load balancing for maglev lb explicitly.
    #[prost(message, optional, tag = "3")]
    pub locality_weighted_lb_config: ::core::option::Option<
        super::super::common::v3::locality_lb_config::LocalityWeightedLbConfig,
    >,
}
