use std::{
    env,
    error::Error,
    fs,
    io::Write,
    path::{Path, PathBuf},
    process::Command,
};

use glob::glob;

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
    "proto/opentelemetry-proto",
    "proto/opentelemetry-proto",
    "proto/protoc-gen-validate",
    "proto/xds",
];

const GENERATED_DIR: &str = "src/generated";
const GENERATED_TMP_DIR: &str = "src/generated_tmp";

fn main() -> Result<(), Box<dyn Error>> {
    let check = env::args().skip(1).any(|arg| arg == "--check");

    let workspace_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("tools is inside a workspace")
        .to_path_buf();
    let envoy_types_dir = workspace_dir.join("envoy-types");

    regenerate(&envoy_types_dir)?;
    if check {
        check_generated(&envoy_types_dir)?;
    }

    Ok(())
}

fn regenerate(envoy_types_dir: &Path) -> Result<(), Box<dyn Error>> {
    let out_dir = envoy_types_dir.join(GENERATED_TMP_DIR);
    if out_dir.exists() {
        fs::remove_dir_all(&out_dir)?;
    }
    fs::create_dir_all(&out_dir)?;

    let mut protos: Vec<PathBuf> = Vec::new();

    for pattern in GLOB_PATTERNS {
        let pattern = envoy_types_dir.join(pattern);
        let pattern = pattern
            .to_str()
            .ok_or("proto glob pattern contains invalid UTF-8")?;
        let mut matches: Vec<PathBuf> = glob(pattern)?.filter_map(Result::ok).collect();
        protos.append(&mut matches);
    }

    let include_paths: Vec<PathBuf> = INCLUDE_PATHS
        .iter()
        .map(|path| envoy_types_dir.join(path))
        .collect();

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

    let mut file = fs::File::create(&response_file)?;

    for include_path in &include_paths {
        writeln!(file, "--proto_path={}", include_path.display())?;
    }

    writeln!(file, "--descriptor_set_out={}", descriptor_set.display())?;
    writeln!(file, "--include_imports")?;
    writeln!(file, "--include_source_info")?;

    for proto in &protos {
        writeln!(file, "{}", proto.display())?;
    }

    drop(file);

    let status = Command::new("protoc")
        .arg(format!("@{}", response_file.display()))
        .status()?;

    if !status.success() {
        return Err(format!("`protoc` failed with exit code: {:?}", status.code()).into());
    }

    config.skip_protoc_run();

    tonic_prost_build::configure()
        .build_server(true)
        .build_client(true)
        .out_dir(&out_dir)
        .compile_with_config(config, &protos, &include_paths)?;

    let generated_dir = envoy_types_dir.join(GENERATED_DIR);
    if generated_dir.exists() {
        fs::remove_dir_all(&generated_dir)?;
    }
    fs::rename(&out_dir, &generated_dir)?;

    Ok(())
}

fn check_generated(envoy_types_dir: &Path) -> Result<(), Box<dyn Error>> {
    let status = Command::new("git")
        .arg("-C")
        .arg(envoy_types_dir)
        .arg("diff")
        .arg("--exit-code")
        .arg("--")
        .arg(GENERATED_DIR)
        .status()?;

    if status.success() {
        Ok(())
    } else {
        Err(format!("generated files in `{GENERATED_DIR}` must be committed").into())
    }
}
