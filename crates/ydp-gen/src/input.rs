use crate::{Result, SourceBundle, error};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::{Path, PathBuf},
};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct Input {
    path: String,
    sha256: String,
}
/// manifest 的严格配置；不支持的覆盖操作显式拒绝。
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Manifest {
    format: u32,
    /// 生成代码引用的核心 crate 名。
    #[serde(default = "runtime")]
    pub(super) runtime_crate: String,
    /// doctest 使用的绑定 crate 名。
    pub(super) generated_crate: String,
    #[serde(default)]
    schema_roots: Vec<String>,
    #[serde(default)]
    docs_zh: Option<String>,
    inputs: Vec<Input>,
    /// 精确符号路径到 Rust 基名的映射。
    #[serde(default)]
    pub(super) rust_names: BTreeMap<String, String>,
    /// 显式允许的非语义扩展键。
    #[serde(default)]
    pub(super) metadata_keys: Vec<String>,
    #[serde(default)]
    overrides: Vec<Override>,
}
/// 一个完整符号的显式替换或删除前置条件。
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Override {
    /// Domain.kind.Name 格式的目标身份。
    pub(super) target: String,
    /// 原符号规范语义 SHA-256。
    pub(super) expected_sha256: String,
    /// replace 或 remove，不执行脚本。
    pub(super) action: String,
    /// replace 的完整 JSON 定义路径。
    pub(super) definition: Option<String>,
}
fn runtime() -> String {
    "ydp_core".into()
}
/// 经摘要绑定的中文说明。
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Translation {
    /// 规范化英文原文的摘要。
    pub(super) source_sha256: String,
    /// 不直接作为 Rust token 解释的中文文本。
    pub(super) text: String,
}
/// 单个经过摘要验证的 JSON 源。
#[derive(Debug, Clone)]
pub(super) struct Source {
    /// manifest 相对路径。
    pub(super) path: String,
    /// 文件的原始字节摘要。
    pub(super) sha256: String,
    /// JSON 模型，后续统一校验和解析。
    pub(super) value: Value,
    /// 是否为显式顶层入口；include fragment 可以不声明版本。
    pub(super) entry: bool,
}
/// 计算原始字节摘要；内部测试覆盖确定性。
pub(super) fn digest(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}
/// 验证 Rust crate 名，不接受任意 token。
pub(super) fn identifier(s: &str) -> bool {
    !s.is_empty()
        && s.bytes()
            .enumerate()
            .all(|(i, c)| c == b'_' || c.is_ascii_alphabetic() || (i > 0 && c.is_ascii_digit()))
        && syn::parse_str::<syn::Ident>(s).is_ok()
}
fn read(path: &Path) -> Result<Vec<u8>> {
    let meta = fs::metadata(path)
        .map_err(|e| error("Parse.IO", path.display().to_string(), e.to_string()))?;
    if meta.len() > 64 * 1024 * 1024 {
        return Err(error(
            "Schema.ResourceLimit",
            path.display().to_string(),
            "单文件超过 64 MiB，请缩减输入",
        ));
    }
    fs::read(path).map_err(|e| error("Parse.IO", path.display().to_string(), e.to_string()))
}
fn canonical(path: &Path) -> Result<PathBuf> {
    fs::canonicalize(path)
        .map_err(|e| error("Parse.Path", path.display().to_string(), e.to_string()))
}
/// 读取输入并检查 root、摘要和配置。模块测试执行错误与正常分支。
pub(super) fn load(path: &Path) -> Result<SourceBundle> {
    let bytes = read(path)?;
    let text = std::str::from_utf8(&bytes)
        .map_err(|_| error("Parse.UTF8", "manifest", "配置必须为 UTF-8"))?;
    let manifest: Manifest =
        toml::from_str(text).map_err(|e| error("Parse.Manifest", "manifest", e.to_string()))?;
    if manifest.format != 1 || manifest.inputs.is_empty() || manifest.inputs.len() > 4096 {
        return Err(error(
            "Schema.Manifest",
            "manifest",
            "format 必须为 1，输入数必须在 1～4096",
        ));
    }
    for name in [&manifest.runtime_crate, &manifest.generated_crate] {
        if !identifier(name) {
            return Err(error(
                "Naming.Crate",
                "manifest",
                "crate 名必须是合法 Rust 标识符",
            ));
        }
    }
    for key in &manifest.metadata_keys {
        if !key.starts_with("x-") || key == "x-cdp-rs" {
            return Err(error(
                "Schema.Metadata",
                key,
                "非语义扩展必须使用独立 x- 名称",
            ));
        }
    }
    let parent = path.parent().unwrap_or(Path::new("."));
    let roots = if manifest.schema_roots.is_empty() {
        vec![canonical(parent)?]
    } else {
        manifest
            .schema_roots
            .iter()
            .map(|p| canonical(&parent.join(p)))
            .collect::<Result<Vec<_>>>()?
    };
    let mut sources: Vec<Source> = Vec::new();
    let mut paths = BTreeSet::new();
    let mut total = 0usize;
    for entry in &manifest.inputs {
        if Path::new(&entry.path).is_absolute() || !paths.insert(entry.path.clone()) {
            return Err(error(
                "Schema.InputPath",
                &entry.path,
                "输入路径必须相对且不能重复",
            ));
        }
        if entry.sha256.len() != 64 || !entry.sha256.bytes().all(|b| b.is_ascii_hexdigit()) {
            return Err(error(
                "Schema.SHA256",
                &entry.path,
                "请填写 64 位真实 SHA-256",
            ));
        }
        let actual = canonical(&parent.join(&entry.path))?;
        if !roots.iter().any(|r| actual.starts_with(r)) {
            return Err(error(
                "Schema.RootEscape",
                &entry.path,
                "输入或 symlink 已逃离 schema_roots",
            ));
        }
        let bytes = read(&actual)?;
        let sha256 = digest(&bytes);
        if !entry.sha256.eq_ignore_ascii_case(&sha256) {
            return Err(error(
                "Schema.HashMismatch",
                &entry.path,
                "源文件摘要不符，请审查变更后更新 manifest",
            ));
        }
        load_tree(
            &actual,
            &parent
                .canonicalize()
                .map_err(|e| error("Parse.Path", "manifest", e.to_string()))?,
            &roots,
            &mut sources,
            &mut Vec::new(),
            &mut total,
            true,
            0,
        )?;
    }
    sources.sort_by(|a, b| a.path.cmp(&b.path));
    let docs = match &manifest.docs_zh {
        Some(p) => serde_json::from_slice(&read(&parent.join(p))?)
            .map_err(|e| error("Parse.Docs", p, e.to_string()))?,
        None => BTreeMap::new(),
    };
    let mut overrides = Vec::new();
    let mut targets = BTreeSet::new();
    for config in &manifest.overrides {
        if !targets.insert(config.target.clone()) {
            return Err(error(
                "Merge.DuplicateOverride",
                &config.target,
                "同一目标只能有一项 override",
            ));
        }
        if config.expected_sha256.len() != 64
            || !config
                .expected_sha256
                .bytes()
                .all(|b| b.is_ascii_hexdigit())
        {
            return Err(error(
                "Merge.OverrideHash",
                &config.target,
                "expected_sha256 必须为真实 64 位摘要",
            ));
        }
        let definition = match (config.action.as_str(), &config.definition) {
            ("remove", None) => None,
            ("replace", Some(path)) => {
                let actual = canonical(&parent.join(path))?;
                if Path::new(path).is_absolute()
                    || !roots.iter().any(|root| actual.starts_with(root))
                {
                    return Err(error(
                        "Schema.RootEscape",
                        path,
                        "overlay 定义必须为 roots 内相对路径",
                    ));
                }
                let bytes = read(&actual)?;
                total = total.saturating_add(bytes.len());
                if total > 64 * 1024 * 1024 {
                    return Err(error("Schema.ResourceLimit", path, "输入总量超过 64 MiB"));
                }
                Some(
                    serde_json::from_slice(&bytes)
                        .map_err(|e| error("Parse.JSON", path, e.to_string()))?,
                )
            }
            _ => {
                return Err(error(
                    "Merge.OverrideAction",
                    &config.target,
                    "remove 不带 definition，replace 必须带完整 definition",
                ));
            }
        };
        overrides.push((config.clone(), definition));
    }
    Ok(SourceBundle {
        manifest,
        overrides,
        sources,
        docs,
    })
}
/// 对规范化生成配置与中文目录计算摘要，不受输入排列影响。
pub(super) fn configuration(bundle: &SourceBundle) -> Result<String> {
    let mut manifest = bundle.manifest.clone();
    manifest.inputs.sort_by(|a, b| a.path.cmp(&b.path));
    manifest.schema_roots.sort();
    manifest.metadata_keys.sort();
    let data = serde_json::to_vec(&(manifest, &bundle.docs, &bundle.overrides))
        .map_err(|e| error("Output.Metadata", "manifest", e.to_string()))?;
    Ok(digest(&data))
}

