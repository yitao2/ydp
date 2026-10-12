//! 统一 JSON 管线、严格语义、确定性和输出事务的回归测试。
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::{fs, path::Path};
use ydp_gen::{GenerateOptions, check_output, compile, load_manifest, write_output};

fn fixture(value: Value) -> tempfile::TempDir {
    let dir = tempfile::tempdir().expect("临时目录");
    let bytes = serde_json::to_vec(&value).expect("测试 JSON");
    fs::write(dir.path().join("schema.json"), &bytes).expect("源文件");
    fs::write(dir.path().join("protocol.toml"),format!("format=1\ngenerated_crate=\"fixture\"\n[[inputs]]\npath=\"schema.json\"\nsha256=\"{:x}\"\n",Sha256::digest(&bytes))).expect("manifest");
    dir
}
fn basic() -> Value {
    json!({"version":{"major":"1","minor":"0"},"domains":[{"domain":"Echo","commands":[{"name":"echo","parameters":[{"name":"text","type":"string"}],"returns":[{"name":"text","type":"string"}]}]}]})
}
fn build(dir: &Path) -> ydp_gen::Result<ydp_gen::GeneratedOutput> {
    compile(
        &load_manifest(&dir.join("protocol.toml"))?,
        &GenerateOptions::default(),
    )
}

