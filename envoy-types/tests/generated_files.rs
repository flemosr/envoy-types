use std::{path::Path, process::Command};

#[test]
#[ignore = "requires `protoc` and refreshes collected protos and generated files"]
fn generated_protos_are_current() {
    let workspace_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("envoy-types is inside a workspace");

    let proto_collect_status = Command::new("cargo")
        .current_dir(workspace_dir)
        .args(["run", "--bin", "proto-collect", "--", "--check"])
        .status()
        .unwrap();

    assert!(proto_collect_status.success(), "`proto-collect` failed");

    let proto_codegen_status = Command::new("cargo")
        .current_dir(workspace_dir)
        .args(["run", "--bin", "proto-codegen", "--", "--check"])
        .status()
        .unwrap();

    assert!(proto_codegen_status.success(), "`proto-codegen` failed");
}
