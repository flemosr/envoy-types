use glob::glob;
use std::path::PathBuf;

#[test]
fn compile_collected_protos() {
    let out_dir = PathBuf::from("tests/generated");

    let envoy_protos: Vec<PathBuf> = glob("proto/data-plane-api/envoy/**/v3/*.proto")
        .unwrap()
        .filter_map(Result::ok)
        .collect();

    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        .compile_well_known_types(true)
        .out_dir(&out_dir)
        .file_descriptor_set_path(out_dir.join("types.bin"))
        .include_file("mod.rs")
        .compile(
            &envoy_protos,
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
