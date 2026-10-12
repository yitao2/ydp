# ydp

[English](README.md)

[![Crates.io](https://img.shields.io/crates/v/ydp.svg)](https://crates.io/crates/ydp)
[![Documentation](https://docs.rs/ydp/badge.svg)](https://docs.rs/ydp)
[![CI](https://github.com/yitao2/ydp/actions/workflows/ci.yml/badge.svg)](https://github.com/yitao2/ydp/actions/workflows/ci.yml)
[![License](https://img.shields.io/crates/l/ydp.svg)](LICENSE)

`ydp` 是一个强类型、仅异步的 Rust Chrome DevTools Protocol 通信库。

项目提供固定 Chromium 协议快照生成的类型绑定、有界异步运行时、WebSocket/TLS 传输、扁平 Session 路由、事件订阅，以及用于自定义 CDP 兼容协议的离线生成器。

`ydp` 只负责协议通信。它不负责启动浏览器，也不提供 `Browser`、`Page`、`Frame`、`Locator`、导航策略、自动重试、自动重连或命令重放等上层自动化语义。

## 项目状态

当前版本为 `0.1.0-alpha.2`。

核心协议链路、官方绑定、自定义协议生成、WebSocket/TLS、真实 Chromium 接入、确定性测试和资源 soak 已经在 Linux 上完成实现与验证。

项目仍处于 alpha 阶段，正式版发布前还需完成长时间 fuzz、完整性能矩阵、异地主机 TLS 验收，以及最终的 macOS 和 Windows 验证。

alpha 阶段可能调整公开 API 或更新内置协议快照。

## 设计目标

- 强类型命令、响应和事件
- 基于 Tokio 的纯异步网络 API
- 每个连接一个 Driver task
- 有界队列、pending、订阅、Session 和字节缓冲
- 明确的取消和资源所有权
- 运行时不加载协议 schema
- 正常 typed path 不绕经 `serde_json::Value`
- 官方协议与自定义协议共享同一生成器和运行时
- 从固定摘要输入进行可复现的离线生成
- 项目自身 crate 禁止 unsafe 代码

## Crate 组成

| Crate | 用途 |
|---|---|
| [`ydp`](https://crates.io/crates/ydp) | 官方 CDP 生成绑定和统一通信 API |
| [`ydp-core`](https://crates.io/crates/ydp-core) | 有界异步运行时、传输契约、Session、请求、事件、WebSocket 和 TLS |
| [`ydp-gen`](https://crates.io/crates/ydp-gen) | 官方及自定义协议的离线确定性生成器和命令行工具 |

## 环境要求

- Rust 1.98.1 或更高版本
- Rust Edition 2024
- 启用 time 的 Tokio runtime
- 一个 CDP 兼容的 WebSocket endpoint

`ydp` 不会启动 Chrome 或 Chromium。调用方负责启动 endpoint，并取得 DevTools WebSocket URL。

## 安装

```toml
[dependencies]
ydp = "=0.1.0-alpha.2"
tokio = { version = "1", features = ["rt-multi-thread", "macros", "time"] }
```

项目处于 alpha 阶段时，建议使用精确版本。

## 快速开始

将 `CDP_ENDPOINT` 设置为浏览器级 DevTools WebSocket URL，例如 Chrome `/json/version` 接口返回的地址。

```rust
use ydp::{
    Connection,
    SessionDomains as _,
    browser::commands::GetVersionParams,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let endpoint = std::env::var("CDP_ENDPOINT")?;

    let (connection, all_events) =
        Connection::connect(&endpoint, Default::default()).await?;

    let version = connection
        .root_session()
        .browser()
        .get_version(GetVersionParams {})
        .await?;

    println!("浏览器：{}", version.product);
    println!("协议版本：{}", version.protocol_version);

    let report = connection.close().await;
    assert!(report.driver_resources_released);

    drop(all_events);
    Ok(())
}
```

生成的 Domain 方法返回冷 Request。Request 第一次被 poll 时才会提交命令。

丢弃 Request 只会取消本地等待。如果命令已经发布，远端 endpoint 仍可能继续执行。

## Session 与事件

`ydp` 支持扁平 CDP Session。远端 Session 必须显式接管后，才会成为本地强类型路由：

```rust
use ydp::{Connection, SessionId};

fn adopt(
    connection: &Connection,
    remote_id: String,
) -> ydp::Result<ydp::Session> {
    let session_id = SessionId::new(remote_id)?;
    connection.adopt_session(session_id)
}
```

强类型事件订阅会同步完成安装：

```rust
use ydp::runtime::events::ExecutionContextCreated;

fn subscribe(
    session: &ydp::Session,
) -> ydp::Result<ydp::EventStream<ExecutionContextCreated>> {
    session.events::<ExecutionContextCreated>(Default::default())
}
```

应先安装订阅，再发送对应协议域的 `enable` 命令。`ydp` 不会自动 enable Domain，也不会补回订阅建立前已经到达的事件。

每个事件都携带本地 Session 身份，以及在当前连接中单调递增的接收序号。

## 直接发送原始命令

公开 API 支持通过 `Session::send_raw` 直接发送 CDP 命令。

它适用于 Vendor 扩展，或者内置官方快照尚未包含的方法。原始命令仍然使用与强类型命令相同的准入限制、请求关联、取消规则、Session 路由和关闭流程。

存在生成绑定时，建议优先使用强类型 API。

## 自定义协议

`ydp-gen` 对官方 Chromium schema 和自定义 CDP 兼容协议使用相同的模型和生成管线。

安装生成器：

```console
cargo install ydp-gen --version 0.1.0-alpha.2 --locked
```

最小 manifest：

```toml
format = 1
runtime_crate = "ydp_core"
generated_crate = "my_protocol"
schema_roots = ["."]

[[inputs]]
path = "protocol.json"
sha256 = "<protocol.json 的 SHA-256>"
```

生成绑定：

```console
ydp-gen generate \
  --manifest protocol.toml \
  --out src/generated
```

只读检查已提交绑定是否与输入一致：

```console
ydp-gen check \
  --manifest protocol.toml \
  --out src/generated
```

比较两个协议 manifest：

```console
ydp-gen diff \
  --old old/protocol.toml \
  --new new/protocol.toml
```

生成过程完全离线。所有输入都会在解析前验证声明的 SHA-256，输出保持确定性，并携带来源元数据。

协议 description 保留源语言，生成过程不会在线翻译。

## 自定义 Transport

已经拥有自定义传输层的应用可以直接依赖 `ydp-core`：

```toml
[dependencies]
ydp-core = { version = "=0.1.0-alpha.2", default-features = false }
```

实现异步 `Transport`、`MessageReader` 和 `MessageWriter` trait，然后将 Transport 传给 `Connection::from_transport`。

Transport 负责交换完整的 UTF-8 JSON 消息。帧处理、I/O 所有权、关闭过程和传输错误由具体 Transport 实现负责。

## Feature flags

`ydp-core` 提供以下 feature：

| Feature | 默认启用 | 用途 |
|---|---:|---|
| `websocket` | 是 | 基于 `tokio-tungstenite` 的 WebSocket 传输 |
| `rustls` | 是 | 使用 Web PKI 根证书及可选附加 CA 的 WSS 支持 |

使用自定义 Transport 时可以关闭默认 feature。

## 运行时语义

每个连接使用一个 Driver task，并发 poll 传输层的读取和写入。

运行时提供：

- Normal 和 Control 流量的独立有界准入；
- 有界 pending 请求和出站帧；
- 保留 wire 数据的字节预算；
- 有界强类型和原始事件订阅；
- 可观察的队列溢出终态；
- 严格的响应及 Session 关联；
- 确定性的本地 Session 退役；
- 显式异步连接关闭报告。

运行时不会自动重试、重连、重放命令，也不会释放协议对象代表的浏览器远端资源。上层代码需要自行管理远端资源的语义生命周期。

## 仓库示例

仓库包含一个真实 Chromium 示例：启动有头浏览器、访问百度、打印页面 HTML、保存 PNG 截图并关闭浏览器。

```console
cargo run -p ydp --example chromium -- --baidu
```

可用环境变量：

- `CHROMIUM`：Chromium 或 Chrome 可执行文件
- `CDP_SCREENSHOT`：截图输出路径
- `CDP_PROFILE_PARENT`：浏览器临时 profile 的父目录

该示例中的浏览器进程管理只用于接入验收，不属于 `ydp` 的库 API。

## API 文档

公开 API 文档位于 [docs.rs](https://docs.rs/ydp)。

## 开发检查

仓库使用 Rust 1.98.1。

主要检查命令：

```console
cargo fmt --all
cargo test --workspace --locked
cargo clippy --workspace --all-targets --all-features --locked -- -D warnings
cargo test --workspace --doc --locked
```

生成器修改必须保持官方协议和自定义协议输出的确定性。运行时修改应覆盖取消、关闭竞态、背压、资源限制、晚到消息和资源清理。

## 贡献政策

本项目不接受 Pull Request。

请不要创建 Pull Request。Bug 报告和兼容性问题可以通过 GitHub Issues 提交。

MIT 许可证仍然允许第三方独立修改、再分发、创建 fork，以及直接用于商业项目。

## 许可证

项目自身源码使用 [MIT License](LICENSE)。

固定 Chromium 协议快照和生成绑定保留上游许可证及版权声明，详情见 [`protocol/official/LICENSE`](protocol/official/LICENSE)。
