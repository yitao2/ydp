//! 完整官方产物可复现性与正式检查；协议描述使用源快照原文。
use std::{
    collections::BTreeSet,
    path::{Path, PathBuf},
};

fn files(root: &Path) -> BTreeSet<String> {
    let mut paths = BTreeSet::new();
    let mut stack = vec![root.to_owned()];
    while let Some(directory) = stack.pop() {
        for entry in std::fs::read_dir(directory).unwrap() {
            let path = entry.unwrap().path();
            if path.is_dir() {
                stack.push(path);
            } else {
                paths.insert(
                    path.strip_prefix(root)
                        .unwrap()
                        .to_str()
                        .unwrap()
                        .replace('\\', "/"),
                );
            }
        }
    }
    paths
}

#[test]
fn r35_r49_r51_official_output_matches_checked_in_bytes() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let bundle = ydp_gen::load_manifest(&root.join("protocol/official/protocol.toml")).unwrap();
    let output = ydp_gen::compile(&bundle, &Default::default()).unwrap();
    let directory = root.join("crates/ydp/src/generated");
    assert_eq!(
        output.files.keys().cloned().collect::<BTreeSet<_>>(),
        files(&directory)
    );
    for (path, expected) in &output.files {
        assert_eq!(
            std::fs::read(directory.join(path)).unwrap(),
            expected.as_bytes(),
            "{path}"
        );
    }
    assert!(output.release_ready);
    assert!(output.diagnostics.is_empty());
    assert!(
        ydp_gen::check_output(&output, &directory)
            .unwrap()
            .changed
            .is_empty()
    );
    assert!(output.files["page/commands.rs"].contains("Navigates current page to the given URL."));
    assert!(output.files["page/mod.rs"].contains("Navigates current page to the given URL."));
}

#[test]
fn full_official_json_and_pdl_have_same_semantic_projection() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../protocol/official");
    let json = ydp_gen::load_manifest(&root.join("protocol.toml")).expect("完整官方 JSON");
    let pdl =
        ydp_gen::load_manifest(&root.join("protocol-pdl.toml")).expect("完整官方 PDL 与 include");
    let diff = ydp_gen::diff(&json, &pdl).expect("投影");
    assert!(diff.changes.is_empty(), "{:?}", diff.changes);
}
