//! 核心内存传输的可复现性能矩阵；输出机器可读结果，不作为功能正确性测试。

use bytes::Bytes;
use futures_util::{StreamExt, stream::FuturesUnordered};
use serde::{Deserialize, Serialize};
use std::{
    error::Error,
    io,
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
    time::{Duration, Instant},
};
use tokio::sync::{Notify, mpsc};
use ydp_core::{
    Command, Connection, MessageReader, MessageWriter, RequestOptions, RuntimeOptions, SessionId,
    SubscriptionOptions, TrafficClass, Transport, TransportError,
};

type Result<T> = std::result::Result<T, Box<dyn Error + Send + Sync>>;

#[derive(Clone, Serialize)]
struct Payload {
    payload: String,
}
#[derive(Deserialize)]
struct Reply {
    payload: String,
}
struct Echo;
impl Command for Echo {
    type Params = Payload;
    type Response = Reply;
    const METHOD: &'static str = "Bench.echo";
}
struct Never;
impl Command for Never {
    type Params = Payload;
    type Response = Reply;
    const METHOD: &'static str = "Bench.never";
}
#[derive(Serialize)]
struct EmitParams {
    count: usize,
    payload_bytes: usize,
    target_per_second: usize,
}
#[derive(Deserialize)]
struct EmitReply {
    sent: usize,
}
struct Emit;
impl Command for Emit {
    type Params = EmitParams;
    type Response = EmitReply;
    const METHOD: &'static str = "Bench.emit";
}

struct Reader(mpsc::Receiver<Bytes>);
struct Writer(Option<mpsc::Sender<Bytes>>);
struct Endpoint {
    input: mpsc::Receiver<Bytes>,
    output: mpsc::Sender<Bytes>,
}
impl Transport for Endpoint {
    type Reader = Reader;
    type Writer = Writer;
    fn split(self) -> (Reader, Writer) {
        (Reader(self.input), Writer(Some(self.output)))
    }
}
impl MessageReader for Reader {
    async fn recv(&mut self) -> std::result::Result<Option<Bytes>, TransportError> {
        Ok(self.0.recv().await)
    }
}
impl MessageWriter for Writer {
    async fn send(&mut self, bytes: Bytes) -> std::result::Result<(), TransportError> {
        self.0
            .as_ref()
            .ok_or_else(|| TransportError::new(io::Error::other("写端已经关闭")))?
            .send(bytes)
            .await
            .map_err(|e| TransportError::new(io::Error::other(e)))
    }
    async fn flush(&mut self) -> std::result::Result<(), TransportError> {
        Ok(())
    }
    async fn close(&mut self) -> std::result::Result<(), TransportError> {
        self.0.take();
        Ok(())
    }
}

#[derive(Default)]
struct Metrics {
    received: AtomicUsize,
    responses: AtomicUsize,
    events: AtomicUsize,
    queue_high: AtomicUsize,
    frame_high: AtomicUsize,
    changed: Notify,
}
impl Metrics {
    fn observe(&self, tx: &mpsc::Sender<Bytes>, len: usize) {
        self.frame_high.fetch_max(len, Ordering::Relaxed);
        self.queue_high.fetch_max(
            tx.max_capacity().saturating_sub(tx.capacity()),
            Ordering::Relaxed,
        );
    }
    async fn wait_received(&self, target: usize) {
        while self.received.load(Ordering::Acquire) < target {
            let notified = self.changed.notified();
            if self.received.load(Ordering::Acquire) >= target {
                break;
            }
            notified.await;
        }
    }
}
async fn send(tx: &mpsc::Sender<Bytes>, metrics: &Metrics, value: serde_json::Value) -> Result<()> {
    let bytes = Bytes::from(serde_json::to_vec(&value)?);
    metrics.observe(tx, bytes.len());
    tx.send(bytes).await.map_err(io::Error::other)?;
    Ok(())
}
async fn endpoint(
    mut rx: mpsc::Receiver<Bytes>,
    tx: mpsc::Sender<Bytes>,
    metrics: Arc<Metrics>,
) -> Result<usize> {
    let mut total = 0;
    while let Some(bytes) = rx.recv().await {
        let value: serde_json::Value = serde_json::from_slice(&bytes)?;
        let id = value["id"]
            .as_i64()
            .ok_or_else(|| io::Error::other("缺少 id"))?;
        let method = value["method"]
            .as_str()
            .ok_or_else(|| io::Error::other("缺少 method"))?;
        let session = value.get("sessionId").cloned();
        total += 1;
        metrics.received.store(total, Ordering::Release);
        metrics.changed.notify_waiters();
        if method == Never::METHOD {
            continue;
        }
        if method == Emit::METHOD {
            let count = value["params"]["count"]
                .as_u64()
                .and_then(|v| usize::try_from(v).ok())
                .ok_or_else(|| io::Error::other("缺少事件数"))?;
            let size = value["params"]["payload_bytes"]
                .as_u64()
                .and_then(|v| usize::try_from(v).ok())
                .ok_or_else(|| io::Error::other("缺少事件大小"))?;
            let rate = value["params"]["target_per_second"].as_u64().unwrap_or(0);
            let started = tokio::time::Instant::now();
            let payload = "e".repeat(size);
            for index in 0..count {
                if rate > 0 {
                    tokio::time::sleep_until(
                        started + Duration::from_secs_f64((index + 1) as f64 / rate as f64),
                    )
                    .await;
                }
                let mut event = serde_json::json!({"method":"Bench.event","params":{"index":index,"payload":payload}});
                if let Some(s) = &session {
                    event["sessionId"] = s.clone();
                }
                send(&tx, &metrics, event).await?;
                metrics.events.fetch_add(1, Ordering::Relaxed);
            }
            let mut response = serde_json::json!({"id":id,"result":{"sent":count}});
            if let Some(s) = session {
                response["sessionId"] = s;
            }
            send(&tx, &metrics, response).await?;
            metrics.responses.fetch_add(1, Ordering::Relaxed);
            continue;
        }
        let payload = value
            .pointer("/params/payload")
            .and_then(|v| v.as_str())
            .ok_or_else(|| io::Error::other("缺少 payload"))?;
        let mut response = serde_json::json!({"id":id,"result":{"payload":payload}});
        if let Some(s) = session {
            response["sessionId"] = s;
        }
        send(&tx, &metrics, response).await?;
        metrics.responses.fetch_add(1, Ordering::Relaxed);
    }
    Ok(total)
}

