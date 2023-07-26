# Examples

## Envoy's ExtAuthz gRPC Authorization Server

This is an example showing how to implement an [Envoy External Authorization]
gRPC Server written in Rust, with [`tonic`].

[Envoy External Authorization]: https://www.envoyproxy.io/docs/envoy/latest/configuration/http/http_filters/ext_authz_filter

To run the `AuthorizationServer`:

```bash
$ cargo run --example authorization-server
```

For a more complete example, including the connection to an actual `envoy`
container, checkout the [envoy-extauthz-rust] repo.

[envoy-extauthz-rust]: https://github.com/flemosr/envoy-extauthz-rust
[`tonic`]: https://github.com/hyperium/tonic