#[test]
fn independently_generated_echo_is_deterministic_across_roots() {
    let root = fixture(basic());
    let a = build(root.path()).expect("自包含 Echo 编译");
    let b = build(root.path()).expect("独立第二次编译");
    assert_eq!(a.files, b.files);
    let mirror = tempfile::tempdir().expect("镜像根");
    for name in ["schema.json", "protocol.toml"] {
        fs::copy(root.path().join(name), mirror.path().join(name)).expect("复制输入");
    }
    assert_eq!(a.files, build(mirror.path()).expect("不同根").files);
    let generated = root.path().join("generated");
    write_output(&a, &generated).expect("发布自包含产物");
    assert!(
        check_output(&a, &generated)
            .expect("只读检查")
            .changed
            .is_empty()
    );
}
#[test]
fn check_is_read_only_and_reports_extra_files() {
    let dir = fixture(basic());
    let output = build(dir.path()).expect("生成");
    let dest = dir.path().join("generated");
    assert!(
        !check_output(&output, &dest)
            .expect("缺目录")
            .changed
            .is_empty()
    );
    assert!(!dest.exists());
    write_output(&output, &dest).expect("发布");
    fs::write(dest.join("manual.txt"), "人工内容").expect("额外文件");
    assert_eq!(
        check_output(&output, &dest).expect("检查").changed,
        vec!["manual.txt"]
    );
    assert_eq!(
        write_output(&output, &dest)
            .expect_err("不能删除人工文件")
            .code,
        "Output.ExtraFile"
    );
    assert_eq!(
        fs::read_to_string(dest.join("manual.txt")).expect("保留"),
        "人工内容"
    );
}
#[test]
fn publication_replaces_complete_output_and_lock_rejects_concurrency() {
    let dir = fixture(basic());
    let mut output = build(dir.path()).expect("生成");
    let dest = dir.path().join("generated");
    write_output(&output, &dest).expect("首次发布");
    output.files.insert("provenance.json".into(), "更改".into());
    fs::write(dir.path().join(".generated.lock"), "").expect("模拟并发");
    assert_eq!(
        write_output(&output, &dest).expect_err("锁拒绝").code,
        "Output.Lock"
    );
    fs::remove_file(dir.path().join(".generated.lock")).expect("解锁");
    write_output(&output, &dest).expect("替换");
    assert!(
        check_output(&output, &dest)
            .expect("检查")
            .changed
            .is_empty()
    );
    assert!(!dir.path().join(".generated.backup").exists());
}
#[test]
fn output_cannot_escape_directory() {
    let dir = fixture(basic());
    let mut output = build(dir.path()).expect("生成");
    output.files.insert("../escape".into(), String::new());
    assert_eq!(
        write_output(&output, &dir.path().join("generated"))
            .expect_err("逃逸")
            .code,
        "Output.Path"
    );
}
#[test]
fn input_hash_is_verified_before_parsing() {
    let dir = fixture(basic());
    fs::write(dir.path().join("schema.json"), "{}").expect("改变输入");
    assert_eq!(
        build(dir.path()).expect_err("摘要失败").code,
        "Schema.HashMismatch"
    );
}
#[test]
fn unknown_manifest_key_is_rejected() {
    let dir = fixture(basic());
    let p = dir.path().join("protocol.toml");
    let s = fs::read_to_string(&p).expect("读取");
    fs::write(p, format!("typo=true\n{s}")).expect("写入");
    assert_eq!(
        build(dir.path()).expect_err("未知键").code,
        "Parse.Manifest"
    );
}
#[test]
fn schema_shapes_are_strict() {
    for field in [
        json!({"name":"v"}),
        json!({"name":"v","type":"string","$ref":"X"}),
        json!({"name":"v","type":"array"}),
        json!({"name":"v","type":"integer","enum":["x"]}),
        json!({"name":"v","type":"any","x-cdp-rs":{"nullable":true}}),
        json!({"name":"v","type":"string","oneOf":[]}),
        json!({"name":"v","type":"string","x-cdp-rs":{"nullabe":true}}),
        json!({"name":"v","type":"object","properties":[],"additionalProperties":true}),
    ] {
        let mut v = basic();
        v["domains"][0]["commands"][0]["parameters"] = json!([field]);
        let dir = fixture(v);
        assert!(build(dir.path()).is_err());
    }
}
#[test]
fn rust_name_collisions_and_dangling_refs_fail() {
    let mut v = basic();
    v["domains"][0]["commands"][0]["parameters"] =
        json!([{"name":"fooBar","type":"string"},{"name":"foo_bar","type":"string"}]);
    assert_eq!(
        build(fixture(v).path()).expect_err("命名冲突").code,
        "Naming.FieldCollision"
    );
    let mut v = basic();
    v["domains"][0]["commands"][0]["parameters"] = json!([{"name":"v","$ref":"Missing"}]);
    assert_eq!(
        build(fixture(v).path()).expect_err("悬空引用").code,
        "Reference.Missing"
    );
}
#[test]
fn forward_cross_domain_reference_and_named_identity() {
    let mut v = basic();
    v["domains"][0]["commands"][0]["parameters"] = json!([{"name":"v","$ref":"Other.ID"}]);
    v["domains"]
        .as_array_mut()
        .expect("数组")
        .push(json!({"domain":"Other","types":[{"id":"ID","type":"string"}]}));
    let o = build(fixture(v).path()).expect("跨域引用");
    assert!(o.files["echo/commands.rs"].contains("crate::other::types::Id"));
    assert!(o.files["other/types.rs"].contains("pub struct Id(String)"));
}
#[test]
fn same_domain_fragments_merge_and_conflicts_fail() {
    let mut v = basic();
    let same = v["domains"][0].clone();
    v["domains"].as_array_mut().expect("数组").push(same);
    assert!(build(fixture(v.clone()).path()).is_ok());
    v["domains"][1]["commands"][0]["returns"] = json!([]);
    assert_eq!(
        build(fixture(v).path()).expect_err("冲突").code,
        "Merge.SymbolConflict"
    );
}
#[test]
fn optional_recursive_layout_boxes_direct_edges() {
    let mut v = basic();
    v["domains"][0]["types"] = json!([{"id":"Node","type":"object","properties":[{"name":"next","$ref":"Node","optional":true}]}]);
    let output = build(fixture(v).path()).expect("optional 递归可构造");
    assert!(output.files["echo/types.rs"].contains("Option<Box<crate::echo::types::Node>>"));
}
#[test]
fn original_descriptions_are_release_ready_and_optional_translation_is_hash_bound() {
    let mut v = basic();
    v["domains"][0]["commands"][0]["description"] = json!("Echo text.");
    let dir = fixture(v);
    let original = build(dir.path()).expect("原文正式生成");
    assert!(original.release_ready);
    assert!(original.files["echo/commands.rs"].contains("<pre>Echo text.</pre>"));
    assert!(original.files["echo/mod.rs"].contains("<pre>Echo text.</pre>"));
    let bundle = load_manifest(&dir.path().join("protocol.toml")).expect("读取");
    let legacy = compile(
        &bundle,
        &GenerateOptions {
            allow_incomplete_docs: true,
        },
    )
    .expect("兼容旧选项");
    assert_eq!(original.files, legacy.files);
    assert_eq!(original.diagnostics, legacy.diagnostics);
    assert!(legacy.release_ready);
    let out = dir.path().join("out");
    write_output(&original, &out).expect("写入");
    assert!(
        check_output(&original, &out)
            .expect("正式检查")
            .changed
            .is_empty()
    );
    let p = dir.path().join("protocol.toml");
    let old = fs::read_to_string(&p).expect("读取");
    fs::write(&p, format!("docs_zh=\"docs.json\"\n{old}")).expect("配置");
    let catalog = dir.path().join("docs.json");
    fs::write(&catalog,serde_json::to_vec(&json!({"Echo.command.echo":{"source_sha256":format!("{:x}",Sha256::digest(b"Echo text.")),"text":"原样回送文本。"}})).expect("编码")).expect("说明");
    let translated = build(dir.path()).expect("可选说明");
    assert!(translated.files["echo/commands.rs"].contains("<pre>Echo text.</pre>"));
    assert!(translated.files["echo/commands.rs"].contains("原样回送文本。"));
    assert!(translated.files["echo/mod.rs"].contains("<pre>Echo text.</pre>"));
    assert!(translated.files["echo/mod.rs"].contains("原样回送文本。"));
    fs::write(
        &catalog,
        serde_json::to_vec(
            &json!({"Echo.command.echo":{"source_sha256":"wrong","text":"旧译文。"}}),
        )
        .expect("编码"),
    )
    .expect("说明");
    assert_eq!(
        build(dir.path()).expect_err("拒绝过时补充").code,
        "Documentation.HashMismatch"
    );
}