struct Fixture {
    connection: Connection,
    initial: Option<ydp_core::RawEventStream>,
    sessions: Vec<ydp_core::Session>,
    task: tokio::task::JoinHandle<Result<usize>>,
    metrics: Arc<Metrics>,
}
impl Fixture {
    fn new(sessions: usize, capacity: usize, initial: SubscriptionOptions) -> Result<Self> {
        let cap = capacity.max(1);
        let (out_tx, out_rx) = mpsc::channel(cap);
        let (in_tx, in_rx) = mpsc::channel(cap);
        let metrics = Arc::new(Metrics::default());
        let task = tokio::spawn(endpoint(out_rx, in_tx, metrics.clone()));
        let mut options = RuntimeOptions {
            initial_events: initial,
            ..Default::default()
        };
        let normal = cap.max(2_048);
        options.limits.normal.requests = normal;
        options.limits.normal.admission_waiters = normal;
        options.limits.normal.outbound_slots = normal;
        options.limits.control.requests = cap.max(32);
        options.limits.control.admission_waiters = cap.max(64);
        options.limits.control.outbound_slots = cap.max(8);
        let (connection, events) = Connection::from_transport(
            Endpoint {
                input: in_rx,
                output: out_tx,
            },
            options,
        )?;
        let mut routes = vec![connection.root_session()];
        for i in 1..sessions {
            routes.push(connection.adopt_session(SessionId::new(format!("bench-{i}"))?)?);
        }
        Ok(Self {
            connection,
            initial: Some(events),
            sessions: routes,
            task,
            metrics,
        })
    }
    async fn close(mut self) -> Result<Close> {
        self.initial.take();
        self.sessions.clear();
        let report = self.connection.close().await;
        if !report.driver_resources_released {
            return Err(io::Error::other("Driver 资源未释放").into());
        }
        Ok(Close {
            endpoint_commands: self.task.await??,
            pending_at_close: report.pending_count,
            discarded_outbound: report.discarded_outbound,
            endpoint_output_queue_high_water: self.metrics.queue_high.load(Ordering::Relaxed),
            max_response_frame_bytes: self.metrics.frame_high.load(Ordering::Relaxed),
        })
    }
}
#[derive(Serialize)]
struct Close {
    endpoint_commands: usize,
    pending_at_close: usize,
    discarded_outbound: usize,
    endpoint_output_queue_high_water: usize,
    max_response_frame_bytes: usize,
}
#[derive(Serialize)]
struct Latency {
    p50: u64,
    p95: u64,
    p99: u64,
    max: u64,
}
fn percentile(values: &[u64], n: usize) -> u64 {
    if values.is_empty() {
        0
    } else {
        values[values
            .len()
            .saturating_mul(n)
            .div_ceil(100)
            .saturating_sub(1)
            .min(values.len() - 1)]
    }
}
fn latency(mut values: Vec<u64>) -> Latency {
    values.sort_unstable();
    Latency {
        p50: percentile(&values, 50),
        p95: percentile(&values, 95),
        p99: percentile(&values, 99),
        max: values.last().copied().unwrap_or(0),
    }
}
#[cfg(target_os = "linux")]
fn rss() -> Option<u64> {
    let s = std::fs::read_to_string("/proc/self/status").ok()?;
    s.lines()
        .find_map(|l| {
            l.strip_prefix("VmRSS:")?
                .split_whitespace()
                .next()?
                .parse::<u64>()
                .ok()
        })?
        .checked_mul(1024)
}
#[cfg(not(target_os = "linux"))]
fn rss() -> Option<u64> {
    None
}

