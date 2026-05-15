use std::{
    env,
    error::Error,
    fs::{self, File},
    io::{self, Write},
    path::{Path, PathBuf},
    process::Command,
};

use glob::glob;

const PROTO_DIR: &str = "proto";

fn main() -> Result<(), Box<dyn Error>> {
    let check = env::args().skip(1).any(|arg| arg == "--check");

    let workspace_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("tools is inside a workspace")
        .to_path_buf();
    let tools_dir = workspace_dir.join("tools");
    let envoy_types_dir = workspace_dir.join("envoy-types");

    collect(&tools_dir, &envoy_types_dir)?;
    if check {
        check_protos(&envoy_types_dir)?;
    }

    Ok(())
}

fn collect(tools_dir: &Path, envoy_types_dir: &Path) -> Result<(), Box<dyn Error>> {
    let out_dir = envoy_types_dir.join(PROTO_DIR);

    if out_dir.exists() {
        fs::remove_dir_all(&out_dir)?;
    }
    fs::create_dir_all(&out_dir)?;

    println!("Collecting protos to {:?}...", out_dir);

    let envoy_license = apache_v2(2025, "Envoy Project Authors");
    collect_protos(
        tools_dir,
        &out_dir,
        "data-plane-api",
        &[
            "envoy/**/v3/*.proto", // Main protos
            "envoy/annotations/*.proto",
            "envoy/type/http_status.proto", // Optional proto
        ],
        Some(&envoy_license),
    )?;

    // Files from `cel-spec` already have an Apache-2.0 declaration
    collect_protos(
        tools_dir,
        &out_dir,
        "cel-spec",
        &[
            "proto/cel/expr/checked.proto",
            "proto/cel/expr/syntax.proto",
        ],
        None,
    )?;

    let xds_license = apache_v2(2025, "CNCF xDS API Working Group (xDS-WG) Authors");
    collect_protos(
        tools_dir,
        &out_dir,
        "xds",
        &[
            "udpa/annotations/*.proto",
            "xds/annotations/v3/status.proto",
            "xds/core/v3/authority.proto",
            "xds/core/v3/cidr.proto",
            "xds/core/v3/collection_entry.proto",
            "xds/core/v3/context_params.proto",
            "xds/core/v3/extension.proto",
            "xds/core/v3/resource_locator.proto",
            "xds/type/matcher/v3/matcher.proto",
            "xds/type/matcher/v3/regex.proto",
            "xds/type/matcher/v3/string.proto",
            "xds/type/v3/cel.proto",
        ],
        Some(&xds_license),
    )?;

    let protoc_gen_validate_license = apache_v2(2025, "Buf Technologies, Inc.");
    collect_protos(
        tools_dir,
        &out_dir,
        "protoc-gen-validate",
        &["**/validate.proto"],
        Some(&protoc_gen_validate_license),
    )?;

    // Files from `googleapis` already have an Apache-2.0 declaration
    collect_protos(
        tools_dir,
        &out_dir,
        "googleapis",
        &[
            "google/api/annotations.proto",
            "google/api/expr/v1alpha1/checked.proto",
            "google/api/expr/v1alpha1/syntax.proto",
            "google/api/http.proto",
            "google/rpc/code.proto", // Optional proto
            "google/rpc/status.proto",
        ],
        None,
    )?;

    // Files from `opentelemetry-proto` already have an Apache-2.0 declaration
    collect_protos(
        tools_dir,
        &out_dir,
        "opentelemetry-proto",
        &["**/opentelemetry/proto/common/v1/common.proto"],
        None,
    )?;

    // Files from `client_model` already have an Apache-2.0 declaration
    collect_protos(
        tools_dir,
        &out_dir,
        "client_model",
        &["io/prometheus/client/metrics.proto"],
        None,
    )?;

    Ok(())
}

fn collect_protos(
    tools_dir: &Path,
    out_dir: &Path,
    repo: &str,
    glob_patterns: &[&str],
    add_license: Option<&str>,
) -> Result<(), Box<dyn Error>> {
    let repo_dir = tools_dir.join("submodules").join(repo);

    if !repo_dir.is_dir() {
        return Err(format!("submodule repo `{}` not found at {:?}", repo, repo_dir).into());
    }

    let mut source_protos: Vec<PathBuf> = Vec::new();

    for pattern in glob_patterns {
        let pattern = repo_dir.join(pattern);
        let pattern = pattern
            .to_str()
            .ok_or("proto glob pattern contains invalid UTF-8")?;
        let mut matches: Vec<PathBuf> = glob(pattern)?.filter_map(Result::ok).collect();
        source_protos.append(&mut matches);
    }

    for proto in &source_protos {
        let target_name = out_dir.join(
            proto
                .strip_prefix(repo_dir.parent().expect("repo has parent"))
                .expect("base is a prefix of proto"),
        );

        fs::create_dir_all(target_name.parent().expect("target has parent"))?;

        if let Some(license) = add_license {
            let mut target = File::create(&target_name)?;
            let mut source = File::open(&proto)?;
            target.write_all(license.as_bytes())?;
            io::copy(&mut source, &mut target)?;
        } else {
            fs::copy(proto, target_name)?;
        }
    }

    println!("Collected {} protos from `{repo}`.", source_protos.len());

    Ok(())
}

fn check_protos(envoy_types_dir: &Path) -> Result<(), Box<dyn Error>> {
    let output = Command::new("git")
        .arg("-C")
        .arg(envoy_types_dir)
        .arg("status")
        .arg("--porcelain")
        .arg("--untracked-files=all")
        .arg("--")
        .arg(PROTO_DIR)
        .output()?;

    if !output.status.success() {
        return Err("failed to check proto files with `git status`".into());
    }

    if output.stdout.is_empty() {
        Ok(())
    } else {
        Err(format!("proto files in `{PROTO_DIR}` must be committed").into())
    }
}

/// Generate an Apache-2.0 declaration, in the form of a Protocol Buffer
/// comment, from the copyright `year` and the `name` of the copyright owner.
///
/// Text source: https://www.apache.org/licenses/LICENSE-2.0
fn apache_v2(year: u32, owner: &str) -> String {
    format!(
        "// Copyright {year} {owner}\
        \n//\
        \n// Licensed under the Apache License, Version 2.0 (the \"License\");\
        \n// you may not use this file except in compliance with the License.\
        \n// You may obtain a copy of the License at\
        \n// \
        \n//     http://www.apache.org/licenses/LICENSE-2.0\
        \n// \
        \n// Unless required by applicable law or agreed to in writing, software\
        \n// distributed under the License is distributed on an \"AS IS\" BASIS,\
        \n// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.\
        \n// See the License for the specific language governing permissions and\
        \n// limitations under the License.\n\n"
    )
}
