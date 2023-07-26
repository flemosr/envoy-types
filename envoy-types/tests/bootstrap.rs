use glob::glob;
use std::path::PathBuf;

#[test]
fn bootstrap() {
    let out_dir = PathBuf::from("src/generated");

    let glob_patterns = vec![
        "proto/data-plane-api/envoy/**/v3/*.proto",
        // Add protos not included in the envoy proto's import tree (optional)
        "proto/data-plane-api/envoy/type/http_status.proto",
        "proto/googleapis/google/rpc/code.proto",
    ];
    let mut protos: Vec<PathBuf> = Vec::new();

    for pattern in glob_patterns {
        let mut matches: Vec<PathBuf> = glob(pattern).unwrap().filter_map(Result::ok).collect();
        protos.append(&mut matches);
    }

    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        .compile_well_known_types(true)
        .out_dir(&out_dir)
        .file_descriptor_set_path(out_dir.join("types.bin"))
        .include_file("mod.rs")
        .compile(
            &protos,
            &[
                "proto/data-plane-api",
                "proto/xds",
                "proto/protoc-gen-validate",
                "proto/googleapis",
                "proto/opencensus-proto/src",
                "proto/opentelemetry-proto",
                "proto/opentelemetry-proto",
                "proto/client_model",
            ],
        )
        .unwrap();
}