#[derive(Clone, Copy)]
struct Round {
    concurrency: usize,
    requests: usize,
    sessions: usize,
    bytes: usize,
}
#[derive(Serialize)]
struct RoundReport {
    kind: &'static str,
    concurrency: usize,
    requests: usize,
    sessions: usize,
    payload_bytes: usize,
    successes: usize,
    failures: usize,
    failure_rate: f64,
    elapsed_ns: u128,
    throughput_per_second: f64,
    latency_ns: Latency,
    initial_rss_bytes: Option<u64>,
    peak_rss_bytes: Option<u64>,
    final_rss_bytes: Option<u64>,
    request_high_water: usize,
    close: Close,
}
async fn round(case: Round) -> Result<RoundReport> {
    let mut fixture = Fixture::new(
        case.sessions,
        case.concurrency,
        SubscriptionOptions::default(),
    )?;
    fixture.initial.take();
    let sessions = fixture.sessions.clone();
    let payload = "p".repeat(case.bytes);
    let initial = rss();
    let started = Instant::now();
    let mut next = 0;
    let mut futures = FuturesUnordered::new();
    let mut times = Vec::with_capacity(case.requests);
    let (mut ok, mut failed) = (0, 0);
    while next < case.requests || !futures.is_empty() {
        while next < case.requests && futures.len() < case.concurrency {
            let session = sessions[next % sessions.len()].clone();
            let payload = payload.clone();
            futures.push(async move {
                let start = Instant::now();
                let result = session.send::<Echo>(Payload { payload }).await;
                (start.elapsed(), result)
            });
            next += 1;
        }
        if let Some((request_elapsed, result)) = futures.next().await {
            times.push(u64::try_from(request_elapsed.as_nanos())?);
            match result {
                Ok(v) if v.payload.len() == case.bytes => ok += 1,
                _ => failed += 1,
            }
        }
    }
    let elapsed = started.elapsed();
    let peak = rss();
    let close = fixture.close().await?;
    let final_rss = rss();
    Ok(RoundReport {
        kind: "round-trip",
        concurrency: case.concurrency,
        requests: case.requests,
        sessions: case.sessions,
        payload_bytes: case.bytes,
        successes: ok,
        failures: failed,
        failure_rate: failed as f64 / case.requests as f64,
        elapsed_ns: elapsed.as_nanos(),
        throughput_per_second: case.requests as f64 / elapsed.as_secs_f64(),
        latency_ns: latency(times),
        initial_rss_bytes: initial,
        peak_rss_bytes: peak,
        final_rss_bytes: final_rss,
        request_high_water: case.concurrency.min(case.requests),
        close,
    })
}