#[test]
fn description_html_and_markdown_are_data_not_executable_docs() {
    let mut v = basic();
    v["domains"][0]["commands"][0]["description"] = json!(
        "</pre><script>alert(1)</script>\n```rust\ncompile_error!(\"injected\");\n```\n& [Broken]\r\nend"
    );
    let output = build(fixture(v).path()).expect("安全原文");
    let source = &output.files["echo/commands.rs"];
    assert!(source.contains("<pre>&lt;/pre&gt;&lt;script&gt;alert(1)&lt;/script&gt;"));
    assert!(source.contains("&amp; [Broken]"));
    assert!(!source.contains("<script>"));
    assert!(!source.contains("\\r"));
}

#[cfg(unix)]
#[test]
fn schema_symlink_escape_is_rejected() {
    use std::os::unix::fs::symlink;
    let dir = fixture(basic());
    let elsewhere = tempfile::tempdir().expect("外部");
    fs::copy(
        dir.path().join("schema.json"),
        elsewhere.path().join("outside.json"),
    )
    .expect("复制");
    fs::remove_file(dir.path().join("schema.json")).expect("移除");
    symlink(
        elsewhere.path().join("outside.json"),
        dir.path().join("schema.json"),
    )
    .expect("链接");
    assert_eq!(
        build(dir.path()).expect_err("逃逸").code,
        "Schema.RootEscape"
    );
}

#[test]
fn optional_on_array_items_is_rejected_instead_of_ignored() {
    let mut v = basic();
    v["domains"][0]["commands"][0]["parameters"] =
        json!([{"name":"values","type":"array","items":{"type":"string","optional":true}}]);
    assert_eq!(
        build(fixture(v).path())
            .expect_err("items 没有字段 presence")
            .code,
        "Schema.UnknownKey"
    );
}

#[test]
fn raw_keyword_module_uses_unprefixed_file_path() {
    let mut v = basic();
    v["domains"][0]["domain"] = json!("type");
    let output = build(fixture(v).path()).expect("关键字模块");
    assert!(output.files.contains_key("type/mod.rs"));
    assert!(!output.files.contains_key("r#type/mod.rs"));
    assert!(output.files["mod.rs"].contains("pub mod r#type"));
}

