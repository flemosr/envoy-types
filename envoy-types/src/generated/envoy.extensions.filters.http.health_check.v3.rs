/// [#next-free-field: 6]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HealthCheck {
    /// Specifies whether the filter operates in pass through mode or not.
    #[prost(message, optional, tag = "1")]
    pub pass_through_mode: ::core::option::Option<
        super::super::super::super::super::super::google::protobuf::BoolValue,
    >,
    /// If operating in pass through mode, the amount of time in milliseconds
    /// that the filter should cache the upstream response.
    #[prost(message, optional, tag = "3")]
    pub cache_time: ::core::option::Option<
        super::super::super::super::super::super::google::protobuf::Duration,
    >,
    /// If operating in non-pass-through mode, specifies a set of upstream cluster
    /// names and the minimum percentage of servers in each of those clusters that
    /// must be healthy or degraded in order for the filter to return a 200.
    ///
    /// .. note::
    ///
    ///     This value is interpreted as an integer by truncating, so 12.50% will be calculated
    ///     as if it were 12%.
    #[prost(map = "string, message", tag = "4")]
    pub cluster_min_healthy_percentages: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        super::super::super::super::super::r#type::v3::Percent,
    >,
    /// Specifies a set of health check request headers to match on. The health check filter will
    /// check a request’s headers against all the specified headers. To specify the health check
    /// endpoint, set the ``:path`` header to match on.
    #[prost(message, repeated, tag = "5")]
    pub headers: ::prost::alloc::vec::Vec<
        super::super::super::super::super::config::route::v3::HeaderMatcher,
    >,
}
