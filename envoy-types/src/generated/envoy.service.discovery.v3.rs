/// Specifies a resource to be subscribed to.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResourceLocator {
    /// The resource name to subscribe to.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// A set of dynamic parameters used to match against the dynamic parameter
    /// constraints on the resource. This allows clients to select between
    /// multiple variants of the same resource.
    #[prost(map = "string, string", tag = "2")]
    pub dynamic_parameters: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
}
/// Specifies a concrete resource name.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResourceName {
    /// The name of the resource.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Dynamic parameter constraints associated with this resource. To be used by client-side caches
    /// (including xDS proxies) when matching subscribed resource locators.
    #[prost(message, optional, tag = "2")]
    pub dynamic_parameter_constraints: ::core::option::Option<
        DynamicParameterConstraints,
    >,
}
/// A DiscoveryRequest requests a set of versioned resources of the same type for
/// a given Envoy node on some API.
/// \[\#next-free-field: 8\]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DiscoveryRequest {
    /// The version_info provided in the request messages will be the version_info
    /// received with the most recent successfully processed response or empty on
    /// the first request. It is expected that no new request is sent after a
    /// response is received until the Envoy instance is ready to ACK/NACK the new
    /// configuration. ACK/NACK takes place by returning the new API config version
    /// as applied or the previous API config version respectively. Each type_url
    /// (see below) has an independent version associated with it.
    #[prost(string, tag = "1")]
    pub version_info: ::prost::alloc::string::String,
    /// The node making the request.
    #[prost(message, optional, tag = "2")]
    pub node: ::core::option::Option<super::super::super::config::core::v3::Node>,
    /// List of resources to subscribe to, e.g. list of cluster names or a route
    /// configuration name. If this is empty, all resources for the API are
    /// returned. LDS/CDS may have empty resource_names, which will cause all
    /// resources for the Envoy instance to be returned. The LDS and CDS responses
    /// will then imply a number of resources that need to be fetched via EDS/RDS,
    /// which will be explicitly enumerated in resource_names.
    #[prost(string, repeated, tag = "3")]
    pub resource_names: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// \[\#not-implemented-hide:\]
    /// Alternative to `resource_names` field that allows specifying dynamic
    /// parameters along with each resource name. Clients that populate this
    /// field must be able to handle responses from the server where resources
    /// are wrapped in a Resource message.
    /// Note that it is legal for a request to have some resources listed
    /// in `resource_names` and others in `resource_locators`.
    #[prost(message, repeated, tag = "7")]
    pub resource_locators: ::prost::alloc::vec::Vec<ResourceLocator>,
    /// Type of the resource that is being requested, e.g.
    /// "type.googleapis.com/envoy.api.v2.ClusterLoadAssignment". This is implicit
    /// in requests made via singleton xDS APIs such as CDS, LDS, etc. but is
    /// required for ADS.
    #[prost(string, tag = "4")]
    pub type_url: ::prost::alloc::string::String,
    /// nonce corresponding to DiscoveryResponse being ACK/NACKed. See above
    /// discussion on version_info and the DiscoveryResponse nonce comment. This
    /// may be empty only if 1) this is a non-persistent-stream xDS such as HTTP,
    /// or 2) the client has not yet accepted an update in this xDS stream (unlike
    /// delta, where it is populated only for new explicit ACKs).
    #[prost(string, tag = "5")]
    pub response_nonce: ::prost::alloc::string::String,
    /// This is populated when the previous :ref:`DiscoveryResponse <envoy_v3_api_msg_service.discovery.v3.DiscoveryResponse>`
    /// failed to update configuration. The `message` field in `error_details` provides the Envoy
    /// internal exception related to the failure. It is only intended for consumption during manual
    /// debugging, the string provided is not guaranteed to be stable across Envoy versions.
    #[prost(message, optional, tag = "6")]
    pub error_detail: ::core::option::Option<
        super::super::super::super::google::rpc::Status,
    >,
}
/// \[\#next-free-field: 7\]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DiscoveryResponse {
    /// The version of the response data.
    #[prost(string, tag = "1")]
    pub version_info: ::prost::alloc::string::String,
    /// The response resources. These resources are typed and depend on the API being called.
    #[prost(message, repeated, tag = "2")]
    pub resources: ::prost::alloc::vec::Vec<
        super::super::super::super::google::protobuf::Any,
    >,
    /// \[\#not-implemented-hide:\]
    /// Canary is used to support two Envoy command line flags:
    ///
    /// * --terminate-on-canary-transition-failure. When set, Envoy is able to
    ///   terminate if it detects that configuration is stuck at canary. Consider
    ///   this example sequence of updates:
    ///   * Management server applies a canary config successfully.
    ///   * Management server rolls back to a production config.
    ///   * Envoy rejects the new production config.
    ///     Since there is no sensible way to continue receiving configuration
    ///     updates, Envoy will then terminate and apply production config from a
    ///     clean slate.
    /// * --dry-run-canary. When set, a canary response will never be applied, only
    ///   validated via a dry run.
    #[prost(bool, tag = "3")]
    pub canary: bool,
    /// Type URL for resources. Identifies the xDS API when muxing over ADS.
    /// Must be consistent with the type_url in the 'resources' repeated Any (if non-empty).
    #[prost(string, tag = "4")]
    pub type_url: ::prost::alloc::string::String,
    /// For gRPC based subscriptions, the nonce provides a way to explicitly ack a
    /// specific DiscoveryResponse in a following DiscoveryRequest. Additional
    /// messages may have been sent by Envoy to the management server for the
    /// previous version on the stream prior to this DiscoveryResponse, that were
    /// unprocessed at response send time. The nonce allows the management server
    /// to ignore any further DiscoveryRequests for the previous version until a
    /// DiscoveryRequest bearing the nonce. The nonce is optional and is not
    /// required for non-stream based xDS implementations.
    #[prost(string, tag = "5")]
    pub nonce: ::prost::alloc::string::String,
    /// The control plane instance that sent the response.
    #[prost(message, optional, tag = "6")]
    pub control_plane: ::core::option::Option<
        super::super::super::config::core::v3::ControlPlane,
    >,
}
/// DeltaDiscoveryRequest and DeltaDiscoveryResponse are used in a new gRPC
/// endpoint for Delta xDS.
///
/// With Delta xDS, the DeltaDiscoveryResponses do not need to include a full
/// snapshot of the tracked resources. Instead, DeltaDiscoveryResponses are a
/// diff to the state of a xDS client.
/// In Delta XDS there are per-resource versions, which allow tracking state at
/// the resource granularity.
/// An xDS Delta session is always in the context of a gRPC bidirectional
/// stream. This allows the xDS server to keep track of the state of xDS clients
/// connected to it.
///
/// In Delta xDS the nonce field is required and used to pair
/// DeltaDiscoveryResponse to a DeltaDiscoveryRequest ACK or NACK.
/// Optionally, a response message level system_version_info is present for
/// debugging purposes only.
///
/// DeltaDiscoveryRequest plays two independent roles. Any DeltaDiscoveryRequest
/// can be either or both of: \[1\] informing the server of what resources the
/// client has gained/lost interest in (using resource_names_subscribe and
/// resource_names_unsubscribe), or \[2\] (N)ACKing an earlier resource update from
/// the server (using response_nonce, with presence of error_detail making it a NACK).
/// Additionally, the first message (for a given type_url) of a reconnected gRPC stream
/// has a third role: informing the server of the resources (and their versions)
/// that the client already possesses, using the initial_resource_versions field.
///
/// As with state-of-the-world, when multiple resource types are multiplexed (ADS),
/// all requests/acknowledgments/updates are logically walled off by type_url:
/// a Cluster ACK exists in a completely separate world from a prior Route NACK.
/// In particular, initial_resource_versions being sent at the "start" of every
/// gRPC stream actually entails a message for each type_url, each with its own
/// initial_resource_versions.
/// \[\#next-free-field: 10\]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeltaDiscoveryRequest {
    /// The node making the request.
    #[prost(message, optional, tag = "1")]
    pub node: ::core::option::Option<super::super::super::config::core::v3::Node>,
    /// Type of the resource that is being requested, e.g.
    /// `type.googleapis.com/envoy.api.v2.ClusterLoadAssignment`. This does not need to be set if
    /// resources are only referenced via `xds_resource_subscribe` and
    /// `xds_resources_unsubscribe`.
    #[prost(string, tag = "2")]
    pub type_url: ::prost::alloc::string::String,
    /// DeltaDiscoveryRequests allow the client to add or remove individual
    /// resources to the set of tracked resources in the context of a stream.
    /// All resource names in the resource_names_subscribe list are added to the
    /// set of tracked resources and all resource names in the resource_names_unsubscribe
    /// list are removed from the set of tracked resources.
    ///
    /// *Unlike* state-of-the-world xDS, an empty resource_names_subscribe or
    /// resource_names_unsubscribe list simply means that no resources are to be
    /// added or removed to the resource list.
    /// *Like* state-of-the-world xDS, the server must send updates for all tracked
    /// resources, but can also send updates for resources the client has not subscribed to.
    ///
    /// NOTE: the server must respond with all resources listed in resource_names_subscribe,
    /// even if it believes the client has the most recent version of them. The reason:
    /// the client may have dropped them, but then regained interest before it had a chance
    /// to send the unsubscribe message. See DeltaSubscriptionStateTest.RemoveThenAdd.
    ///
    /// These two fields can be set in any DeltaDiscoveryRequest, including ACKs
    /// and initial_resource_versions.
    ///
    /// A list of Resource names to add to the list of tracked resources.
    #[prost(string, repeated, tag = "3")]
    pub resource_names_subscribe: ::prost::alloc::vec::Vec<
        ::prost::alloc::string::String,
    >,
    /// A list of Resource names to remove from the list of tracked resources.
    #[prost(string, repeated, tag = "4")]
    pub resource_names_unsubscribe: ::prost::alloc::vec::Vec<
        ::prost::alloc::string::String,
    >,
    /// \[\#not-implemented-hide:\]
    /// Alternative to `resource_names_subscribe` field that allows specifying dynamic parameters
    /// along with each resource name.
    /// Note that it is legal for a request to have some resources listed
    /// in `resource_names_subscribe` and others in `resource_locators_subscribe`.
    #[prost(message, repeated, tag = "8")]
    pub resource_locators_subscribe: ::prost::alloc::vec::Vec<ResourceLocator>,
    /// \[\#not-implemented-hide:\]
    /// Alternative to `resource_names_unsubscribe` field that allows specifying dynamic parameters
    /// along with each resource name.
    /// Note that it is legal for a request to have some resources listed
    /// in `resource_names_unsubscribe` and others in `resource_locators_unsubscribe`.
    #[prost(message, repeated, tag = "9")]
    pub resource_locators_unsubscribe: ::prost::alloc::vec::Vec<ResourceLocator>,
    /// Informs the server of the versions of the resources the xDS client knows of, to enable the
    /// client to continue the same logical xDS session even in the face of gRPC stream reconnection.
    /// It will not be populated: \[1\] in the very first stream of a session, since the client will
    /// not yet have any resources,  \[2\] in any message after the first in a stream (for a given
    /// type_url), since the server will already be correctly tracking the client's state.
    /// (In ADS, the first message *of each type_url* of a reconnected stream populates this map.)
    /// The map's keys are names of xDS resources known to the xDS client.
    /// The map's values are opaque resource versions.
    #[prost(map = "string, string", tag = "5")]
    pub initial_resource_versions: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// When the DeltaDiscoveryRequest is a ACK or NACK message in response
    /// to a previous DeltaDiscoveryResponse, the response_nonce must be the
    /// nonce in the DeltaDiscoveryResponse.
    /// Otherwise (unlike in DiscoveryRequest) response_nonce must be omitted.
    #[prost(string, tag = "6")]
    pub response_nonce: ::prost::alloc::string::String,
    /// This is populated when the previous :ref:`DiscoveryResponse <envoy_v3_api_msg_service.discovery.v3.DiscoveryResponse>`
    /// failed to update configuration. The `message` field in `error_details`
    /// provides the Envoy internal exception related to the failure.
    #[prost(message, optional, tag = "7")]
    pub error_detail: ::core::option::Option<
        super::super::super::super::google::rpc::Status,
    >,
}
/// \[\#next-free-field: 9\]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeltaDiscoveryResponse {
    /// The version of the response data (used for debugging).
    #[prost(string, tag = "1")]
    pub system_version_info: ::prost::alloc::string::String,
    /// The response resources. These are typed resources, whose types must match
    /// the type_url field.
    #[prost(message, repeated, tag = "2")]
    pub resources: ::prost::alloc::vec::Vec<Resource>,
    /// Type URL for resources. Identifies the xDS API when muxing over ADS.
    /// Must be consistent with the type_url in the Any within 'resources' if 'resources' is non-empty.
    #[prost(string, tag = "4")]
    pub type_url: ::prost::alloc::string::String,
    /// Resources names of resources that have be deleted and to be removed from the xDS Client.
    /// Removed resources for missing resources can be ignored.
    #[prost(string, repeated, tag = "6")]
    pub removed_resources: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Alternative to removed_resources that allows specifying which variant of
    /// a resource is being removed. This variant must be used for any resource
    /// for which dynamic parameter constraints were sent to the client.
    #[prost(message, repeated, tag = "8")]
    pub removed_resource_names: ::prost::alloc::vec::Vec<ResourceName>,
    /// The nonce provides a way for DeltaDiscoveryRequests to uniquely
    /// reference a DeltaDiscoveryResponse when (N)ACKing. The nonce is required.
    #[prost(string, tag = "5")]
    pub nonce: ::prost::alloc::string::String,
    /// \[\#not-implemented-hide:\]
    /// The control plane instance that sent the response.
    #[prost(message, optional, tag = "7")]
    pub control_plane: ::core::option::Option<
        super::super::super::config::core::v3::ControlPlane,
    >,
}
/// A set of dynamic parameter constraints associated with a variant of an individual xDS resource.
/// These constraints determine whether the resource matches a subscription based on the set of
/// dynamic parameters in the subscription, as specified in the
/// :ref:`ResourceLocator.dynamic_parameters<envoy_v3_api_field_service.discovery.v3.ResourceLocator.dynamic_parameters>`
/// field. This allows xDS implementations (clients, servers, and caching proxies) to determine
/// which variant of a resource is appropriate for a given client.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DynamicParameterConstraints {
    #[prost(oneof = "dynamic_parameter_constraints::Type", tags = "1, 2, 3, 4")]
    pub r#type: ::core::option::Option<dynamic_parameter_constraints::Type>,
}
/// Nested message and enum types in `DynamicParameterConstraints`.
pub mod dynamic_parameter_constraints {
    /// A single constraint for a given key.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SingleConstraint {
        /// The key to match against.
        #[prost(string, tag = "1")]
        pub key: ::prost::alloc::string::String,
        #[prost(oneof = "single_constraint::ConstraintType", tags = "2, 3")]
        pub constraint_type: ::core::option::Option<single_constraint::ConstraintType>,
    }
    /// Nested message and enum types in `SingleConstraint`.
    pub mod single_constraint {
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Exists {}
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum ConstraintType {
            /// Matches this exact value.
            #[prost(string, tag = "2")]
            Value(::prost::alloc::string::String),
            /// Key is present (matches any value except for the key being absent).
            /// This allows setting a default constraint for clients that do
            /// not send a key at all, while there may be other clients that need
            /// special configuration based on that key.
            #[prost(message, tag = "3")]
            Exists(Exists),
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ConstraintList {
        #[prost(message, repeated, tag = "1")]
        pub constraints: ::prost::alloc::vec::Vec<super::DynamicParameterConstraints>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Type {
        /// A single constraint to evaluate.
        #[prost(message, tag = "1")]
        Constraint(SingleConstraint),
        /// A list of constraints that match if any one constraint in the list
        /// matches.
        #[prost(message, tag = "2")]
        OrConstraints(ConstraintList),
        /// A list of constraints that must all match.
        #[prost(message, tag = "3")]
        AndConstraints(ConstraintList),
        /// The inverse (NOT) of a set of constraints.
        #[prost(message, tag = "4")]
        NotConstraints(::prost::alloc::boxed::Box<super::DynamicParameterConstraints>),
    }
}
/// \[\#next-free-field: 10\]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Resource {
    /// The resource's name, to distinguish it from others of the same type of resource.
    /// Only one of `name` or `resource_name` may be set.
    #[prost(string, tag = "3")]
    pub name: ::prost::alloc::string::String,
    /// Alternative to the `name` field, to be used when the server supports
    /// multiple variants of the named resource that are differentiated by
    /// dynamic parameter constraints.
    /// Only one of `name` or `resource_name` may be set.
    #[prost(message, optional, tag = "8")]
    pub resource_name: ::core::option::Option<ResourceName>,
    /// The aliases are a list of other names that this resource can go by.
    #[prost(string, repeated, tag = "4")]
    pub aliases: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The resource level version. It allows xDS to track the state of individual
    /// resources.
    #[prost(string, tag = "1")]
    pub version: ::prost::alloc::string::String,
    /// The resource being tracked.
    #[prost(message, optional, tag = "2")]
    pub resource: ::core::option::Option<
        super::super::super::super::google::protobuf::Any,
    >,
    /// Time-to-live value for the resource. For each resource, a timer is started. The timer is
    /// reset each time the resource is received with a new TTL. If the resource is received with
    /// no TTL set, the timer is removed for the resource. Upon expiration of the timer, the
    /// configuration for the resource will be removed.
    ///
    /// The TTL can be refreshed or changed by sending a response that doesn't change the resource
    /// version. In this case the resource field does not need to be populated, which allows for
    /// light-weight "heartbeat" updates to keep a resource with a TTL alive.
    ///
    /// The TTL feature is meant to support configurations that should be removed in the event of
    /// a management server failure. For example, the feature may be used for fault injection
    /// testing where the fault injection should be terminated in the event that Envoy loses contact
    /// with the management server.
    #[prost(message, optional, tag = "6")]
    pub ttl: ::core::option::Option<
        super::super::super::super::google::protobuf::Duration,
    >,
    /// Cache control properties for the resource.
    /// \[\#not-implemented-hide:\]
    #[prost(message, optional, tag = "7")]
    pub cache_control: ::core::option::Option<resource::CacheControl>,
    /// The Metadata field can be used to provide additional information for the resource.
    /// E.g. the trace data for debugging.
    #[prost(message, optional, tag = "9")]
    pub metadata: ::core::option::Option<
        super::super::super::config::core::v3::Metadata,
    >,
}
/// Nested message and enum types in `Resource`.
pub mod resource {
    /// Cache control properties for the resource.
    /// \[\#not-implemented-hide:\]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CacheControl {
        /// If true, xDS proxies may not cache this resource.
        /// Note that this does not apply to clients other than xDS proxies, which must cache resources
        /// for their own use, regardless of the value of this field.
        #[prost(bool, tag = "1")]
        pub do_not_cache: bool,
    }
}
/// \[\#not-implemented-hide:\] Not configuration. Workaround c++ protobuf issue with importing
/// services: <https://github.com/google/protobuf/issues/4221>
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdsDummy {}
/// Generated client implementations.
pub mod aggregated_discovery_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// See https://github.com/envoyproxy/envoy-api#apis for a description of the role of
    /// ADS and how it is intended to be used by a management server. ADS requests
    /// have the same structure as their singleton xDS counterparts, but can
    /// multiplex many resource types on a single stream. The type_url in the
    /// DiscoveryRequest/DiscoveryResponse provides sufficient information to recover
    /// the multiplexed singleton APIs at the Envoy instance and management server.
    #[derive(Debug, Clone)]
    pub struct AggregatedDiscoveryServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> AggregatedDiscoveryServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> AggregatedDiscoveryServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            AggregatedDiscoveryServiceClient::new(
                InterceptedService::new(inner, interceptor),
            )
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        /// This is a gRPC-only API.
        pub async fn stream_aggregated_resources(
            &mut self,
            request: impl tonic::IntoStreamingRequest<Message = super::DiscoveryRequest>,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::DiscoveryResponse>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/envoy.service.discovery.v3.AggregatedDiscoveryService/StreamAggregatedResources",
            );
            let mut req = request.into_streaming_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "envoy.service.discovery.v3.AggregatedDiscoveryService",
                        "StreamAggregatedResources",
                    ),
                );
            self.inner.streaming(req, path, codec).await
        }
        pub async fn delta_aggregated_resources(
            &mut self,
            request: impl tonic::IntoStreamingRequest<
                Message = super::DeltaDiscoveryRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::DeltaDiscoveryResponse>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/envoy.service.discovery.v3.AggregatedDiscoveryService/DeltaAggregatedResources",
            );
            let mut req = request.into_streaming_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "envoy.service.discovery.v3.AggregatedDiscoveryService",
                        "DeltaAggregatedResources",
                    ),
                );
            self.inner.streaming(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod aggregated_discovery_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with AggregatedDiscoveryServiceServer.
    #[async_trait]
    pub trait AggregatedDiscoveryService: Send + Sync + 'static {
        /// Server streaming response type for the StreamAggregatedResources method.
        type StreamAggregatedResourcesStream: tonic::codegen::tokio_stream::Stream<
                Item = std::result::Result<super::DiscoveryResponse, tonic::Status>,
            >
            + Send
            + 'static;
        /// This is a gRPC-only API.
        async fn stream_aggregated_resources(
            &self,
            request: tonic::Request<tonic::Streaming<super::DiscoveryRequest>>,
        ) -> std::result::Result<
            tonic::Response<Self::StreamAggregatedResourcesStream>,
            tonic::Status,
        >;
        /// Server streaming response type for the DeltaAggregatedResources method.
        type DeltaAggregatedResourcesStream: tonic::codegen::tokio_stream::Stream<
                Item = std::result::Result<super::DeltaDiscoveryResponse, tonic::Status>,
            >
            + Send
            + 'static;
        async fn delta_aggregated_resources(
            &self,
            request: tonic::Request<tonic::Streaming<super::DeltaDiscoveryRequest>>,
        ) -> std::result::Result<
            tonic::Response<Self::DeltaAggregatedResourcesStream>,
            tonic::Status,
        >;
    }
    /// See https://github.com/envoyproxy/envoy-api#apis for a description of the role of
    /// ADS and how it is intended to be used by a management server. ADS requests
    /// have the same structure as their singleton xDS counterparts, but can
    /// multiplex many resource types on a single stream. The type_url in the
    /// DiscoveryRequest/DiscoveryResponse provides sufficient information to recover
    /// the multiplexed singleton APIs at the Envoy instance and management server.
    #[derive(Debug)]
    pub struct AggregatedDiscoveryServiceServer<T: AggregatedDiscoveryService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: AggregatedDiscoveryService> AggregatedDiscoveryServiceServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None,
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.max_decoding_message_size = Some(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.max_encoding_message_size = Some(limit);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>>
    for AggregatedDiscoveryServiceServer<T>
    where
        T: AggregatedDiscoveryService,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/envoy.service.discovery.v3.AggregatedDiscoveryService/StreamAggregatedResources" => {
                    #[allow(non_camel_case_types)]
                    struct StreamAggregatedResourcesSvc<T: AggregatedDiscoveryService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: AggregatedDiscoveryService,
                    > tonic::server::StreamingService<super::DiscoveryRequest>
                    for StreamAggregatedResourcesSvc<T> {
                        type Response = super::DiscoveryResponse;
                        type ResponseStream = T::StreamAggregatedResourcesStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                tonic::Streaming<super::DiscoveryRequest>,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as AggregatedDiscoveryService>::stream_aggregated_resources(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = StreamAggregatedResourcesSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/envoy.service.discovery.v3.AggregatedDiscoveryService/DeltaAggregatedResources" => {
                    #[allow(non_camel_case_types)]
                    struct DeltaAggregatedResourcesSvc<T: AggregatedDiscoveryService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: AggregatedDiscoveryService,
                    > tonic::server::StreamingService<super::DeltaDiscoveryRequest>
                    for DeltaAggregatedResourcesSvc<T> {
                        type Response = super::DeltaDiscoveryResponse;
                        type ResponseStream = T::DeltaAggregatedResourcesStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                tonic::Streaming<super::DeltaDiscoveryRequest>,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as AggregatedDiscoveryService>::delta_aggregated_resources(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DeltaAggregatedResourcesSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: AggregatedDiscoveryService> Clone for AggregatedDiscoveryServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
                max_decoding_message_size: self.max_decoding_message_size,
                max_encoding_message_size: self.max_encoding_message_size,
            }
        }
    }
    impl<T: AggregatedDiscoveryService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: AggregatedDiscoveryService> tonic::server::NamedService
    for AggregatedDiscoveryServiceServer<T> {
        const NAME: &'static str = "envoy.service.discovery.v3.AggregatedDiscoveryService";
    }
}
