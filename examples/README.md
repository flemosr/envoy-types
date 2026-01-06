# Examples

## Quick Templates

Direct source code links for quick reference:

| Category | Raw Source |
|----------|------------|
| **ExtAuthz Server Basic** | [authorization_server_basic.rs](https://raw.githubusercontent.com/flemosr/envoy-types/refs/heads/main/examples/src/ext_authz/authorization_server_basic.rs) |
| **ExtAuthz Server Full** | [authorization_server_full.rs](https://raw.githubusercontent.com/flemosr/envoy-types/refs/heads/main/examples/src/ext_authz/authorization_server_full.rs) |
| **ExtAuthz Server PB Only** | [authorization_server_pb_only.rs](https://raw.githubusercontent.com/flemosr/envoy-types/refs/heads/main/examples/src/ext_authz/authorization_server_pb_only.rs) |

## Envoy's ExtAuthz gRPC Authorization Server

There are three examples showing how to implement an
[Envoy External Authorization](https://www.envoyproxy.io/docs/envoy/latest/configuration/http/http_filters/ext_authz_filter)
gRPC Server written in Rust, with [`tonic`](https://github.com/hyperium/tonic).

A bare-bones implementation:

```bash
$ cargo run --example authorization-server-basic
```

An `AuthorizationServer` with HTTP header and query parameter manipulation:

```bash
$ cargo run --example authorization-server-full
```

Same as `authorization-server-full`, but implemented using the protobuf types
directly:

```bash
$ cargo run --example authorization-server-pb-only
```

For more complete examples, including the connection to an actual `envoy`
container, check the [envoy-extauthz-rust](https://github.com/flemosr/envoy-extauthz-rust) and [envoy-extauthz-rust-rate-limit](https://github.com/flemosr/envoy-extauthz-rust-rate-limit)
repositories.
