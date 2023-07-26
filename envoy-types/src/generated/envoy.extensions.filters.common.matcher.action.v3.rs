/// Configuration for the SkipFilter match action. When matching results in this action, the
/// associated filter will be ignored for all filter callbacks (e.g. `encodeHeaders`, `encodeData`,
/// etc. for HTTP filters) after the matcher arrives at the match, including the callback that
/// caused the match result. For example, when used with a HTTP filter and the match result was
/// resolved after receiving the HTTP response headers, the HTTP filter will *not* receive the
/// response header callback.
///
/// As a result, if this match action is resolved before the first filter callback (e.g. HTTP request
/// headers), the filter will be completely skipped.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SkipFilter {}
