/// A GuardDogAction that will terminate the process by killing the
/// stuck thread. This would allow easier access to the call stack of the stuck
/// thread since we would run signal handlers on that thread. By default
/// this will be registered to run as the last watchdog action on KILL and
/// MULTIKILL events if those are enabled.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AbortActionConfig {
    /// How long to wait for the thread to respond to the thread kill function
    /// before killing the process from this action. This is a blocking action.
    /// By default this is 5 seconds.
    #[prost(message, optional, tag = "1")]
    pub wait_duration: ::core::option::Option<
        super::super::super::google::protobuf::Duration,
    >,
}