#[allow(clippy::too_many_arguments)]
fn load_tree(
    path: &Path,
    base: &Path,
    roots: &[PathBuf],
    sources: &mut Vec<Source>,
    active: &mut Vec<PathBuf>,
    total: &mut usize,
    entry: bool,
    depth: usize,
) -> Result<()> {
    let actual = canonical(path)?;
    let base_components = base.components().collect::<Vec<_>>();
    let actual_components = actual.components().collect::<Vec<_>>();
    let common = base_components
        .iter()
        .zip(&actual_components)
        .take_while(|(a, b)| a == b)
        .count();
    let mut relative = PathBuf::new();
    for _ in common..base_components.len() {
        relative.push("..");
    }
    for component in &actual_components[common..] {
        relative.push(component.as_os_str());
    }
    let display = relative.to_string_lossy().replace('\\', "/");
    if !roots.iter().any(|root| actual.starts_with(root)) {
        return Err(error(
            "Schema.RootEscape",
            display,
            "include 或 symlink 逃离 schema_roots",
        ));
    }
    if active.contains(&actual) {
        return Err(error(
            "Parse.IncludeCycle",
            display,
            format!(
                "include 环：{}",
                active
                    .iter()
                    .map(|p| p.display().to_string())
                    .collect::<Vec<_>>()
                    .join(" → ")
            ),
        ));
    }
    if let Some(old) = sources.iter_mut().find(|source| source.path == display) {
        if entry {
            old.entry = true;
            if old.value.get("version").is_none() {
                return Err(error("Schema.Version", display, "显式入口必须声明 version"));
            }
        }
        return Ok(());
    }
    if depth > 64 || sources.len() >= 4096 {
        return Err(error(
            "Schema.ResourceLimit",
            display,
            "include 深度超过 64 或文件数超过 4096",
        ));
    }
    let bytes = read(&actual)?;
    *total = total.saturating_add(bytes.len());
    if *total > 64 * 1024 * 1024 {
        return Err(error(
            "Schema.ResourceLimit",
            display,
            "输入总量超过 64 MiB",
        ));
    }
    let (value, includes) = match actual.extension().and_then(|s| s.to_str()) {
        Some("json") => (
            serde_json::from_slice(&bytes)
                .map_err(|e| error("Parse.JSON", &display, e.to_string()))?,
            Vec::new(),
        ),
        Some("pdl") => crate::pdl::parse(
            std::str::from_utf8(&bytes)
                .map_err(|e| error("Parse.UTF8", &display, e.to_string()))?,
            &display,
        )?,
        _ => return Err(error("Parse.Format", display, "输入仅支持 JSON/PDL")),
    };
    if entry && value.get("version").is_none() {
        return Err(error("Schema.Version", display, "显式入口必须声明 version"));
    }
    active.push(actual.clone());
    sources.push(Source {
        path: display,
        sha256: digest(&bytes),
        value,
        entry,
    });
    for include in includes {
        let parent = actual
            .parent()
            .ok_or_else(|| error("Parse.Path", "include", "include 缺少父目录"))?;
        if Path::new(&include).is_absolute() {
            return Err(error(
                "Schema.RootEscape",
                include,
                "include 必须为相对路径",
            ));
        }
        load_tree(
            &parent.join(include),
            base,
            roots,
            sources,
            active,
            total,
            false,
            depth + 1,
        )?;
    }
    active.pop();
    Ok(())
}