#[derive(Clone, Copy, Serialize)]
#[serde(rename_all = "snake_case")]
enum Mode {
    None,
    Single,
    Multiple,
    Slow,
}
#[derive(Clone, Copy)]
struct EventCase {
    count: usize,
    bytes: usize,
    rate: usize,
    mode: Mode,
}
#[derive(Serialize)]
struct EventReport {
    kind: &'static str,
    mode: Mode,
    events: usize,
    payload_bytes: usize,
    target_per_second: usize,
    observed_per_second: f64,
    consumed: usize,
    terminal_errors: usize,
    failures: usize,
    elapsed_ns: u128,
    subscription_queue_high_water_estimate: usize,
    close: Close,
}
async fn drain(mut stream: ydp_core::RawEventStream, expected: usize) -> (usize, usize) {
    let (mut count, mut errors) = (0, 0);
    while count < expected {
        match stream.next().await {
            Some(Ok(_)) => count += 1,
            Some(Err(_)) => {
                errors += 1;
                break;
            }
            None => break,
        }
    }
    (count, errors)
}
async fn events(case: EventCase) -> Result<EventReport> {
    let options = if matches!(case.mode, Mode::Slow) {
        SubscriptionOptions {
            max_events: 32,
            max_bytes: (case.bytes + 256) * 32,
        }
    } else {
        SubscriptionOptions {
            max_events: case.count.max(1),
            max_bytes: (case.bytes + 256) * case.count.max(1),
        }
    };
    let mut fixture = Fixture::new(1, 256, options)?;
    let initial = fixture
        .initial
        .take()
        .ok_or_else(|| io::Error::other("缺少初始事件流"))?;
    let mut streams = Vec::new();
    match case.mode {
        Mode::None => drop(initial),
        Mode::Single | Mode::Slow => streams.push(initial),
        Mode::Multiple => {
            streams.push(initial);
            for _ in 0..3 {
                streams.push(fixture.connection.raw_events(options)?);
            }
        }
    }
    let fast = !matches!(case.mode, Mode::Slow);
    let mut drains = Vec::new();
    if fast {
        for s in streams.drain(..) {
            drains.push(tokio::spawn(drain(s, case.count)));
        }
    }
    let started = Instant::now();
    let reply = fixture.sessions[0]
        .send::<Emit>(EmitParams {
            count: case.count,
            payload_bytes: case.bytes,
            target_per_second: case.rate,
        })
        .await;
    let elapsed = started.elapsed();
    let (mut consumed, mut terminal) = (0, 0);
    if fast {
        for task in drains {
            let (c, e) = task.await?;
            consumed += c;
            terminal += e
        }
    } else {
        for s in streams {
            let (_, e) = drain(s, 1).await;
            terminal += e
        }
    }
    let failures = usize::from(reply.as_ref().map(|r| r.sent != case.count).unwrap_or(true));
    let close = fixture.close().await?;
    Ok(EventReport {
        kind: "events",
        mode: case.mode,
        events: case.count,
        payload_bytes: case.bytes,
        target_per_second: case.rate,
        observed_per_second: case.count as f64 / elapsed.as_secs_f64(),
        consumed,
        terminal_errors: terminal,
        failures,
        elapsed_ns: elapsed.as_nanos(),
        subscription_queue_high_water_estimate: if matches!(case.mode, Mode::Slow) {
            32
        } else {
            0
        },
        close,
    })
}

#[derive(Serialize)]
struct CancelReport {
    kind: &'static str,
    submitted: usize,
    cancelled: usize,
    task_join_failures: usize,
    elapsed_ns: u128,
    close: Close,
}
async fn cancellation(count: usize) -> Result<CancelReport> {
    let fixture = Fixture::new(1, count, SubscriptionOptions::default())?;
    let session = fixture.sessions[0].clone();
    let started = Instant::now();
    let mut tasks = Vec::new();
    for _ in 0..count {
        let s = session.clone();
        tasks.push(tokio::spawn(async move {
            s.send::<Never>(Payload {
                payload: String::new(),
            })
            .await
        }));
    }
    fixture.metrics.wait_received(count).await;
    for t in &tasks {
        t.abort()
    }
    let mut failures = 0;
    for t in tasks {
        if !t.await.is_err_and(|e| e.is_cancelled()) {
            failures += 1
        }
    }
    let elapsed = started.elapsed();
    let close = fixture.close().await?;
    Ok(CancelReport {
        kind: "cancellation-storm",
        submitted: count,
        cancelled: count - failures,
        task_join_failures: failures,
        elapsed_ns: elapsed.as_nanos(),
        close,
    })
}
#[derive(Serialize)]
struct Competition {
    kind: &'static str,
    normal_requests: usize,
    control_requests: usize,
    normal_latency_ns: Latency,
    control_latency_ns: Latency,
    failures: usize,
    elapsed_ns: u128,
    close: Close,
}
async fn competition(count: usize) -> Result<Competition> {
    let fixture = Fixture::new(1, count * 2, SubscriptionOptions::default())?;
    let session = fixture.sessions[0].clone();
    let started = Instant::now();
    let mut futures = FuturesUnordered::new();
    for i in 0..count * 2 {
        let s = session.clone();
        let class = if i % 2 == 0 {
            TrafficClass::Normal
        } else {
            TrafficClass::Control
        };
        futures.push(async move {
            let t = Instant::now();
            (
                class,
                t,
                s.send_with::<Echo>(
                    Payload {
                        payload: "control".into(),
                    },
                    RequestOptions {
                        traffic_class: class,
                        ..Default::default()
                    },
                )
                .await,
            )
        });
    }
    let (mut normal, mut control, mut failed) = (Vec::new(), Vec::new(), 0);
    while let Some((class, start, result)) = futures.next().await {
        if result.is_err() {
            failed += 1
        }
        let target = if class == TrafficClass::Normal {
            &mut normal
        } else {
            &mut control
        };
        target.push(u64::try_from(start.elapsed().as_nanos())?);
    }
    let elapsed = started.elapsed();
    let close = fixture.close().await?;
    Ok(Competition {
        kind: "normal-control-competition",
        normal_requests: count,
        control_requests: count,
        normal_latency_ns: latency(normal),
        control_latency_ns: latency(control),
        failures: failed,
        elapsed_ns: elapsed.as_nanos(),
        close,
    })
}

