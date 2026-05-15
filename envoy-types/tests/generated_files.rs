use std::{path::Path, process::Command};

#[test]
#[ignore = "requires protoc and refreshes src/generated"]
fn generated_protos_are_current() {
    let workspace_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("envoy-types is inside a workspace");

    let status = Command::new("cargo")
        .current_dir(workspace_dir)
        .args([
            "run",
            "-p",
            "envoy-types-tools",
            "--bin",
            "proto-codegen",
            "--",
            "--check",
        ])
        .status()
        .unwrap();

    assert!(status.success(), "proto-codegen failed");
}
