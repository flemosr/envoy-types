// This file is @generated by prost-build.
/// Indicates that during forwarding, portions of the path that match the
/// pattern should be rewritten, even allowing the substitution of variables
/// from the match pattern into the new path as specified by the rewrite template.
/// This is useful to allow application paths to be
/// rewritten in a way that is aware of segments with variable content like
/// identifiers. The router filter will place the original path as it was
/// before the rewrite into the :ref:`x-envoy-original-path  <config_http_filters_router_x-envoy-original-path>` header.
///
///
/// Only one of :ref:`prefix_rewrite <envoy_v3_api_field_config.route.v3.RouteAction.prefix_rewrite>`,
/// : ref:`regex_rewrite <envoy_v3_api_field_config.route.v3.RouteAction.regex_rewrite>`,
///   or *path_template_rewrite* may be specified.
///
///
/// Template pattern matching types:
///
/// * `*` : Matches a single path component, up to the next path separator: /
///
/// * `**` : Matches zero or more path segments. If present, must be the last operator.
///
/// * `{name} or {name=*}` :  A named variable matching one path segment up to the next path separator: /.
///
/// * `{name=videos/*}` : A named variable matching more than one path segment.
///   The path component matching videos/\* is captured as the named variable.
///
/// * `{name=**}` : A named variable matching zero or more path segments.
///
/// Only named matches can be used to perform rewrites.
///
/// Examples using path_template_rewrite:
///
/// * The pattern `/{one}/{two}` paired with a substitution string of `/{two}/{one}` would
///   transform `/cat/dog` into `/dog/cat`.
///
/// * The pattern `/videos/{language=lang/*}/*` paired with a substitution string of
///   `/{language}` would transform `/videos/lang/en/video.m4s` into `lang/en`.
///
/// * The path pattern `/content/{format}/{lang}/{id}/{file}.vtt` paired with a substitution
///   string of `/{lang}/{format}/{file}.vtt` would transform `/content/hls/en-us/12345/en_193913.vtt`
///   into `/en-us/hls/en_193913.vtt`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UriTemplateRewriteConfig {
    #[prost(string, tag = "1")]
    pub path_template_rewrite: ::prost::alloc::string::String,
}
