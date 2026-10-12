//! 离线 CDP 协议编译器；输入经摘要验证后才进入统一生成管线。
#![forbid(unsafe_code)]
mod diff;
mod input;
/// 协议差异报告与离线比较入口。
pub use diff::{ProtocolChange, ProtocolDiff, diff};
mod layout;
mod output;
mod pdl;
mod render;

use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, path::Path};

/// 生成器错误，含稳定类别码、来源位置和中文修复提示；不依赖通信运行时。
#[derive(Debug, thiserror::Error)]
#[error("{code}: {location}: {message}")]
pub struct GeneratorError {
    /// 稳定诊断码，例如 Schema 或 Output。
    pub code: &'static str,
    /// 相对文件或符号路径，不包含输入正文。
    pub location: String,
    /// 中文原因和解决建议。
    pub message: String,
}
fn error(
    code: &'static str,
    location: impl Into<String>,
    message: impl Into<String>,
) -> GeneratorError {
    GeneratorError {
        code,
        location: location.into(),
        message: message.into(),
    }
}
/// 生成器各阶段的结构化结果，与 core 的通信错误独立。
pub type Result<T> = std::result::Result<T, GeneratorError>;

/// 经 SHA-256 校验的拥有型离线输入；内部保存源数据和配置，不持有打开的文件。
#[derive(Debug, Clone)]
pub struct SourceBundle {
    manifest: input::Manifest,
    sources: Vec<input::Source>,
    docs: BTreeMap<String, input::Translation>,
    overrides: Vec<(input::Override, Option<serde_json::Value>)>,
}
/// 纯编译阶段选项；始终保留协议原文，不要求中文翻译。
#[derive(Debug, Clone, Default)]
pub struct GenerateOptions {
    /// 兼容旧调用的保留选项，目前不影响输出或发布资格；新调用使用默认值即可。
    pub allow_incomplete_docs: bool,
}
/// 生成源码和 provenance 的拥有型集合；文件名均为受限相对路径。
#[derive(Debug, Clone)]
pub struct GeneratedOutput {
    /// 应有文件全集，按相对路径稳定排序；内容为 UTF-8。
    pub files: BTreeMap<String, String>,
    /// 非致命中文诊断，稳定排序；不能用于绕过不支持的语义。
    pub diagnostics: Vec<String>,
    /// 是否满足生成器发布条件；协议原文完整保留，不以中文翻译数量判断。
    pub release_ready: bool,
}
/// 只读比较的结果；不会修改被比较目录。
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct OutputDiff {
    /// 缺失、内容不同或多余的相对文件路径，稳定排序。
    pub changed: Vec<String>,
}
/// 源文件 provenance，不编造浏览器 revision。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceMetadata {
    /// 相对 manifest 的源路径。
    pub path: String,
    /// 源文件原始字节的 SHA-256。
    pub sha256: String,
    /// 源文件声明的协议版本。
    pub version: BTreeMap<String, String>,
}
/// 生成版本及源摘要；可序列化保存，运行时无需加载 schema。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneratedMetadata {
    /// 与 core 共享的生成契约版本。
    pub generated_contract_version: u32,
    /// 生成器包版本。
    pub generator_version: String,
    /// 绑定包 Rust crate 名称。
    pub binding_package: String,
    /// 每个输入自己的源版本和摘要。
    pub inputs: Vec<SourceMetadata>,
    /// 规范化配置和说明目录的摘要。
    pub configuration_sha256: String,
}
/// 读取 manifest 并校验所有输入摘要和根路径；不联网。路径、解析或摘要失败返回有来源的错误。
///
/// ```
/// # use sha2::Digest;
/// # let input = tempfile::tempdir()?;
/// # let schema = br#"{"version":{"major":"1","minor":"0"},"domains":[{"domain":"Echo","commands":[{"name":"ping"}]}]}"#;
/// # std::fs::write(input.path().join("schema.json"), schema)?;
/// # let hash = format!("{:x}", sha2::Sha256::digest(schema));
/// # let manifest = input.path().join("protocol.toml");
/// # std::fs::write(&manifest, format!("format=1\ngenerated_crate=\"example_bindings\"\n[[inputs]]\npath=\"schema.json\"\nsha256=\"{hash}\"\n"))?;
/// let bundle = ydp_gen::load_manifest(&manifest)?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn load_manifest(path: &Path) -> Result<SourceBundle> {
    input::load(path)
}
/// 将拥有型输入编译为确定性 Rust 源码；不写文件也不联网，未知语义或说明失配会失败。
///
/// ```
/// # use sha2::Digest;
/// # let input = tempfile::tempdir()?;
/// # let schema = br#"{"version":{"major":"1","minor":"0"},"domains":[{"domain":"Echo","commands":[{"name":"ping"}]}]}"#;
/// # std::fs::write(input.path().join("schema.json"), schema)?;
/// # let hash = format!("{:x}", sha2::Sha256::digest(schema));
/// # let manifest = input.path().join("protocol.toml");
/// # std::fs::write(&manifest, format!("format=1\ngenerated_crate=\"example_bindings\"\n[[inputs]]\npath=\"schema.json\"\nsha256=\"{hash}\"\n"))?;
/// let bundle = ydp_gen::load_manifest(&manifest)?;
/// let output = ydp_gen::compile(&bundle, &ydp_gen::GenerateOptions::default())?;
/// assert!(output.release_ready);
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn compile(bundle: &SourceBundle, options: &GenerateOptions) -> Result<GeneratedOutput> {
    render::compile(bundle, options)
}
/// 在同一文件系统暂存完整目录后发布；并发写入互斥，替换失败尝试恢复备份。拒绝额外人工文件。
///
/// ```
/// # use sha2::Digest;
/// # let input = tempfile::tempdir()?;
/// # let schema = br#"{"version":{"major":"1","minor":"0"},"domains":[{"domain":"Echo","commands":[{"name":"ping"}]}]}"#;
/// # std::fs::write(input.path().join("schema.json"), schema)?;
/// # let hash = format!("{:x}", sha2::Sha256::digest(schema));
/// # let manifest = input.path().join("protocol.toml");
/// # std::fs::write(&manifest, format!("format=1\ngenerated_crate=\"example_bindings\"\n[[inputs]]\npath=\"schema.json\"\nsha256=\"{hash}\"\n"))?;
/// let bundle = ydp_gen::load_manifest(&manifest)?;
/// let output = ydp_gen::compile(&bundle, &Default::default())?;
/// let directory = tempfile::tempdir()?;
/// ydp_gen::write_output(&output, &directory.path().join("generated"))?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn write_output(output: &GeneratedOutput, directory: &Path) -> Result<()> {
    output::write(output, directory)
}
/// 比较应有文件集和字节，不创建或修改文件；拒绝调用方标记为不可发布的产物。
///
/// ```
/// # use sha2::Digest;
/// # let input = tempfile::tempdir()?;
/// # let schema = br#"{"version":{"major":"1","minor":"0"},"domains":[{"domain":"Echo","commands":[{"name":"ping"}]}]}"#;
/// # std::fs::write(input.path().join("schema.json"), schema)?;
/// # let hash = format!("{:x}", sha2::Sha256::digest(schema));
/// # let manifest = input.path().join("protocol.toml");
/// # std::fs::write(&manifest, format!("format=1\ngenerated_crate=\"example_bindings\"\n[[inputs]]\npath=\"schema.json\"\nsha256=\"{hash}\"\n"))?;
/// let bundle = ydp_gen::load_manifest(&manifest)?;
/// let output = ydp_gen::compile(&bundle, &Default::default())?;
/// let directory = input.path().join("generated");
/// ydp_gen::write_output(&output, &directory)?;
/// let diff = ydp_gen::check_output(&output, &directory)?;
/// assert!(diff.changed.is_empty());
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn check_output(output: &GeneratedOutput, directory: &Path) -> Result<OutputDiff> {
    output::check(output, directory)
}