#[test]
fn output_alias_paths_are_rejected() {
    let dir = fixture(basic());
    for path in ["a//b", "a/./b", "a/b/"] {
        let mut output = build(dir.path()).expect("生成");
        output.files.insert(path.into(), String::new());
        assert_eq!(
            write_output(&output, &dir.path().join("generated"))
                .expect_err("非规范路径")
                .code,
            "Output.Path"
        );
    }
}

#[test]
fn named_object_description_uses_symbol_path() {
    let mut value = basic();
    value["domains"][0]["types"] =
        json!([{"id":"Item","type":"object","description":"Item.","properties":[]}]);
    let dir = fixture(value);
    let manifest = dir.path().join("protocol.toml");
    let original = fs::read_to_string(&manifest).expect("配置");
    fs::write(manifest, format!("docs_zh=\"docs.json\"\n{original}")).expect("配置");
    fs::write(dir.path().join("docs.json"), serde_json::to_vec(&json!({"Echo.type.Item":{"source_sha256":format!("{:x}",Sha256::digest(b"Item.")),"text":"命名条目。"}})).expect("编码")).expect("说明");
    assert!(build(dir.path()).is_ok());
}

#[test]
fn removed_generated_files_are_deleted_only_when_unmodified() {
    let dir = fixture(basic());
    let original = build(dir.path()).expect("生成");
    let destination = dir.path().join("generated");
    write_output(&original, &destination).expect("发布");
    let mut reduced = original.clone();
    reduced.files.remove("echo/events.rs");
    fs::write(destination.join("echo/events.rs"), "人工修改").expect("修改");
    assert_eq!(
        write_output(&reduced, &destination)
            .expect_err("不可删除人工修改")
            .code,
        "Output.ExtraFile"
    );
    fs::write(
        destination.join("echo/events.rs"),
        &original.files["echo/events.rs"],
    )
    .expect("恢复生成字节");
    write_output(&reduced, &destination).expect("合法删除旧生成文件");
    assert!(!destination.join("echo/events.rs").exists());
}

