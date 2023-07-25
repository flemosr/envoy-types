use glob::glob;
use std::fs;
use std::path::PathBuf;

fn collect_protos(repo: &str, glob_pattern: &str) {
    let out_dir = PathBuf::from("proto");

    let repo = PathBuf::from("submodules").join(repo);
    let source_protos: Vec<PathBuf> = glob(repo.join(glob_pattern).to_str().unwrap())
        .unwrap()
        .filter_map(Result::ok)
        .collect();

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
    collect_protos("data-plane-api", "envoy/**/v3/*.proto");
    collect_protos("data-plane-api", "envoy/annotations/*.proto");
    collect_protos("xds", "udpa/annotations/*.proto");
    collect_protos("xds", "xds/core/v3/**/*.proto");
    collect_protos("xds", "xds/annotations/v3/**/*.proto");
    collect_protos("xds", "xds/type/matcher/v3/**/*.proto");
    collect_protos("protoc-gen-validate", "**/validate.proto");
    collect_protos("googleapis", "google/api/expr/v1alpha1/*.proto");
    collect_protos("opencensus-proto", "**/opencensus/proto/trace/v1/*.proto");
    collect_protos("opentelemetry-proto", "**/common/v1/*.proto");
    collect_protos("googleapis", "google/rpc/*.proto");
    collect_protos("googleapis", "google/api/*.proto");
    collect_protos("client_model", "io/prometheus/client/*.proto");
    collect_protos(
        "opencensus-proto",
        "**/opencensus/proto/resource/v1/*.proto",
    );
}
