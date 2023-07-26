/// [#extension: envoy.route.early_data_policy.default]
/// The default rule to allow/disallow a request to be sent as early data. It's an empty config now. Configuring it will disallow any request to be sent over early data.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DefaultEarlyDataPolicy {}
