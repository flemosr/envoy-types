/// [#next-free-field: 12]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Gzip {
    /// Value from 1 to 9 that controls the amount of internal memory used by zlib. Higher values
    /// use more memory, but are faster and produce better compression results. The default value is 5.
    #[prost(message, optional, tag = "1")]
    pub memory_level: ::core::option::Option<
        super::super::super::super::super::super::google::protobuf::UInt32Value,
    >,
    /// A value used for selecting the zlib compression level. This setting will affect speed and
    /// amount of compression applied to the content. "BEST" provides higher compression at the cost of
    /// higher latency, "SPEED" provides lower compression with minimum impact on response time.
    /// "DEFAULT" provides an optimal result between speed and compression. This field will be set to
    /// "DEFAULT" if not specified.
    #[prost(enumeration = "gzip::compression_level::Enum", tag = "3")]
    pub compression_level: i32,
    /// A value used for selecting the zlib compression strategy which is directly related to the
    /// characteristics of the content. Most of the time "DEFAULT" will be the best choice, though
    /// there are situations which changing this parameter might produce better results. For example,
    /// run-length encoding (RLE) is typically used when the content is known for having sequences
    /// which same data occurs many consecutive times. For more information about each strategy, please
    /// refer to zlib manual.
    #[prost(enumeration = "gzip::CompressionStrategy", tag = "4")]
    pub compression_strategy: i32,
    /// Value from 9 to 15 that represents the base two logarithmic of the compressor's window size.
    /// Larger window results in better compression at the expense of memory usage. The default is 12
    /// which will produce a 4096 bytes window. For more details about this parameter, please refer to
    /// zlib manual > deflateInit2.
    #[prost(message, optional, tag = "9")]
    pub window_bits: ::core::option::Option<
        super::super::super::super::super::super::google::protobuf::UInt32Value,
    >,
    /// Set of configuration parameters common for all compression filters. You can define
    /// ``content_length``, ``content_type`` and other parameters in this field.
    #[prost(message, optional, tag = "10")]
    pub compressor: ::core::option::Option<super::super::compressor::v3::Compressor>,
    /// Value for Zlib's next output buffer. If not set, defaults to 4096.
    /// See <https://www.zlib.net/manual.html> for more details. Also see
    /// <https://github.com/envoyproxy/envoy/issues/8448> for context on this filter's performance.
    #[prost(message, optional, tag = "11")]
    pub chunk_size: ::core::option::Option<
        super::super::super::super::super::super::google::protobuf::UInt32Value,
    >,
}
/// Nested message and enum types in `Gzip`.
pub mod gzip {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CompressionLevel {}
    /// Nested message and enum types in `CompressionLevel`.
    pub mod compression_level {
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
        pub enum Enum {
            Default = 0,
            Best = 1,
            Speed = 2,
        }
        impl Enum {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    Enum::Default => "DEFAULT",
                    Enum::Best => "BEST",
                    Enum::Speed => "SPEED",
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "DEFAULT" => Some(Self::Default),
                    "BEST" => Some(Self::Best),
                    "SPEED" => Some(Self::Speed),
                    _ => None,
                }
            }
        }
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
    pub enum CompressionStrategy {
        Default = 0,
        Filtered = 1,
        Huffman = 2,
        Rle = 3,
    }
    impl CompressionStrategy {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                CompressionStrategy::Default => "DEFAULT",
                CompressionStrategy::Filtered => "FILTERED",
                CompressionStrategy::Huffman => "HUFFMAN",
                CompressionStrategy::Rle => "RLE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "DEFAULT" => Some(Self::Default),
                "FILTERED" => Some(Self::Filtered),
                "HUFFMAN" => Some(Self::Huffman),
                "RLE" => Some(Self::Rle),
                _ => None,
            }
        }
    }
}
