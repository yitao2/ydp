//! 生成器公开入口的稳定版确定性随机测试。

use serde_json::json;
use sha2::{Digest, Sha256};
use std::{fs, path::Path};
use ydp_gen::{GenerateOptions, GeneratedOutput, compile, load_manifest};

/// 固定算法与种子的伪随机源，使失败可以仅凭迭代编号重放。
struct Random(u64);

impl Random {
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }

    fn range(&mut self, upper: usize) -> usize {
        (self.next() as usize) % upper
    }
}

fn write_fixture(extension: &str, bytes: &[u8]) -> tempfile::TempDir {
    let directory = tempfile::tempdir().expect("建立随机测试目录");
    let source = format!("schema.{extension}");
    fs::write(directory.path().join(&source), bytes).expect("写入随机输入");
    let manifest = format!(
        "format=1\ngenerated_crate=\"fuzz_fixture\"\n[[inputs]]\npath=\"{source}\"\nsha256=\"{:x}\"\n",
        Sha256::digest(bytes)
    );
    fs::write(directory.path().join("protocol.toml"), manifest).expect("写入随机 manifest");
    directory
}

fn run(path: &Path) -> ydp_gen::Result<GeneratedOutput> {
    let bundle = load_manifest(&path.join("protocol.toml"))?;
    compile(&bundle, &GenerateOptions::default())
}

fn assert_reproducible(path: &Path, iteration: usize) {
    let first = run(path);
    let second = run(path);
    match (first, second) {
        (Ok(first), Ok(second)) => assert_eq!(first.files, second.files, "迭代 {iteration}"),
        (Err(first), Err(second)) => {
            assert_eq!(first.code, second.code, "迭代 {iteration}");
            assert_eq!(first.location, second.location, "迭代 {iteration}");
        }
        _ => panic!("相同输入在迭代 {iteration} 得到不确定结果"),
    }
}

fn random_text(random: &mut Random, max_len: usize) -> String {
    const PIECES: &[&str] = &[
        "a", "Z", "0", "_", "-", ".", "/", " ", "self", "type", "\n", "\r\n", "<pre>", "&",
        "```rust", "中文", "é", "💻", "\0",
    ];
    let mut text = String::new();
    for _ in 0..random.range(max_len + 1) {
        text.push_str(PIECES[random.range(PIECES.len())]);
    }
    text
}

#[test]
fn random_json_and_pdl_inputs_are_bounded_and_reproducible() {
    let mut random = Random(0x43d2_f891_ae07_6c15);
    for iteration in 0..384 {
        let len = random.range(2049);
        let mut bytes = vec![0_u8; len];
        for byte in &mut bytes {
            *byte = random.next() as u8;
        }
        let extension = if iteration % 2 == 0 { "json" } else { "pdl" };
        let fixture = write_fixture(extension, &bytes);
        assert_reproducible(fixture.path(), iteration);
    }
}

#[test]
fn random_layout_names_and_descriptions_are_reproducible() {
    let mut random = Random(0xa713_5e29_4cb8_d06f);
    for iteration in 0..192 {
        let domain = random_text(&mut random, 5);
        let first = random_text(&mut random, 6);
        let second = random_text(&mut random, 6);
        let description = random_text(&mut random, 24);
        let relation = random.range(5);
        let first_type = match relation {
            0 => json!({"id": first, "$ref": second}),
            1 => json!({"id": first, "type":"array", "items":{"$ref":second}}),
            2 => {
                json!({"id": first, "type":"object", "properties":[{"name":"next","$ref":second,"optional":true}]})
            }
            _ => json!({"id": first, "type":"string", "description":description}),
        };
        let second_type = match relation {
            0 => json!({"id": second, "$ref": first}),
            1 => {
                json!({"id": second, "type":"object", "properties":[{"name":"back","$ref":first}]})
            }
            _ => json!({"id": second, "type":"string"}),
        };
        let schema = json!({
            "version":{"major":"1","minor":"0"},
            "domains":[{
                "domain":domain,
                "description":description,
                "types":[first_type, second_type],
                "commands":[{"name":random_text(&mut random, 7),"description":random_text(&mut random, 18)}]
            }]
        });
        let bytes = serde_json::to_vec(&schema).expect("编码随机 schema");
        let fixture = write_fixture("json", &bytes);
        assert_reproducible(fixture.path(), iteration);
        if let Ok(output) = run(fixture.path()) {
            for (name, source) in &output.files {
                if !name.ends_with(".rs") {
                    continue;
                }
                syn::parse_file(source).unwrap_or_else(|error| {
                    panic!(
                        "迭代 {iteration} 的 {name} 不是有效 Rust：{error}\nschema={schema}\nsource={source}"
                    )
                });
            }
        }
    }
}

#[test]
fn structured_description_escaping_never_creates_active_rustdoc() {
    let mut random = Random(0xe420_9bc1_75d3_68af);
    for iteration in 0..128 {
        let marker = format!("FUZZ_MARKER_{iteration}");
        let description = format!(
            "{}\n```rust\ncompile_error!(\"{marker}\");\n```\n</pre><script>&{}",
            random_text(&mut random, 12),
            random_text(&mut random, 12)
        );
        let schema = json!({
            "version":{"major":"1","minor":"0"},
            "domains":[{"domain":"Echo","commands":[{"name":"ping","description":description}]}]
        });
        let bytes = serde_json::to_vec(&schema).expect("编码 description schema");
        let fixture = write_fixture("json", &bytes);
        let output = run(fixture.path()).expect("固定合法 schema 应生成成功");
        let commands = &output.files["echo/commands.rs"];
        assert!(commands.contains("&lt;/pre&gt;&lt;script&gt;&amp;"));
        assert!(commands.contains(&marker));
        syn::parse_file(commands).expect("转义后的 Rust 源码必须可解析");
    }
}
