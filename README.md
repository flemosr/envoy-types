# Envoy Types

Collection of protobuf types and other assets to work with the [Envoy Proxy]
through Rust services.

Among other use cases, this crate can be used to implement an
[Envoy External Authorization] (ExtAuthz) gRPC Server written in Rust.

[![Crates.io](https://img.shields.io/crates/v/envoy-types)](https://crates.io/crates/envoy-types)
[![Documentation](https://docs.rs/envoy-types/badge.svg)](https://docs.rs/envoy-types)
[![Crates.io](https://img.shields.io/crates/l/envoy-types)](LICENSE)

[Examples] | [Docs]

## Getting Started

```toml
[dependencies]
envoy-types = "<envoy-types-version>"
```

The protobuf types made available are already pre-compiled, so you only need to
have the Protocol Buffer Compiler (`protoc`) installed to run the crate's tests.
Installation instructions can be found [here][protoc-install].

## Examples

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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = format!("0.0.0.0:50051").parse().unwrap();
    let server = MyServer::default();

    println!("AuthorizationServer listening on {addr}");

    Server::builder()
        .add_service(AuthorizationServer::new(server))
        .serve(addr)
        .await?;

    Ok(())
}
```

## License

This project is licensed under the [Apache License (Version 2.0)](LICENSE).

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion by you, shall be licensed as Apache-2.0, without any additional
terms or conditions.

[Envoy Proxy]: https://www.envoyproxy.io
[Envoy External Authorization]: https://www.envoyproxy.io/docs/envoy/latest/configuration/http/http_filters/ext_authz_filter
[examples]: https://github.com/flemosr/envoy-types/tree/main/examples
[Docs]: https://docs.rs/envoy-types/latest/envoy-types/
[protoc-install]: https://grpc.io/docs/protoc-installation/
[`tonic`]: https://github.com/hyperium/tonic