#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Lua {
    /// The Lua code that Envoy will execute. This can be a very small script that
    /// further loads code from disk if desired. Note that if JSON configuration is used, the code must
    /// be properly escaped. YAML configuration may be easier to read since YAML supports multi-line
    /// strings so complex scripts can be easily expressed inline in the configuration.
    ///
    /// This field is deprecated. Please use
    /// :ref:`default_source_code <envoy_v3_api_field_extensions.filters.http.lua.v3.Lua.default_source_code>`.
    /// Only one of :ref:`inline_code <envoy_v3_api_field_extensions.filters.http.lua.v3.Lua.inline_code>`
    /// or :ref:`default_source_code <envoy_v3_api_field_extensions.filters.http.lua.v3.Lua.default_source_code>`
    /// can be set for the Lua filter.
    #[deprecated]
    #[prost(string, tag = "1")]
    pub inline_code: ::prost::alloc::string::String,
    /// Map of named Lua source codes that can be referenced in :ref:`LuaPerRoute
    /// <envoy_v3_api_msg_extensions.filters.http.lua.v3.LuaPerRoute>`. The Lua source codes can be
    /// loaded from inline string or local files.
    ///
    /// Example:
    ///
    /// .. code-block:: yaml
    ///
    ///    source_codes:
    ///      hello.lua:
    ///        inline_string: |
    ///          function envoy_on_response(response_handle)
    ///            -- Do something.
    ///          end
    ///      world.lua:
    ///        filename: /etc/lua/world.lua
    ///
    #[prost(map = "string, message", tag = "2")]
    pub source_codes: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        super::super::super::super::super::config::core::v3::DataSource,
    >,
    /// The default Lua code that Envoy will execute. If no per route config is provided
    /// for the request, this Lua code will be applied.
    #[prost(message, optional, tag = "3")]
    pub default_source_code: ::core::option::Option<
        super::super::super::super::super::config::core::v3::DataSource,
    >,
    /// Optional additional prefix to use when emitting statistics. By default
    /// metrics are emitted in *.lua.* namespace. If multiple lua filters are
    /// configured in a filter chain, the stats from each filter instance can
    /// be emitted using custom stat prefix to distinguish emitted
    /// statistics. For example:
    ///
    /// .. code-block:: yaml
    ///
    ///    http_filters:
    ///      - name: envoy.filters.http.lua
    ///        typed_config:
    ///          "@type": type.googleapis.com/envoy.extensions.filters.http.lua.v3.Lua
    ///          stat_prefix: foo_script # This emits lua.foo_script.errors etc.
    ///      - name: envoy.filters.http.lua
    ///        typed_config:
    ///          "@type": type.googleapis.com/envoy.extensions.filters.http.lua.v3.Lua
    ///          stat_prefix: bar_script # This emits lua.bar_script.errors etc.
    ///
    #[prost(string, tag = "4")]
    pub stat_prefix: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LuaPerRoute {
    #[prost(oneof = "lua_per_route::Override", tags = "1, 2, 3")]
    pub r#override: ::core::option::Option<lua_per_route::Override>,
}
/// Nested message and enum types in `LuaPerRoute`.
pub mod lua_per_route {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Override {
        /// Disable the Lua filter for this particular vhost or route. If disabled is specified in
        /// multiple per-filter-configs, the most specific one will be used.
        #[prost(bool, tag = "1")]
        Disabled(bool),
        /// A name of a Lua source code stored in
        /// :ref:`Lua.source_codes <envoy_v3_api_field_extensions.filters.http.lua.v3.Lua.source_codes>`.
        #[prost(string, tag = "2")]
        Name(::prost::alloc::string::String),
        /// A configured per-route Lua source code that can be served by RDS or provided inline.
        #[prost(message, tag = "3")]
        SourceCode(
            super::super::super::super::super::super::config::core::v3::DataSource,
        ),
    }
}
