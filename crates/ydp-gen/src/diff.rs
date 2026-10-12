//! 固定对象键序的符号摘要与 wire/Rust 两维差异，不执行网络操作。
use crate::{Result, SourceBundle, error, input};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::BTreeSet;
/// 一个完整符号的变更，wire 与 Rust 源码维度分开报告。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProtocolChange {
    /// Domain.kind.Name 符号身份。
    pub symbol: String,
    /// 旧定义规范语义摘要；新增符号为 None。
    pub old_sha256: Option<String>,
    /// 新定义规范语义摘要；删除符号为 None。
    pub new_sha256: Option<String>,
    /// 类型、presence 或协议元数据是否改变；不包含描述翻译。
    pub wire_changed: bool,
    /// 字段构造、类型或名称是否改变；新增 optional 字段也属于源码变更。
    pub rust_changed: bool,
    /// 中文变更说明；不把尚未判断的变更冒称 SemVer 兼容。
    pub description: String,
}
/// 已排序的符号差异，空列表表示两维均一致。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProtocolDiff {
    /// 每个变更符号的分类及旧/新摘要。
    pub changes: Vec<ProtocolChange>,
}
fn normalize(value: &Value) -> Value {
    match value {
        Value::Object(object) => {
            let mut result = serde_json::Map::new();
            for (key, value) in object {
                if key == "description" {
                    continue;
                }
                if ["optional", "experimental", "deprecated"].contains(&key.as_str())
                    && value == &Value::Bool(false)
                {
                    continue;
                }
                let normalized = if key == "type" && value.as_str() == Some("binary") {
                    Value::String("string".into())
                } else {
                    normalize(value)
                };
                if ["parameters", "returns"].contains(&key.as_str())
                    && normalized == Value::Array(Vec::new())
                {
                    continue;
                }
                result.insert(key.clone(), normalized);
            }
            Value::Object(result)
        }
        Value::Array(values) => Value::Array(values.iter().map(normalize).collect()),
        _ => value.clone(),
    }
}
/// 对完整符号做规范语义摘要；保留数组声明顺序，排除描述与位置。
pub(super) fn semantic_hash(value: &Value) -> Result<String> {
    let bytes = serde_json::to_vec(&normalize(value))
        .map_err(|e| error("Output.Diff", "symbol", e.to_string()))?;
    Ok(input::digest(&bytes))
}
/// 比较两个已验证输入，不写文件；符号添加/删除、wire 形状和 Rust 名称分别报告。
///
/// ```
/// # use sha2::Digest;
/// # let input = tempfile::tempdir()?;
/// # let schema = br#"{"version":{"major":"1","minor":"0"},"domains":[{"domain":"Echo","commands":[{"name":"ping"}]}]}"#;
/// # std::fs::write(input.path().join("schema.json"), schema)?;
/// # let hash = format!("{:x}", sha2::Sha256::digest(schema));
/// # let manifest = input.path().join("protocol.toml");
/// # std::fs::write(&manifest, format!("format=1\ngenerated_crate=\"example_bindings\"\n[[inputs]]\npath=\"schema.json\"\nsha256=\"{hash}\"\n"))?;
/// let bundle=ydp_gen::load_manifest(&manifest)?;
/// let diff=ydp_gen::diff(&bundle,&bundle)?;
/// assert!(diff.changes.is_empty());
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn diff(old: &SourceBundle, new: &SourceBundle) -> Result<ProtocolDiff> {
    let old = crate::render::snapshot(old)?;
    let new = crate::render::snapshot(new)?;
    let keys = BTreeSet::from_iter(old.keys().chain(new.keys()));
    let mut changes = Vec::new();
    for key in keys {
        let a = old.get(key);
        let b = new.get(key);
        let ah = a.map(|x| semantic_hash(&x.0)).transpose()?;
        let bh = b.map(|x| semantic_hash(&x.0)).transpose()?;
        let wire_changed = ah != bh;
        let rust_changed = wire_changed || a.map(|x| &x.1) != b.map(|x| &x.1);
        if wire_changed || rust_changed {
            changes.push(ProtocolChange {
                symbol: key.clone(),
                old_sha256: ah,
                new_sha256: bh,
                wire_changed,
                rust_changed,
                description: match (a, b) {
                    (None, Some(_)) => "新增符号；已有 wire 方法不被替换",
                    (Some(_), None) => "删除符号；需要协议与源码兼容性审查",
                    _ if !wire_changed => "wire 不变，Rust 名称配置改变",
                    _ => "wire 定义改变；字段构造或类型同步改变，须审查源码兼容性",
                }
                .into(),
            });
        }
    }
    Ok(ProtocolDiff { changes })
}
