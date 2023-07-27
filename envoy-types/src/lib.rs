/*!
Collection of protobuf types to work with the [Envoy Proxy] through Rust
services.

Among other use cases, this crate can be used to implement an
[Envoy External Authorization] (ExtAuthz) gRPC Server written in Rust.

# Getting Started

```toml
[dependencies]
envoy-types = "<envoy-types-version>"
```

The protobuf types made available are already pre-compiled, so you only need to
have the Protocol Buffer Compiler (`protoc`) installed to run the crate's tests.
Installation instructions can be found [here][protoc-install].

# Examples

The example bellow covers a bare-bones implementation of an Envoy ExtAuthz gRPC
`AuthorizationServer`, with [`tonic`]. A more complete implementation, including
query parameters and header manipulation, can be found at the [examples]
directory.

```rust
use std::collections::HashMap;
use tonic::{transport::Server, Request, Response, Status};

use envoy_types::ext_authz::v3::pb::{
    Address, Authorization, AuthorizationServer, CheckRequest, CheckResponse,
};
use envoy_types::pb::google::rpc;

#[derive(Default)]
struct MyServer;

fn get_external_request_data(
    request: Request<CheckRequest>,
) -> Option<(String, HashMap<String, String>)> {
    let attributes = request.into_inner().attributes?;
    let client_address = match attributes.source?.address?.address? {
        Address::SocketAddress(socket_address) => socket_address.address,
        _ => return None,
    };
    let client_headers = attributes.request?.http?.headers;
    Some((client_address, client_headers))
}

#[tonic::async_trait]
impl Authorization for MyServer {
    async fn check(
        &self,
        request: Request<CheckRequest>,
    ) -> Result<Response<CheckResponse>, Status> {
        // Request is unauthenticated by default
        let mut response_status = rpc::Status::default();
        response_status.code = rpc::Code::Unauthenticated.into();

        if let Some((client_address, client_headers)) = get_external_request_data(request) {
            // Validate `client_address` and/or `client_headers`
            // ...

            if let Some(authorization) = client_headers.get("authorization") {
                if authorization == "Bearer valid-token" {
                    // Mark request as authenticated
                    response_status.code = rpc::Code::Ok.into();
                }
            }
        }

        let mut response = CheckResponse::default();
        response.status = Some(response_status);

        Ok(Response::new(response))
    }
}

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let addr = format!("0.0.0.0:50051").parse().unwrap();
//     let server = MyServer::default();

//     println!("AuthorizationServer listening on 50051");

//     Server::builder()
//         .add_service(AuthorizationServer::new(server))
//         .serve(addr)
//         .await?;

//     Ok(())
// }
```

[Envoy Proxy]: https://www.envoyproxy.io
[Envoy External Authorization]: https://www.envoyproxy.io/docs/envoy/latest/configuration/http/http_filters/ext_authz_filter
[protoc-install]: https://grpc.io/docs/protoc-installation/
[`tonic`]: https://github.com/hyperium/tonic
[examples]: https://github.com/flemosr/envoy-types/tree/main/examples
*/

#![warn(missing_debug_implementations, rust_2018_idioms)]
#![allow(missing_docs, rustdoc::invalid_html_tags, rustdoc::bare_urls)]

mod generated;

/// Compiled protobuf types
pub mod pb {
    pub use crate::generated::*;
}

/// Re-exporting selected pbs for more convenient imports when implementing
/// an ExtAuthz server.
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
