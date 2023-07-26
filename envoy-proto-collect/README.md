# Envoy Proto Collect

This crate is a tool used to collect selected proto files from the repos
contained in the `submodules` directory, adding said files to the repo's
`envoy-types/proto` directory.

Besides that, `Apache-2.0` declarations are added to the top of the proto files
lacking them.

## Run the Proto Collection Tool

First, delete the contents of the repo's `envoy-types/proto` directory, to
ensure that only the protos that were just collected are present in the
directory by the end of the process.

Then, run the tool:

```bash
$ cargo run --bin collect-protos
```

The compilation of the protos can be achieved by running the `bootstrap` test
of the `envoy-types` crate. This can be done by simply running:

```bash
$ cargo test bootstrap
```

On the repo's root directory. If `git` detects any changes in the resulting
generated files, the test will fail.