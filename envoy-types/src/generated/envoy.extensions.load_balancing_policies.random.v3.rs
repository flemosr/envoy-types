// This file is @generated by prost-build.
/// This configuration allows the built-in Random LB policy to be configured via the LB policy
/// extension point. See the :ref:`load balancing architecture overview  <arch_overview_load_balancing_types>` for more information.
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Random {
    /// Configuration for local zone aware load balancing or locality weighted load balancing.
    #[prost(message, optional, tag = "1")]
    pub locality_lb_config: ::core::option::Option<
        super::super::common::v3::LocalityLbConfig,
    >,
}
