use glob::glob;
use std::fs;
use std::path::PathBuf;

fn collect_protos(repo: &str, glob_patterns: Vec<&str>) {
    let out_dir = PathBuf::from("proto");

    let repo = PathBuf::from("submodules").join(repo);
    let mut source_protos: Vec<PathBuf> = Vec::new();

    for pattern in glob_patterns {
        let mut matches: Vec<PathBuf> = glob(repo.join(pattern).to_str().unwrap())
            .unwrap()
            .filter_map(Result::ok)
            .collect();

        source_protos.append(&mut matches);
    }

    for proto in source_protos {
        println!("Found proto {:?}", proto);

        let target_name = out_dir.join(
            proto
                .strip_prefix(repo.parent().expect("repo has parent"))
                .expect("base is a prefix of proto"),
        );

        println!("Target {:?}\nCopying...", target_name);

        fs::create_dir_all(target_name.parent().expect("target has parent"))
            .expect("can create parent dir");
        fs::copy(proto, &target_name).expect("can copy proto to target");

        println!("Proto copied successfully.\n");
    }
}

fn main() {
    let out_dir = PathBuf::from("proto");
    if !out_dir.is_dir() {
        panic!("out_dir {:?} not found", out_dir);
    }
    fs::remove_dir_all(&out_dir).expect("can remove proto dir");
    fs::create_dir(&out_dir).expect("can create proto dir");

    collect_protos(
        "data-plane-api",
        vec!["envoy/**/v3/*.proto", "envoy/annotations/*.proto"],
    );
    collect_protos(
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
    );
    collect_protos("protoc-gen-validate", vec!["**/validate.proto"]);
    collect_protos(
        "googleapis",
        vec![
            "google/api/expr/v1alpha1/checked.proto",
            "google/api/expr/v1alpha1/syntax.proto",
            "google/rpc/status.proto",
            "google/api/annotations.proto",
            "google/api/http.proto",
        ],
    );
    collect_protos(
        "opencensus-proto",
        vec![
            "**/opencensus/proto/trace/v1/*.proto",
            "**/opencensus/proto/resource/v1/resource.proto",
        ],
    );
    collect_protos(
        "opentelemetry-proto",
        vec!["**/opentelemetry/proto/common/v1/common.proto"],
    );
    collect_protos("client_model", vec!["io/prometheus/client/metrics.proto"]);
}