#[derive(Serialize)]
struct Matrix {
    profile: String,
    round_trip: Vec<RoundReport>,
    events: Vec<EventReport>,
    cancellation: CancelReport,
    competition: Competition,
    resource_measurement: &'static str,
    latency_measurement: &'static str,
}
async fn run() -> Result<()> {
    let profile = std::env::var("YDP_BENCH_PROFILE").unwrap_or_else(|_| "smoke".into());
    let (rounds, event_cases, cancel, comp) = match profile.as_str() {
        "smoke" => (
            vec![
                Round {
                    concurrency: 1,
                    requests: 1_000,
                    sessions: 1,
                    bytes: 32,
                },
                Round {
                    concurrency: 100,
                    requests: 2_000,
                    sessions: 10,
                    bytes: 1 << 10,
                },
            ],
            vec![
                EventCase {
                    count: 1_000,
                    bytes: 32,
                    rate: 10_000,
                    mode: Mode::None,
                },
                EventCase {
                    count: 1_000,
                    bytes: 32,
                    rate: 10_000,
                    mode: Mode::Single,
                },
                EventCase {
                    count: 1_000,
                    bytes: 32,
                    rate: 10_000,
                    mode: Mode::Multiple,
                },
                EventCase {
                    count: 128,
                    bytes: 1 << 10,
                    rate: 10_000,
                    mode: Mode::Slow,
                },
            ],
            100,
            100,
        ),
        "full" => (
            vec![
                Round {
                    concurrency: 1,
                    requests: 50_000,
                    sessions: 1,
                    bytes: 32,
                },
                Round {
                    concurrency: 100,
                    requests: 50_000,
                    sessions: 1,
                    bytes: 32,
                },
                Round {
                    concurrency: 1_000,
                    requests: 50_000,
                    sessions: 100,
                    bytes: 1 << 10,
                },
                Round {
                    concurrency: 10_000,
                    requests: 50_000,
                    sessions: 100,
                    bytes: 32,
                },
                Round {
                    concurrency: 100,
                    requests: 10_000,
                    sessions: 100,
                    bytes: 64 << 10,
                },
                Round {
                    concurrency: 16,
                    requests: 2_000,
                    sessions: 100,
                    bytes: 1 << 20,
                },
            ],
            vec![
                EventCase {
                    count: 10_000,
                    bytes: 32,
                    rate: 10_000,
                    mode: Mode::None,
                },
                EventCase {
                    count: 10_000,
                    bytes: 1 << 10,
                    rate: 10_000,
                    mode: Mode::Single,
                },
                EventCase {
                    count: 100_000,
                    bytes: 32,
                    rate: 100_000,
                    mode: Mode::Multiple,
                },
                EventCase {
                    count: 512,
                    bytes: 64 << 10,
                    rate: 10_000,
                    mode: Mode::Slow,
                },
            ],
            10_000,
            10_000,
        ),
        _ => return Err(io::Error::other("YDP_BENCH_PROFILE 只能是 smoke 或 full").into()),
    };
    let mut round_reports = Vec::new();
    for case in rounds {
        round_reports.push(round(case).await?)
    }
    let mut event_reports = Vec::new();
    for case in event_cases {
        event_reports.push(events(case).await?)
    }
    let report = Matrix {
        profile,
        round_trip: round_reports,
        events: event_reports,
        cancellation: cancellation(cancel).await?,
        competition: competition(comp).await?,
        resource_measurement: "request 高水位来自负载调度器；端点输出队列和最大响应帧为直接观测；关闭时 pending/outbound 来自 CloseReport。core 内部运行中 pending/outbox/wire 精确高水位尚无公开诊断 API，本报告不以代理值冒充内部计数。",
        latency_measurement: "round-trip 延迟从单个 Request 开始 poll 前测到 typed Response 返回；event 延迟从 Emit Request 开始 poll 前测到其响应返回，事件消费计数另列。",
    };
    println!("YDP_BENCH_REPORT={}", serde_json::to_string(&report)?);
    Ok(())
}
fn main() -> Result<()> {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?
        .block_on(run())
}
