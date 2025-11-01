use glob::glob;
use std::{fs, io::Write, path::PathBuf, process::Command};

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

    // Use `protoc`'s response file feature to avoid command-line length limits.
    // This is critical on Windows due to its character limit, but the same
    // approach is followed on all platforms for consistency.
    // Here `protoc` is invoked directly with `@response_file` syntax, then
    // `skip_protoc_run()` is used to let `prost-build` process the generated
    // file descriptor set.

    let descriptor_set = out_dir.join("types.bin");
    let response_file = out_dir.join("protoc_args.txt");

    let mut file = fs::File::create(&response_file).unwrap();

    for include_path in &include_paths {
        writeln!(file, "--proto_path={}", include_path.display()).unwrap();
    }

    writeln!(file, "--descriptor_set_out={}", descriptor_set.display()).unwrap();
    writeln!(file, "--include_imports").unwrap();
    writeln!(file, "--include_source_info").unwrap();

    for proto in &protos {
        writeln!(file, "{}", proto.display()).unwrap();
    }

    drop(file);

    let status = Command::new("protoc")
        .arg(format!("@{}", response_file.display()))
        .status()
        .unwrap();

    assert!(
        status.success(),
        "`protoc` failed with exit code: {:?}",
        status.code()
    );

    config.skip_protoc_run();

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
