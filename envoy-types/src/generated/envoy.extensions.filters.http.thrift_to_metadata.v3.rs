#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeyValuePair {
    /// The namespace — if this is empty, the filter's namespace will be used.
    #[prost(string, tag = "1")]
    pub metadata_namespace: ::prost::alloc::string::String,
    /// The key to use within the namespace.
    #[prost(string, tag = "2")]
    pub key: ::prost::alloc::string::String,
    /// When used for on_present case, if value is non-empty it'll be used instead
    /// of the field value.
    ///
    /// When used for on_missing case, a non-empty value must be provided.
    #[prost(message, optional, tag = "3")]
    pub value: ::core::option::Option<
        super::super::super::super::super::super::google::protobuf::Value,
    >,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FieldSelector {
    /// field name to log
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// field id to match
    #[prost(int32, tag = "2")]
    pub id: i32,
    /// next node of the field selector
    #[prost(message, optional, boxed, tag = "3")]
    pub child: ::core::option::Option<::prost::alloc::boxed::Box<FieldSelector>>,
}
/// \[\#next-free-field: 6\]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Rule {
    /// The field to match on. If set, takes precedence over field_selector.
    #[prost(enumeration = "Field", tag = "1")]
    pub field: i32,
    /// Specifies that a match will be performed on the value of a field in the thrift body.
    /// If set, the whole http body will be buffered to extract the field value, which
    /// may have performance implications.
    ///
    /// It's a thrift over http version of
    /// :ref:`field_selector<envoy_v3_api_field_extensions.filters.network.thrift_proxy.filters.payload_to_metadata.v3.PayloadToMetadata.Rule.field_selector>`.
    ///
    /// See also `payload-to-metadata <<https://www.envoyproxy.io/docs/envoy/latest/configuration/other_protocols/thrift_filters/payload_to_metadata_filter>`\_>
    /// for more reference.
    ///
    /// Example:
    ///
    /// .. code-block:: yaml
    ///
    /// ```text
    /// method_name: foo
    /// field_selector:
    ///   name: info
    ///   id: 2
    ///   child:
    ///     name: version
    ///     id: 1
    /// ```
    ///
    /// The above yaml will match on value of `info.version` in the below thrift schema as input of
    /// :ref:`on_present<envoy_v3_api_field_extensions.filters.http.thrift_to_metadata.v3.Rule.on_present>` or
    /// :ref:`on_missing<envoy_v3_api_field_extensions.filters.http.thrift_to_metadata.v3.Rule.on_missing>`
    /// while we are processing `foo` method. This rule won't be applied to `bar` method.
    ///
    /// .. code-block:: thrift
    ///
    /// ```text
    /// struct Info {
    ///   1: required string version;
    /// }
    /// service Server {
    ///   bool foo(1: i32 id, 2: Info info);
    ///   bool bar(1: i32 id, 2: Info info);
    /// }
    /// ```
    #[prost(message, optional, tag = "2")]
    pub field_selector: ::core::option::Option<FieldSelector>,
    /// If specified, :ref:`field_selector<envoy_v3_api_field_extensions.filters.http.thrift_to_metadata.v3.Rule.field_selector>`
    /// will be used to extract the field value *only* on the thrift message with method name.
    #[prost(string, tag = "3")]
    pub method_name: ::prost::alloc::string::String,
    /// The key-value pair to set in the *filter metadata* if the field is present
    /// in *thrift metadata*.
    ///
    /// If the value in the KeyValuePair is non-empty, it'll be used instead
    /// of field value.
    #[prost(message, optional, tag = "4")]
    pub on_present: ::core::option::Option<KeyValuePair>,
    /// The key-value pair to set in the *filter metadata* if the field is missing
    /// in *thrift metadata*.
    ///
    /// The value in the KeyValuePair must be set, since it'll be used in lieu
    /// of the missing field value.
    #[prost(message, optional, tag = "5")]
    pub on_missing: ::core::option::Option<KeyValuePair>,
}
/// The configuration for transforming thrift metadata into filter metadata.
///
/// \[\#next-free-field: 7\]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ThriftToMetadata {
    /// The list of rules to apply to http request body to extract thrift metadata.
    #[prost(message, repeated, tag = "1")]
    pub request_rules: ::prost::alloc::vec::Vec<Rule>,
    /// The list of rules to apply to http response body to extract thrift metadata.
    #[prost(message, repeated, tag = "2")]
    pub response_rules: ::prost::alloc::vec::Vec<Rule>,
    /// Supplies the type of transport that the Thrift proxy should use. Defaults to
    /// :ref:`AUTO_TRANSPORT<envoy_v3_api_enum_value_extensions.filters.network.thrift_proxy.v3.TransportType.AUTO_TRANSPORT>`.
    #[prost(
        enumeration = "super::super::super::network::thrift_proxy::v3::TransportType",
        tag = "3"
    )]
    pub transport: i32,
    /// Supplies the type of protocol that the Thrift proxy should use. Defaults to
    /// :ref:`AUTO_PROTOCOL<envoy_v3_api_enum_value_extensions.filters.network.thrift_proxy.v3.ProtocolType.AUTO_PROTOCOL>`.
    /// Note that :ref:`LAX_BINARY<envoy_v3_api_enum_value_extensions.filters.network.thrift_proxy.v3.ProtocolType.LAX_BINARY>`
    /// is not distinguished by :ref:`AUTO_PROTOCOL<envoy_v3_api_enum_value_extensions.filters.network.thrift_proxy.v3.ProtocolType.AUTO_PROTOCOL>`,
    /// which is the same with :ref:`thrift_proxy network filter <envoy_v3_api_msg_extensions.filters.network.thrift_proxy.v3.ThriftProxy>`.
    /// Note that :ref:`TWITTER<envoy_v3_api_enum_value_extensions.filters.network.thrift_proxy.v3.ProtocolType.TWITTER>` is
    /// not supported due to deprecation in envoy.
    #[prost(
        enumeration = "super::super::super::network::thrift_proxy::v3::ProtocolType",
        tag = "4"
    )]
    pub protocol: i32,
    /// Allowed content-type for thrift payload to filter metadata transformation.
    /// Default to `{"application/x-thrift"}`.
    ///
    /// Set `allow_empty_content_type` if empty/missing content-type header
    /// is allowed.
    #[prost(string, repeated, tag = "5")]
    pub allow_content_types: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Allowed empty content-type for thrift payload to filter metadata transformation.
    /// Default to false.
    #[prost(bool, tag = "6")]
    pub allow_empty_content_type: bool,
}
/// Thrift to metadata configuration on a per-route basis, which overrides the global configuration for
/// request rules and responses rules.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ThriftToMetadataPerRoute {
    /// The list of rules to apply to http request body to extract thrift metadata.
    #[prost(message, repeated, tag = "1")]
    pub request_rules: ::prost::alloc::vec::Vec<Rule>,
    /// The list of rules to apply to http response body to extract thrift metadata.
    #[prost(message, repeated, tag = "2")]
    pub response_rules: ::prost::alloc::vec::Vec<Rule>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Field {
    /// The Thrift method name, string value.
    MethodName = 0,
    /// The Thrift protocol name, string value. Values are "binary", "binary/non-strict", and "compact", with "(auto)" suffix if
    /// :ref:`protocol <envoy_v3_api_field_extensions.filters.http.thrift_to_metadata.v3.ThriftToMetadata.protocol>`
    /// is set to :ref:`AUTO_PROTOCOL<envoy_v3_api_enum_value_extensions.filters.network.thrift_proxy.v3.ProtocolType.AUTO_PROTOCOL>`
    Protocol = 1,
    /// The Thrift transport name, string value. Values are "framed", "header", and "unframed", with "(auto)" suffix if
    /// :ref:`transport <envoy_v3_api_field_extensions.filters.http.thrift_to_metadata.v3.ThriftToMetadata.transport>`
    /// is set to :ref:`AUTO_TRANSPORT<envoy_v3_api_enum_value_extensions.filters.network.thrift_proxy.v3.TransportType.AUTO_TRANSPORT>`
    Transport = 2,
    /// The Thrift message type, singed 16-bit integer value.
    HeaderFlags = 3,
    /// The Thrift sequence ID, singed 32-bit integer value.
    SequenceId = 4,
    /// The Thrift message type, string value. Values in request are "call" and "oneway", and in response are "reply" and "exception".
    MessageType = 5,
    /// The Thrift reply type, string value. This is only valid for response rules. Values are "success" and "error".
    ReplyType = 6,
}
impl Field {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Field::MethodName => "METHOD_NAME",
            Field::Protocol => "PROTOCOL",
            Field::Transport => "TRANSPORT",
            Field::HeaderFlags => "HEADER_FLAGS",
            Field::SequenceId => "SEQUENCE_ID",
            Field::MessageType => "MESSAGE_TYPE",
            Field::ReplyType => "REPLY_TYPE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "METHOD_NAME" => Some(Self::MethodName),
            "PROTOCOL" => Some(Self::Protocol),
            "TRANSPORT" => Some(Self::Transport),
            "HEADER_FLAGS" => Some(Self::HeaderFlags),
            "SEQUENCE_ID" => Some(Self::SequenceId),
            "MESSAGE_TYPE" => Some(Self::MessageType),
            "REPLY_TYPE" => Some(Self::ReplyType),
            _ => None,
        }
    }
}
