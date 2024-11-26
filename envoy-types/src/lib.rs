/*!
Collection of protobuf types and other assets to work with the [Envoy Proxy]
through Rust gRPC services.

Among other use cases, this crate can be used to implement an
[Envoy External Authorization] (ExtAuthz) gRPC Server written in Rust.

# Getting Started

### Rust Version

This project's MSRV is `1.71.1`.

### Dependencies

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
use tonic::{transport::Server, Request, Response, Status};

use envoy_types::ext_authz::v3::pb::{
    Authorization, AuthorizationServer, CheckRequest, CheckResponse,
};
use envoy_types::ext_authz::v3::{CheckRequestExt, CheckResponseExt};

#[derive(Default)]
struct MyServer;

#[tonic::async_trait]
impl Authorization for MyServer {
    async fn check(
        &self,
        request: Request<CheckRequest>,
    ) -> Result<Response<CheckResponse>, Status> {
        let request = request.into_inner();

        let client_headers = request
            .get_client_headers()
            .ok_or_else(|| Status::invalid_argument("client headers not populated by envoy"))?;

        let mut request_status = Status::unauthenticated("not authorized");

        if let Some(authorization) = client_headers.get("authorization") {
            if authorization == "Bearer valid-token" {
                request_status = Status::ok("request is valid");
            }
        }

        Ok(Response::new(CheckResponse::with_status(request_status)))
    }
}

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let addr = format!("0.0.0.0:50051").parse().unwrap();
//     let server = MyServer::default();

//     println!("AuthorizationServer listening on {addr}");

//     Server::builder()
//         .add_service(AuthorizationServer::new(server))
//         .serve(addr)
//         .await?;

//     Ok(())
// }
```

You can check the currently supported version of [`tonic`] at this crate's
[`Cargo.toml`] file. If you want to work with a previous version, consider
using a [previous version of `envoy-types`].

[Envoy Proxy]: https://www.envoyproxy.io
[Envoy External Authorization]: https://www.envoyproxy.io/docs/envoy/latest/configuration/http/http_filters/ext_authz_filter
[protoc-install]: https://grpc.io/docs/protoc-installation/
[`tonic`]: https://github.com/hyperium/tonic
[examples]: https://github.com/flemosr/envoy-types/tree/main/examples
[`Cargo.toml`]: https://github.com/flemosr/envoy-types/blob/main/envoy-types/Cargo.toml
[previous version of `envoy-types`]: https://crates.io/crates/envoy-types/versions
*/

#![warn(missing_debug_implementations, rust_2018_idioms)]
#![allow(missing_docs, rustdoc::invalid_html_tags, rustdoc::bare_urls)]

mod generated;

/// Compiled protobuf types
pub mod pb {
    pub use crate::generated::*;
}

/// Convenience mod for `ext_authz` server implementation
pub mod ext_authz;

mod sealed {
    pub trait Sealed {}
}
