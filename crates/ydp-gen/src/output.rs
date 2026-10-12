use crate::{GeneratedOutput, OutputDiff, Result, error};
use std::{
    collections::BTreeSet,
    fs,
    path::{Component, Path, PathBuf},
};
fn io(e: std::io::Error) -> crate::GeneratorError {
    error("Output.IO", "generated", e.to_string())
}
fn validate(output: &GeneratedOutput) -> Result<()> {
    for p in output.files.keys() {
        if p.is_empty()
            || Path::new(p)
                .components()
                .any(|c| !matches!(c, Component::Normal(_)))
            || p.contains('\\')
            || p.split('/')
                .any(|part| part.is_empty() || part == "." || part == "..")
        {
            return Err(error("Output.Path", p, "产物只允许规范相对文件路径"));
        }
    }
    Ok(())
}
fn inventory(directory: &Path) -> Result<BTreeSet<String>> {
    let mut result = BTreeSet::new();
    if !directory.exists() {
        return Ok(result);
    }
    let mut stack = vec![directory.to_owned()];
    while let Some(dir) = stack.pop() {
        for entry in fs::read_dir(&dir).map_err(io)? {
            let entry = entry.map_err(io)?;
            let ty = entry.file_type().map_err(io)?;
            if ty.is_symlink() {
                return Err(error(
                    "Output.Symlink",
                    "generated",
                    "生成目录不允许 symlink",
                ));
            }
            if ty.is_dir() {
                stack.push(entry.path());
            } else if ty.is_file() {
                let path = entry.path();
                let relative = path
                    .strip_prefix(directory)
                    .map_err(|_| error("Output.Path", "generated", "目录路径不一致"))?;
                result.insert(relative.to_string_lossy().replace('\\', "/"));
            } else {
                return Err(error(
                    "Output.FileType",
                    "generated",
                    "生成目录中存在非常规文件",
                ));
            }
        }
    }
    Ok(result)
}
/// 只读比较文件集合和字节；调用方标记为不可发布的产物拒绝正式检查。
pub(super) fn check(output: &GeneratedOutput, directory: &Path) -> Result<OutputDiff> {
    validate(output)?;
    if !output.release_ready {
        return Err(error(
            "Output.NotReleaseReady",
            "generated",
            "产物被标记为不可发布，不能通过正式 check",
        ));
    }
    let actual = inventory(directory)?;
    let mut changed = BTreeSet::new();
    for path in &actual {
        if !output.files.contains_key(path) {
            changed.insert(path.clone());
        }
    }
    for (path, expected) in &output.files {
        if !actual.contains(path)
            || fs::read(directory.join(path)).map_err(io)? != expected.as_bytes()
        {
            changed.insert(path.clone());
        }
    }
    Ok(OutputDiff {
        changed: changed.into_iter().collect(),
    })
}
struct Lock {
    path: PathBuf,
    file: Option<fs::File>,
}
impl Drop for Lock {
    fn drop(&mut self) {
        // 先关闭文件再移除锁路径，避免 Windows 拒绝删除仍打开的句柄。
        drop(self.file.take());
        let _ = fs::remove_file(&self.path);
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Step {
    StageWrite,
    MarkerWrite,
    BackupRename,
    PublishRename,
    RestoreRename,
}
/// 锁保护完整目录替换；失败恢复备份，未完成恢复会保留标记供后续诊断。
/// 内部使用示例：先由 compile 构造拥有型输出，再调用 write 发布；模块内故障测试覆盖恢复。
pub(super) fn write(output: &GeneratedOutput, directory: &Path) -> Result<()> {
    write_using(output, directory, |_| Ok(()))
}
// 生产入口使用无状态静态闭包，故障点不公开、不分配，也不改变输出字节。
fn write_using(
    output: &GeneratedOutput,
    directory: &Path,
    mut before: impl FnMut(Step) -> std::io::Result<()>,
) -> Result<()> {
    validate(output)?;
    let parent = directory
        .parent()
        .filter(|p| !p.as_os_str().is_empty())
        .unwrap_or(Path::new("."));
    fs::create_dir_all(parent).map_err(io)?;
    let name = directory
        .file_name()
        .and_then(|s| s.to_str())
        .ok_or_else(|| error("Output.Path", "generated", "输出目录必须有 UTF-8 名称"))?;
    let lockpath = parent.join(format!(".{name}.lock"));
    let file = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&lockpath)
        .map_err(|e| {
            error(
                "Output.Lock",
                "generated",
                format!("无法取得生成锁；请确认没有并发生成或待恢复事务：{e}"),
            )
        })?;
    let _lock = Lock {
        path: lockpath,
        file: Some(file),
    };
    let stage = parent.join(format!(".{name}.stage"));
    let backup = parent.join(format!(".{name}.backup"));
    let marker = parent.join(format!(".{name}.recovery"));
    if marker.exists() || backup.exists() || stage.exists() {
        return Err(error(
            "Output.RecoveryRequired",
            "generated",
            "发现未完成事务，请先检查 recovery、backup 与 stage 后恢复",
        ));
    }
    let actual = inventory(directory)?;
    let old_inventory = directory.join(".ydp-gen-files.json");
    let old_owned: std::collections::BTreeMap<String, String> = if old_inventory.exists() {
        serde_json::from_slice(&fs::read(old_inventory).map_err(io)?)
            .map_err(|e| error("Output.Inventory", "generated", e.to_string()))?
    } else {
        Default::default()
    };
    for path in &actual {
        if !output.files.contains_key(path) {
            let unchanged = match old_owned.get(path) {
                Some(expected) => {
                    crate::input::digest(&fs::read(directory.join(path)).map_err(io)?) == *expected
                }
                None => false,
            };
            if !unchanged {
                return Err(error(
                    "Output.ExtraFile",
                    path,
                    "目录含人工文件或已修改的旧产物，请显式移走后重试",
                ));
            }
        }
    }
    fs::create_dir(&stage).map_err(io)?;
    let staged = (|| -> Result<()> {
        for (path, contents) in &output.files {
            let dest = stage.join(path);
            if let Some(p) = dest.parent() {
                fs::create_dir_all(p).map_err(io)?;
            }
            before(Step::StageWrite).map_err(io)?;
            fs::write(dest, contents).map_err(io)?;
        }
        Ok(())
    })();
    if let Err(e) = staged {
        let _ = fs::remove_dir_all(&stage);
        return Err(e);
    }
    if let Err(e) = before(Step::MarkerWrite)
        .and_then(|()| fs::write(&marker, b"ydp-gen complete-directory replacement\n"))
    {
        let _ = fs::remove_dir_all(&stage);
        let _ = fs::remove_file(&marker);
        return Err(io(e));
    }
    let existed = directory.exists();
    if existed
        && let Err(e) = before(Step::BackupRename).and_then(|()| fs::rename(directory, &backup))
    {
        let _ = fs::remove_dir_all(&stage);
        let _ = fs::remove_file(&marker);
        return Err(io(e));
    }
    if let Err(e) = before(Step::PublishRename).and_then(|()| fs::rename(&stage, directory)) {
        if existed
            && before(Step::RestoreRename)
                .and_then(|()| fs::rename(&backup, directory))
                .is_err()
        {
            return Err(error(
                "Output.RecoveryRequired",
                "generated",
                "发布失败且恢复旧目录失败；backup 保留，请人工恢复",
            ));
        }
        let _ = fs::remove_dir_all(&stage);
        let _ = fs::remove_file(&marker);
        return Err(io(e));
    }
    if existed {
        fs::remove_dir_all(&backup).map_err(io)?;
    }
    fs::remove_file(marker).map_err(io)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        collections::BTreeMap,
        sync::{Arc, Barrier},
    };

