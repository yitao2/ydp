# ydp

[简体中文](README.zh-CN.md)

[![Crates.io](https://img.shields.io/crates/v/ydp.svg)](https://crates.io/crates/ydp)
[![Documentation](https://docs.rs/ydp/badge.svg)](https://docs.rs/ydp)
[![CI](https://github.com/yitao2/ydp/actions/workflows/ci.yml/badge.svg)](https://github.com/yitao2/ydp/actions/workflows/ci.yml)
[![License](https://img.shields.io/crates/l/ydp.svg)](LICENSE)

`ydp` is a strongly typed, async-only Chrome DevTools Protocol communication library for Rust.

It provides generated bindings for a pinned Chromium protocol snapshot, a bounded asynchronous runtime, WebSocket/TLS transport, flattened session routing, event subscriptions, and an offline generator for custom CDP-compatible protocols.

`ydp` focuses exclusively on protocol communication. It does not launch browsers or provide high-level automation concepts such as `Browser`, `Page`, `Frame`, `Locator`, navigation policies, automatic retries, reconnection, or command replay.

## Status

The current release is `0.1.0-alpha.2`.

The core protocol path, official bindings, custom protocol generation, WebSocket/TLS transport, Chromium integration, deterministic tests, and resource soak tests have been implemented and exercised on Linux.

The project remains in alpha while long-running fuzzing, the complete performance matrix, external-host TLS validation, and final macOS/Windows validation are completed.

Alpha releases may change public APIs or update the bundled protocol snapshot.

## Design goals

- Strongly typed commands, responses, and events
- Async-only networking based on Tokio
- One driver task per connection
- Bounded queues, pending requests, subscriptions, sessions, and byte buffers
- Explicit cancellation and resource ownership
- No runtime schema loading
- No normal typed-path conversion through `serde_json::Value`
- The same generator and runtime for official and custom protocols
- Reproducible offline generation from hash-pinned inputs
- No unsafe code in project-owned crates

## Crates

| Crate | Purpose |
|---|---|
| [`ydp`](https://crates.io/crates/ydp) | Official generated CDP bindings and the shared communication API |
| [`ydp-core`](https://crates.io/crates/ydp-core) | Bounded async runtime, transport contracts, sessions, requests, events, WebSocket, and TLS |
| [`ydp-gen`](https://crates.io/crates/ydp-gen) | Offline deterministic generator and CLI for official or custom protocols |

## Requirements

- Rust 1.98.1 or later
- Rust Edition 2024
- Tokio runtime with time support
- A CDP-compatible WebSocket endpoint

`ydp` does not start Chrome or Chromium. The caller is responsible for starting the endpoint and obtaining its DevTools WebSocket URL.

## Installation

```toml
[dependencies]
ydp = "=0.1.0-alpha.2"
tokio = { version = "1", features = ["rt-multi-thread", "macros", "time"] }
```

Using an exact version is recommended while the API is in alpha.

## Quick start

Set `CDP_ENDPOINT` to a browser-level DevTools WebSocket URL, such as the value returned by Chrome's `/json/version` endpoint.

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

    println!("Browser: {}", version.product);
    println!("Protocol: {}", version.protocol_version);

    let report = connection.close().await;
    assert!(report.driver_resources_released);

    drop(all_events);
    Ok(())
}
```

Generated domain methods return cold request futures. A command is submitted when its request is first polled.

Dropping a request only cancels the local wait. If the command has already been published, the remote endpoint may continue executing it.

## Sessions and events

`ydp` supports flattened CDP sessions. A remote session becomes a local typed route after it is explicitly adopted:

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

Typed event subscriptions are installed synchronously:

```rust
use ydp::runtime::events::ExecutionContextCreated;

fn subscribe(
    session: &ydp::Session,
) -> ydp::Result<ydp::EventStream<ExecutionContextCreated>> {
    session.events::<ExecutionContextCreated>(Default::default())
}
```

Install subscriptions before sending the corresponding protocol `enable` command. `ydp` does not automatically enable domains or reconstruct events that arrived before a subscription existed.

Each event includes its local session identity and a monotonically increasing receive sequence for the connection.

## Raw commands

The public API supports direct CDP commands through `Session::send_raw`.

This is useful for vendor extensions or methods absent from the bundled official snapshot. Raw commands use the same admission limits, request correlation, cancellation rules, session routing, and close behavior as typed commands.

Typed APIs are recommended when generated bindings are available.

## Custom protocols

`ydp-gen` uses the same model and generation pipeline for official Chromium schemas and custom CDP-compatible protocols.

Install the generator:

```console
cargo install ydp-gen --version 0.1.0-alpha.2 --locked
```

A minimal manifest:

```toml
format = 1
runtime_crate = "ydp_core"
generated_crate = "my_protocol"
schema_roots = ["."]

[[inputs]]
path = "protocol.json"
sha256 = "<SHA-256 of protocol.json>"
```

Generate bindings:

```console
ydp-gen generate \
  --manifest protocol.toml \
  --out src/generated
```

Verify checked-in bindings without modifying them:

```console
ydp-gen check \
  --manifest protocol.toml \
  --out src/generated
```

Compare two protocol manifests:

```console
ydp-gen diff \
  --old old/protocol.toml \
  --new new/protocol.toml
```

Generation is offline. Every input is verified against its declared SHA-256 digest before parsing. Output is deterministic and includes provenance metadata.

Protocol descriptions retain their source language. Generation never performs online translation.

## Custom transports

Applications that already own a transport can depend directly on `ydp-core`:

```toml
[dependencies]
ydp-core = { version = "=0.1.0-alpha.2", default-features = false }
```

Implement the asynchronous `Transport`, `MessageReader`, and `MessageWriter` traits, then pass the transport to `Connection::from_transport`.

The transport exchanges complete UTF-8 JSON messages. Framing, I/O ownership, shutdown, and transport-specific errors remain the transport implementation's responsibility.

## Feature flags

`ydp-core` provides these features:

| Feature | Default | Description |
|---|---:|---|
| `websocket` | Yes | WebSocket transport using `tokio-tungstenite` |
| `rustls` | Yes | WSS support with Web PKI roots and optional additional CA certificates |

Disable default features when supplying a custom transport.

## Runtime behavior

A connection uses one driver task that concurrently polls transport reads and writes.

The runtime provides:

- separate bounded admission for normal and control traffic;
- bounded pending requests and outbound frames;
- byte budgets for retained wire data;
- bounded typed and raw event subscriptions;
- explicit overflow terminal errors;
- strict response and session correlation;
- deterministic local session retirement;
- explicit asynchronous connection close reports.

The runtime does not automatically retry, reconnect, replay commands, or release browser resources represented by protocol objects. Higher-level code must manage the semantic lifetime of remote resources.

## Repository example

The repository contains a Chromium example that starts a headed browser, opens Baidu, prints the page HTML, saves a PNG screenshot, and closes the browser:

```console
cargo run -p ydp --example chromium -- --baidu
```

Supported environment variables:

- `CHROMIUM`: Chromium or Chrome executable
- `CDP_SCREENSHOT`: screenshot output path
- `CDP_PROFILE_PARENT`: parent directory for the temporary browser profile

Browser process management in this example is test scaffolding and is not part of the `ydp` library API.

## API documentation

API documentation is available on [docs.rs](https://docs.rs/ydp).

## Development checks

The repository uses Rust 1.98.1.

The primary checks are:

```console
cargo fmt --all
cargo test --workspace --locked
cargo clippy --workspace --all-targets --all-features --locked -- -D warnings
cargo test --workspace --doc --locked
```

Generator changes must preserve deterministic official and custom protocol output. Runtime changes should cover cancellation, close races, backpressure, resource limits, late messages, and cleanup.

## Contributions

This project does not accept pull requests.

Please do not open a pull request. Bug reports and compatibility reports may be submitted through GitHub Issues.

The MIT License still permits independent modification, redistribution, forks, and commercial use.

## License

Project-owned source code is licensed under the [MIT License](LICENSE).

The pinned Chromium protocol snapshot and generated bindings retain their upstream notices. See [`protocol/official/LICENSE`](protocol/official/LICENSE) for details.
