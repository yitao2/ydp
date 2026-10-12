//! 真实 Chromium 接入验收及有头百度示例；启动进程仅属于示例，不属于库。
//!
//! `cargo run -p ydp --example chromium -- --baidu` 启动有头 Chromium，访问百度，
//! 将页面 HTML 写到标准输出，保存 `baidu.png`，随后关闭浏览器并回收临时 profile。
//! `CHROMIUM` 可指定浏览器可执行文件，`CDP_SCREENSHOT` 可指定 PNG 输出路径；
//! `CDP_PROFILE_PARENT` 可指定浏览器能访问的临时 profile 父目录（例如 snap 环境）。
//! 不带 `--baidu` 时仍使用 `CDP_TEST_ENDPOINT` 执行原有双 Session 验收。
use base64::Engine;
use futures_util::StreamExt;
use std::{error::Error, time::Duration};
use tokio::{
    io::{AsyncBufReadExt, BufReader},
    time::sleep,
};
use ydp::{
    Connection, Event, SessionId,
    browser::commands as browser,
    page::{commands as page, events as page_events},
    runtime::{commands as runtime, events as runtime_events},
    target::{commands as target, events as target_events},
};

type CheckResult<T> = Result<T, Box<dyn Error + Send + Sync>>;

fn check(condition: bool, message: &str) -> CheckResult<()> {
    if condition {
        Ok(())
    } else {
        Err(std::io::Error::other(message).into())
    }
}

async fn scenario(endpoint: &str) -> CheckResult<()> {
    let (connection, mut all) = Connection::connect(endpoint, Default::default()).await?;
    let root = connection.root_session();
    let version = root
        .send::<browser::GetVersion>(browser::GetVersionParams {})
        .await?;
    let first = root
        .send::<target::CreateTarget>(serde_json::from_str(
            r#"{"url":"about:blank","background":true}"#,
        )?)
        .await?;
    let second = root
        .send::<target::CreateTarget>(serde_json::from_str(
            r#"{"url":"about:blank","background":true}"#,
        )?)
        .await?;
    let first_attach = root
        .send::<target::AttachToTarget>(target::AttachToTargetParams {
            target_id: first.target_id.clone(),
            flatten: Some(true),
        })
        .await?;
    let second_attach = root
        .send::<target::AttachToTarget>(target::AttachToTargetParams {
            target_id: second.target_id.clone(),
            flatten: Some(true),
        })
        .await?;
    let mut attached = std::collections::BTreeSet::new();
    let mut last_seq = 0;
    while attached.len() < 2 {
        let event = all
            .next()
            .await
            .ok_or_else(|| std::io::Error::other("初始事件流提前结束"))??;
        check(event.receive_seq().get() > last_seq, "All 接收序号必须递增")?;
        last_seq = event.receive_seq().get();
        if event.method() == target_events::AttachedToTarget::METHOD {
            let params = event
                .params()
                .decode::<target_events::AttachedToTargetParams>()?;
            attached.insert(params.session_id.into_inner());
        }
    }
    check(
        attached.contains(first_attach.session_id.as_str())
            && attached.contains(second_attach.session_id.as_str()),
        "必须取得两个真实 attach 事实",
    )?;
    let first_session =
        connection.adopt_session(SessionId::new(first_attach.session_id.as_str().to_owned())?)?;
    let second_session = connection.adopt_session(SessionId::new(
        second_attach.session_id.as_str().to_owned(),
    )?)?;
    check(
        first_session.key() != second_session.key(),
        "两个 child 必须具有不同本地身份",
    )?;
    // 同步订阅完成后才发送 enable，不以晚订阅尝试补回事件。
    let mut first_events =
        first_session.events::<runtime_events::ExecutionContextCreated>(Default::default())?;
    let mut second_events =
        second_session.events::<runtime_events::ExecutionContextCreated>(Default::default())?;
    let (enabled_first, enabled_second) = tokio::join!(
        first_session.send::<runtime::Enable>(runtime::EnableParams {}),
        second_session.send::<runtime::Enable>(runtime::EnableParams {}),
    );
    enabled_first?;
    enabled_second?;
    let first_event = first_events
        .next()
        .await
        .ok_or_else(|| std::io::Error::other("第一个 child 缺少事件"))??;
    let second_event = second_events
        .next()
        .await
        .ok_or_else(|| std::io::Error::other("第二个 child 缺少事件"))??;
    check(
        first_event.session_key == first_session.key()
            && second_event.session_key == second_session.key(),
        "typed 事件不可跨 Session 误路由",
    )?;
    let (first_response, second_response) = tokio::join!(
        first_session.send::<runtime::Evaluate>(serde_json::from_str(
            r#"{"expression":"21 * 2","returnByValue":true}"#
        )?),
        second_session.send::<runtime::Evaluate>(serde_json::from_str(
            r#"{"expression":"6 * 7","returnByValue":true}"#
        )?),
    );
    check(
        first_response?.result.value == Some(serde_json::json!(42)),
        "第一个 child 返回值不符",
    )?;
    check(
        second_response?.result.value == Some(serde_json::json!(42)),
        "第二个 child 返回值不符",
    )?;
    root.send::<target::DetachFromTarget>(target::DetachFromTargetParams {
        session_id: Some(first_attach.session_id),
        target_id: None,
    })
    .await?;
    first_session.close();
    check(first_session.is_closed(), "本地 Session 关闭后不得复活")?;
    let still_live = second_session
        .send::<runtime::Evaluate>(serde_json::from_str(
            r#"{"expression":"7","returnByValue":true}"#,
        )?)
        .await?;
    check(
        still_live.result.value == Some(serde_json::json!(7)),
        "关闭一个 child 不应终结另一个",
    )?;
    root.send::<target::DetachFromTarget>(target::DetachFromTargetParams {
        session_id: Some(second_attach.session_id),
        target_id: None,
    })
    .await?;
    second_session.close();
    root.send::<target::CloseTarget>(target::CloseTargetParams {
        target_id: first.target_id,
    })
    .await?;
    root.send::<target::CloseTarget>(target::CloseTargetParams {
        target_id: second.target_id,
    })
    .await?;
    let report = connection.close().await;
    check(
        report.driver_resources_released,
        "关闭报告必须等待传输资源释放",
    )?;
    println!(
        "{}",
        serde_json::json!({
            "passed": true,
            "browser": version.product,
            "browser_revision": version.revision,
            "protocol_version": version.protocol_version,
            "javascript_version": version.js_version,
            "children": 2,
            "initial_attach_events": attached.len(),
            "transport_end": format!("{:?}", report.transport_end),
            "driver_resources_released": report.driver_resources_released,
            "scope": "真实 Chromium 的官方生成命令、两个 child、初始 attach、订阅后 enable、typed 事件、并发响应与 detach/close；不替代完整 browser-automation 或发布验收"
        })
    );
    Ok(())
}

