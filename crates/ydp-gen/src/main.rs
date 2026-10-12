//! 离线 ydp-gen 命令行；成功为 0，check 有差异为 1，工具错误为 2。
#![forbid(unsafe_code)]
use std::path::PathBuf;
fn main() -> std::process::ExitCode {
    match run() {
        Ok(code) => std::process::ExitCode::from(code),
        Err(message) => {
            eprintln!("{message}");
            std::process::ExitCode::from(2)
        }
    }
}
fn run() -> Result<u8, String> {
    let mut args = std::env::args().skip(1);
    let operation = args
        .next()
        .ok_or("用法：ydp-gen generate|check --manifest protocol.toml --out src/generated")?;
    if operation == "diff" {
        let mut old = None;
        let mut new = None;
        while let Some(arg) = args.next() {
            match arg.as_str() {
                "--old" if old.is_none() => {
                    old = Some(PathBuf::from(args.next().ok_or("--old 缺少路径")?))
                }
                "--new" if new.is_none() => {
                    new = Some(PathBuf::from(args.next().ok_or("--new 缺少路径")?))
                }
                _ => return Err(format!("未知或重复参数：{arg}")),
            }
        }
        let old = ydp_gen::load_manifest(&old.ok_or("缺少 --old")?).map_err(|e| e.to_string())?;
        let new = ydp_gen::load_manifest(&new.ok_or("缺少 --new")?).map_err(|e| e.to_string())?;
        let diff = ydp_gen::diff(&old, &new).map_err(|e| e.to_string())?;
        println!(
            "{}",
            serde_json::to_string_pretty(&diff).map_err(|e| e.to_string())?
        );
        return Ok(u8::from(!diff.changes.is_empty()));
    }
    if operation != "generate" && operation != "check" {
        return Err("当前支持 generate/check/diff".into());
    }
    let mut manifest = None;
    let mut out = None;
    let mut allow = false;
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--manifest" if manifest.is_none() => {
                manifest = Some(PathBuf::from(args.next().ok_or("--manifest 缺少路径")?))
            }
            "--out" if out.is_none() => {
                out = Some(PathBuf::from(args.next().ok_or("--out 缺少路径")?))
            }
            // 兼容历史命令；原文描述已满足发布要求，该选项不再改变生成行为。
            "--allow-incomplete-docs" if !allow && operation == "generate" => allow = true,
            _ => return Err(format!("未知或重复参数：{arg}")),
        }
    }
    let manifest = manifest.ok_or("缺少 --manifest")?;
    let out = out.ok_or("缺少 --out")?;
    let bundle = ydp_gen::load_manifest(&manifest).map_err(|e| e.to_string())?;
    let output = ydp_gen::compile(
        &bundle,
        &ydp_gen::GenerateOptions {
            allow_incomplete_docs: allow,
        },
    )
    .map_err(|e| e.to_string())?;
    for diagnostic in &output.diagnostics {
        eprintln!("{diagnostic}");
    }
    if operation == "generate" {
        ydp_gen::write_output(&output, &out).map_err(|e| e.to_string())?;
        Ok(0)
    } else {
        let diff = ydp_gen::check_output(&output, &out).map_err(|e| e.to_string())?;
        for p in &diff.changed {
            eprintln!("差异：{p}");
        }
        Ok(u8::from(!diff.changed.is_empty()))
    }
}
