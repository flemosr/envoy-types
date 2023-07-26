/// This extension allows the session state to be tracked via request headers.
///
/// This extension encodes the address of the upstream host selected by the load balancer
/// into a response header with the :ref:`header configuration <envoy_v3_api_msg_extensions.http.stateful_session.header.v3.HeaderBasedSessionState>`.
/// When new requests are incoming, this extension will try to parse the specific upstream host
/// address by header name. If the address parsed from the header corresponds to a valid
/// upstream host, this upstream host will be selected first. See :ref:`stateful session filter <envoy_v3_api_msg_extensions.filters.http.stateful_session.v3.StatefulSession>`.
///
/// For example, if the header name is set to `session-header`, envoy will prefer `1.2.3.4:80`
/// as the upstream host when the request contains the following header:
///
/// .. code-block:: none
///
/// ```text
/// session-header: "MS4yLjMuNDo4MA=="
/// ```
///
/// When processing the upstream response, if `1.2.3.4:80` is indeed the final choice the extension
/// does nothing. If `1.2.3.4:80` is not the final choice, the new selected host will be set to
/// response headers (via the `session-header` response header).
///
/// \[\#extension: envoy.http.stateful_session.header\]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HeaderBasedSessionState {
    /// The name that will be used to obtain header value from downstream HTTP request or generate
    /// new header for downstream.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
