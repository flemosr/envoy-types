# Envoy Types

Collection of protobuf types and other assets to work with the [Envoy Proxy]
through Rust gRPC services.

Among other use cases, this crate can be used to implement an
[Envoy External Authorization] (ExtAuthz) gRPC Server written in Rust.

[![Crates.io Badge](https://img.shields.io/crates/v/envoy-types)](https://crates.io/crates/envoy-types)
[![Documentation Badge](https://docs.rs/envoy-types/badge.svg)](https://docs.rs/envoy-types)
[![License Badge](https://img.shields.io/crates/l/envoy-types)](LICENSE)
![CI Badge](https://github.com/flemosr/envoy-types/actions/workflows/ci.yml/badge.svg)

[Examples] | [Docs]

## Getting Started

### Rust Version

This project's MSRV is `1.75`.

### Dependencies

```toml
[dependencies]
envoy-types = "<envoy-types-version>"
```

The protobuf types made available are already pre-compiled, so you only need the
latest stable Protocol Buffer Compiler (`protoc`) to run the crate's tests.
Generated code may vary across `protoc` versions, and the use of the latest
stable version is enforced by CI.
Installation instructions can be found [here][protoc-install].

## Examples

The example bellow covers a bare-bones implementation of an Envoy ExtAuthz gRPC
`AuthorizationServer`, with [`tonic`]. A more complete implementation,
including query parameters and header manipulation, can be found at the
[examples] directory.

```rust
use std::env;
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
    let server_port = env::var("SERVER_PORT").unwrap_or("50051".into());
    let addr = format!("0.0.0.0:{server_port}").parse().unwrap();
    let server = MyServer;

    println!("AuthorizationServer listening on {addr}");

    Server::builder()
        .add_service(AuthorizationServer::new(server))
        .serve(addr)
        .await?;

    Ok(())
}
```

## Compatibility

The table bellow outlines the correspondence between the versions of [`tonic`]
and the compatible versions of [`envoy-types`].

`tonic` | `envoy-types`
:-      | :-
v0.13   | [v0.6](https://crates.io/crates/envoy-types/0.6.0)
v0.12   | [v0.5](https://crates.io/crates/envoy-types/0.5.6)
v0.11   | [v0.4](https://crates.io/crates/envoy-types/0.4.0)
v0.10   | [v0.3](https://crates.io/crates/envoy-types/0.3.0)
v0.9    | [v0.2](https://crates.io/crates/envoy-types/0.2.0)

## License

This project is licensed under the [Apache License (Version 2.0)](LICENSE).

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion by you, shall be licensed as Apache-2.0, without any additional
terms or conditions.

[Envoy Proxy]: https://www.envoyproxy.io
[Envoy External Authorization]: https://www.envoyproxy.io/docs/envoy/latest/configuration/http/http_filters/ext_authz_filter
[examples]: https://github.com/flemosr/envoy-types/tree/main/examples
[`envoy-types`]: https://crates.io/crates/envoy-types
[Docs]: https://docs.rs/envoy-types/latest/envoy_types
[protoc-install]: https://grpc.io/docs/protoc-installation/
[`tonic`]: https://github.com/hyperium/tonic
