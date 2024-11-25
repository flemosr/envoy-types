// This file is @generated by prost-build.
/// A filter state key and value pair.
/// \[\#next-free-field: 7\]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FilterStateValue {
    /// Optional filter object factory lookup key. See :ref:`the well-known filter state keys <well_known_filter_state>`
    /// for a list of valid factory keys.
    #[prost(string, tag = "6")]
    pub factory_key: ::prost::alloc::string::String,
    /// If marked as read-only, the filter state key value is locked, and cannot
    /// be overridden by any filter, including this filter.
    #[prost(bool, tag = "3")]
    pub read_only: bool,
    /// Configures the object to be shared with the upstream internal connections. See :ref:`internal upstream  transport <config_internal_upstream_transport>` for more details on the filter state sharing with
    /// the internal connections.
    #[prost(enumeration = "filter_state_value::SharedWithUpstream", tag = "4")]
    pub shared_with_upstream: i32,
    /// Skip the update if the value evaluates to an empty string.
    /// This option can be used to supply multiple alternatives for the same filter state object key.
    #[prost(bool, tag = "5")]
    pub skip_if_empty: bool,
    #[prost(oneof = "filter_state_value::Key", tags = "1")]
    pub key: ::core::option::Option<filter_state_value::Key>,
    #[prost(oneof = "filter_state_value::Value", tags = "2")]
    pub value: ::core::option::Option<filter_state_value::Value>,
}
/// Nested message and enum types in `FilterStateValue`.
pub mod filter_state_value {
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
    pub enum SharedWithUpstream {
        /// Object is not shared with the upstream internal connections.
        None = 0,
        /// Object is shared with the upstream internal connection.
        Once = 1,
        /// Object is shared with the upstream internal connection and any internal connection upstream from it.
        Transitive = 2,
    }
    impl SharedWithUpstream {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Self::None => "NONE",
                Self::Once => "ONCE",
                Self::Transitive => "TRANSITIVE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "NONE" => Some(Self::None),
                "ONCE" => Some(Self::Once),
                "TRANSITIVE" => Some(Self::Transitive),
                _ => None,
            }
        }
    }
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Key {
        ///
        /// Filter state object key. The key is used to lookup the object factory, unless :ref:`factory_key  <envoy_v3_api_field_extensions.filters.common.set_filter_state.v3.FilterStateValue.factory_key>` is set. See
        /// : ref:`the well-known filter state keys <well_known_filter_state>` for a list of valid object keys.
        #[prost(string, tag = "1")]
        ObjectKey(::prost::alloc::string::String),
    }
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Value {
        /// Uses the :ref:`format string <config_access_log_format_strings>` to
        /// instantiate the filter state object value.
        #[prost(message, tag = "2")]
        FormatString(
            super::super::super::super::super::super::config::core::v3::SubstitutionFormatString,
        ),
    }
}
