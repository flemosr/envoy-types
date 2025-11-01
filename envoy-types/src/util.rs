use prost::{Message, Name};

use crate::pb::google::protobuf::Any;

/// Packs a protobuf message into a [`google.protobuf.Any`](Any) type.
pub fn pack_any<T: Name + Message>(value: T) -> Any {
    Any {
        type_url: T::type_url(),
        value: value.encode_to_vec(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pb::envoy::extensions::transport_sockets::tls::v3::UpstreamTlsContext;

    #[test]
    fn test_pack_any() {
        let tls_context = UpstreamTlsContext {
            sni: "www.envoyproxy.io".to_string(),
            ..Default::default()
        };

        assert_eq!(pack_any(tls_context.clone()), Any {
            type_url: "type.googleapis.com/envoy.extensions.transport_sockets.tls.v3.UpstreamTlsContext".to_string(),
            value: tls_context.encode_to_vec(),
        });
    }
}
