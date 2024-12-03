# Examples

## Envoy's ExtAuthz gRPC Authorization Server

There are three examples showing how to implement an
[Envoy External Authorization] gRPC Server written in Rust, with [`tonic`].

[Envoy External Authorization]: https://www.envoyproxy.io/docs/envoy/latest/configuration/http/http_filters/ext_authz_filter

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
container, check the [envoy-extauthz-rust] and [envoy-extauthz-rust-rate-limit]
repositories.

[envoy-extauthz-rust]: https://github.com/flemosr/envoy-extauthz-rust
[envoy-extauthz-rust-rate-limit]: https://github.com/flemosr/envoy-extauthz-rust-rate-limit
[`tonic`]: https://github.com/hyperium/tonic