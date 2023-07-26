/// Configuration of on-demand CDS.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OnDemandCds {
    /// A configuration source for the service that will be used for
    /// on-demand cluster discovery.
    #[prost(message, optional, tag = "1")]
    pub source: ::core::option::Option<
        super::super::super::super::super::config::core::v3::ConfigSource,
    >,
    /// xdstp:// resource locator for on-demand cluster collection.
    #[prost(string, tag = "2")]
    pub resources_locator: ::prost::alloc::string::String,
    /// The timeout for on demand cluster lookup. If not set, defaults to 5 seconds.
    #[prost(message, optional, tag = "3")]
    pub timeout: ::core::option::Option<
        super::super::super::super::super::super::google::protobuf::Duration,
    >,
}
/// On Demand Discovery filter config.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OnDemand {
    /// An optional configuration for on-demand cluster discovery
    /// service. If not specified, the on-demand cluster discovery will
    /// be disabled. When it's specified, the filter will pause the
    /// request to an unknown cluster and will begin a cluster discovery
    /// process. When the discovery is finished (successfully or not), the
    /// request will be resumed for further processing.
    #[prost(message, optional, tag = "1")]
    pub odcds: ::core::option::Option<OnDemandCds>,
}
/// Per-route configuration for On Demand Discovery.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PerRouteConfig {
    /// An optional configuration for on-demand cluster discovery
    /// service. If not specified, the on-demand cluster discovery will
    /// be disabled. When it's specified, the filter will pause the
    /// request to an unknown cluster and will begin a cluster discovery
    /// process. When the discovery is finished (successfully or not), the
    /// request will be resumed for further processing.
    #[prost(message, optional, tag = "1")]
    pub odcds: ::core::option::Option<OnDemandCds>,
}