#[tokio::main]
async fn main() -> CheckResult<()> {
    if std::env::args().any(|arg| arg == "--baidu") {
        return baidu().await;
    }
    let endpoint = std::env::var("CDP_TEST_ENDPOINT")?;
    if std::env::args().any(|arg| arg == "--close-browser") {
        let (connection, events) = Connection::connect(&endpoint, Default::default()).await?;
        drop(events);
        let result = connection
            .root_session()
            .send::<browser::Close>(browser::CloseParams {})
            .await;
        if let Err(error) = result
            && !matches!(error.kind, ydp::ErrorKind::ConnectionClosed)
        {
            return Err(error.into());
        }
        connection.closed().await;
    } else {
        tokio::time::timeout(Duration::from_secs(30), scenario(&endpoint)).await??;
    }
    Ok(())
}

// 先启用并订阅生命周期事件，按本次 loader 匹配 load，避免误认初始空白页。
async fn baidu_page(connection: &Connection) -> CheckResult<()> {
    let root = connection.root_session();
    let target = root
        .send::<target::CreateTarget>(serde_json::from_str(
            r#"{"url":"about:blank","background":false}"#,
        )?)
        .await?;
    let attached = root
        .send::<target::AttachToTarget>(target::AttachToTargetParams {
            target_id: target.target_id,
            flatten: Some(true),
        })
        .await?;
    let session = connection.adopt_session(SessionId::new(attached.session_id.into_inner())?)?;
    let mut lifecycle = session.events::<page_events::LifecycleEvent>(Default::default())?;
    session
        .send::<page::Enable>(serde_json::from_str("{}")?)
        .await?;
    session
        .send::<page::SetLifecycleEventsEnabled>(page::SetLifecycleEventsEnabledParams {
            enabled: true,
        })
        .await?;
    let navigation = session
        .send::<page::Navigate>(serde_json::from_str(r#"{"url":"https://www.baidu.com"}"#)?)
        .await?;
    if let Some(error) = navigation.error_text {
        return Err(std::io::Error::other(format!("百度导航失败：{error}")).into());
    }
    let loader = navigation
        .loader_id
        .ok_or_else(|| std::io::Error::other("导航没有创建文档"))?;
    loop {
        let event = lifecycle
            .next()
            .await
            .ok_or_else(|| std::io::Error::other("等待页面加载时事件流结束"))??;
        if event.params.frame_id.as_str() == navigation.frame_id.as_str()
            && event.params.loader_id.as_str() == loader.as_str()
            && event.params.name == "load"
        {
            break;
        }
    }
    session
        .send::<page::BringToFront>(page::BringToFrontParams {})
        .await?;
    let html = session
        .send::<runtime::Evaluate>(serde_json::from_str(
            r#"{"expression":"document.documentElement.outerHTML","returnByValue":true}"#,
        )?)
        .await?;
    check(
        html.exception_details.is_none(),
        "读取页面 HTML 时 JavaScript 抛出异常",
    )?;
    let html = html
        .result
        .value
        .and_then(|value| value.as_str().map(str::to_owned))
        .ok_or_else(|| std::io::Error::other("页面 HTML 不是字符串"))?;
    println!("{html}");
    sleep(Duration::from_secs(10)).await;
    let screenshot = session
        .send::<page::CaptureScreenshot>(serde_json::from_str(
            r#"{"format":"png","captureBeyondViewport":false}"#,
        )?)
        .await?;
    let png = base64::engine::general_purpose::STANDARD.decode(screenshot.data)?;
    check(png.starts_with(b"\x89PNG\r\n\x1a\n"), "截图不是 PNG")?;
    let path = std::env::var_os("CDP_SCREENSHOT")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| std::path::PathBuf::from("baidu.png"));
    tokio::fs::write(&path, png).await?;
    eprintln!("截图已保存：{}", std::path::absolute(path)?.display());
    Ok(())
}

// 独立临时 profile 防止连接到用户已有浏览器；无 --headless 参数。
async fn baidu() -> CheckResult<()> {
    let executable = std::env::var_os("CHROMIUM").unwrap_or_else(|| "chromium".into());
    let parent = if let Some(parent) = std::env::var_os("CDP_PROFILE_PARENT") {
        std::path::PathBuf::from(parent)
    } else if std::path::Path::new("/snap/bin/chromium").exists()
        && (executable == "chromium" || executable.to_string_lossy().contains("/snap/"))
    {
        std::path::PathBuf::from(
            std::env::var_os("HOME")
                .ok_or_else(|| std::io::Error::other("snap Chromium 需要 HOME"))?,
        )
        .join("snap/chromium/common")
    } else {
        std::env::temp_dir()
    };
    tokio::fs::create_dir_all(&parent).await?;
    let profile = tempfile::Builder::new()
        .prefix("ydp-baidu-")
        .tempdir_in(parent)?;
    let mut child = tokio::process::Command::new(executable)
        .args([
            "--disable-blink-features=AutomationControlled",
            "--disable-infobars",
            "--remote-debugging-port=0",
            "--no-first-run",
            "--no-default-browser-check",
            "--password-store=basic",
            "--window-size=1280,900",
        ])
        .arg(format!("--user-data-dir={}", profile.path().display()))
        .arg("about:blank")
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::piped())
        .kill_on_drop(true)
        .spawn()?;
    let stderr = child
        .stderr
        .take()
        .ok_or_else(|| std::io::Error::other("缺少浏览器日志管道"))?;
    let mut lines = BufReader::new(stderr).lines();
    let endpoint = tokio::time::timeout(Duration::from_secs(30), async {
        while let Some(line) = lines.next_line().await? {
            if let Some(endpoint) = line.strip_prefix("DevTools listening on ") {
                return Ok::<_, std::io::Error>(endpoint.to_owned());
            }
        }
        Err(std::io::Error::other(
            "浏览器未提供调试 endpoint；请检查图形桌面与 CHROMIUM 路径",
        ))
    })
    .await;
    let endpoint = match endpoint {
        Ok(Ok(endpoint)) => endpoint,
        result => {
            child.kill().await?;
            return Err(std::io::Error::other(format!("启动浏览器失败：{result:?}")).into());
        }
    };
    // 持续排空外部进程 stderr，防止日志管道背压；这不是 SDK 的 Driver task。
    let drain = tokio::spawn(async move { while let Ok(Some(_)) = lines.next_line().await {} });
    let result = async {
        let (connection, events) = Connection::connect(&endpoint, Default::default()).await?;
        drop(events);
        let result = tokio::time::timeout(Duration::from_secs(60), baidu_page(&connection)).await;
        // 场景成功、失败或超时都显式关闭本示例创建的浏览器。
        let root = connection.root_session();
        let close = tokio::time::timeout(
            Duration::from_secs(5),
            root.send::<browser::Close>(browser::CloseParams {}),
        )
        .await;
        connection.close().await;
        match close {
            Ok(Err(error)) if !matches!(error.kind, ydp::ErrorKind::ConnectionClosed) => {
                eprintln!("Browser.close 返回错误：{error}");
            }
            Err(_) => eprintln!("Browser.close 超时，将等待或终止测试进程"),
            _ => {}
        }
        result??;
        Ok::<_, Box<dyn Error + Send + Sync>>(())
    }
    .await;
    let reaped = match tokio::time::timeout(Duration::from_secs(10), child.wait()).await {
        Ok(status) => status.map(|_| ()),
        Err(_) => child.kill().await,
    };
    drain.abort();
    let _ = drain.await;
    reaped?;
    profile.close()?;
    result
}
