pub mod envoy {
    pub mod admin {
        pub mod v3 {
            include!("envoy.admin.v3.rs");
        }
    }
    pub mod annotations {
        include!("envoy.annotations.rs");
    }
    pub mod config {
        pub mod accesslog {
            pub mod v3 {
                include!("envoy.config.accesslog.v3.rs");
            }
        }
        pub mod bootstrap {
            pub mod v3 {
                include!("envoy.config.bootstrap.v3.rs");
            }
        }
        pub mod cluster {
            pub mod v3 {
                include!("envoy.config.cluster.v3.rs");
            }
        }
        pub mod common {
            pub mod key_value {
                pub mod v3 {
                    include!("envoy.config.common.key_value.v3.rs");
                }
            }
            pub mod matcher {
                pub mod v3 {
                    include!("envoy.config.common.matcher.v3.rs");
                }
            }
            pub mod mutation_rules {
                pub mod v3 {
                    include!("envoy.config.common.mutation_rules.v3.rs");
                }
            }
        }
        pub mod core {
            pub mod v3 {
                include!("envoy.config.core.v3.rs");
            }
        }
        pub mod endpoint {
            pub mod v3 {
                include!("envoy.config.endpoint.v3.rs");
            }
        }
        pub mod grpc_credential {
            pub mod v3 {
                include!("envoy.config.grpc_credential.v3.rs");
            }
        }
        pub mod listener {
            pub mod v3 {
                include!("envoy.config.listener.v3.rs");
            }
        }
        pub mod metrics {
            pub mod v3 {
                include!("envoy.config.metrics.v3.rs");
            }
        }
        pub mod overload {
            pub mod v3 {
                include!("envoy.config.overload.v3.rs");
            }
        }
        pub mod ratelimit {
            pub mod v3 {
                include!("envoy.config.ratelimit.v3.rs");
            }
        }
        pub mod rbac {
            pub mod v3 {
                include!("envoy.config.rbac.v3.rs");
            }
        }
        pub mod route {
            pub mod v3 {
                include!("envoy.config.route.v3.rs");
            }
        }
        pub mod tap {
            pub mod v3 {
                include!("envoy.config.tap.v3.rs");
            }
        }
        pub mod trace {
            pub mod v3 {
                include!("envoy.config.trace.v3.rs");
            }
        }
        pub mod upstream {
            pub mod local_address_selector {
                pub mod v3 {
                    include!("envoy.config.upstream.local_address_selector.v3.rs");
                }
            }
        }
    }
    pub mod data {
        pub mod accesslog {
            pub mod v3 {
                include!("envoy.data.accesslog.v3.rs");
            }
        }
        pub mod cluster {
            pub mod v3 {
                include!("envoy.data.cluster.v3.rs");
            }
        }
        pub mod core {
            pub mod v3 {
                include!("envoy.data.core.v3.rs");
            }
        }
        pub mod dns {
            pub mod v3 {
                include!("envoy.data.dns.v3.rs");
            }
        }
        pub mod tap {
            pub mod v3 {
                include!("envoy.data.tap.v3.rs");
            }
        }
    }
    pub mod extensions {
        pub mod access_loggers {
            pub mod file {
                pub mod v3 {
                    include!("envoy.extensions.access_loggers.file.v3.rs");
                }
            }
            pub mod filters {
                pub mod cel {
                    pub mod v3 {
                        include!("envoy.extensions.access_loggers.filters.cel.v3.rs");
                    }
                }
            }
            pub mod fluentd {
                pub mod v3 {
                    include!("envoy.extensions.access_loggers.fluentd.v3.rs");
                }
            }
            pub mod grpc {
                pub mod v3 {
                    include!("envoy.extensions.access_loggers.grpc.v3.rs");
                }
            }
            pub mod open_telemetry {
                pub mod v3 {
                    include!("envoy.extensions.access_loggers.open_telemetry.v3.rs");
                }
            }
            pub mod stream {
                pub mod v3 {
                    include!("envoy.extensions.access_loggers.stream.v3.rs");
                }
            }
            pub mod wasm {
                pub mod v3 {
                    include!("envoy.extensions.access_loggers.wasm.v3.rs");
                }
            }
        }
        pub mod bootstrap {
            pub mod internal_listener {
                pub mod v3 {
                    include!("envoy.extensions.bootstrap.internal_listener.v3.rs");
                }
            }
        }
        pub mod clusters {
            pub mod aggregate {
                pub mod v3 {
                    include!("envoy.extensions.clusters.aggregate.v3.rs");
                }
            }
            pub mod dynamic_forward_proxy {
                pub mod v3 {
                    include!("envoy.extensions.clusters.dynamic_forward_proxy.v3.rs");
                }
            }
            pub mod redis {
                pub mod v3 {
                    include!("envoy.extensions.clusters.redis.v3.rs");
                }
            }
        }
        pub mod common {
            pub mod async_files {
                pub mod v3 {
                    include!("envoy.extensions.common.async_files.v3.rs");
                }
            }
            pub mod dynamic_forward_proxy {
                pub mod v3 {
                    include!("envoy.extensions.common.dynamic_forward_proxy.v3.rs");
                }
            }
            pub mod matching {
                pub mod v3 {
                    include!("envoy.extensions.common.matching.v3.rs");
                }
            }
            pub mod ratelimit {
                pub mod v3 {
                    include!("envoy.extensions.common.ratelimit.v3.rs");
                }
            }
            pub mod tap {
                pub mod v3 {
                    include!("envoy.extensions.common.tap.v3.rs");
                }
            }
        }
        pub mod compression {
            pub mod brotli {
                pub mod compressor {
                    pub mod v3 {
                        include!("envoy.extensions.compression.brotli.compressor.v3.rs");
                    }
                }
                pub mod decompressor {
                    pub mod v3 {
                        include!("envoy.extensions.compression.brotli.decompressor.v3.rs");
                    }
                }
            }
            pub mod gzip {
                pub mod compressor {
                    pub mod v3 {
                        include!("envoy.extensions.compression.gzip.compressor.v3.rs");
                    }
                }
                pub mod decompressor {
                    pub mod v3 {
                        include!("envoy.extensions.compression.gzip.decompressor.v3.rs");
                    }
                }
            }
            pub mod zstd {
                pub mod compressor {
                    pub mod v3 {
                        include!("envoy.extensions.compression.zstd.compressor.v3.rs");
                    }
                }
                pub mod decompressor {
                    pub mod v3 {
                        include!("envoy.extensions.compression.zstd.decompressor.v3.rs");
                    }
                }
            }
        }
        pub mod config {
            pub mod validators {
                pub mod minimum_clusters {
                    pub mod v3 {
                        include!("envoy.extensions.config.validators.minimum_clusters.v3.rs");
                    }
                }
            }
        }
        pub mod early_data {
            pub mod v3 {
                include!("envoy.extensions.early_data.v3.rs");
            }
        }
        pub mod filters {
            pub mod common {
                pub mod dependency {
                    pub mod v3 {
                        include!("envoy.extensions.filters.common.dependency.v3.rs");
                    }
                }
                pub mod fault {
                    pub mod v3 {
                        include!("envoy.extensions.filters.common.fault.v3.rs");
                    }
                }
                pub mod matcher {
                    pub mod action {
                        pub mod v3 {
                            include!("envoy.extensions.filters.common.matcher.action.v3.rs");
                        }
                    }
                }
                pub mod set_filter_state {
                    pub mod v3 {
                        include!("envoy.extensions.filters.common.set_filter_state.v3.rs");
                    }
                }
            }
            pub mod http {
                pub mod adaptive_concurrency {
                    pub mod v3 {
                        include!("envoy.extensions.filters.http.adaptive_concurrency.v3.rs");
                    }
                }
                pub mod admission_control {
                    pub mod v3 {
                        include!("envoy.extensions.filters.http.admission_control.v3.rs");
                    }
                }
                pub mod alternate_protocols_cache {
                    pub mod v3 {
                        include!("envoy.extensions.filters.http.alternate_protocols_cache.v3.rs");
                    }
                }
                pub mod aws_lambda {
                    pub mod v3 {
                        include!("envoy.extensions.filters.http.aws_lambda.v3.rs");
                    }
                }
                pub mod aws_request_signing {
                    pub mod v3 {
                        include!("envoy.extensions.filters.http.aws_request_signing.v3.rs");
                    }
                }
                pub mod bandwidth_limit {
                    pub mod v3 {
                        include!("envoy.extensions.filters.http.bandwidth_limit.v3.rs");
                    }
                }
                pub mod basic_auth {
                    pub mod v3 {
                        include!("envoy.extensions.filters.http.basic_auth.v3.rs");
                    }
                }
                pub mod buffer {
                    pub mod v3 {
                        include!("envoy.extensions.filters.http.buffer.v3.rs");
                    }
                }
                pub mod cache {
                    pub mod v3 {
                        include!("envoy.extensions.filters.http.cache.v3.rs");
                    }
                }
                pub mod cdn_loop {
                    pub mod v3 {
                        include!("envoy.extensions.filters.http.cdn_loop.v3.rs");
                    }
                }
                pub mod composite {
                    pub mod v3 {
                        include!("envoy.extensions.filters.http.composite.v3.rs");
                    }
                }
                pub mod compressor {
                    pub mod v3 {
                        include!("envoy.extensions.filters.http.compressor.v3.rs");
                    }
                }
                pub mod connect_grpc_bridge {
                    pub mod v3 {
                        include!("envoy.extensions.filters.http.connect_grpc_bridge.v3.rs");
                    }
                }
                pub mod cors {
                    pub mod v3 {
                        include!("envoy.extensions.filters.http.cors.v3.rs");
                    }
                }
                pub mod credential_injector {
                    pub mod v3 {
                        include!("envoy.extensions.filters.http.credential_injector.v3.rs");
                    }
                }
                pub mod csrf {
                    pub mod v3 {
                        include!("envoy.extensions.filters.http.csrf.v3.rs");
                    }
                }
                pub mod custom_response {
                    pub mod v3 {
                        include!("envoy.extensions.filters.http.custom_response.v3.rs");
                    }
                }
                pub mod decompressor {
                    pub mod v3 {
                        include!("envoy.extensions.filters.http.decompressor.v3.rs");
                    }
                }
                pub mod dynamic_forward_proxy {
                    pub mod v3 {
                        include!("envoy.extensions.filters.http.dynamic_forward_proxy.v3.rs");
                    }
                }
                pub mod ext_authz {
                    pub mod v3 {
                        include!("envoy.extensions.filters.http.ext_authz.v3.rs");
                    }
                }
                pub mod ext_proc {
                    pub mod v3 {
                        include!("envoy.extensions.filters.http.ext_proc.v3.rs");
                    }
                }
                pub mod fault {
                    pub mod v3 {
                        include!("envoy.extensions.filters.http.fault.v3.rs");
                    }
                }
                pub mod file_system_buffer {
                    pub mod v3 {
                        include!("envoy.extensions.filters.http.file_system_buffer.v3.rs");
                    }
                }
                pub mod gcp_authn {
                    pub mod v3 {
                        include!("envoy.extensions.filters.http.gcp_authn.v3.rs");
                    }
                }
                pub mod geoip {
                    pub mod v3 {
                        include!("envoy.extensions.filters.http.geoip.v3.rs");
                    }
                }
                pub mod grpc_field_extraction {
                    pub mod v3 {
                        include!("envoy.extensions.filters.http.grpc_field_extraction.v3.rs");
                    }
                }
                pub mod grpc_http1_bridge {
                    pub mod v3 {
                        include!("envoy.extensions.filters.http.grpc_http1_bridge.v3.rs");
                    }
                }
                pub mod grpc_http1_reverse_bridge {
                    pub mod v3 {
                        include!("envoy.extensions.filters.http.grpc_http1_reverse_bridge.v3.rs");
                    }
                }
                pub mod grpc_json_transcoder {
                    pub mod v3 {
                        include!("envoy.extensions.filters.http.grpc_json_transcoder.v3.rs");
                    }
                }
                pub mod grpc_stats {
                    pub mod v3 {
                        include!("envoy.extensions.filters.http.grpc_stats.v3.rs");
                    }
                }
                pub mod grpc_web {
                    pub mod v3 {
                        include!("envoy.extensions.filters.http.grpc_web.v3.rs");
                    }
                }
                pub mod gzip {
                    pub mod v3 {
                        include!("envoy.extensions.filters.http.gzip.v3.rs");
                    }
                }
                pub mod header_mutation {
                    pub mod v3 {
                        include!("envoy.extensions.filters.http.header_mutation.v3.rs");
                    }
                }
                pub mod header_to_metadata {
                    pub mod v3 {
                        include!("envoy.extensions.filters.http.header_to_metadata.v3.rs");
                    }
                }
                pub mod health_check {
                    pub mod v3 {
                        include!("envoy.extensions.filters.http.health_check.v3.rs");
                    }
                }
                pub mod ip_tagging {
                    pub mod v3 {
                        include!("envoy.extensions.filters.http.ip_tagging.v3.rs");
                    }
                }
                pub mod json_to_metadata {
                    pub mod v3 {
                        include!("envoy.extensions.filters.http.json_to_metadata.v3.rs");
                    }
                }
                pub mod jwt_authn {
                    pub mod v3 {
                        include!("envoy.extensions.filters.http.jwt_authn.v3.rs");
                    }
                }
                pub mod kill_request {
                    pub mod v3 {
                        include!("envoy.extensions.filters.http.kill_request.v3.rs");
                    }
                }
                pub mod local_ratelimit {
                    pub mod v3 {
                        include!("envoy.extensions.filters.http.local_ratelimit.v3.rs");
                    }
                }
                pub mod lua {
                    pub mod v3 {
                        include!("envoy.extensions.filters.http.lua.v3.rs");
                    }
                }
                pub mod oauth2 {
                    pub mod v3 {
                        include!("envoy.extensions.filters.http.oauth2.v3.rs");
                    }
                }
                pub mod on_demand {
                    pub mod v3 {
                        include!("envoy.extensions.filters.http.on_demand.v3.rs");
                    }
                }
                pub mod original_src {
                    pub mod v3 {
                        include!("envoy.extensions.filters.http.original_src.v3.rs");
                    }
                }
                pub mod proto_message_logging {
                    pub mod v3 {
                        include!("envoy.extensions.filters.http.proto_message_logging.v3.rs");
                    }
                }
                pub mod rate_limit_quota {
                    pub mod v3 {
                        include!("envoy.extensions.filters.http.rate_limit_quota.v3.rs");
                    }
                }
                pub mod ratelimit {
                    pub mod v3 {
                        include!("envoy.extensions.filters.http.ratelimit.v3.rs");
                    }
                }
                pub mod rbac {
                    pub mod v3 {
                        include!("envoy.extensions.filters.http.rbac.v3.rs");
                    }
                }
                pub mod router {
                    pub mod v3 {
                        include!("envoy.extensions.filters.http.router.v3.rs");
                    }
                }
                pub mod set_filter_state {
                    pub mod v3 {
                        include!("envoy.extensions.filters.http.set_filter_state.v3.rs");
                    }
                }
                pub mod set_metadata {
                    pub mod v3 {
                        include!("envoy.extensions.filters.http.set_metadata.v3.rs");
                    }
                }
                pub mod stateful_session {
                    pub mod v3 {
                        include!("envoy.extensions.filters.http.stateful_session.v3.rs");
                    }
                }
                pub mod tap {
                    pub mod v3 {
                        include!("envoy.extensions.filters.http.tap.v3.rs");
                    }
                }
                pub mod upstream_codec {
                    pub mod v3 {
                        include!("envoy.extensions.filters.http.upstream_codec.v3.rs");
                    }
                }
                pub mod wasm {
                    pub mod v3 {
                        include!("envoy.extensions.filters.http.wasm.v3.rs");
                    }
                }
            }
            pub mod listener {
                pub mod http_inspector {
                    pub mod v3 {
                        include!("envoy.extensions.filters.listener.http_inspector.v3.rs");
                    }
                }
                pub mod local_ratelimit {
                    pub mod v3 {
                        include!("envoy.extensions.filters.listener.local_ratelimit.v3.rs");
                    }
                }
                pub mod original_dst {
                    pub mod v3 {
                        include!("envoy.extensions.filters.listener.original_dst.v3.rs");
                    }
                }
                pub mod original_src {
                    pub mod v3 {
                        include!("envoy.extensions.filters.listener.original_src.v3.rs");
                    }
                }
                pub mod proxy_protocol {
                    pub mod v3 {
                        include!("envoy.extensions.filters.listener.proxy_protocol.v3.rs");
                    }
                }
                pub mod tls_inspector {
                    pub mod v3 {
                        include!("envoy.extensions.filters.listener.tls_inspector.v3.rs");
                    }
                }
            }
            pub mod network {
                pub mod connection_limit {
                    pub mod v3 {
                        include!("envoy.extensions.filters.network.connection_limit.v3.rs");
                    }
                }
                pub mod direct_response {
                    pub mod v3 {
                        include!("envoy.extensions.filters.network.direct_response.v3.rs");
                    }
                }
                pub mod dubbo_proxy {
                    pub mod router {
                        pub mod v3 {
                            include!("envoy.extensions.filters.network.dubbo_proxy.router.v3.rs");
                        }
                    }
                    pub mod v3 {
                        include!("envoy.extensions.filters.network.dubbo_proxy.v3.rs");
                    }
                }
                pub mod echo {
                    pub mod v3 {
                        include!("envoy.extensions.filters.network.echo.v3.rs");
                    }
                }
                pub mod ext_authz {
                    pub mod v3 {
                        include!("envoy.extensions.filters.network.ext_authz.v3.rs");
                    }
                }
                pub mod http_connection_manager {
                    pub mod v3 {
                        include!("envoy.extensions.filters.network.http_connection_manager.v3.rs");
                    }
                }
                pub mod local_ratelimit {
                    pub mod v3 {
                        include!("envoy.extensions.filters.network.local_ratelimit.v3.rs");
                    }
                }
                pub mod mongo_proxy {
                    pub mod v3 {
                        include!("envoy.extensions.filters.network.mongo_proxy.v3.rs");
                    }
                }
                pub mod ratelimit {
                    pub mod v3 {
                        include!("envoy.extensions.filters.network.ratelimit.v3.rs");
                    }
                }
                pub mod rbac {
                    pub mod v3 {
                        include!("envoy.extensions.filters.network.rbac.v3.rs");
                    }
                }
                pub mod redis_proxy {
                    pub mod v3 {
                        include!("envoy.extensions.filters.network.redis_proxy.v3.rs");
                    }
                }
                pub mod set_filter_state {
                    pub mod v3 {
                        include!("envoy.extensions.filters.network.set_filter_state.v3.rs");
                    }
                }
                pub mod sni_cluster {
                    pub mod v3 {
                        include!("envoy.extensions.filters.network.sni_cluster.v3.rs");
                    }
                }
                pub mod sni_dynamic_forward_proxy {
                    pub mod v3 {
                        include!("envoy.extensions.filters.network.sni_dynamic_forward_proxy.v3.rs");
                    }
                }
                pub mod tcp_proxy {
                    pub mod v3 {
                        include!("envoy.extensions.filters.network.tcp_proxy.v3.rs");
                    }
                }
                pub mod thrift_proxy {
                    pub mod filters {
                        pub mod header_to_metadata {
                            pub mod v3 {
                                include!("envoy.extensions.filters.network.thrift_proxy.filters.header_to_metadata.v3.rs");
                            }
                        }
                        pub mod payload_to_metadata {
                            pub mod v3 {
                                include!("envoy.extensions.filters.network.thrift_proxy.filters.payload_to_metadata.v3.rs");
                            }
                        }
                        pub mod ratelimit {
                            pub mod v3 {
                                include!("envoy.extensions.filters.network.thrift_proxy.filters.ratelimit.v3.rs");
                            }
                        }
                    }
                    pub mod router {
                        pub mod v3 {
                            include!("envoy.extensions.filters.network.thrift_proxy.router.v3.rs");
                        }
                    }
                    pub mod v3 {
                        include!("envoy.extensions.filters.network.thrift_proxy.v3.rs");
                    }
                }
                pub mod wasm {
                    pub mod v3 {
                        include!("envoy.extensions.filters.network.wasm.v3.rs");
                    }
                }
                pub mod zookeeper_proxy {
                    pub mod v3 {
                        include!("envoy.extensions.filters.network.zookeeper_proxy.v3.rs");
                    }
                }
            }
            pub mod udp {
                pub mod dns_filter {
                    pub mod v3 {
                        include!("envoy.extensions.filters.udp.dns_filter.v3.rs");
                    }
                }
                pub mod udp_proxy {
                    pub mod session {
                        pub mod dynamic_forward_proxy {
                            pub mod v3 {
                                include!("envoy.extensions.filters.udp.udp_proxy.session.dynamic_forward_proxy.v3.rs");
                            }
                        }
                        pub mod http_capsule {
                            pub mod v3 {
                                include!("envoy.extensions.filters.udp.udp_proxy.session.http_capsule.v3.rs");
                            }
                        }
                    }
                    pub mod v3 {
                        include!("envoy.extensions.filters.udp.udp_proxy.v3.rs");
                    }
                }
            }
        }
        pub mod formatter {
            pub mod cel {
                pub mod v3 {
                    include!("envoy.extensions.formatter.cel.v3.rs");
                }
            }
            pub mod metadata {
                pub mod v3 {
                    include!("envoy.extensions.formatter.metadata.v3.rs");
                }
            }
            pub mod req_without_query {
                pub mod v3 {
                    include!("envoy.extensions.formatter.req_without_query.v3.rs");
                }
            }
        }
        pub mod geoip_providers {
            pub mod common {
                pub mod v3 {
                    include!("envoy.extensions.geoip_providers.common.v3.rs");
                }
            }
            pub mod maxmind {
                pub mod v3 {
                    include!("envoy.extensions.geoip_providers.maxmind.v3.rs");
                }
            }
        }
        pub mod health_check {
            pub mod event_sinks {
                pub mod file {
                    pub mod v3 {
                        include!("envoy.extensions.health_check.event_sinks.file.v3.rs");
                    }
                }
            }
        }
        pub mod health_checkers {
            pub mod redis {
                pub mod v3 {
                    include!("envoy.extensions.health_checkers.redis.v3.rs");
                }
            }
            pub mod thrift {
                pub mod v3 {
                    include!("envoy.extensions.health_checkers.thrift.v3.rs");
                }
            }
        }
        pub mod http {
            pub mod cache {
                pub mod file_system_http_cache {
                    pub mod v3 {
                        include!("envoy.extensions.http.cache.file_system_http_cache.v3.rs");
                    }
                }
                pub mod simple_http_cache {
                    pub mod v3 {
                        include!("envoy.extensions.http.cache.simple_http_cache.v3.rs");
                    }
                }
            }
            pub mod custom_response {
                pub mod local_response_policy {
                    pub mod v3 {
                        include!("envoy.extensions.http.custom_response.local_response_policy.v3.rs");
                    }
                }
                pub mod redirect_policy {
                    pub mod v3 {
                        include!("envoy.extensions.http.custom_response.redirect_policy.v3.rs");
                    }
                }
            }
            pub mod early_header_mutation {
                pub mod header_mutation {
                    pub mod v3 {
                        include!("envoy.extensions.http.early_header_mutation.header_mutation.v3.rs");
                    }
                }
            }
            pub mod header_formatters {
                pub mod preserve_case {
                    pub mod v3 {
                        include!("envoy.extensions.http.header_formatters.preserve_case.v3.rs");
                    }
                }
            }
            pub mod header_validators {
                pub mod envoy_default {
                    pub mod v3 {
                        include!("envoy.extensions.http.header_validators.envoy_default.v3.rs");
                    }
                }
            }
            pub mod original_ip_detection {
                pub mod custom_header {
                    pub mod v3 {
                        include!("envoy.extensions.http.original_ip_detection.custom_header.v3.rs");
                    }
                }
                pub mod xff {
                    pub mod v3 {
                        include!("envoy.extensions.http.original_ip_detection.xff.v3.rs");
                    }
                }
            }
            pub mod stateful_session {
                pub mod cookie {
                    pub mod v3 {
                        include!("envoy.extensions.http.stateful_session.cookie.v3.rs");
                    }
                }
                pub mod header {
                    pub mod v3 {
                        include!("envoy.extensions.http.stateful_session.header.v3.rs");
                    }
                }
            }
        }
        pub mod injected_credentials {
            pub mod generic {
                pub mod v3 {
                    include!("envoy.extensions.injected_credentials.generic.v3.rs");
                }
            }
            pub mod oauth2 {
                pub mod v3 {
                    include!("envoy.extensions.injected_credentials.oauth2.v3.rs");
                }
            }
        }
        pub mod internal_redirect {
            pub mod allow_listed_routes {
                pub mod v3 {
                    include!("envoy.extensions.internal_redirect.allow_listed_routes.v3.rs");
                }
            }
            pub mod previous_routes {
                pub mod v3 {
                    include!("envoy.extensions.internal_redirect.previous_routes.v3.rs");
                }
            }
            pub mod safe_cross_scheme {
                pub mod v3 {
                    include!("envoy.extensions.internal_redirect.safe_cross_scheme.v3.rs");
                }
            }
        }
        pub mod key_value {
            pub mod file_based {
                pub mod v3 {
                    include!("envoy.extensions.key_value.file_based.v3.rs");
                }
            }
        }
        pub mod load_balancing_policies {
            pub mod client_side_weighted_round_robin {
                pub mod v3 {
                    include!("envoy.extensions.load_balancing_policies.client_side_weighted_round_robin.v3.rs");
                }
            }
            pub mod cluster_provided {
                pub mod v3 {
                    include!("envoy.extensions.load_balancing_policies.cluster_provided.v3.rs");
                }
            }
            pub mod common {
                pub mod v3 {
                    include!("envoy.extensions.load_balancing_policies.common.v3.rs");
                }
            }
            pub mod least_request {
                pub mod v3 {
                    include!("envoy.extensions.load_balancing_policies.least_request.v3.rs");
                }
            }
            pub mod maglev {
                pub mod v3 {
                    include!("envoy.extensions.load_balancing_policies.maglev.v3.rs");
                }
            }
            pub mod pick_first {
                pub mod v3 {
                    include!("envoy.extensions.load_balancing_policies.pick_first.v3.rs");
                }
            }
            pub mod random {
                pub mod v3 {
                    include!("envoy.extensions.load_balancing_policies.random.v3.rs");
                }
            }
            pub mod ring_hash {
                pub mod v3 {
                    include!("envoy.extensions.load_balancing_policies.ring_hash.v3.rs");
                }
            }
            pub mod round_robin {
                pub mod v3 {
                    include!("envoy.extensions.load_balancing_policies.round_robin.v3.rs");
                }
            }
            pub mod subset {
                pub mod v3 {
                    include!("envoy.extensions.load_balancing_policies.subset.v3.rs");
                }
            }
            pub mod wrr_locality {
                pub mod v3 {
                    include!("envoy.extensions.load_balancing_policies.wrr_locality.v3.rs");
                }
            }
        }
        pub mod matching {
            pub mod common_inputs {
                pub mod environment_variable {
                    pub mod v3 {
                        include!("envoy.extensions.matching.common_inputs.environment_variable.v3.rs");
                    }
                }
                pub mod network {
                    pub mod v3 {
                        include!("envoy.extensions.matching.common_inputs.network.v3.rs");
                    }
                }
                pub mod ssl {
                    pub mod v3 {
                        include!("envoy.extensions.matching.common_inputs.ssl.v3.rs");
                    }
                }
            }
            pub mod input_matchers {
                pub mod consistent_hashing {
                    pub mod v3 {
                        include!("envoy.extensions.matching.input_matchers.consistent_hashing.v3.rs");
                    }
                }
                pub mod ip {
                    pub mod v3 {
                        include!("envoy.extensions.matching.input_matchers.ip.v3.rs");
                    }
                }
                pub mod runtime_fraction {
                    pub mod v3 {
                        include!("envoy.extensions.matching.input_matchers.runtime_fraction.v3.rs");
                    }
                }
            }
        }
        pub mod network {
            pub mod dns_resolver {
                pub mod apple {
                    pub mod v3 {
                        include!("envoy.extensions.network.dns_resolver.apple.v3.rs");
                    }
                }
                pub mod cares {
                    pub mod v3 {
                        include!("envoy.extensions.network.dns_resolver.cares.v3.rs");
                    }
                }
                pub mod getaddrinfo {
                    pub mod v3 {
                        include!("envoy.extensions.network.dns_resolver.getaddrinfo.v3.rs");
                    }
                }
            }
            pub mod socket_interface {
                pub mod v3 {
                    include!("envoy.extensions.network.socket_interface.v3.rs");
                }
            }
        }
        pub mod outlier_detection_monitors {
            pub mod common {
                pub mod v3 {
                    include!("envoy.extensions.outlier_detection_monitors.common.v3.rs");
                }
            }
            pub mod consecutive_errors {
                pub mod v3 {
                    include!("envoy.extensions.outlier_detection_monitors.consecutive_errors.v3.rs");
                }
            }
        }
        pub mod path {
            pub mod r#match {
                pub mod uri_template {
                    pub mod v3 {
                        include!("envoy.extensions.path.r#match.uri_template.v3.rs");
                    }
                }
            }
            pub mod rewrite {
                pub mod uri_template {
                    pub mod v3 {
                        include!("envoy.extensions.path.rewrite.uri_template.v3.rs");
                    }
                }
            }
        }
        pub mod quic {
            pub mod connection_id_generator {
                pub mod v3 {
                    include!("envoy.extensions.quic.connection_id_generator.v3.rs");
                }
            }
            pub mod crypto_stream {
                pub mod v3 {
                    include!("envoy.extensions.quic.crypto_stream.v3.rs");
                }
            }
            pub mod proof_source {
                pub mod v3 {
                    include!("envoy.extensions.quic.proof_source.v3.rs");
                }
            }
            pub mod server_preferred_address {
                pub mod v3 {
                    include!("envoy.extensions.quic.server_preferred_address.v3.rs");
                }
            }
        }
        pub mod rate_limit_descriptors {
            pub mod expr {
                pub mod v3 {
                    include!("envoy.extensions.rate_limit_descriptors.expr.v3.rs");
                }
            }
        }
        pub mod rbac {
            pub mod audit_loggers {
                pub mod stream {
                    pub mod v3 {
                        include!("envoy.extensions.rbac.audit_loggers.stream.v3.rs");
                    }
                }
            }
            pub mod matchers {
                pub mod upstream_ip_port {
                    pub mod v3 {
                        include!("envoy.extensions.rbac.matchers.upstream_ip_port.v3.rs");
                    }
                }
            }
        }
        pub mod regex_engines {
            pub mod v3 {
                include!("envoy.extensions.regex_engines.v3.rs");
            }
        }
        pub mod request_id {
            pub mod uuid {
                pub mod v3 {
                    include!("envoy.extensions.request_id.uuid.v3.rs");
                }
            }
        }
        pub mod resource_monitors {
            pub mod downstream_connections {
                pub mod v3 {
                    include!("envoy.extensions.resource_monitors.downstream_connections.v3.rs");
                }
            }
            pub mod fixed_heap {
                pub mod v3 {
                    include!("envoy.extensions.resource_monitors.fixed_heap.v3.rs");
                }
            }
            pub mod injected_resource {
                pub mod v3 {
                    include!("envoy.extensions.resource_monitors.injected_resource.v3.rs");
                }
            }
        }
        pub mod retry {
            pub mod host {
                pub mod omit_canary_hosts {
                    pub mod v3 {
                        include!("envoy.extensions.retry.host.omit_canary_hosts.v3.rs");
                    }
                }
                pub mod omit_host_metadata {
                    pub mod v3 {
                        include!("envoy.extensions.retry.host.omit_host_metadata.v3.rs");
                    }
                }
                pub mod previous_hosts {
                    pub mod v3 {
                        include!("envoy.extensions.retry.host.previous_hosts.v3.rs");
                    }
                }
            }
            pub mod priority {
                pub mod previous_priorities {
                    pub mod v3 {
                        include!("envoy.extensions.retry.priority.previous_priorities.v3.rs");
                    }
                }
            }
        }
        pub mod router {
            pub mod cluster_specifiers {
                pub mod lua {
                    pub mod v3 {
                        include!("envoy.extensions.router.cluster_specifiers.lua.v3.rs");
                    }
                }
            }
        }
        pub mod stat_sinks {
            pub mod graphite_statsd {
                pub mod v3 {
                    include!("envoy.extensions.stat_sinks.graphite_statsd.v3.rs");
                }
            }
            pub mod open_telemetry {
                pub mod v3 {
                    include!("envoy.extensions.stat_sinks.open_telemetry.v3.rs");
                }
            }
            pub mod wasm {
                pub mod v3 {
                    include!("envoy.extensions.stat_sinks.wasm.v3.rs");
                }
            }
        }
        pub mod string_matcher {
            pub mod lua {
                pub mod v3 {
                    include!("envoy.extensions.string_matcher.lua.v3.rs");
                }
            }
        }
        pub mod tracers {
            pub mod opentelemetry {
                pub mod resource_detectors {
                    pub mod v3 {
                        include!("envoy.extensions.tracers.opentelemetry.resource_detectors.v3.rs");
                    }
                }
                pub mod samplers {
                    pub mod v3 {
                        include!("envoy.extensions.tracers.opentelemetry.samplers.v3.rs");
                    }
                }
            }
        }
        pub mod transport_sockets {
            pub mod alts {
                pub mod v3 {
                    include!("envoy.extensions.transport_sockets.alts.v3.rs");
                }
            }
            pub mod http_11_proxy {
                pub mod v3 {
                    include!("envoy.extensions.transport_sockets.http_11_proxy.v3.rs");
                }
            }
            pub mod internal_upstream {
                pub mod v3 {
                    include!("envoy.extensions.transport_sockets.internal_upstream.v3.rs");
                }
            }
            pub mod proxy_protocol {
                pub mod v3 {
                    include!("envoy.extensions.transport_sockets.proxy_protocol.v3.rs");
                }
            }
            pub mod quic {
                pub mod v3 {
                    include!("envoy.extensions.transport_sockets.quic.v3.rs");
                }
            }
            pub mod raw_buffer {
                pub mod v3 {
                    include!("envoy.extensions.transport_sockets.raw_buffer.v3.rs");
                }
            }
            pub mod s2a {
                pub mod v3 {
                    include!("envoy.extensions.transport_sockets.s2a.v3.rs");
                }
            }
            pub mod starttls {
                pub mod v3 {
                    include!("envoy.extensions.transport_sockets.starttls.v3.rs");
                }
            }
            pub mod tap {
                pub mod v3 {
                    include!("envoy.extensions.transport_sockets.tap.v3.rs");
                }
            }
            pub mod tcp_stats {
                pub mod v3 {
                    include!("envoy.extensions.transport_sockets.tcp_stats.v3.rs");
                }
            }
            pub mod tls {
                pub mod v3 {
                    include!("envoy.extensions.transport_sockets.tls.v3.rs");
                }
            }
        }
        pub mod udp_packet_writer {
            pub mod v3 {
                include!("envoy.extensions.udp_packet_writer.v3.rs");
            }
        }
        pub mod upstreams {
            pub mod http {
                pub mod generic {
                    pub mod v3 {
                        include!("envoy.extensions.upstreams.http.generic.v3.rs");
                    }
                }
                pub mod http {
                    pub mod v3 {
                        include!("envoy.extensions.upstreams.http.http.v3.rs");
                    }
                }
                pub mod tcp {
                    pub mod v3 {
                        include!("envoy.extensions.upstreams.http.tcp.v3.rs");
                    }
                }
                pub mod udp {
                    pub mod v3 {
                        include!("envoy.extensions.upstreams.http.udp.v3.rs");
                    }
                }
                pub mod v3 {
                    include!("envoy.extensions.upstreams.http.v3.rs");
                }
            }
            pub mod tcp {
                pub mod generic {
                    pub mod v3 {
                        include!("envoy.extensions.upstreams.tcp.generic.v3.rs");
                    }
                }
                pub mod v3 {
                    include!("envoy.extensions.upstreams.tcp.v3.rs");
                }
            }
        }
        pub mod wasm {
            pub mod v3 {
                include!("envoy.extensions.wasm.v3.rs");
            }
        }
        pub mod watchdog {
            pub mod profile_action {
                pub mod v3 {
                    include!("envoy.extensions.watchdog.profile_action.v3.rs");
                }
            }
        }
    }
    pub mod r#type {
        pub mod http {
            pub mod v3 {
                include!("envoy.r#type.http.v3.rs");
            }
        }
        pub mod matcher {
            pub mod v3 {
                include!("envoy.r#type.matcher.v3.rs");
            }
        }
        pub mod metadata {
            pub mod v3 {
                include!("envoy.r#type.metadata.v3.rs");
            }
        }
        pub mod tracing {
            pub mod v3 {
                include!("envoy.r#type.tracing.v3.rs");
            }
        }
        pub mod v3 {
            include!("envoy.r#type.v3.rs");
        }
        include!("envoy.r#type.rs");
    }
    pub mod service {
        pub mod accesslog {
            pub mod v3 {
                include!("envoy.service.accesslog.v3.rs");
            }
        }
        pub mod auth {
            pub mod v3 {
                include!("envoy.service.auth.v3.rs");
            }
        }
        pub mod cluster {
            pub mod v3 {
                include!("envoy.service.cluster.v3.rs");
            }
        }
        pub mod discovery {
            pub mod v3 {
                include!("envoy.service.discovery.v3.rs");
            }
        }
        pub mod endpoint {
            pub mod v3 {
                include!("envoy.service.endpoint.v3.rs");
            }
        }
        pub mod event_reporting {
            pub mod v3 {
                include!("envoy.service.event_reporting.v3.rs");
            }
        }
        pub mod ext_proc {
            pub mod v3 {
                include!("envoy.service.ext_proc.v3.rs");
            }
        }
        pub mod extension {
            pub mod v3 {
                include!("envoy.service.extension.v3.rs");
            }
        }
        pub mod health {
            pub mod v3 {
                include!("envoy.service.health.v3.rs");
            }
        }
        pub mod listener {
            pub mod v3 {
                include!("envoy.service.listener.v3.rs");
            }
        }
        pub mod load_stats {
            pub mod v3 {
                include!("envoy.service.load_stats.v3.rs");
            }
        }
        pub mod metrics {
            pub mod v3 {
                include!("envoy.service.metrics.v3.rs");
            }
        }
        pub mod rate_limit_quota {
            pub mod v3 {
                include!("envoy.service.rate_limit_quota.v3.rs");
            }
        }
        pub mod ratelimit {
            pub mod v3 {
                include!("envoy.service.ratelimit.v3.rs");
            }
        }
        pub mod route {
            pub mod v3 {
                include!("envoy.service.route.v3.rs");
            }
        }
        pub mod runtime {
            pub mod v3 {
                include!("envoy.service.runtime.v3.rs");
            }
        }
        pub mod secret {
            pub mod v3 {
                include!("envoy.service.secret.v3.rs");
            }
        }
        pub mod status {
            pub mod v3 {
                include!("envoy.service.status.v3.rs");
            }
        }
        pub mod tap {
            pub mod v3 {
                include!("envoy.service.tap.v3.rs");
            }
        }
        pub mod trace {
            pub mod v3 {
                include!("envoy.service.trace.v3.rs");
            }
        }
    }
    pub mod watchdog {
        pub mod v3 {
            include!("envoy.watchdog.v3.rs");
        }
    }
}
pub mod google {
    pub mod api {
        pub mod expr {
            pub mod v1alpha1 {
                include!("google.api.expr.v1alpha1.rs");
            }
        }
        include!("google.api.rs");
    }
    pub mod protobuf {
        include!("google.protobuf.rs");
    }
    pub mod rpc {
        include!("google.rpc.rs");
    }
}
pub mod io {
    pub mod prometheus {
        pub mod client {
            include!("io.prometheus.client.rs");
        }
    }
}
pub mod opencensus {
    pub mod proto {
        pub mod resource {
            pub mod v1 {
                include!("opencensus.proto.resource.v1.rs");
            }
        }
        pub mod trace {
            pub mod v1 {
                include!("opencensus.proto.trace.v1.rs");
            }
        }
    }
}
pub mod opentelemetry {
    pub mod proto {
        pub mod common {
            pub mod v1 {
                include!("opentelemetry.proto.common.v1.rs");
            }
        }
    }
}
pub mod udpa {
    pub mod annotations {
        include!("udpa.annotations.rs");
    }
}
pub mod validate {
    include!("validate.rs");
}
pub mod xds {
    pub mod annotations {
        pub mod v3 {
            include!("xds.annotations.v3.rs");
        }
    }
    pub mod core {
        pub mod v3 {
            include!("xds.core.v3.rs");
        }
    }
    pub mod r#type {
        pub mod matcher {
            pub mod v3 {
                include!("xds.r#type.matcher.v3.rs");
            }
        }
    }
}
