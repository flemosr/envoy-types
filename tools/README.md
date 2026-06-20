# envoy-types tools

This crate contains maintainer tools for the repo.

## Run the `proto-collect` tool

The `proto-collect` binary collects selected proto files from the repos contained in the
`submodules` directory, adding those files to the repo's `envoy-types/proto` directory.

Besides that, `Apache-2.0` declarations are added to the top of the proto files lacking them.

The `submodules` directory is kept as git submodules and is empty after a fresh clone. It must
be populated first from the repo root:

```bash
$ git submodule update --init --recursive tools/submodules
```

Then the tool can be run. It refreshes `envoy-types/proto` from a clean directory:

```bash
$ cargo run --bin proto-collect
```

To also check that the refreshed proto files match the committed files, run:

```bash
$ cargo run --bin proto-collect -- --check
```

## Run the `proto-codegen` tool

The compilation of the protos can be achieved by running this from the repo root:

```bash
$ cargo run --bin proto-codegen
```

To also check that the generated files match the committed files, run:

```bash
$ cargo run --bin proto-codegen -- --check
```
