pub mod v3 {
    /// Re-exporting selected pbs for more convenient imports when implementing
    /// an ExtAuthz server.
    pub mod pb {
        pub use crate::pb::envoy::config::core::v3::{
            address::Address, header_value_option::HeaderAppendAction, HeaderValue,
            HeaderValueOption, QueryParameter,
        };
        pub use crate::pb::envoy::r#type::v3::{HttpStatus, StatusCode as HttpStatusCode};
        pub use crate::pb::envoy::service::auth::v3::{
            authorization_server::{Authorization, AuthorizationServer},
            check_response::HttpResponse,
            CheckRequest, CheckResponse, DeniedHttpResponse, OkHttpResponse,
        };
    }

    use pb::{Address, CheckRequest};
    use std::collections::HashMap;

    pub trait CheckRequestExt: crate::sealed::Sealed {
        fn get_client_address(&self) -> Option<&String>;

        fn get_client_headers(&self) -> Option<&HashMap<String, String>>;
    }

    impl crate::sealed::Sealed for CheckRequest {}

    impl CheckRequestExt for CheckRequest {
        fn get_client_address(&self) -> Option<&String> {
            let peer = self.attributes.as_ref()?.source.as_ref()?;
            let address = peer.address.as_ref()?.address.as_ref()?;
            if let Address::SocketAddress(socket_address) = address {
                return Some(&socket_address.address);
            }
            None
        }

        fn get_client_headers(&self) -> Option<&HashMap<String, String>> {
            let request = self.attributes.as_ref()?.request.as_ref()?;
            Some(&request.http.as_ref()?.headers)
        }
    }
}
