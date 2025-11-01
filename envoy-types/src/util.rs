use prost::{Message, Name};

use crate::pb::google::protobuf::Any;

/// Packs a protobuf message into a [`google.protobuf.Any`]
///
/// # Examples
///
/// ```
/// use prost::{Message, Name};
/// use envoy_types::pb::google::protobuf::Any;
/// use envoy_types::pb::envoy::extensions::transport_sockets::tls::v3::UpstreamTlsContext;
/// use envoy_types::util::pack_any;
///
/// let tls_context = UpstreamTlsContext {
///     sni: "www.envoyproxy.io".to_string(),
///     ..Default::default()
/// };
///
/// let packed = pack_any(tls_context.clone());
///
/// assert_eq!(packed, Any {
///     type_url: "type.googleapis.com/envoy.extensions.transport_sockets.tls.v3.UpstreamTlsContext".to_string(),
///     value: tls_context.encode_to_vec(),
/// });
/// ```
///
pub fn pack_any<T: Name + Message>(value: T) -> Any {
    Any {
        type_url: T::type_url(),
        value: value.encode_to_vec(),
    }
}
