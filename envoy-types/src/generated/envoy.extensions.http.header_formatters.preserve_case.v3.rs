// This file is @generated by prost-build.
/// Configuration for the preserve case header formatter.
/// See the :ref:`header casing <config_http_conn_man_header_casing>` configuration guide for more
/// information.
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct PreserveCaseFormatterConfig {
    /// Allows forwarding reason phrase text.
    /// This is off by default, and a standard reason phrase is used for a corresponding HTTP response code.
    #[prost(bool, tag = "1")]
    pub forward_reason_phrase: bool,
    /// Type of formatter to use on headers which are added by Envoy (which are lower case by default).
    /// The default type is DEFAULT, use LowerCase on Envoy headers.
    #[prost(
        enumeration = "preserve_case_formatter_config::FormatterTypeOnEnvoyHeaders",
        tag = "2"
    )]
    pub formatter_type_on_envoy_headers: i32,
}
/// Nested message and enum types in `PreserveCaseFormatterConfig`.
pub mod preserve_case_formatter_config {
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
    pub enum FormatterTypeOnEnvoyHeaders {
        /// Use LowerCase on Envoy added headers.
        Default = 0,
        /// Use ProperCaseHeaderKeyFormatter on Envoy added headers that upper cases the first character
        /// in each word. The first character as well as any alpha character following a special
        /// character is upper cased.
        ProperCase = 1,
    }
    impl FormatterTypeOnEnvoyHeaders {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Self::Default => "DEFAULT",
                Self::ProperCase => "PROPER_CASE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "DEFAULT" => Some(Self::Default),
                "PROPER_CASE" => Some(Self::ProperCase),
                _ => None,
            }
        }
    }
}
