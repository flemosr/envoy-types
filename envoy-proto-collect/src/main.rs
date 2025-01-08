use glob::glob;
use std::{
    fs::{self, File},
    io::{self, Write},
    path::{Path, PathBuf},
    process,
};

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

fn collect_protos(
    out_dir: &Path,
    repo: &str,
    glob_patterns: Vec<&str>,
    add_license: Option<&String>,
) {
    let manifest_dir = std::env!("CARGO_MANIFEST_DIR");
    let repo_dir = Path::new(manifest_dir).join("submodules").join(repo);

    if !repo_dir.is_dir() {
        eprintln!("Error: submodule repo {:?} not found.", repo);
        process::exit(1);
    }

    let mut source_protos: Vec<PathBuf> = Vec::new();

    for pattern in glob_patterns {
        let mut matches: Vec<PathBuf> = glob(repo_dir.join(pattern).to_str().unwrap())
            .unwrap()
            .filter_map(Result::ok)
            .collect();

        source_protos.append(&mut matches);
    }

    for proto in source_protos {
        println!("Found proto {:?}", proto);

        let target_name = out_dir.join(
            proto
                .strip_prefix(repo_dir.parent().expect("repo has parent"))
                .expect("base is a prefix of proto"),
        );

        println!("Target {:?}\nCopying...", target_name);

        fs::create_dir_all(target_name.parent().expect("target has parent"))
            .expect("can create parent dir");

        if let Some(license) = add_license.as_ref() {
            let mut target = File::create(&target_name).expect("can create new target");
            let mut source = File::open(&proto).expect("can open the source");
            target
                .write_all(license.as_bytes())
                .expect("can write the license");
            io::copy(&mut source, &mut target).expect("can copy original proto contents");
        } else {
            fs::copy(proto, &target_name).expect("can copy proto to target");
        }

        println!("Proto copied successfully.\n");
    }
}

fn main() {
    let manifest_dir = std::env!("CARGO_MANIFEST_DIR");
    let out_dir = Path::new(manifest_dir)
        .parent()
        .expect("envoy-proto-collect is inside a workspace")
        .join("envoy-types")
        .join("proto");

    if !out_dir.is_dir() {
        eprintln!("Error: out_dir {:?} not found.", out_dir);
        process::exit(1);
    }

    if out_dir.read_dir().expect("read contents").next().is_some() {
        eprintln!(
            "Error: out_dir {:?} is not empty.\nPlease, delete all old protos before collecting new ones.",
            out_dir
        );
        process::exit(1);
    }

    println!("Collecting protos to {:?}...", out_dir);

    let envoy_license = apache_v2(2023, "Envoy Project Authors");
    collect_protos(
        &out_dir,
        "data-plane-api",
        vec![
            "envoy/**/v3/*.proto", // Main protos
            "envoy/annotations/*.proto",
            "envoy/type/http_status.proto", // Optional proto
        ],
        Some(&envoy_license),
    );

    let xds_license = apache_v2(2023, "CNCF xDS API Working Group (xDS-WG) Authors");
    collect_protos(
        &out_dir,
        "xds",
        vec![
            "udpa/annotations/*.proto",
            "xds/core/v3/context_params.proto",
            "xds/core/v3/extension.proto",
            "xds/core/v3/authority.proto",
            "xds/core/v3/collection_entry.proto",
            "xds/core/v3/resource_locator.proto",
            "xds/annotations/v3/status.proto",
            "xds/type/matcher/v3/matcher.proto",
            "xds/type/matcher/v3/string.proto",
            "xds/type/matcher/v3/regex.proto",
        ],
        Some(&xds_license),
    );

    let protoc_gen_validate_license = apache_v2(2023, "Buf Technologies, Inc.");
    collect_protos(
        &out_dir,
        "protoc-gen-validate",
        vec!["**/validate.proto"],
        Some(&protoc_gen_validate_license),
    );

    // Files from `googleapis` already have an Apache-2.0 declaration
    collect_protos(
        &out_dir,
        "googleapis",
        vec![
            "google/api/expr/v1alpha1/checked.proto",
            "google/api/expr/v1alpha1/syntax.proto",
            "google/rpc/status.proto",
            "google/rpc/code.proto", // Optional proto
            "google/api/annotations.proto",
            "google/api/http.proto",
        ],
        None,
    );

    // Files from `opencensus-proto` already have an Apache-2.0 declaration
    collect_protos(
        &out_dir,
        "opencensus-proto",
        vec![
            "**/opencensus/proto/trace/v1/*.proto",
            "**/opencensus/proto/resource/v1/resource.proto",
        ],
        None,
    );

    // Files from `opentelemetry-proto` already have an Apache-2.0 declaration
    collect_protos(
        &out_dir,
        "opentelemetry-proto",
        vec!["**/opentelemetry/proto/common/v1/common.proto"],
        None,
    );

    // Files from `client_model` already have an Apache-2.0 declaration
    collect_protos(
        &out_dir,
        "client_model",
        vec!["io/prometheus/client/metrics.proto"],
        None,
    );
}