#[test]
fn vector_recursion_has_no_box_and_required_cycle_is_rejected() {
    let mut v = basic();
    v["domains"][0]["types"] = json!([{"id":"Node","type":"object","properties":[{"name":"children","type":"array","items":{"$ref":"Node"}}]}]);
    let output = build(fixture(v.clone()).path()).expect("Vec 有有限空值");
    assert!(output.files["echo/types.rs"].contains("Vec<crate::echo::types::Node>"));
    assert!(!output.files["echo/types.rs"].contains("Box<"));
    v["domains"][0]["types"][0]["properties"] = json!([{"name":"next","$ref":"Node"}]);
    assert_eq!(
        build(fixture(v).path())
            .expect_err("required 自环无有限值")
            .code,
        "Layout.NoFiniteValue"
    );
}
#[test]
fn alias_cycle_and_deep_chains_are_bounded() {
    let mut v = basic();
    v["domains"][0]["types"] = json!([{"id":"A","$ref":"B"},{"id":"B","$ref":"A"}]);
    assert_eq!(
        build(fixture(v).path()).expect_err("别名空环").code,
        "Layout.NoFiniteValue"
    );
    let mut v = basic();
    let mut types = Vec::new();
    for i in 0..129 {
        types.push(if i == 128 {
            json!({"id":format!("T{i}"),"type":"string"})
        } else {
            json!({"id":format!("T{i}"),"$ref":format!("T{}",i+1)})
        });
    }
    v["domains"][0]["types"] = json!(types);
    assert_eq!(
        build(fixture(v).path()).expect_err("深链").code,
        "Layout.ResourceLimit"
    );
}
fn pdl_fixture(text: &str) -> tempfile::TempDir {
    let dir = tempfile::tempdir().expect("目录");
    fs::write(dir.path().join("entry.pdl"), text).expect("PDL");
    fs::write(dir.path().join("protocol.toml"),format!("format=1\ngenerated_crate=\"fixture\"\n[[inputs]]\npath=\"entry.pdl\"\nsha256=\"{:x}\"\n",Sha256::digest(text.as_bytes()))).expect("manifest");
    dir
}
#[test]
fn pdl_include_reuse_cycle_and_escape_are_distinct() {
    let dir = pdl_fixture("version\n  major 1\n  minor 0\ninclude a.pdl\ninclude b.pdl\n");
    fs::write(dir.path().join("a.pdl"), "include common.pdl\n").expect("a");
    fs::write(dir.path().join("b.pdl"), "include common.pdl\n").expect("b");
    fs::write(
        dir.path().join("common.pdl"),
        "domain Echo\n  command echo\n",
    )
    .expect("共享");
    assert!(build(dir.path()).is_ok());
    fs::write(dir.path().join("common.pdl"), "include a.pdl\n").expect("造环");
    assert_eq!(
        build(dir.path()).expect_err("真环").code,
        "Parse.IncludeCycle"
    );
    let outside = tempfile::tempdir().expect("外部");
    let external = outside.path().join("external.pdl");
    fs::write(&external, "domain Outside\n").expect("外部PDL");
    fs::write(
        dir.path().join("common.pdl"),
        format!("include {}\n", external.display()),
    )
    .expect("逃逸");
    assert_eq!(
        build(dir.path()).expect_err("绝对路径").code,
        "Schema.RootEscape"
    );
}
#[test]
fn pdl_unknown_grammar_and_duplicate_sections_fail() {
    for line in [
        "  explode wow\n",
        "  command x\n    parameters\n    parameters\n",
        "  type T extends string\n    arbitrary\n",
    ] {
        let dir = pdl_fixture(&format!(
            "version\n  major 1\n  minor 0\ndomain Echo\n{line}"
        ));
        assert_eq!(build(dir.path()).expect_err("未知语法").code, "Parse.PDL");
    }
}
#[test]
fn overlay_hash_replace_remove_and_dangling_reference() {
    let old = fixture(basic());
    let mut changed = basic();
    changed["domains"][0]["commands"][0]["returns"] =
        json!([{"name":"text","type":"string"},{"name":"note","type":"string","optional":true}]);
    let new = fixture(changed.clone());
    let diff = ydp_gen::diff(
        &load_manifest(&old.path().join("protocol.toml")).expect("旧"),
        &load_manifest(&new.path().join("protocol.toml")).expect("新"),
    )
    .expect("差异");
    assert_eq!(diff.changes.len(), 1);
    assert!(diff.changes[0].wire_changed && diff.changes[0].rust_changed);
    let hash = diff.changes[0].old_sha256.as_ref().expect("旧hash");
    let p = old.path().join("protocol.toml");
    let original = fs::read_to_string(&p).expect("配置");
    fs::write(
        old.path().join("replacement.json"),
        serde_json::to_vec(&changed["domains"][0]["commands"][0]).expect("编码"),
    )
    .expect("替换定义");
    fs::write(&p,format!("{original}\n[[overrides]]\ntarget=\"Echo.command.echo\"\nexpected_sha256=\"{hash}\"\naction=\"replace\"\ndefinition=\"replacement.json\"\n")).expect("覆盖");
    assert!(
        build(old.path()).expect("真实覆盖").files["echo/commands.rs"]
            .contains("pub note: Option<String>")
    );
    fs::write(&p,format!("{original}\n[[overrides]]\ntarget=\"Echo.command.echo\"\nexpected_sha256=\"{}\"\naction=\"remove\"\n","0".repeat(64))).expect("错误前置条件");
    assert_eq!(
        build(old.path()).expect_err("hash失配").code,
        "Merge.OverrideHashMismatch"
    );
    fs::write(&p,format!("{original}\n[[overrides]]\ntarget=\"Echo.command.echo\"\nexpected_sha256=\"{hash}\"\naction=\"remove\"\n")).expect("删除");
    assert!(
        build(old.path()).expect("删除命令").files["echo/commands.rs"]
            .trim()
            .is_empty()
    );
}

