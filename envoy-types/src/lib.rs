mod generated;

pub mod pb {
    pub use crate::generated::*;
}

/// Re-exporting selected pbs for more convenient imports.
pub mod ext_authz {
    pub mod v3 {
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
    }
}