    fn output(version: &str) -> GeneratedOutput {
        GeneratedOutput {
            files: BTreeMap::from([
                ("a.rs".into(), format!("第一份完整源码：{version}")),
                ("nested/b.rs".into(), format!("第二份完整源码：{version}")),
            ]),
            diagnostics: Vec::new(),
            release_ready: true,
        }
    }
    fn injected() -> std::io::Error {
        std::io::Error::other("测试注入的文件系统故障")
    }
    fn assert_clean(parent: &Path) {
        for path in [
            ".generated.stage",
            ".generated.backup",
            ".generated.recovery",
            ".generated.lock",
        ] {
            assert!(!parent.join(path).exists(), "事务残留：{path}");
        }
    }
    #[test]
    fn stage_marker_and_backup_failures_leave_old_directory_complete() {
        for failed in [Step::StageWrite, Step::MarkerWrite, Step::BackupRename] {
            let dir = tempfile::tempdir().expect("临时目录");
            let dest = dir.path().join("generated");
            let old = output("旧");
            let new = output("新");
            write(&old, &dest).expect("旧版发布");
            let mut writes = 0;
            let result = write_using(&new, &dest, |step| {
                if step == Step::StageWrite {
                    writes += 1;
                }
                // 第二个文件故障时暂存目录已真实写入第一个文件。
                if step == failed && (step != Step::StageWrite || writes == 2) {
                    Err(injected())
                } else {
                    Ok(())
                }
            });
            assert_eq!(result.expect_err("故障必须外显").code, "Output.IO");
            assert!(
                check(&old, &dest)
                    .expect("旧目录只读比较")
                    .changed
                    .is_empty()
            );
            assert_clean(dir.path());
            write(&new, &dest).expect("故障恢复后可重试");
            assert!(check(&new, &dest).expect("完整新版").changed.is_empty());
        }
    }
    #[test]
    fn publish_failure_restores_old_directory_before_returning() {
        let dir = tempfile::tempdir().expect("临时目录");
        let dest = dir.path().join("generated");
        let old = output("旧");
        let new = output("新");
        write(&old, &dest).expect("旧版");
        let mut restored = false;
        let result = write_using(&new, &dest, |step| {
            if step == Step::RestoreRename {
                restored = true;
            }
            if step == Step::PublishRename {
                Err(injected())
            } else {
                Ok(())
            }
        });
        assert_eq!(result.expect_err("发布故障").code, "Output.IO");
        assert!(restored);
        assert!(check(&old, &dest).expect("恢复旧版").changed.is_empty());
        assert_clean(dir.path());
    }
    #[test]
    fn restore_failure_preserves_complete_backup_and_recovery_marker() {
        let dir = tempfile::tempdir().expect("临时目录");
        let dest = dir.path().join("generated");
        let old = output("旧");
        let new = output("新");
        write(&old, &dest).expect("旧版");
        let result = write_using(&new, &dest, |step| {
            if matches!(step, Step::PublishRename | Step::RestoreRename) {
                Err(injected())
            } else {
                Ok(())
            }
        });
        assert_eq!(
            result.expect_err("恢复失败").code,
            "Output.RecoveryRequired"
        );
        assert!(!dest.exists());
        let backup = dir.path().join(".generated.backup");
        let stage = dir.path().join(".generated.stage");
        let marker = dir.path().join(".generated.recovery");
        assert!(check(&old, &backup).expect("备份完整").changed.is_empty());
        assert!(check(&new, &stage).expect("暂存完整").changed.is_empty());
        assert!(marker.is_file());
        assert_eq!(
            write(&new, &dest).expect_err("禁止覆盖待恢复证据").code,
            "Output.RecoveryRequired"
        );
        fs::rename(&backup, &dest).expect("按保留证据人工恢复");
        fs::remove_dir_all(stage).expect("移除未发布暂存");
        fs::remove_file(marker).expect("移除恢复标记");
        assert!(check(&old, &dest).expect("人工恢复旧版").changed.is_empty());
        assert_clean(dir.path());
        write(&new, &dest).expect("恢复后重新发布");
    }
    #[test]
    fn first_publication_failure_leaves_no_partial_destination() {
        let dir = tempfile::tempdir().expect("临时目录");
        let dest = dir.path().join("generated");
        assert!(
            write_using(&output("新"), &dest, |step| {
                if step == Step::PublishRename {
                    Err(injected())
                } else {
                    Ok(())
                }
            })
            .is_err()
        );
        assert!(!dest.exists());
        assert_clean(dir.path());
    }
    #[test]
    fn concurrent_writer_is_rejected_while_first_holds_lock() {
        let dir = tempfile::tempdir().expect("临时目录");
        let dest = dir.path().join("generated");
        let old = output("旧");
        let new = output("新");
        write(&old, &dest).expect("旧版");
        let reached = Arc::new(Barrier::new(2));
        let resume = Arc::new(Barrier::new(2));
        let worker = {
            let reached = reached.clone();
            let resume = resume.clone();
            let dest = dest.clone();
            let new = new.clone();
            std::thread::spawn(move || {
                let mut parked = false;
                write_using(&new, &dest, |step| {
                    if step == Step::StageWrite && !parked {
                        parked = true;
                        reached.wait();
                        resume.wait();
                    }
                    Ok(())
                })
            })
        };
        reached.wait();
        let competing = write(&output("竞争者"), &dest);
        let old_comparison = check(&old, &dest);
        resume.wait();
        assert_eq!(competing.expect_err("并发生成必须互斥").code, "Output.Lock");
        assert!(old_comparison.expect("发布前旧版仍完整").changed.is_empty());
        worker.join().expect("线程正常结束").expect("首次写入成功");
        assert!(check(&new, &dest).expect("新版本完整").changed.is_empty());
        assert_clean(dir.path());
    }
    #[test]
    fn check_does_not_create_locks_or_change_files_on_any_comparison() {
        let dir = tempfile::tempdir().expect("临时目录");
        let dest = dir.path().join("generated");
        let old = output("旧");
        write(&old, &dest).expect("发布");
        fs::write(dest.join("manual.txt"), "人工文本").expect("人工文件");
        let paths = inventory(&dest).expect("初始文件集");
        let before = paths
            .iter()
            .map(|path| (path.clone(), fs::read(dest.join(path)).expect("初始字节")))
            .collect::<BTreeMap<_, _>>();
        let diff = check(&output("新"), &dest).expect("差异检查");
        assert_eq!(diff.changed.len(), 3);
        let mut incomplete = old;
        incomplete.release_ready = false;
        assert_eq!(
            check(&incomplete, &dest)
                .expect_err("不可发布产物拒绝")
                .code,
            "Output.NotReleaseReady"
        );
        assert_eq!(inventory(&dest).expect("最终文件集"), paths);
        for (path, contents) in before {
            assert_eq!(fs::read(dest.join(path)).expect("最终字节"), contents);
        }
        assert_clean(dir.path());
        let missing = dir.path().join("missing");
        assert!(
            !check(&output("新"), &missing)
                .expect("不存在目录")
                .changed
                .is_empty()
        );
        assert!(!missing.exists());
    }
}