#[test]
fn include_depth_is_bounded() {
    let dir = pdl_fixture("version\n  major 1\n  minor 0\ninclude p1.pdl\n");
    for depth in 1..=65 {
        fs::write(
            dir.path().join(format!("p{depth}.pdl")),
            if depth == 65 {
                "domain Last\n".into()
            } else {
                format!("include p{}.pdl\n", depth + 1)
            },
        )
        .expect("深链");
    }
    assert_eq!(
        build(dir.path()).expect_err("include 深度上限").code,
        "Schema.ResourceLimit"
    );
}
#[test]
fn override_duplicate_targets_and_deleted_references_fail() {
    let mut v = basic();
    v["domains"][0]["types"] = json!([{"id":"Id","type":"string"}]);
    v["domains"][0]["commands"][0]["parameters"] = json!([{"name":"id","$ref":"Id"}]);
    let old = fixture(v.clone());
    let mut newer = v;
    newer["domains"][0]["types"][0]["type"] = json!("integer");
    let new = fixture(newer);
    let diff = ydp_gen::diff(
        &load_manifest(&old.path().join("protocol.toml")).expect("old"),
        &load_manifest(&new.path().join("protocol.toml")).expect("new"),
    )
    .expect("diff");
    let hash = diff
        .changes
        .iter()
        .find(|c| c.symbol == "Echo.type.Id")
        .and_then(|c| c.old_sha256.as_ref())
        .expect("旧hash");
    let manifest = old.path().join("protocol.toml");
    let base = fs::read_to_string(&manifest).expect("配置");
    let overlay = format!(
        "\n[[overrides]]\ntarget=\"Echo.type.Id\"\nexpected_sha256=\"{hash}\"\naction=\"remove\"\n"
    );
    fs::write(&manifest, format!("{base}{overlay}")).expect("删除");
    assert_eq!(
        build(old.path()).expect_err("引用悬空").code,
        "Reference.Missing"
    );
    fs::write(&manifest, format!("{base}{overlay}{overlay}")).expect("重复");
    assert_eq!(
        build(old.path()).expect_err("重复目标").code,
        "Merge.DuplicateOverride"
    );
}
#[test]
fn source_reordering_is_byte_deterministic_and_rust_only_diff_is_distinct() {
    let dir = fixture(basic());
    let first = fs::read_to_string(dir.path().join("protocol.toml")).expect("manifest");
    let second = json!({"version":{"major":"1","minor":"0"},"domains":[{"domain":"Second","types":[{"id":"Id","type":"string"}]}]});
    let bytes = serde_json::to_vec(&second).expect("JSON");
    fs::write(dir.path().join("second.json"), &bytes).expect("输入");
    let entry = format!(
        "[[inputs]]\npath=\"second.json\"\nsha256=\"{:x}\"\n",
        Sha256::digest(&bytes)
    );
    fs::write(
        dir.path().join("protocol.toml"),
        format!("{first}\n{entry}"),
    )
    .expect("顺序1");
    let a = build(dir.path()).expect("a");
    let split = first.find("[[inputs]]").expect("条目");
    fs::write(
        dir.path().join("protocol.toml"),
        format!("{}{}{}", &first[..split], entry, &first[split..]),
    )
    .expect("顺序2");
    assert_eq!(a.files, build(dir.path()).expect("b").files);
    let old = fixture(basic());
    let new = fixture(basic());
    let path = new.path().join("protocol.toml");
    let text = fs::read_to_string(&path).expect("manifest");
    fs::write(
        path,
        format!("{text}\n[rust_names]\n\"Echo.command.echo\"=\"repeat\"\n"),
    )
    .expect("改名");
    let diff = ydp_gen::diff(
        &load_manifest(&old.path().join("protocol.toml")).expect("旧"),
        &load_manifest(&new.path().join("protocol.toml")).expect("新"),
    )
    .expect("diff");
    assert_eq!(diff.changes.len(), 1);
    assert!(!diff.changes[0].wire_changed && diff.changes[0].rust_changed);
}

#[test]
fn domain_fragments_merge_metadata_and_empty_field_lists_are_equivalent() {
    let mut value = basic();
    value["domains"][0]["dependencies"] = json!(["Base"]);
    let extra = json!({"domain":"Echo","types":[{"id":"ID","type":"string"}]});
    value["domains"].as_array_mut().expect("数组").push(extra);
    assert!(build(fixture(value).path()).is_ok());
    let value = json!({"version":{"major":"1","minor":"0"},"domains":[{"domain":"Echo","commands":[{"name":"ping"},{"name":"ping","parameters":[],"returns":[]}]}]});
    assert!(build(fixture(value).path()).is_ok());
}
