/// [#next-free-field: 7]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Brotli {
    /// Value from 0 to 11 that controls the main compression speed-density lever.
    /// The higher quality, the slower compression. The default value is 3.
    #[prost(message, optional, tag = "1")]
    pub quality: ::core::option::Option<
        super::super::super::super::super::super::google::protobuf::UInt32Value,
    >,
    /// A value used to tune encoder for specific input. For more information about modes,
    /// please refer to brotli manual: <https://brotli.org/encode.html#aa6f>
    /// This field will be set to "DEFAULT" if not specified.
    #[prost(enumeration = "brotli::EncoderMode", tag = "2")]
    pub encoder_mode: i32,
    /// Value from 10 to 24 that represents the base two logarithmic of the compressor's window size.
    /// Larger window results in better compression at the expense of memory usage. The default is 18.
    /// For more details about this parameter, please refer to brotli manual:
    /// <https://brotli.org/encode.html#a9a8>
    #[prost(message, optional, tag = "3")]
    pub window_bits: ::core::option::Option<
        super::super::super::super::super::super::google::protobuf::UInt32Value,
    >,
    /// Value from 16 to 24 that represents the base two logarithmic of the compressor's input block
    /// size. Larger input block results in better compression at the expense of memory usage. The
    /// default is 24. For more details about this parameter, please refer to brotli manual:
    /// <https://brotli.org/encode.html#a9a8>
    #[prost(message, optional, tag = "4")]
    pub input_block_bits: ::core::option::Option<
        super::super::super::super::super::super::google::protobuf::UInt32Value,
    >,
    /// Value for compressor's next output buffer. If not set, defaults to 4096.
    #[prost(message, optional, tag = "5")]
    pub chunk_size: ::core::option::Option<
        super::super::super::super::super::super::google::protobuf::UInt32Value,
    >,
    /// If true, disables "literal context modeling" format feature.
    /// This flag is a "decoding-speed vs compression ratio" trade-off.
    #[prost(bool, tag = "6")]
    pub disable_literal_context_modeling: bool,
}
/// Nested message and enum types in `Brotli`.
pub mod brotli {
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
    pub enum EncoderMode {
        Default = 0,
        Generic = 1,
        Text = 2,
        Font = 3,
    }
    impl EncoderMode {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                EncoderMode::Default => "DEFAULT",
                EncoderMode::Generic => "GENERIC",
                EncoderMode::Text => "TEXT",
                EncoderMode::Font => "FONT",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "DEFAULT" => Some(Self::Default),
                "GENERIC" => Some(Self::Generic),
                "TEXT" => Some(Self::Text),
                "FONT" => Some(Self::Font),
                _ => None,
            }
        }
    }
}
