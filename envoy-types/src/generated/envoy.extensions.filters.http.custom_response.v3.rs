/// The filter configuration is a collection of custom response
/// policies in a matcher tree. The configuration can be defined at the filter,
/// virtual host or route level. The response will be matched against the most
/// specific to the least specific config, till a match is found.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomResponse {
    /// Matcher to match against the original response to select a
    /// :ref:`Custom Response Policy <extension_category_envoy.http.custom_response>`
    /// that will override the original response. The matching is done by matching
    /// against :ref:`response header values<extension_category_envoy.matching.http.input>`
    /// Example:
    ///
    /// .. validated-code-block:: yaml
    ///    :type-name: xds.type.matcher.v3.Matcher
    ///
    ///    matcher_list:
    ///      matchers:
    ///        # Apply a locally stored custom response to any 4xx response.
    ///      - predicate:
    ///          single_predicate:
    ///            input:
    ///              name: 4xx_response
    ///              typed_config:
    ///                "@type": type.googleapis.com/envoy.type.matcher.v3.HttpResponseStatusCodeClassMatchInput
    ///            value_match:
    ///              exact: "4xx"
    ///        on_match:
    ///          action:
    ///            name: action
    ///            typed_config:
    ///              "@type": type.googleapis.com/envoy.extensions.http.custom_response.local_response_policy.v3.LocalResponsePolicy
    ///              status_code: 499
    ///              body:
    ///                inline_string: "not allowed"
    ///              body_format:
    ///                json_format:
    ///                  status: "%RESPONSE_CODE%"
    ///                  message: "%LOCAL_REPLY_BODY%"
    ///              response_headers_to_add:
    ///              - header:
    ///                  key: "foo"
    ///                  value: "x-bar"
    ///        # Redirect to different upstream if the status code is one of 502, 503 or 504.
    ///      - predicate:
    ///          or_matcher:
    ///            predicate:
    ///            - single_predicate:
    ///                input:
    ///                  name: "502_response"
    ///                  typed_config:
    ///                    "@type": type.googleapis.com/envoy.type.matcher.v3.HttpResponseStatusCodeMatchInput
    ///                value_match:
    ///                  exact: "502"
    ///            - single_predicate:
    ///                input:
    ///                  name: "503_response"
    ///                  typed_config:
    ///                    "@type": type.googleapis.com/envoy.type.matcher.v3.HttpResponseStatusCodeMatchInput
    ///                value_match:
    ///                  exact: "503"
    ///            - single_predicate:
    ///                input:
    ///                  name: "504_response"
    ///                  typed_config:
    ///                    "@type": type.googleapis.com/envoy.type.matcher.v3.HttpResponseStatusCodeMatchInput
    ///                value_match:
    ///                  exact: "504"
    ///        on_match:
    ///          action:
    ///            name: action
    ///            typed_config:
    ///              "@type": type.googleapis.com/envoy.extensions.http.custom_response.redirect_policy.v3.RedirectPolicy
    ///              status_code: 299
    ///              uri: "<https://foo.example/gateway_error">
    ///              response_headers_to_add:
    ///              - header:
    ///                  key: "foo2"
    ///                  value: "x-bar2"
    ///
    /// -- attention::
    ///   The first matched policy wins. Once the response is matched, matcher
    ///   evaluations end.
    ///
    /// Refer to :ref:`Unified Matcher API <envoy_v3_api_msg_.xds.type.matcher.v3.Matcher>`
    /// documentation for more information on the matcher trees.
    /// [#extension-category: envoy.http.custom_response]
    #[prost(message, optional, tag = "1")]
    pub custom_response_matcher: ::core::option::Option<
        super::super::super::super::super::super::xds::r#type::matcher::v3::Matcher,
    >,
}
