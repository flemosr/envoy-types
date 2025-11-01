use glob::glob;
use std::{path::PathBuf, process::Command};

const GLOB_PATTERNS: &[&str] = &[
    "proto/data-plane-api/envoy/**/v3/*.proto",
    // Add protos not included in the envoy proto's import tree (optional)
    "proto/data-plane-api/envoy/type/http_status.proto",
    "proto/googleapis/google/rpc/code.proto",
];

const INCLUDE_PATHS: &[&str] = &[
    "proto/cel-spec/proto",
    "proto/client_model",
    "proto/data-plane-api",
    "proto/googleapis",
    "proto/opencensus-proto/src",
    "proto/opentelemetry-proto",
    "proto/opentelemetry-proto",
    "proto/protoc-gen-validate",
    "proto/xds",
];

#[test]
fn bootstrap() {
    let out_dir = PathBuf::from("src/generated");

    let mut protos: Vec<PathBuf> = Vec::new();

    for pattern in GLOB_PATTERNS {
        let mut matches: Vec<PathBuf> = glob(pattern).unwrap().filter_map(Result::ok).collect();
        protos.append(&mut matches);
    }

    let include_paths: Vec<PathBuf> = INCLUDE_PATHS.iter().map(PathBuf::from).collect();

    let mut config = prost_build::Config::new();
    config
        .enable_type_names()
        .type_name_domain(["."], "type.googleapis.com")
        .compile_well_known_types()
        .file_descriptor_set_path(out_dir.join("types.bin"))
        .include_file("mod.rs");

    tonic_prost_build::configure()
        .build_server(true)
        .build_client(true)
        .out_dir(&out_dir)
        .compile_with_config(config, &protos, &include_paths)
        .unwrap();

    let status = Command::new("git")
        .arg("diff")
        .arg("--exit-code")
        .arg("--")
        .arg(&out_dir)
        .status()
        .unwrap();

    assert!(status.success(), "The generated files must be committed.");
}
