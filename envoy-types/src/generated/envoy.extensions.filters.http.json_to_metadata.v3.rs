#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct JsonToMetadata {
    /// At least one of request_rules and response_rules must be provided.
    /// Rules to match json body of requests.
    #[prost(message, optional, tag = "1")]
    pub request_rules: ::core::option::Option<json_to_metadata::MatchRules>,
    /// Rules to match json body of responses.
    #[prost(message, optional, tag = "2")]
    pub response_rules: ::core::option::Option<json_to_metadata::MatchRules>,
}
/// Nested message and enum types in `JsonToMetadata`.
pub mod json_to_metadata {
    /// \[\#next-free-field: 6\]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct KeyValuePair {
        /// The namespace — if this is empty, the filter's namespace will be used.
        #[prost(string, tag = "1")]
        pub metadata_namespace: ::prost::alloc::string::String,
        /// The key to use within the namespace.
        #[prost(string, tag = "2")]
        pub key: ::prost::alloc::string::String,
        /// The value's type — defaults to protobuf.Value.
        #[prost(enumeration = "ValueType", tag = "4")]
        pub r#type: i32,
        /// False if we want to overwrite the existing metadata value. Default to false.
        #[prost(bool, tag = "5")]
        pub preserve_existing_metadata_value: bool,
        #[prost(oneof = "key_value_pair::ValueType", tags = "3")]
        pub value_type: ::core::option::Option<key_value_pair::ValueType>,
    }
    /// Nested message and enum types in `KeyValuePair`.
    pub mod key_value_pair {
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum ValueType {
            /// The value to pair with the given key.
            ///
            /// When used for on_present case, if value is non-empty it'll be used instead
            /// of the the value of the JSON key. If both are empty, the the value of the
            /// JSON key is used as-is.
            ///
            /// When used for on_missing/on_error case, a non-empty value
            /// must be provided.
            ///
            /// It ignores ValueType, i.e., not type casting.
            #[prost(message, tag = "3")]
            Value(
                super::super::super::super::super::super::super::super::google::protobuf::Value,
            ),
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Selector {
        #[prost(oneof = "selector::Selector", tags = "1")]
        pub selector: ::core::option::Option<selector::Selector>,
    }
    /// Nested message and enum types in `Selector`.
    pub mod selector {
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Selector {
            /// key to match
            #[prost(string, tag = "1")]
            Key(::prost::alloc::string::String),
        }
    }
    /// A Rule defines what metadata to apply when a key-value is present, missing in the json
    /// or fail to parse the payload.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Rule {
        /// Specifies that a match will be performed on the value of a property.
        /// Here's an example to match on 1 in {"foo": {"bar": 1}, "bar": 2}
        ///
        /// selectors:
        ///
        /// * key: foo
        /// * key: bar
        #[prost(message, repeated, tag = "1")]
        pub selectors: ::prost::alloc::vec::Vec<Selector>,
        /// If the attribute is present, apply this metadata KeyValuePair.
        #[prost(message, optional, tag = "2")]
        pub on_present: ::core::option::Option<KeyValuePair>,
        /// If the attribute is missing, apply this metadata KeyValuePair.
        ///
        /// The value in the KeyValuePair must be set.
        #[prost(message, optional, tag = "3")]
        pub on_missing: ::core::option::Option<KeyValuePair>,
        /// If the body is too large or fail to parse, apply this metadata KeyValuePair.
        ///
        /// The value in the KeyValuePair must be set.
        #[prost(message, optional, tag = "4")]
        pub on_error: ::core::option::Option<KeyValuePair>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct MatchRules {
        /// The list of rules to apply.
        #[prost(message, repeated, tag = "1")]
        pub rules: ::prost::alloc::vec::Vec<Rule>,
        /// Allowed content-type for json to metadata transformation.
        /// Default to `{"application/json"}`.
        ///
        /// Set `allow_empty_content_type` if empty/missing content-type header
        /// is allowed.
        #[prost(string, repeated, tag = "2")]
        pub allow_content_types: ::prost::alloc::vec::Vec<
            ::prost::alloc::string::String,
        >,
        /// Allowed empty content-type for json to metadata transformation.
        /// Default to false.
        #[prost(bool, tag = "3")]
        pub allow_empty_content_type: bool,
    }
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
    pub enum ValueType {
        /// The value is a serialized `protobuf.Value <<https://github.com/protocolbuffers/protobuf/blob/master/src/google/protobuf/struct.proto#L62>`\_.>
        ProtobufValue = 0,
        String = 1,
        Number = 2,
    }
    impl ValueType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ValueType::ProtobufValue => "PROTOBUF_VALUE",
                ValueType::String => "STRING",
                ValueType::Number => "NUMBER",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "PROTOBUF_VALUE" => Some(Self::ProtobufValue),
                "STRING" => Some(Self::String),
                "NUMBER" => Some(Self::Number),
                _ => None,
            }
        }
    }
}
