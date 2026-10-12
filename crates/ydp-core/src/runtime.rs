use crate::raw::{ByteBudget, Frame, FrameText};
use crate::*;
use bytes::Bytes;
use futures_core::{Stream, future::FusedFuture};
use futures_util::task::AtomicWaker;
use pin_project_lite::pin_project;
use std::{
    collections::{BTreeMap, BTreeSet, VecDeque},
    future::Future,
    io,
    marker::PhantomData,
    pin::Pin,
    sync::{
        Arc, Mutex, MutexGuard, Weak,
        atomic::{AtomicU64, Ordering},
    },
    task::{Context, Poll, Waker},
    time::Duration,
};
use tokio::sync::{Notify, oneshot};
use tokio::time::{Instant, Sleep};

fn lock<T>(value: &Mutex<T>) -> MutexGuard<'_, T> {
    match value.lock() {
        Ok(g) => g,
        Err(p) => p.into_inner(),
    }
}
fn wake(wakers: Vec<Waker>) {
    for w in wakers {
        w.wake();
    }
}
static NEXT_CONNECTION: AtomicU64 = AtomicU64::new(1);
struct Route {
    key: SessionKey,
    id: Option<SessionId>,
}
struct ApiLease {
    shared: Arc<Shared>,
}
impl Drop for ApiLease {
    fn drop(&mut self) {
        self.shared
            .begin_close(Error::new(ErrorKind::ConnectionClosed));
    }
}
struct Shared {
    key: ConnectionKey,
    control: Mutex<Control>,
    outbox: Notify,
    stop: Notify,
    closed: Notify,
    options: RuntimeOptions,
    wire_budget: Arc<ByteBudget>,
    #[cfg(test)]
    task: Mutex<Option<tokio::task::JoinHandle<()>>>,
}
struct Control {
    state: ConnectionState,
    reason: Option<Error>,
    close_started: Option<Instant>,
    report: Option<CloseReport>,
    supplemental_errors: Vec<Error>,
    supplemental_seen: [bool; 2],
    pending: BTreeMap<i32, Pending>,
    queues: [VecDeque<Outbound>; 2],
    counts: [Counts; 2],
    waiters: BTreeMap<u64, Waiter>,
    fifo: [VecDeque<u64>; 2],
    next_waiter: u64,
    next_request: i64,
    routes: BTreeMap<SessionId, Arc<Route>>,
    retired: std::collections::BTreeSet<SessionId>,
    next_session: u64,
    subscriptions: BTreeMap<u64, Arc<Subscriber>>,
    all: BTreeSet<u64>,
    wild: BTreeMap<SessionKey, BTreeSet<u64>>,
    exact: BTreeMap<SessionKey, BTreeMap<&'static str, BTreeSet<u64>>>,
    active_keys: BTreeSet<SessionKey>,
    subscription_count: usize,
    next_subscription: u64,
    seq: u64,
    control_streak: usize,
    closed_pending: usize,
    closed_outbound: usize,
}
impl Control {
    fn remove_subscription(&mut self, id: u64) -> Option<Arc<Subscriber>> {
        let subscriber = self.subscriptions.remove(&id)?;
        match subscriber.route {
            None => {
                self.all.remove(&id);
            }
            Some(key) => {
                if let Some(method) = subscriber.method {
                    if let Some(methods) = self.exact.get_mut(&key) {
                        if let Some(ids) = methods.get_mut(method) {
                            ids.remove(&id);
                            if ids.is_empty() {
                                methods.remove(method);
                            }
                        }
                        if methods.is_empty() {
                            self.exact.remove(&key);
                        }
                    }
                } else if let Some(ids) = self.wild.get_mut(&key) {
                    ids.remove(&id);
                    if ids.is_empty() {
                        self.wild.remove(&key);
                    }
                }
            }
        }
        Some(subscriber)
    }
}
#[derive(Default)]
struct Counts {
    requests: usize,
    slots: usize,
    bytes: usize,
    waiters: usize,
}
struct Waiter {
    route: SessionKey,
    class: usize,
    granted: bool,
    waker: Option<Waker>,
}
struct Pending {
    route: Arc<Route>,
    sender: oneshot::Sender<Delivery>,
    permit: Permit,
}
struct Delivery {
    result: Result<RawReply>,
    _permit: Permit,
}
struct Outbound {
    bytes: Bytes,
    _permit: Permit,
}
#[derive(Clone, Copy)]
enum PermitKind {
    Request,
    Outbound,
}
struct Permit {
    shared: Weak<Shared>,
    class: usize,
    kind: PermitKind,
    bytes: usize,
}
impl Drop for Permit {
    fn drop(&mut self) {
        if let Some(s) = self.shared.upgrade() {
            let ws = {
                let mut c = s.control();
                match self.kind {
                    PermitKind::Request => c.counts[self.class].requests -= 1,
                    PermitKind::Outbound => {
                        c.counts[self.class].slots -= 1;
                        c.counts[self.class].bytes -= self.bytes;
                    }
                }
                s.grant(&mut c)
            };
            wake(ws);
        }
    }
}
struct Guard {
    shared: Weak<Shared>,
    waiter: Option<u64>,
    id: Option<i32>,
}
impl Drop for Guard {
    fn drop(&mut self) {
        if let Some(s) = self.shared.upgrade() {
            let (removed, waker, ws) = {
                let mut c = s.control();
                let removed = self.id.and_then(|id| c.pending.remove(&id));
                let mut old = None;
                if let Some(id) = self.waiter
                    && let Some(w) = c.waiters.remove(&id)
                {
                    c.counts[w.class].waiters -= 1;
                    if w.granted {
                        c.counts[w.class].requests -= 1;
                        c.counts[w.class].slots -= 1;
                    }
                    c.fifo[w.class].retain(|v| *v != id);
                    old = w.waker;
                }
                let ws = s.grant(&mut c);
                (removed, old, ws)
            };
            drop(removed);
            drop(waker);
            wake(ws);
        }
    }
}
impl Shared {
    fn control(&self) -> MutexGuard<'_, Control> {
        match self.control.lock() {
            Ok(guard) => guard,
            Err(poisoned) => {
                drop(poisoned.into_inner());
                self.control.clear_poison();
                self.begin_close(Error::new(ErrorKind::DriverFailed));
                lock(&self.control)
            }
        }
    }

    fn class(&self, i: usize) -> ClassLimits {
        if i == 0 {
            self.options.limits.normal
        } else {
            self.options.limits.control
        }
    }
    fn route_open(c: &Control, key: SessionKey) -> bool {
        key.sequence == 0 || c.active_keys.contains(&key)
    }
    fn gate(c: &Control, key: SessionKey) -> Result<()> {
        if c.state != ConnectionState::Open {
            Err(Error::new(ErrorKind::ConnectionClosed))
        } else if !Self::route_open(c, key) {
            Err(Error::new(ErrorKind::SessionClosed))
        } else {
            Ok(())
        }
    }
    fn grant(&self, c: &mut Control) -> Vec<Waker> {
        let mut ws = Vec::new();
        if c.state != ConnectionState::Open {
            return ws;
        }
        for i in 0..2 {
            let limit = self.class(i);
            while c.counts[i].requests < limit.requests && c.counts[i].slots < limit.outbound_slots
            {
                let Some(id) = c.fifo[i].pop_front() else {
                    break;
                };
                if let Some(w) = c.waiters.get_mut(&id) {
                    w.granted = true;
                    c.counts[i].requests += 1;
                    c.counts[i].slots += 1;
                    if let Some(w) = w.waker.take() {
                        ws.push(w);
                    }
                }
            }
        }
        ws
    }
    fn permits(self: &Arc<Self>, class: usize) -> (Permit, Permit) {
        (
            Permit {
                shared: Arc::downgrade(self),
                class,
                kind: PermitKind::Request,
                bytes: 0,
            },
            Permit {
                shared: Arc::downgrade(self),
                class,
                kind: PermitKind::Outbound,
                bytes: 0,
            },
        )
    }
    fn admission(
        self: &Arc<Self>,
        route: SessionKey,
        class: usize,
        guard: &mut Guard,
        cx: &Context<'_>,
    ) -> Poll<Result<(Permit, Permit)>> {
        let incoming = cx.waker().clone();
        let mut old = None;
        let result = {
            let mut c = self.control();
            match Self::gate(&c, route) {
                Err(e) => Poll::Ready(Err(e)),
                Ok(()) => {
                    if let Some(id) = guard.waiter {
                        if c.waiters.get(&id).is_some_and(|w| w.granted) {
                            let waiter = c.waiters.remove(&id);
                            c.counts[class].waiters -= 1;
                            guard.waiter = None;
                            old = waiter.and_then(|w| w.waker);
                            Poll::Ready(Ok(self.permits(class)))
                        } else if let Some(w) = c.waiters.get_mut(&id) {
                            old = w.waker.replace(incoming);
                            Poll::Pending
                        } else {
                            Poll::Ready(Err(Error::new(ErrorKind::SessionClosed)))
                        }
                    } else {
                        let limit = self.class(class);
                        if c.fifo[class].is_empty()
                            && c.counts[class].requests < limit.requests
                            && c.counts[class].slots < limit.outbound_slots
                        {
                            c.counts[class].requests += 1;
                            c.counts[class].slots += 1;
                            Poll::Ready(Ok(self.permits(class)))
                        } else if c.counts[class].waiters >= limit.admission_waiters {
                            Poll::Ready(Err(Error::new(ErrorKind::AdmissionFull)))
                        } else if let Some(id) = c.next_waiter.checked_add(1) {
                            c.next_waiter = id;
                            c.counts[class].waiters += 1;
                            c.fifo[class].push_back(id);
                            c.waiters.insert(
                                id,
                                Waiter {
                                    route,
                                    class,
                                    granted: false,
                                    waker: Some(incoming),
                                },
                            );
                            guard.waiter = Some(id);
                            Poll::Pending
                        } else {
                            Poll::Ready(Err(Error::new(ErrorKind::ResourceLimit {
                                resource: "admission_id",
                            })))
                        }
                    }
                }
            }
        };
        drop(old);
        result
    }
    fn begin_close(&self, error: Error) -> bool {
        let (pending, queues, waiters, subs) = {
            let mut c = self.control();
            if c.state != ConnectionState::Open {
                return false;
            }
            c.state = ConnectionState::Closing;
            c.reason = Some(error.clone());
            c.close_started = Some(Instant::now());
            let pending = std::mem::take(&mut c.pending);
            let queues = std::mem::take(&mut c.queues);
            let waiters = std::mem::take(&mut c.waiters);
            for w in waiters.values() {
                c.counts[w.class].waiters -= 1;
                if w.granted {
                    c.counts[w.class].requests -= 1;
                    c.counts[w.class].slots -= 1;
                }
            }
            c.fifo = [VecDeque::new(), VecDeque::new()];
            c.closed_pending = pending.len();
            c.closed_outbound = queues.iter().map(VecDeque::len).sum();
            c.all.clear();
            c.wild.clear();
            c.exact.clear();
            c.active_keys.clear();
            let subs = std::mem::take(&mut c.subscriptions);
            (pending, queues, waiters, subs)
        };
        for (_, p) in pending {
            let _ = p.sender.send(Delivery {
                result: Err(error.clone()),
                _permit: p.permit,
            });
        }
        drop(queues);
        for (_, w) in waiters {
            if let Some(w) = w.waker {
                w.wake();
            }
        }
        for (_, s) in subs {
            s.terminate(error.clone());
        }
        self.stop.notify_waiters();
        self.outbox.notify_one();
        true
    }
    fn supplement(&self, direction: usize, error: Error) {
        let mut error = Some(error);
        {
            let mut c = self.control();
            if !c.supplemental_seen[direction] {
                c.supplemental_seen[direction] = true;
                if let Some(error) = error.take() {
                    c.supplemental_errors.push(error);
                }
            }
        }
        drop(error);
    }
    fn finish(&self, end: TransportEnd) {
        let registry = {
            let mut c = self.control();
            if c.state == ConnectionState::Closed {
                return;
            }
            (
                std::mem::take(&mut c.routes),
                std::mem::take(&mut c.retired),
            )
        };
        drop(registry);
        {
            let mut c = self.control();
            let supplemental_errors = std::mem::take(&mut c.supplemental_errors);
            let report = CloseReport {
                supplemental_errors,
                reason: c
                    .reason
                    .clone()
                    .unwrap_or_else(|| Error::new(ErrorKind::DriverFailed)),
                transport_end: end,
                pending_count: c.closed_pending,
                discarded_outbound: c.closed_outbound,
                driver_resources_released: true,
            };
            c.state = ConnectionState::Closed;
            c.report = Some(report);
        }
        self.closed.notify_waiters();
    }
    async fn stopped(&self) {
        loop {
            let n = self.stop.notified();
            tokio::pin!(n);
            n.as_mut().enable();
            if self.control().state != ConnectionState::Open {
                return;
            }
            n.await;
        }
    }
    fn pop(&self) -> Option<Outbound> {
        self.pop_with_budget(None)
    }
    fn pop_with_budget(&self, remaining: Option<usize>) -> Option<Outbound> {
        let mut c = self.control();
        let normal = !c.queues[0].is_empty();
        let control = !c.queues[1].is_empty();
        let class = if control && (!normal || c.control_streak < 8) {
            1
        } else if normal {
            0
        } else {
            return None;
        };
        if remaining.is_some_and(|remaining| {
            c.queues[class]
                .front()
                .is_some_and(|frame| frame.bytes.len() > remaining)
        }) {
            return None;
        }
        c.control_streak = if class == 1 {
            (c.control_streak + 1).min(8)
        } else {
            0
        };
        c.queues[class].pop_front()
    }
}
struct BudgetWriter {
    shared: Arc<Shared>,
    permit: Permit,
    bytes: Vec<u8>,
    limit: usize,
    exceeded: bool,
}
impl io::Write for BudgetWriter {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let Some(total) = self.bytes.len().checked_add(buf.len()) else {
            self.exceeded = true;
            return Err(io::Error::other("编码预算耗尽"));
        };
        let allowed = {
            let mut c = self.shared.control();
            let count = &mut c.counts[self.permit.class];
            if total > self.limit
                || count
                    .bytes
                    .checked_add(buf.len())
                    .is_none_or(|v| v > self.shared.class(self.permit.class).outbound_bytes)
            {
                false
            } else {
                count.bytes += buf.len();
                true
            }
        };
        if !allowed {
            self.exceeded = true;
            return Err(io::Error::other("编码预算耗尽"));
        }
        self.permit.bytes += buf.len();
        self.bytes.extend_from_slice(buf);
        Ok(buf.len())
    }
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

/// 可 Clone 的连接访问句柄；最后公开访问 lease 消失会请求退出。
#[derive(Clone)]
pub struct Connection {
    lease: Arc<ApiLease>,
    root: Arc<Route>,
}
impl Connection {
    /// 接管已建立传输，同步安装初始 All 订阅，再启动唯一 Driver。
    /// 无 Tokio Handle 返回 NoRuntime；runtime 必须启用 time，否则 Tokio 可能 panic。
    /// ```no_run
    /// fn create<T: ydp_core::Transport>(transport: T) -> ydp_core::Result<()> {
    /// let (connection, events) = ydp_core::Connection::from_transport(transport, Default::default())?;
    /// drop(events); drop(connection); Ok(()) }
    /// ```
    pub fn from_transport<T: Transport>(
        transport: T,
        options: RuntimeOptions,
    ) -> Result<(Self, RawEventStream)> {
        let handle =
            tokio::runtime::Handle::try_current().map_err(|_| Error::new(ErrorKind::NoRuntime))?;
        validate_options(&options)?;
        let key = NEXT_CONNECTION
            .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |v| v.checked_add(1))
            .map(ConnectionKey)
            .map_err(|_| {
                Error::new(ErrorKind::ResourceLimit {
                    resource: "connection_key",
                })
            })?;
        let root = Arc::new(Route {
            key: SessionKey {
                connection: key,
                sequence: 0,
            },
            id: None,
        });
        let shared = Arc::new(Shared {
            key,
            #[cfg(test)]
            task: Mutex::new(None),
            wire_budget: Arc::new(ByteBudget::new(options.limits.wire_bytes)),
            control: Mutex::new(Control {
                state: ConnectionState::Open,
                reason: None,
                close_started: None,
                report: None,
                supplemental_errors: Vec::new(),
                supplemental_seen: [false; 2],
                pending: BTreeMap::new(),
                queues: [VecDeque::new(), VecDeque::new()],
                counts: [Counts::default(), Counts::default()],
                waiters: BTreeMap::new(),
                fifo: [VecDeque::new(), VecDeque::new()],
                next_waiter: 0,
                next_request: 1,
                routes: BTreeMap::new(),
                retired: Default::default(),
                next_session: 0,
                subscriptions: BTreeMap::new(),
                all: BTreeSet::new(),
                wild: BTreeMap::new(),
                exact: BTreeMap::new(),
                active_keys: BTreeSet::new(),
                subscription_count: 0,
                next_subscription: 0,
                seq: 0,
                control_streak: 0,
                closed_pending: 0,
                closed_outbound: 0,
            }),
            outbox: Notify::new(),
            stop: Notify::new(),
            closed: Notify::new(),
            options,
        });
        let connection = Self {
            lease: Arc::new(ApiLease {
                shared: shared.clone(),
            }),
            root,
        };
        let events = connection.raw_events(shared.options.initial_events)?;
        let driver = DriverGuard {
            transport: Some(transport.split()),
            outbound_batch: Vec::with_capacity(32),
            shared: shared.clone(),
            finished: false,
        };
        let task = handle.spawn(run_driver(driver));
        #[cfg(test)]
        {
            *lock(&shared.task) = Some(task);
        }
        #[cfg(not(test))]
        drop(task);
        Ok((connection, events))
    }
    /// 返回本地连接身份，不依赖 endpoint 字符串。
    /// ```no_run
    /// fn inspect(c: &ydp_core::Connection) { let _ = c.key(); }
    /// ```
    pub fn key(&self) -> ConnectionKey {
        self.root.key.connection
    }
    /// 读取本连接最近分配的入站接收序号，不消费事件，也不发送命令。
    ///
    /// 返回值在连接的短控制锁内与响应、事件的序号分配共享仲裁边界。
    /// 尚未处理任何合法入站 envelope 时返回 `None`；否则返回最近的
    /// [`ReceiveSeq`]，包括已接收但尚未被调用者消费的响应或事件。
    /// 该值只表示本地接收进度，不证明订阅已消费、上层状态已归并或远端操作完成。
    ///
    /// 并发读取可能得到相同值；在同一连接上的先后读取不会倒退。
    /// 连接关闭后保留最后值；序号耗尽不会回绕。快照之后仍可能立即收到新消息，
    /// 因此调用者必须自行将此快照与自己的观察登记放在同一同步仲裁边界，
    /// 不能将它当作跨连接或浏览器因果屏障。此方法不取得新的资源许可，
    /// 不改变 Clone、Drop 或显式关闭的所有权规则。
    ///
    /// # 示例
    ///
    /// 使用没有入站消息的内存传输验证初值和关闭后的保留行为；无需浏览器。
    /// ```
    /// # use ydp_core::{Connection, MessageReader, MessageWriter, Transport, TransportError};
    /// # struct Idle;
    /// # impl MessageReader for Idle {
    /// #     async fn recv(&mut self) -> Result<Option<bytes::Bytes>, TransportError> {
    /// #         std::future::pending().await
    /// #     }
    /// # }
    /// # impl MessageWriter for Idle {
    /// #     async fn send(&mut self, _: bytes::Bytes) -> Result<(), TransportError> { Ok(()) }
    /// #     async fn flush(&mut self) -> Result<(), TransportError> { Ok(()) }
    /// #     async fn close(&mut self) -> Result<(), TransportError> { Ok(()) }
    /// # }
    /// # impl Transport for Idle {
    /// #     type Reader = Idle;
    /// #     type Writer = Idle;
    /// #     fn split(self) -> (Idle, Idle) { (Idle, Idle) }
    /// # }
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # tokio::runtime::Builder::new_current_thread().enable_time().build()?.block_on(async {
    /// let options = ydp_core::RuntimeOptions {
    ///     close_timeout: std::time::Duration::ZERO,
    ///     ..Default::default()
    /// };
    /// let (connection, events) = Connection::from_transport(Idle, options)?;
    /// assert_eq!(connection.latest_receive_seq(), None);
    /// connection.close().await;
    /// assert_eq!(connection.latest_receive_seq(), None);
    /// drop(events);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// # }) }
    /// ```
    pub fn latest_receive_seq(&self) -> Option<ReceiveSeq> {
        let sequence = self.lease.shared.control().seq;
        (sequence != 0).then_some(ReceiveSeq(sequence))
    }
    /// 创建根路由视图；不发送协议命令，不假定端点是浏览器。
    /// ```no_run
    /// fn inspect(c: &ydp_core::Connection) { assert!(c.root_session().id().is_none()); }
    /// ```
    pub fn root_session(&self) -> Session {
        Session {
            lease: self.lease.clone(),
            route: self.root.clone(),
        }
    }
    /// 本地绑定 child ID；活跃 ID 复用记录，retired ID 拒绝，绝不自动 attach。
    /// ```no_run
    /// fn adopt(c: &ydp_core::Connection) -> ydp_core::Result<()> { let _ = c.adopt_session(ydp_core::SessionId::new("child".into())?)?; Ok(()) }
    /// ```
    pub fn adopt_session(&self, id: SessionId) -> Result<Session> {
        let s = &self.lease.shared;
        let route = {
            let mut c = s.control();
            Shared::gate(&c, self.root.key)?;
            if let Some(r) = c.routes.get(&id) {
                r.clone()
            } else {
                if c.retired.contains(&id) {
                    return Err(Error::new(ErrorKind::SessionIdRetired));
                }
                if c.routes.len() >= s.options.limits.active_sessions
                    || c.routes.len() + c.retired.len() >= s.options.limits.session_records
                {
                    return Err(Error::new(ErrorKind::ResourceLimit {
                        resource: "session_records",
                    }));
                }
                let sequence = c.next_session.checked_add(1).ok_or_else(|| {
                    Error::new(ErrorKind::ResourceLimit {
                        resource: "session_key",
                    })
                })?;
                c.next_session = sequence;
                let r = Arc::new(Route {
                    key: SessionKey {
                        connection: self.key(),
                        sequence,
                    },
                    id: Some(id.clone()),
                });
                c.active_keys.insert(r.key);
                c.routes.insert(id, r.clone());
                r
            }
        };
        Ok(Session {
            lease: self.lease.clone(),
            route,
        })
    }
    /// 同步安装 All 订阅，覆盖未知 child；返回前即注册，不回放旧事件。
    /// ```no_run
    /// fn subscribe(c: &ydp_core::Connection) -> ydp_core::Result<()> { let _events = c.raw_events(Default::default())?; Ok(()) }
    /// ```
    pub fn raw_events(&self, options: SubscriptionOptions) -> Result<RawEventStream> {
        subscribe(self.lease.clone(), None, None, options)
    }
    /// 返回本地生命周期快照。
    /// ```no_run
    /// fn inspect(c: &ydp_core::Connection) { let _ = c.state(); }
    /// ```
    pub fn state(&self) -> ConnectionState {
        self.lease.shared.control().state
    }
    /// 首次 poll 关门并加入共享退出；丢弃本 Future 不撤销退出。
    /// ```no_run
    /// async fn close(c: &ydp_core::Connection) { let report = c.close().await; assert!(report.driver_resources_released); }
    /// ```
    pub async fn close(&self) -> CloseReport {
        self.lease
            .shared
            .begin_close(Error::new(ErrorKind::ConnectionClosed));
        self.closed().await
    }
    /// 等待 Driver 释放资源，不主动发起关闭；报告不含用户仍持有的数据。
    /// ```no_run
    /// async fn observe(c: &ydp_core::Connection) { let _ = c.closed().await; }
    /// ```
    pub async fn closed(&self) -> CloseReport {
        loop {
            let n = self.lease.shared.closed.notified();
            tokio::pin!(n);
            n.as_mut().enable();
            if let Some(r) = self.lease.shared.control().report.clone() {
                return r;
            }
            n.await;
        }
    }
}
/// 在连接建立前验证资源范围，不发生 IO；例如 `validate_options(&RuntimeOptions::default())` 成功，模块测试覆盖零容量拒绝。
pub(crate) fn validate_options(o: &RuntimeOptions) -> Result<()> {
    let l = &o.limits;
    let classes = [l.normal, l.control];
    if Instant::now().checked_add(o.close_timeout).is_none()
        || o.initial_events.max_events == 0
        || o.initial_events.max_bytes == 0
        || l.max_message_bytes == 0
        || l.max_message_bytes > 128 << 20
        || l.wire_bytes == 0
        || l.max_json_depth == 0
        || l.max_json_depth > 128
        || l.subscriptions == 0
        || l.active_sessions > l.session_records
        || classes.iter().any(|c| {
            c.requests == 0
                || c.outbound_slots == 0
                || c.max_command_bytes == 0
                || c.max_command_bytes > c.outbound_bytes
                || c.requests.checked_add(c.outbound_slots).is_none()
        })
    {
        Err(Error::new(ErrorKind::InvalidArgument))
    } else {
        Ok(())
    }
}

/// 只管理本地 flattened 路由的廉价 Clone 句柄；Drop 不自动 detach。
#[derive(Clone)]
pub struct Session {
    lease: Arc<ApiLease>,
    route: Arc<Route>,
}
impl Session {
    /// 返回连接与本地不可重用会话序号。
    /// ```no_run
    /// fn inspect(s: &ydp_core::Session) { let _ = s.key(); }
    /// ```
    pub fn key(&self) -> SessionKey {
        self.route.key
    }
    /// 借用 child wire ID；根返回 None。
    /// ```no_run
    /// fn inspect(s: &ydp_core::Session) { let _ = s.id(); }
    /// ```
    pub fn id(&self) -> Option<&SessionId> {
        self.route.id.as_ref()
    }
    /// 查看本地 gate；不证明远端资源已经释放。
    /// ```no_run
    /// fn inspect(s: &ydp_core::Session) { let _ = s.is_closed(); }
    /// ```
    pub fn is_closed(&self) -> bool {
        Shared::gate(&self.lease.shared.control(), self.key()).is_err()
    }
    /// 同步终结 child 的本地请求和订阅，不发 detach；根发起连接退出。
    /// 已发布帧仍可能发出；必须由上层管理远端语义责任。
    /// ```no_run
    /// fn close(s: &ydp_core::Session) { s.close(); assert!(s.is_closed()); }
    /// ```
    pub fn close(&self) {
        let s = &self.lease.shared;
        if self.id().is_none() {
            s.begin_close(Error::new(ErrorKind::ConnectionClosed));
            return;
        }
        let (pending, waiters, subs, removed, ws) = {
            let mut c = s.control();
            let Some(id) = self.id() else { return };
            let removed = c.routes.remove(id);
            if removed.is_none() {
                return;
            }
            c.active_keys.remove(&self.key());
            c.retired.insert(id.clone());
            let ids: Vec<_> = c
                .pending
                .iter()
                .filter(|(_, p)| p.route.key == self.key())
                .map(|(id, _)| *id)
                .collect();
            let pending: Vec<_> = ids
                .into_iter()
                .filter_map(|id| c.pending.remove(&id))
                .collect();
            let ids: Vec<_> = c
                .waiters
                .iter()
                .filter(|(_, w)| w.route == self.key())
                .map(|(id, _)| *id)
                .collect();
            let mut waiters = Vec::new();
            for id in ids {
                if let Some(w) = c.waiters.remove(&id) {
                    c.counts[w.class].waiters -= 1;
                    if w.granted {
                        c.counts[w.class].requests -= 1;
                        c.counts[w.class].slots -= 1;
                    }
                    c.fifo[w.class].retain(|v| *v != id);
                    waiters.push(w);
                }
            }
            let ids: Vec<_> = c
                .subscriptions
                .iter()
                .filter(|(_, v)| v.route == Some(self.key()))
                .map(|(id, _)| *id)
                .collect();
            let subs: Vec<_> = ids
                .into_iter()
                .filter_map(|id| c.remove_subscription(id))
                .collect();
            let ws = s.grant(&mut c);
            (pending, waiters, subs, removed, ws)
        };
        drop(removed);
        for p in pending {
            let _ = p.sender.send(Delivery {
                result: Err(Error::new(ErrorKind::SessionClosed)),
                _permit: p.permit,
            });
        }
        for w in waiters {
            if let Some(w) = w.waker {
                w.wake();
            }
        }
        for sub in subs {
            sub.terminate(Error::new(ErrorKind::SessionClosed));
        }
        wake(ws);
    }
    /// 创建拥有参数的冷请求；首次 poll 前没有编码、ID 或网络副作用。
    /// ```no_run
    /// fn send<C: ydp_core::Command>(s: &ydp_core::Session, params: C::Params) { let _request = s.send::<C>(params); }
    /// ```
    pub fn send<C: Command>(&self, params: C::Params) -> Request<C> {
        self.send_with::<C>(params, RequestOptions::default())
    }
    /// 使用指定期限和独立流量类别创建同一种冷请求；不重试。
    /// ```no_run
    /// fn send<C: ydp_core::Command>(s: &ydp_core::Session, params: C::Params) { let _request = s.send_with::<C>(params, Default::default()); }
    /// ```
    pub fn send_with<C: Command>(&self, params: C::Params, options: RequestOptions) -> Request<C> {
        Request {
            inner: RequestBody::new(
                self.clone(),
                std::borrow::Cow::Borrowed(C::METHOD),
                params,
                options,
            ),
            marker: PhantomData,
        }
    }
    /// 创建动态方法请求；参数在首次 poll 校验为 JSON object。
    /// ```no_run
    /// fn send(s: &ydp_core::Session) -> ydp_core::Result<()> { let params=ydp_core::RawJson::from_bytes(bytes::Bytes::from_static(b"{}"))?; let _request=s.send_raw(ydp_core::MethodName::new("Echo.echo".into())?,params,Default::default()); Ok(()) }
    /// ```
    pub fn send_raw(
        &self,
        method: MethodName,
        params: RawJson,
        options: RequestOptions,
    ) -> RawRequest {
        RawRequest {
            inner: RequestBody::new(
                self.clone(),
                std::borrow::Cow::Owned(method.as_str().to_owned()),
                params,
                options,
            ),
        }
    }
    /// 同步安装精确 Session 与 METHOD 订阅；消费时才解码，不自动 enable。
    /// ```no_run
    /// fn subscribe<E: ydp_core::Event>(s: &ydp_core::Session) -> ydp_core::Result<()> { let _events=s.events::<E>(Default::default())?; Ok(()) }
    /// ```
    pub fn events<E: Event>(&self, options: SubscriptionOptions) -> Result<EventStream<E>> {
        Ok(EventStream {
            raw: subscribe(
                self.lease.clone(),
                Some(self.key()),
                Some(E::METHOD),
                options,
            )?,
            key: self.key(),
            marker: PhantomData,
        })
    }
    /// 同步安装精确本地 Session 的所有方法订阅。
    /// ```no_run
    /// fn subscribe(s: &ydp_core::Session) -> ydp_core::Result<()> { let _events=s.raw_events(Default::default())?; Ok(()) }
    /// ```
    pub fn raw_events(&self, options: SubscriptionOptions) -> Result<RawEventStream> {
        subscribe(self.lease.clone(), Some(self.key()), None, options)
    }
}

struct RequestCore<P> {
    session: Option<Session>,
    key: SessionKey,
    method: std::borrow::Cow<'static, str>,
    params: Option<P>,
    options: RequestOptions,
    guard: Option<Guard>,
    receiver: Option<oneshot::Receiver<Delivery>>,
    deadline: Option<Instant>,
    started: bool,
    done: bool,
    submission: Submission,
    id: Option<RequestId>,
    seq: Option<ReceiveSeq>,
}
pin_project! {struct RequestBody<P> {core:RequestCore<P>,#[pin] timer:Option<Sleep>}}
impl<P> RequestBody<P> {
    fn new(
        session: Session,
        method: std::borrow::Cow<'static, str>,
        params: P,
        options: RequestOptions,
    ) -> Self {
        let key = session.key();
        let guard = Guard {
            shared: Arc::downgrade(&session.lease.shared),
            waiter: None,
            id: None,
        };
        Self {
            core: RequestCore {
                session: Some(session),
                key,
                method,
                params: Some(params),
                options,
                guard: Some(guard),
                receiver: None,
                deadline: None,
                started: false,
                done: false,
                submission: Submission::NotSubmitted,
                id: None,
                seq: None,
            },
            timer: None,
        }
    }
}
impl<P> RequestCore<P> {
    fn context(&self) -> RequestContext {
        RequestContext {
            method: self.method.to_string(),
            request_id: self.id,
            session_key: self.key,
            submission: self.submission,
            receive_seq: self.seq,
        }
    }
    fn error(&self, mut e: Error) -> Error {
        e.context = Some(Box::new(self.context()));
        e
    }
    fn expired(&self) -> bool {
        self.deadline.is_some_and(|d| Instant::now() >= d)
    }
    fn finish(&mut self) {
        self.done = true;
        self.guard.take();
        self.receiver.take();
        self.params.take();
        self.session.take();
    }
}
impl<P: serde::Serialize> RequestBody<P> {
    fn poll_delivery(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<Delivery>> {
        let mut this = self.project();
        let c = this.core;
        if c.done {
            return Poll::Pending;
        }
        if !c.started {
            c.started = true;
            c.deadline = match c.options.timeout {
                Timeout::Unlimited => None,
                Timeout::After(d) => match Instant::now().checked_add(d) {
                    Some(deadline) => Some(deadline),
                    None => {
                        let e = c.error(Error::new(ErrorKind::InvalidArgument));
                        c.finish();
                        return Poll::Ready(Err(e));
                    }
                },
                Timeout::At(d) => Some(d),
            };
            if let Some(d) = c.deadline {
                this.timer.set(Some(tokio::time::sleep_until(d)));
            }
        }
        if c.expired() {
            let e = c.error(Error::new(ErrorKind::Timeout));
            c.finish();
            return Poll::Ready(Err(e));
        }
        if let Some(timer) = this.timer.as_mut().as_pin_mut() {
            let _ = timer.poll(cx);
        }
        if c.receiver.is_none() {
            let Some(session) = c.session.as_ref() else {
                return Poll::Pending;
            };
            let shared = session.lease.shared.clone();
            let route = session.route.clone();
            let class = c.options.traffic_class.index();
            let Some(guard) = c.guard.as_mut() else {
                return Poll::Pending;
            };
            let (request_permit, outbound_permit) = match shared.admission(c.key, class, guard, cx)
            {
                Poll::Pending => return Poll::Pending,
                Poll::Ready(Err(e)) => {
                    let e = c.error(e);
                    c.finish();
                    return Poll::Ready(Err(e));
                }
                Poll::Ready(Ok(p)) => p,
            };
            let id = {
                let mut control = shared.control();
                if control.next_request > i64::from(i32::MAX) {
                    None
                } else {
                    let id = control.next_request as i32;
                    control.next_request += 1;
                    Some(id)
                }
            };
            let Some(id) = id else {
                let e = c.error(Error::new(ErrorKind::IdExhausted));
                c.finish();
                return Poll::Ready(Err(e));
            };
            c.id = Some(RequestId(id));
            let mut writer = BudgetWriter {
                shared: shared.clone(),
                permit: outbound_permit,
                bytes: Vec::new(),
                limit: shared.class(class).max_command_bytes,
                exceeded: false,
            };
            let Some(params) = c.params.take() else {
                return Poll::Pending;
            };
            let encoded = crate::wire::encode(
                id,
                &c.method,
                route.id.as_ref().map(SessionId::as_str),
                &params,
                &mut writer,
            );
            drop(params);
            if let Err(e) = encoded {
                let e = if writer.exceeded {
                    Error::new(ErrorKind::ResourceLimit {
                        resource: "outbound_bytes",
                    })
                } else {
                    e
                };
                let e = c.error(e);
                c.finish();
                return Poll::Ready(Err(e));
            }
            if c.expired() {
                let e = c.error(Error::new(ErrorKind::Timeout));
                c.finish();
                return Poll::Ready(Err(e));
            }
            let (sender, receiver) = oneshot::channel();
            let mut pending = Some(Pending {
                route,
                sender,
                permit: request_permit,
            });
            let mut outbound = Some(Outbound {
                bytes: Bytes::from(writer.bytes),
                _permit: writer.permit,
            });
            let result = {
                let mut control = shared.control();
                Shared::gate(&control, c.key).and_then(|()| {
                    if c.expired() {
                        Err(Error::new(ErrorKind::Timeout))
                    } else {
                        if let Some(p) = pending.take() {
                            control.pending.insert(id, p);
                        }
                        if let Some(o) = outbound.take() {
                            control.queues[class].push_back(o);
                        }
                        c.submission = Submission::PossiblySent;
                        if let Some(g) = c.guard.as_mut() {
                            g.id = Some(id);
                        }
                        Ok(())
                    }
                })
            };
            drop(pending);
            drop(outbound);
            if let Err(e) = result {
                let e = c.error(e);
                c.finish();
                return Poll::Ready(Err(e));
            }
            c.receiver = Some(receiver);
            shared.outbox.notify_one();
        }
        let Some(receiver) = c.receiver.as_mut() else {
            return Poll::Pending;
        };
        match Pin::new(receiver).poll(cx) {
            Poll::Pending => Poll::Pending,
            Poll::Ready(Err(_)) => {
                let e = c.error(Error::new(ErrorKind::DriverFailed));
                c.finish();
                Poll::Ready(Err(e))
            }
            Poll::Ready(Ok(delivery)) => {
                if let Ok(reply) = &delivery.result {
                    c.seq = Some(reply.receive_seq);
                    c.submission = Submission::Responded;
                } else if let Err(e) = &delivery.result
                    && let Some(context) = &e.context
                    && context.receive_seq.is_some()
                {
                    c.seq = context.receive_seq;
                    c.submission = context.submission;
                }
                c.finish();
                Poll::Ready(Ok(delivery))
            }
        }
    }
}
pin_project! {
    /// 拥有参数的冷 typed Future；完成后再次 poll 返回 Pending，Drop 仅取消本地等待。
    #[must_use = "请求必须被 poll 才会提交"]
    pub struct Request<C:Command> {#[pin] inner:RequestBody<C::Params>,marker:PhantomData<fn()->C>}
}
impl<C: Command> Request<C> {
    /// 返回保守发送证据；PossiblySent 不允许据此自动重放。
    /// ```no_run
    /// fn inspect<C: ydp_core::Command>(r: &ydp_core::Request<C>) { let _ = r.submission(); }
    /// ```
    pub fn submission(&self) -> Submission {
        self.inner.core.submission
    }
    /// 返回已分配 ID；冷请求为 None，编码失败可能留下合法 ID 空洞。
    /// ```no_run
    /// fn inspect<C: ydp_core::Command>(r: &ydp_core::Request<C>) { let _ = r.request_id(); }
    /// ```
    pub fn request_id(&self) -> Option<RequestId> {
        self.inner.core.id
    }
}
impl<C: Command> Future for Request<C> {
    type Output = Result<C::Response>;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut this = self.project();
        match this.inner.as_mut().poll_delivery(cx) {
            Poll::Pending => Poll::Pending,
            Poll::Ready(Err(e)) => Poll::Ready(Err(e)),
            Poll::Ready(Ok(delivery)) => {
                let Delivery { result, _permit } = delivery;
                let result = result.and_then(|reply| reply.result.decode());
                let core = &this.inner.as_ref().get_ref().core;
                let result = if core.expired() {
                    Err(Error::new(ErrorKind::Timeout))
                } else {
                    result
                };
                Poll::Ready(result.map_err(|e| core.error(e)))
            }
        }
    }
}
impl<C: Command> FusedFuture for Request<C> {
    fn is_terminated(&self) -> bool {
        self.inner.core.done
    }
}
pin_project! {
    /// 动态方法的拥有型冷 Future，与 typed Request 共享准入和取消路径。
    #[must_use = "请求必须被 poll 才会提交"]
    pub struct RawRequest {#[pin] inner:RequestBody<RawJson>}
}
impl RawRequest {
    /// 返回保守发送证据；不表示远端执行状态。
    /// ```no_run
    /// fn inspect(r: &ydp_core::RawRequest) { let _ = r.submission(); }
    /// ```
    pub fn submission(&self) -> Submission {
        self.inner.core.submission
    }
    /// 返回已分配的请求 ID，首次 poll 前为 None。
    /// ```no_run
    /// fn inspect(r: &ydp_core::RawRequest) { let _ = r.request_id(); }
    /// ```
    pub fn request_id(&self) -> Option<RequestId> {
        self.inner.core.id
    }
}
impl Future for RawRequest {
    type Output = Result<RawReply>;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut this = self.project();
        match this.inner.as_mut().poll_delivery(cx) {
            Poll::Pending => Poll::Pending,
            Poll::Ready(Err(e)) => Poll::Ready(Err(e)),
            Poll::Ready(Ok(d)) => Poll::Ready(
                d.result
                    .map_err(|e| this.inner.as_ref().get_ref().core.error(e)),
            ),
        }
    }
}
impl FusedFuture for RawRequest {
    fn is_terminated(&self) -> bool {
        self.inner.core.done
    }
}
/// 原始成功响应；结果共享 backing frame，Drop 最后引用才释放 wire 配额。
#[derive(Clone, Debug)]
pub struct RawReply {
    /// 关联的请求 ID。
    pub request_id: RequestId,
    /// 已登记的本地会话身份。
    pub session_key: SessionKey,
    /// 该响应的连接接收序号。
    pub receive_seq: ReceiveSeq,
    /// 原始结果 object。
    pub result: RawJson,
}
/// 已绑定本地 Session 的 typed 事件及连接接收顺序。
#[derive(Debug)]
pub struct EventEnvelope<T> {
    /// 精确订阅时的本地路由身份。
    pub session_key: SessionKey,
    /// 原始 envelope 的本地接收序号。
    pub receive_seq: ReceiveSeq,
    /// 在消费者任务解码的拥有型数据。
    pub params: T,
}
#[derive(Clone)]
enum MethodText {
    Shared(FrameText),
    Owned(Arc<str>),
}
impl MethodText {
    fn as_str(&self) -> &str {
        match self {
            Self::Shared(text) => text.as_str(),
            Self::Owned(text) => text,
        }
    }
}
/// 共享原始事件，保留未知 child 来源而不伪造 SessionKey。
#[derive(Clone)]
pub struct RawEvent {
    connection: ConnectionKey,
    session: Option<SessionId>,
    seq: ReceiveSeq,
    method: MethodText,
    params: RawJson,
    frame_len: usize,
}
impl std::fmt::Debug for RawEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RawEvent")
            .field("connection", &self.connection)
            .field("session", &self.session)
            .field("seq", &self.seq)
            .field("method", &self.method.as_str())
            .field("frame_len", &self.frame_len)
            .finish_non_exhaustive()
    }
}
impl RawEvent {
    /// 返回来源连接身份。
    /// ```no_run
    /// fn inspect(e: &ydp_core::RawEvent) { let _ = e.connection_key(); }
    /// ```
    pub fn connection_key(&self) -> ConnectionKey {
        self.connection
    }
    /// 借用原始 child 路由 ID；未知 child 仍保留，根为 None。
    /// ```no_run
    /// fn inspect(e: &ydp_core::RawEvent) { let _ = e.session_id(); }
    /// ```
    pub fn session_id(&self) -> Option<&SessionId> {
        self.session.as_ref()
    }
    /// 返回连接内接收序；事件流允许响应造成的空洞。
    /// ```no_run
    /// fn inspect(e: &ydp_core::RawEvent) { let _ = e.receive_seq(); }
    /// ```
    pub fn receive_seq(&self) -> ReceiveSeq {
        self.seq
    }
    /// 借用已正确反转义的完整方法名。
    /// ```no_run
    /// fn inspect(e: &ydp_core::RawEvent) { let _ = e.method(); }
    /// ```
    pub fn method(&self) -> &str {
        self.method.as_str()
    }
    /// 借用共享参数 object；省略 params 时为同 backing 生命周期的空 object。
    /// ```no_run
    /// fn inspect(e: &ydp_core::RawEvent) { let _ = e.params().as_bytes(); }
    /// ```
    pub fn params(&self) -> &RawJson {
        &self.params
    }
}
struct Subscriber {
    shared: Weak<Shared>,
    route: Option<SessionKey>,
    method: Option<&'static str>,
    options: SubscriptionOptions,
    state: Mutex<SubState>,
    waker: AtomicWaker,
}
struct SubState {
    queue: VecDeque<RawEvent>,
    bytes: usize,
    terminal: Option<Error>,
    ended: bool,
}
impl Subscriber {
    fn state(&self) -> MutexGuard<'_, SubState> {
        match self.state.lock() {
            Ok(guard) => guard,
            Err(poisoned) => {
                drop(poisoned.into_inner());
                self.state.clear_poison();
                if let Some(shared) = self.shared.upgrade() {
                    shared.begin_close(Error::new(ErrorKind::DriverFailed));
                }
                self.terminate(Error::new(ErrorKind::DriverFailed));
                lock(&self.state)
            }
        }
    }

    fn terminate(&self, error: Error) {
        let should_wake = {
            let mut s = self.state();
            if s.ended || s.terminal.is_some() {
                false
            } else {
                s.terminal = Some(error);
                true
            }
        };
        if should_wake {
            self.waker.wake();
        }
    }
    fn deliver(&self, event: RawEvent) -> bool {
        let mut owned = Some(event);
        let live = {
            let mut s = self.state();
            if s.ended || s.terminal.is_some() {
                false
            } else {
                let len = owned.as_ref().map_or(0, |e| e.frame_len);
                if s.queue.len() >= self.options.max_events
                    || s.bytes
                        .checked_add(len)
                        .is_none_or(|v| v > self.options.max_bytes)
                {
                    s.terminal = Some(Error::new(ErrorKind::EventBufferFull));
                    false
                } else {
                    s.bytes += len;
                    if let Some(event) = owned.take() {
                        s.queue.push_back(event);
                    }
                    true
                }
            }
        };
        drop(owned);
        self.waker.wake();
        live
    }
    fn discard(&self) {
        let queue = {
            let mut s = self.state();
            s.ended = true;
            s.bytes = 0;
            std::mem::take(&mut s.queue)
        };
        drop(queue);
    }
}
fn subscribe(
    lease: Arc<ApiLease>,
    route: Option<SessionKey>,
    method: Option<&'static str>,
    options: SubscriptionOptions,
) -> Result<RawEventStream> {
    if options.max_events == 0 || options.max_bytes == 0 {
        return Err(Error::new(ErrorKind::InvalidArgument));
    }
    let subscriber = Arc::new(Subscriber {
        shared: Arc::downgrade(&lease.shared),
        route,
        method,
        options,
        state: Mutex::new(SubState {
            queue: VecDeque::new(),
            bytes: 0,
            terminal: None,
            ended: false,
        }),
        waker: AtomicWaker::new(),
    });
    let id = {
        let shared = &lease.shared;
        let mut c = shared.control();
        if c.state != ConnectionState::Open {
            return Err(Error::new(ErrorKind::ConnectionClosed));
        }
        if let Some(key) = route {
            Shared::gate(&c, key)?;
        }
        if c.subscription_count >= shared.options.limits.subscriptions {
            return Err(Error::new(ErrorKind::ResourceLimit {
                resource: "subscriptions",
            }));
        }
        let id = c.next_subscription.checked_add(1).ok_or_else(|| {
            Error::new(ErrorKind::ResourceLimit {
                resource: "subscription_id",
            })
        })?;
        c.next_subscription = id;
        c.subscription_count += 1;
        match route {
            None => {
                c.all.insert(id);
            }
            Some(key) => {
                if let Some(method) = method {
                    c.exact
                        .entry(key)
                        .or_default()
                        .entry(method)
                        .or_default()
                        .insert(id);
                } else {
                    c.wild.entry(key).or_default().insert(id);
                }
            }
        }
        c.subscriptions.insert(id, subscriber.clone());
        id
    };
    Ok(RawEventStream {
        lease,
        subscriber,
        id,
    })
}
/// 有界原始事件订阅；终态优先于积压数据，Drop 静默退订自身。
pub struct RawEventStream {
    lease: Arc<ApiLease>,
    subscriber: Arc<Subscriber>,
    id: u64,
}
impl Drop for RawEventStream {
    fn drop(&mut self) {
        let removed = {
            let mut c = self.lease.shared.control();
            c.subscription_count -= 1;
            c.remove_subscription(self.id)
        };
        self.subscriber.discard();
        drop(removed);
    }
}
impl Stream for RawEventStream {
    type Item = Result<RawEvent>;
    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = self.get_mut();
        this.subscriber.waker.register(cx.waker());
        let (result, discarded) = {
            let mut s = this.subscriber.state();
            if s.ended {
                (Poll::Ready(None), None)
            } else if let Some(error) = s.terminal.take() {
                s.ended = true;
                s.bytes = 0;
                (
                    Poll::Ready(Some(Err(error))),
                    Some(std::mem::take(&mut s.queue)),
                )
            } else if let Some(event) = s.queue.pop_front() {
                s.bytes -= event.frame_len;
                (Poll::Ready(Some(Ok(event))), None)
            } else {
                (Poll::Pending, None)
            }
        };
        drop(discarded);
        result
    }
}
/// 精确 typed 订阅；解码失败仅终结当前流，不影响其他 raw/typed 消费者。
pub struct EventStream<E: Event> {
    raw: RawEventStream,
    key: SessionKey,
    marker: PhantomData<fn() -> E>,
}
impl<E: Event> Stream for EventStream<E> {
    type Item = Result<EventEnvelope<E::Params>>;
    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = self.get_mut();
        match Pin::new(&mut this.raw).poll_next(cx) {
            Poll::Pending => Poll::Pending,
            Poll::Ready(None) => Poll::Ready(None),
            Poll::Ready(Some(Err(e))) => Poll::Ready(Some(Err(e))),
            Poll::Ready(Some(Ok(event))) => match event.params.decode() {
                Ok(params) => Poll::Ready(Some(Ok(EventEnvelope {
                    session_key: this.key,
                    receive_seq: event.seq,
                    params,
                }))),
                Err(mut e) => {
                    e.context = Some(Box::new(RequestContext {
                        method: event.method.as_str().to_owned(),
                        request_id: None,
                        session_key: this.key,
                        submission: Submission::Responded,
                        receive_seq: Some(event.seq),
                    }));
                    this.raw.subscriber.discard();
                    let removed = this
                        .raw
                        .lease
                        .shared
                        .control()
                        .remove_subscription(this.raw.id);
                    drop(removed);
                    Poll::Ready(Some(Err(e)))
                }
            },
        }
    }
}

struct DriverGuard<R: MessageReader, W: MessageWriter> {
    transport: Option<(R, W)>,
    outbound_batch: Vec<Outbound>,
    shared: Arc<Shared>,
    finished: bool,
}
impl<R: MessageReader, W: MessageWriter> Drop for DriverGuard<R, W> {
    fn drop(&mut self) {
        drop(self.transport.take());
        self.outbound_batch.clear();
        if !self.finished {
            self.shared.begin_close(Error::new(ErrorKind::DriverFailed));
            self.shared.finish(TransportEnd::Failed);
        }
    }
}
async fn run_driver<R: MessageReader, W: MessageWriter>(mut guard: DriverGuard<R, W>) {
    let shared = guard.shared.clone();
    let failure = {
        let Some((reader, writer)) = guard.transport.as_mut() else {
            return;
        };
        let read = read_loop(reader, &shared);
        let write = write_loop(writer, &shared, &mut guard.outbound_batch);
        tokio::pin!(read, write);
        tokio::select! {r=&mut read=>Some((0,r)),r=&mut write=>Some((1,r)),()=shared.stopped()=>None}
    };
    if let Some((direction, error)) = failure
        && !shared.begin_close(error.clone())
        && matches!(error.kind, ErrorKind::Transport)
    {
        shared.supplement(direction, error);
    }
    let deadline = {
        let c = shared.control();
        c.close_started
            .and_then(|t| t.checked_add(shared.options.close_timeout))
            .unwrap_or_else(Instant::now)
    };
    let end = if let Some((reader, writer)) = guard.transport.as_mut() {
        let close = async {
            let read = async {
                let mut count = 0usize;
                let mut since = Instant::now();
                loop {
                    match reader.recv().await {
                        Ok(Some(_)) => {}
                        Ok(None) => return true,
                        Err(error) => {
                            shared.supplement(0, Error::with_source(ErrorKind::Transport, error));
                            return false;
                        }
                    }
                    count += 1;
                    if count >= 256 || since.elapsed() >= Duration::from_millis(2) {
                        tokio::task::yield_now().await;
                        count = 0;
                        since = Instant::now();
                    }
                }
            };
            let write = async {
                match writer.close().await {
                    Ok(()) => true,
                    Err(error) => {
                        shared.supplement(1, Error::with_source(ErrorKind::Transport, error));
                        false
                    }
                }
            };
            let (read_ok, write_ok) = tokio::join!(read, write);
            read_ok && write_ok
        };
        match tokio::time::timeout_at(deadline, close).await {
            Ok(true) => TransportEnd::Graceful,
            Ok(false) => TransportEnd::Failed,
            Err(_) => TransportEnd::Aborted,
        }
    } else {
        TransportEnd::Failed
    };
    drop(guard.transport.take());
    guard.outbound_batch.clear();
    shared.finish(end);
    guard.finished = true;
}
async fn read_loop<R: MessageReader>(reader: &mut R, shared: &Arc<Shared>) -> Error {
    let mut processed = 0usize;
    let mut since = Instant::now();
    loop {
        let bytes = match reader.recv().await {
            Ok(Some(b)) => b,
            Ok(None) => return Error::new(ErrorKind::ConnectionClosed),
            Err(e) => return Error::with_source(ErrorKind::Transport, e),
        };
        if let Err(e) = receive(shared, bytes) {
            return e;
        }
        processed += 1;
        if processed >= 256 || since.elapsed() >= Duration::from_millis(2) {
            tokio::task::yield_now().await;
            processed = 0;
            since = Instant::now();
        }
    }
}
async fn write_loop<W: MessageWriter>(
    writer: &mut W,
    shared: &Arc<Shared>,
    batch: &mut Vec<Outbound>,
) -> Error {
    let mut processed = 0usize;
    let mut since = Instant::now();
    loop {
        let notification = shared.outbox.notified();
        tokio::pin!(notification);
        notification.as_mut().enable();
        let Some(first) = shared.pop() else {
            notification.await;
            continue;
        };
        let mut size = 0;
        let mut next = Some(first);
        while let Some(frame) = next {
            size += frame.bytes.len();
            let bytes = frame.bytes.clone();
            batch.push(frame);
            if let Err(e) = writer.send(bytes).await {
                return Error::with_source(ErrorKind::Transport, e);
            }
            if batch.len() >= 32 || size >= 64 << 10 {
                break;
            }
            next = shared.pop_with_budget(Some((64 << 10) - size));
        }
        if let Err(e) = writer.flush().await {
            return Error::with_source(ErrorKind::Transport, e);
        }
        processed += batch.len();
        batch.clear();
        if processed >= 256 || since.elapsed() >= Duration::from_millis(2) {
            tokio::task::yield_now().await;
            processed = 0;
            since = Instant::now();
        }
    }
}
fn receive(shared: &Arc<Shared>, bytes: Bytes) -> Result<()> {
    let envelope = crate::wire::parse(
        &bytes,
        shared.options.limits.max_message_bytes,
        shared.options.limits.max_json_depth,
    )?;
    let (seq, pending, response_session_mismatch, targets, connection, known_session) = {
        let mut c = shared.control();
        if c.state != ConnectionState::Open {
            return Ok(());
        }
        c.seq = c.seq.checked_add(1).ok_or_else(|| {
            Error::new(ErrorKind::ResourceLimit {
                resource: "receive_seq",
            })
        })?;
        let seq = ReceiveSeq(c.seq);
        if let Some(id) = envelope.id {
            let pending = c.pending.remove(&id);
            let response_session_mismatch = pending.as_ref().is_some_and(|pending| {
                pending.route.id.as_ref().map(SessionId::as_str) != envelope.session_id.as_deref()
            });
            (
                seq,
                pending,
                response_session_mismatch,
                Vec::new(),
                None,
                None,
            )
        } else {
            let session = envelope
                .session_id
                .as_deref()
                .and_then(|id| c.routes.get(id))
                .map(|r| (r.id.clone(), r.key));
            let key = if envelope.session_id.is_none() {
                Some(SessionKey {
                    connection: shared.key,
                    sequence: 0,
                })
            } else {
                session.as_ref().map(|(_, k)| *k)
            };
            let mut ids = c.all.iter().copied().collect::<Vec<_>>();
            if let Some(key) = key {
                if let Some(wild) = c.wild.get(&key) {
                    ids.extend(wild.iter().copied());
                }
                if let Some(method) = envelope.method.as_deref()
                    && let Some(exact) = c.exact.get(&key).and_then(|methods| methods.get(method))
                {
                    ids.extend(exact.iter().copied());
                }
            }
            let targets = ids
                .into_iter()
                .filter_map(|id| c.subscriptions.get(&id).map(|s| (id, s.clone())))
                .collect::<Vec<_>>();
            let connection = targets
                .first()
                .and_then(|(_, s)| s.route.map(|k| k.connection));
            (
                seq,
                None,
                false,
                targets,
                connection,
                session.and_then(|(id, _)| id),
            )
        }
    };
    if let Some(pending) = pending {
        if response_session_mismatch {
            // request ID 在连接内唯一，足以安全终结原 pending；错路由 payload 仍完全拒绝，
            // 但 Chromium 在 child 退役后可能把迟到响应放到根 envelope，不能因此破坏其他路由。
            let mut error = Error::new(ErrorKind::ResponseSessionMismatch);
            error.context = Some(Box::new(RequestContext {
                method: String::new(),
                request_id: envelope.id.map(RequestId),
                session_key: pending.route.key,
                submission: Submission::PossiblySent,
                receive_seq: Some(seq),
            }));
            let _ = pending.sender.send(Delivery {
                result: Err(error),
                _permit: pending.permit,
            });
            return Ok(());
        }
        let prepared: Result<Result<RawReply>> = (|| {
            let frame = Frame::new(bytes.clone(), shared.wire_budget.clone())?;
            if let Some(result) = envelope.result {
                Ok(Ok(RawReply {
                    request_id: RequestId(
                        envelope
                            .id
                            .ok_or_else(|| Error::new(ErrorKind::InvalidMessage))?,
                    ),
                    session_key: pending.route.key,
                    receive_seq: seq,
                    result: frame.json(result)?,
                }))
            } else if let Some(error) = envelope.error {
                let mut error = Error::new(ErrorKind::Protocol {
                    code: error.code,
                    message: error.message.into_owned(),
                    data: error.data.map(|raw| frame.json(raw)).transpose()?,
                });
                error.retain_frame(frame);
                error.context = Some(Box::new(RequestContext {
                    method: String::new(),
                    request_id: envelope.id.map(RequestId),
                    session_key: pending.route.key,
                    submission: Submission::Responded,
                    receive_seq: Some(seq),
                }));
                Ok(Err(error))
            } else {
                Err(Error::new(ErrorKind::InvalidMessage))
            }
        })();
        let (result, failure) = match prepared {
            Ok(result) => (result, None),
            Err(error) => (Err(error.clone()), Some(error)),
        };
        let _ = pending.sender.send(Delivery {
            result,
            _permit: pending.permit,
        });
        if let Some(error) = failure {
            return Err(error);
        }
    } else if !targets.is_empty() {
        let frame = Frame::new(bytes.clone(), shared.wire_budget.clone())?;
        let params = match envelope.params {
            Some(raw) => frame.json(raw)?,
            None => frame.empty_json(),
        };
        let session = match known_session {
            Some(id) => Some(id),
            None => envelope
                .session_id
                .map(|id| SessionId::new(id.into_owned()))
                .transpose()?,
        };
        let connection = connection.unwrap_or(shared.key);
        let event = RawEvent {
            connection,
            session,
            seq,
            method: match envelope.method {
                Some(std::borrow::Cow::Borrowed(text)) => MethodText::Shared(frame.text(text)?),
                Some(std::borrow::Cow::Owned(text)) => MethodText::Owned(text.into()),
                None => return Err(Error::new(ErrorKind::InvalidMessage)),
            },
            params,
            frame_len: frame.len(),
        };
        for (id, target) in targets {
            if !target.deliver(event.clone()) {
                let removed = shared.control().remove_subscription(id);
                drop(removed);
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures_util::{StreamExt, poll};
    use std::sync::atomic::AtomicUsize;

    struct IdleReader(Arc<AtomicUsize>);
    struct IdleWriter(Arc<AtomicUsize>);
    struct IdleTransport(Arc<AtomicUsize>);
    impl Drop for IdleReader {
        fn drop(&mut self) {
            self.0.fetch_add(1, Ordering::SeqCst);
        }
    }
    impl Drop for IdleWriter {
        fn drop(&mut self) {
            self.0.fetch_add(1, Ordering::SeqCst);
        }
    }
    impl Transport for IdleTransport {
        type Reader = IdleReader;
        type Writer = IdleWriter;
        fn split(self) -> (Self::Reader, Self::Writer) {
            (IdleReader(self.0.clone()), IdleWriter(self.0))
        }
    }
    impl MessageReader for IdleReader {
        async fn recv(&mut self) -> std::result::Result<Option<Bytes>, TransportError> {
            std::future::pending().await
        }
    }
    impl MessageWriter for IdleWriter {
        async fn send(&mut self, _: Bytes) -> std::result::Result<(), TransportError> {
            Ok(())
        }
        async fn flush(&mut self) -> std::result::Result<(), TransportError> {
            Ok(())
        }
        async fn close(&mut self) -> std::result::Result<(), TransportError> {
            Ok(())
        }
    }
    #[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq)]
    struct Payload {
        value: u32,
    }
    struct Echo;
    impl Command for Echo {
        type Params = Payload;
        type Response = Payload;
        const METHOD: &'static str = "Echo.echo";
    }
    struct Changed;
    impl Event for Changed {
        type Params = Payload;
        const METHOD: &'static str = "Echo.changed";
    }
    fn fixture(mut options: RuntimeOptions) -> (Connection, RawEventStream, Arc<AtomicUsize>) {
        options.close_timeout = Duration::ZERO;
        let drops = Arc::new(AtomicUsize::new(0));
        let (c, e) = Connection::from_transport(IdleTransport(drops.clone()), options).unwrap();
        (c, e, drops)
    }
    fn response(shared: &Arc<Shared>, id: i32, value: u32) {
        receive(
            shared,
            Bytes::from(format!("{{\"id\":{id},\"result\":{{\"value\":{value}}}}}")),
        )
        .unwrap();
    }
    async fn joined(c: &Connection) {
        let report = c.close().await;
        assert!(report.driver_resources_released);
        let task = lock(&c.lease.shared.task).take().unwrap();
        task.await.unwrap();
    }

    async fn latest_receive_sequence_contract() {
        let (connection, mut events, _) = fixture(Default::default());
        assert_eq!(connection.latest_receive_seq(), None);
        let mut request = Box::pin(connection.root_session().send::<Echo>(Payload { value: 7 }));
        assert!(poll!(&mut request).is_pending());
        assert_eq!(connection.latest_receive_seq(), None);
        receive(
            &connection.lease.shared,
            Bytes::from_static(br#"{"method":"Echo.changed","params":{"value":1}}"#),
        )
        .unwrap();
        // 尚未消费的事件已经进入接收基线；出站命令不会推进它。
        assert_eq!(connection.latest_receive_seq().unwrap().get(), 1);
        response(
            &connection.lease.shared,
            request.request_id().unwrap().get(),
            7,
        );
        // 响应已经排队但原 Request 尚未 poll，基线仍必须包含该响应。
        assert_eq!(connection.latest_receive_seq().unwrap().get(), 2);
        assert_eq!(events.next().await.unwrap().unwrap().receive_seq().get(), 1);
        assert_eq!(connection.latest_receive_seq().unwrap().get(), 2);
        assert_eq!(request.await.unwrap().value, 7);
        // 无人等待的晚到响应也占一个合法接收序号。
        response(&connection.lease.shared, 12345, 8);
        assert_eq!(connection.latest_receive_seq().unwrap().get(), 3);
        assert!(receive(&connection.lease.shared, Bytes::from_static(b"null")).is_err());
        assert_eq!(connection.latest_receive_seq().unwrap().get(), 3);
        joined(&connection).await;
        assert_eq!(connection.latest_receive_seq().unwrap().get(), 3);
        receive(
            &connection.lease.shared,
            Bytes::from_static(br#"{"method":"Echo.changed"}"#),
        )
        .unwrap();
        assert_eq!(connection.latest_receive_seq().unwrap().get(), 3);
    }

    #[tokio::test]
    async fn latest_receive_sequence_current_thread() {
        latest_receive_sequence_contract().await;
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn latest_receive_sequence_multi_thread() {
        latest_receive_sequence_contract().await;
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn latest_receive_sequence_concurrent_snapshot_and_exhaustion() {
        let (connection, events, _) = fixture(Default::default());
        let barrier = Arc::new(tokio::sync::Barrier::new(2));
        let observer = connection.clone();
        let observer_barrier = barrier.clone();
        let task = tokio::spawn(async move {
            for expected in 1..=32 {
                observer_barrier.wait().await;
                assert_eq!(observer.latest_receive_seq().unwrap().get(), expected);
                observer_barrier.wait().await;
            }
        });
        for _ in 1..=32 {
            receive(
                &connection.lease.shared,
                Bytes::from_static(br#"{"method":"Echo.changed"}"#),
            )
            .unwrap();
            barrier.wait().await;
            barrier.wait().await;
        }
        task.await.unwrap();
        connection.lease.shared.control().seq = u64::MAX - 1;
        receive(
            &connection.lease.shared,
            Bytes::from_static(br#"{"method":"Echo.changed"}"#),
        )
        .unwrap();
        assert_eq!(connection.latest_receive_seq().unwrap().get(), u64::MAX);
        let error = receive(
            &connection.lease.shared,
            Bytes::from_static(br#"{"method":"Echo.changed"}"#),
        )
        .unwrap_err();
        assert!(matches!(
            error.kind,
            ErrorKind::ResourceLimit {
                resource: "receive_seq"
            }
        ));
        assert_eq!(connection.latest_receive_seq().unwrap().get(), u64::MAX);
        connection.lease.shared.begin_close(error);
        joined(&connection).await;
        assert_eq!(connection.latest_receive_seq().unwrap().get(), u64::MAX);
        drop(events);
    }

    #[tokio::test]
    async fn response_owner_survives_remove_and_receiver_drop() {
        let (c, events, drops) = fixture(Default::default());
        drop(events);
        let s = c.root_session();
        let shared = &c.lease.shared;
        let mut request = Box::pin(s.send::<Echo>(Payload { value: 1 }));
        assert!(poll!(&mut request).is_pending());
        let id = request.request_id().unwrap().get();
        let outbound = shared.pop().unwrap();
        response(shared, id, 7);
        assert!(shared.control().pending.is_empty());
        assert_eq!(shared.control().counts[0].requests, 1);
        assert!(shared.wire_budget.used() > 0);
        drop(request);
        assert_eq!(shared.control().counts[0].requests, 0);
        assert_eq!(shared.wire_budget.used(), 0);
        assert_eq!(shared.control().counts[0].slots, 1);
        drop(outbound);
        assert_eq!(shared.control().counts[0].slots, 0);
        joined(&c).await;
        assert_eq!(drops.load(Ordering::SeqCst), 2);
    }
    #[tokio::test]
    async fn out_of_order_duplicate_and_unknown_responses_transfer_each_request_once() {
        let (c, events, _) = fixture(Default::default());
        drop(events);
        let shared = &c.lease.shared;
        let mut first = Box::pin(c.root_session().send::<Echo>(Payload { value: 1 }));
        let mut second = Box::pin(c.root_session().send::<Echo>(Payload { value: 2 }));
        let mut third = Box::pin(c.root_session().send::<Echo>(Payload { value: 3 }));
        assert!(poll!(&mut first).is_pending());
        assert!(poll!(&mut second).is_pending());
        assert!(poll!(&mut third).is_pending());
        let first_id = first.request_id().unwrap().get();
        let second_id = second.request_id().unwrap().get();
        let third_id = third.request_id().unwrap().get();

        response(shared, second_id, 20);
        response(shared, 987_654, 99);
        response(shared, first_id, 10);
        response(shared, second_id, 200);
        response(shared, third_id, 30);

        assert_eq!(first.await.unwrap().value, 10);
        assert_eq!(second.await.unwrap().value, 20);
        assert_eq!(third.await.unwrap().value, 30);
        assert!(shared.control().pending.is_empty());
        assert_eq!(shared.control().counts[0].requests, 0);
        assert_eq!(c.latest_receive_seq().unwrap().get(), 5);
        joined(&c).await;
    }
    #[tokio::test]
    async fn cancellation_at_cold_admission_publish_and_delivery_boundaries_releases_owners() {
        let mut options = RuntimeOptions::default();
        options.limits.normal.requests = 1;
        options.limits.normal.outbound_slots = 2;
        let (c, events, _) = fixture(options);
        drop(events);
        let shared = &c.lease.shared;

        let cold = c.root_session().send::<Echo>(Payload { value: 0 });
        drop(cold);
        assert_eq!(shared.control().counts[0].requests, 0);

        let mut published = Box::pin(c.root_session().send::<Echo>(Payload { value: 1 }));
        assert!(poll!(&mut published).is_pending());
        let mut admission = Box::pin(c.root_session().send::<Echo>(Payload { value: 2 }));
        assert!(poll!(&mut admission).is_pending());
        assert_eq!(shared.control().counts[0].waiters, 1);
        drop(admission);
        assert_eq!(shared.control().counts[0].waiters, 0);

        drop(published);
        assert!(shared.control().pending.is_empty());
        assert_eq!(shared.control().counts[0].requests, 0);
        assert_eq!(shared.control().counts[0].slots, 1);
        drop(shared.pop());
        assert_eq!(shared.control().counts[0].slots, 0);
        assert_eq!(shared.control().counts[0].bytes, 0);

        let mut delivered = Box::pin(c.root_session().send::<Echo>(Payload { value: 3 }));
        assert!(poll!(&mut delivered).is_pending());
        response(shared, delivered.request_id().unwrap().get(), 30);
        assert!(shared.control().pending.is_empty());
        assert_eq!(shared.control().counts[0].requests, 1);
        assert!(shared.wire_budget.used() > 0);
        drop(delivered);
        assert_eq!(shared.control().counts[0].requests, 0);
        assert_eq!(shared.wire_budget.used(), 0);
        joined(&c).await;
    }
    #[tokio::test(start_paused = true)]
    async fn queued_response_loses_to_deadline_and_fuses() {
        let (c, events, _) = fixture(Default::default());
        drop(events);
        let s = c.root_session();
        let mut request = Box::pin(s.send_with::<Echo>(
            Payload { value: 1 },
            RequestOptions {
                timeout: Timeout::After(Duration::from_secs(1)),
                ..Default::default()
            },
        ));
        assert!(poll!(&mut request).is_pending());
        response(&c.lease.shared, request.request_id().unwrap().get(), 2);
        tokio::time::advance(Duration::from_secs(1)).await;
        let error = request.as_mut().await.unwrap_err();
        assert!(matches!(error.kind, ErrorKind::Timeout));
        assert_eq!(error.context.unwrap().submission, Submission::PossiblySent);
        assert!(request.is_terminated());
        assert!(poll!(&mut request).is_pending());
        assert_eq!(c.lease.shared.control().counts[0].requests, 0);
        joined(&c).await;
    }
    #[tokio::test]
    async fn grants_reserve_capacity_without_polling_the_head() {
        let mut options = RuntimeOptions::default();
        options.limits.normal.requests = 2;
        options.limits.normal.outbound_slots = 2;
        options.limits.normal.admission_waiters = 3;
        let (c, events, _) = fixture(options);
        drop(events);
        let s = c.root_session();
        let shared = &c.lease.shared;
        let mut active1 = Box::pin(s.send::<Echo>(Payload { value: 1 }));
        let mut active2 = Box::pin(s.send::<Echo>(Payload { value: 2 }));
        assert!(poll!(&mut active1).is_pending());
        assert!(poll!(&mut active2).is_pending());
        let mut first = Box::pin(s.send::<Echo>(Payload { value: 3 }));
        let mut second = Box::pin(s.send::<Echo>(Payload { value: 4 }));
        let mut third = Box::pin(s.send::<Echo>(Payload { value: 5 }));
        assert!(poll!(&mut first).is_pending());
        assert!(poll!(&mut second).is_pending());
        assert!(poll!(&mut third).is_pending());
        let mut full = Box::pin(s.send::<Echo>(Payload { value: 6 }));
        assert!(matches!(
            poll!(&mut full),
            Poll::Ready(Err(Error {
                kind: ErrorKind::AdmissionFull,
                ..
            }))
        ));
        drop(active1);
        drop(active2);
        drop(shared.pop());
        drop(shared.pop());
        assert_eq!(
            shared
                .control()
                .waiters
                .values()
                .filter(|w| w.granted)
                .count(),
            2
        );
        assert!(poll!(&mut second).is_pending());
        let id = second.request_id().unwrap().get();
        response(shared, id, 4);
        assert_eq!(second.await.unwrap().value, 4);
        drop(shared.pop());
        assert!(shared.control().waiters.values().all(|w| w.granted));
        assert!(poll!(&mut third).is_pending());
        assert!(first.request_id().is_none());
        drop(first);
        drop(third);
        joined(&c).await;
    }
    #[tokio::test]
    async fn response_budget_failure_reaches_removed_sender() {
        let mut options = RuntimeOptions::default();
        options.limits.wire_bytes = 1;
        let (c, events, _) = fixture(options);
        drop(events);
        let mut request = Box::pin(c.root_session().send::<Echo>(Payload { value: 1 }));
        assert!(poll!(&mut request).is_pending());
        let error = receive(
            &c.lease.shared,
            Bytes::from(format!(
                "{{\"id\":{},\"result\":{{}}}}",
                request.request_id().unwrap().get()
            )),
        )
        .unwrap_err();
        assert!(matches!(error.kind, ErrorKind::ResourceLimit { .. }));
        c.lease.shared.begin_close(error);
        assert!(matches!(
            request.await.unwrap_err().kind,
            ErrorKind::ResourceLimit { .. }
        ));
        joined(&c).await;
    }
    #[tokio::test]
    async fn sessions_gate_retire_and_leave_published_frames() {
        let (c, events, _) = fixture(Default::default());
        drop(events);
        let id = SessionId::new("child".into()).unwrap();
        let child = c.adopt_session(id.clone()).unwrap();
        assert_eq!(child.key(), c.adopt_session(id.clone()).unwrap().key());
        let mut request = Box::pin(child.send::<Echo>(Payload { value: 1 }));
        assert!(poll!(&mut request).is_pending());
        child.close();
        assert!(matches!(
            request.await.unwrap_err().kind,
            ErrorKind::SessionClosed
        ));
        assert!(c.lease.shared.pop().is_some());
        assert!(matches!(
            c.adopt_session(id).err().unwrap().kind,
            ErrorKind::SessionIdRetired
        ));
        assert!(matches!(
            child
                .send::<Echo>(Payload { value: 1 })
                .await
                .unwrap_err()
                .kind,
            ErrorKind::SessionClosed
        ));
        assert!(!c.root_session().is_closed());
        joined(&c).await;
    }
    #[tokio::test]
    async fn local_session_close_has_no_detach_side_effect_and_preserves_published_frame() {
        let (c, events, _) = fixture(Default::default());
        drop(events);
        let child = c
            .adopt_session(SessionId::new("child-local-close".into()).unwrap())
            .unwrap();
        let mut request = Box::pin(child.send::<Echo>(Payload { value: 1 }));
        assert!(poll!(&mut request).is_pending());
        let before = {
            let control = c.lease.shared.control();
            assert_eq!(control.queues[0].len(), 1);
            control.queues[0].front().unwrap().bytes.clone()
        };

        child.close();

        assert!(matches!(
            request.await.unwrap_err().kind,
            ErrorKind::SessionClosed
        ));
        let after = {
            let control = c.lease.shared.control();
            assert_eq!(control.queues[0].len(), 1);
            control.queues[0].front().unwrap().bytes.clone()
        };
        assert_eq!(after, before);
        let wire = std::str::from_utf8(&after).unwrap();
        assert!(wire.contains("Echo.echo"));
        assert!(!wire.contains("detach"));
        assert_eq!(c.state(), ConnectionState::Open);
        joined(&c).await;
    }
    #[tokio::test]
    async fn connection_close_arbitrates_publish_response_and_registration_orders() {
        // 先关闭的 gate 同时拒绝请求、Session 和订阅，迟到响应不再推进接收序号。
        let (closed_first, mut initial, _) = fixture(Default::default());
        joined(&closed_first).await;
        assert!(matches!(
            closed_first
                .root_session()
                .send::<Echo>(Payload { value: 1 })
                .await
                .unwrap_err()
                .kind,
            ErrorKind::ConnectionClosed
        ));
        let subscription_error = closed_first.raw_events(Default::default()).err().unwrap();
        assert!(matches!(
            subscription_error.kind,
            ErrorKind::ConnectionClosed
        ));
        let adoption_error = closed_first
            .adopt_session(SessionId::new("after-close".into()).unwrap())
            .err()
            .unwrap();
        assert!(matches!(adoption_error.kind, ErrorKind::ConnectionClosed));
        receive(
            &closed_first.lease.shared,
            Bytes::from_static(br#"{"id":1,"result":{"value":1}}"#),
        )
        .unwrap();
        assert_eq!(closed_first.latest_receive_seq(), None);
        assert!(matches!(
            initial.next().await.unwrap().unwrap_err().kind,
            ErrorKind::ConnectionClosed
        ));

        // 发布先赢得仲裁时，关闭终结 pending 并丢弃尚未写出的帧，许可全部归还。
        let (published_first, events, _) = fixture(Default::default());
        drop(events);
        let mut published = Box::pin(
            published_first
                .root_session()
                .send::<Echo>(Payload { value: 2 }),
        );
        assert!(poll!(&mut published).is_pending());
        assert_eq!(published.submission(), Submission::PossiblySent);
        joined(&published_first).await;
        assert!(matches!(
            published.await.unwrap_err().kind,
            ErrorKind::ConnectionClosed
        ));
        {
            let control = published_first.lease.shared.control();
            assert!(control.pending.is_empty());
            assert!(control.queues.iter().all(VecDeque::is_empty));
            assert!(control.counts.iter().all(|count| {
                count.requests == 0 && count.slots == 0 && count.bytes == 0 && count.waiters == 0
            }));
        }

        // 响应先完成所有权移交时，随后关闭不能覆盖已经排队的成功结果。
        let (responded_first, events, _) = fixture(Default::default());
        drop(events);
        let mut responded = Box::pin(
            responded_first
                .root_session()
                .send::<Echo>(Payload { value: 3 }),
        );
        assert!(poll!(&mut responded).is_pending());
        response(
            &responded_first.lease.shared,
            responded.request_id().unwrap().get(),
            30,
        );
        joined(&responded_first).await;
        assert_eq!(responded.await.unwrap().value, 30);
    }
    #[tokio::test]
    async fn overflow_terminal_has_priority_and_holds_subscription_slot() {
        let mut options = RuntimeOptions::default();
        options.initial_events.max_events = 1;
        options.limits.subscriptions = 2;
        let (c, mut all, _) = fixture(options);
        let mut typed = c
            .root_session()
            .events::<Changed>(Default::default())
            .unwrap();
        receive(
            &c.lease.shared,
            Bytes::from_static(br#"{"method":"Echo.changed","params":{"value":1}}"#),
        )
        .unwrap();
        receive(
            &c.lease.shared,
            Bytes::from_static(br#"{"method":"Echo.changed","params":{"value":2}}"#),
        )
        .unwrap();
        assert!(matches!(
            all.next().await.unwrap().unwrap_err().kind,
            ErrorKind::EventBufferFull
        ));
        assert!(all.next().await.is_none());
        assert!(c.raw_events(Default::default()).is_err());
        assert_eq!(typed.next().await.unwrap().unwrap().params.value, 1);
        assert_eq!(typed.next().await.unwrap().unwrap().params.value, 2);
        drop(all);
        assert!(c.raw_events(Default::default()).is_ok());
        joined(&c).await;
    }
    #[tokio::test]
    async fn typed_decode_terminates_only_its_own_stream() {
        let (c, mut all, _) = fixture(Default::default());
        let mut typed = c
            .root_session()
            .events::<Changed>(Default::default())
            .unwrap();
        receive(
            &c.lease.shared,
            Bytes::from_static(br#"{"method":"Echo.changed","params":{"value":"bad"}}"#),
        )
        .unwrap();
        assert!(matches!(
            typed.next().await.unwrap().unwrap_err().kind,
            ErrorKind::Decode
        ));
        assert!(typed.next().await.is_none());
        assert_eq!(all.next().await.unwrap().unwrap().connection_key(), c.key());
        assert_eq!(c.state(), ConnectionState::Open);
        joined(&c).await;
    }
    #[tokio::test]
    async fn registration_completion_delivers_event_or_explicit_terminal_without_lost_wakeup() {
        let (c, mut source, _) = fixture(Default::default());
        receive(
            &c.lease.shared,
            Bytes::from_static(br#"{"method":"Echo.changed","params":{"value":7}}"#),
        )
        .unwrap();
        let event = source.next().await.unwrap().unwrap();

        // 路由快照先于注册取得锁时，新订阅尚不在旧快照中；注册返回后再路由必须可见。
        let (mut delivered, old_snapshot) = {
            let barrier = Arc::new(std::sync::Barrier::new(2));
            let control = c.lease.shared.control();
            let session = c.root_session();
            let register_barrier = barrier.clone();
            let register = std::thread::spawn(move || {
                register_barrier.wait();
                session.raw_events(Default::default()).unwrap()
            });
            barrier.wait();
            let old_snapshot = control
                .all
                .iter()
                .filter_map(|id| control.subscriptions.get(id).cloned())
                .collect::<Vec<_>>();
            drop(control);
            (register.join().unwrap(), old_snapshot)
        };
        for target in old_snapshot {
            target.deliver(event.clone());
        }
        assert!(poll!(delivered.next()).is_pending());
        receive(
            &c.lease.shared,
            Bytes::from_static(br#"{"method":"Echo.changed","params":{"value":9}}"#),
        )
        .unwrap();
        assert_eq!(
            delivered.next().await.unwrap().unwrap().params().as_bytes(),
            br#"{"value":9}"#
        );

        let mut terminated = c.root_session().raw_events(Default::default()).unwrap();
        assert!(poll!(terminated.next()).is_pending());
        terminated
            .subscriber
            .terminate(Error::new(ErrorKind::SessionClosed));
        assert!(matches!(
            terminated.next().await.unwrap().unwrap_err().kind,
            ErrorKind::SessionClosed
        ));
        joined(&c).await;
    }
    #[tokio::test]
    async fn stale_delivery_snapshot_cannot_requeue_after_subscription_removal() {
        let (c, mut source, _) = fixture(Default::default());
        receive(
            &c.lease.shared,
            Bytes::from_static(br#"{"method":"Echo.changed","params":{"value":8}}"#),
        )
        .unwrap();
        let event = source.next().await.unwrap().unwrap();

        // 先移除索引再执行已取得的快照：discard gate 必须拒绝迟到投递。
        let removed_first = c.root_session().raw_events(Default::default()).unwrap();
        let stale = removed_first.subscriber.clone();
        drop(removed_first);
        assert!(!stale.deliver(event.clone()));
        {
            let state = stale.state();
            assert!(state.ended);
            assert!(state.queue.is_empty());
        }

        // 快照先看到终态再移除索引：终态同样不能被迟到投递覆盖。
        let terminal_first = c.root_session().raw_events(Default::default()).unwrap();
        let stale = terminal_first.subscriber.clone();
        stale.terminate(Error::new(ErrorKind::SessionClosed));
        assert!(!stale.deliver(event));
        drop(terminal_first);
        {
            let state = stale.state();
            assert!(state.ended);
            assert!(state.queue.is_empty());
            assert!(state.terminal.is_some());
        }
        joined(&c).await;
    }
    #[tokio::test]
    async fn unknown_large_response_only_advances_envelope_sequence() {
        let (c, events, _) = fixture(Default::default());
        drop(events);
        let payload = "x".repeat(4 << 20);
        receive(
            &c.lease.shared,
            Bytes::from(format!(
                "{{\"id\":987654,\"result\":{{\"payload\":\"{payload}\"}}}}"
            )),
        )
        .unwrap();
        assert_eq!(c.latest_receive_seq().unwrap().get(), 1);
        assert_eq!(c.lease.shared.wire_budget.used(), 0);
        assert!(c.lease.shared.control().pending.is_empty());
        joined(&c).await;
    }
    #[tokio::test]
    async fn last_lease_and_abort_release_transport_before_report() {
        let (c, events, drops) = fixture(Default::default());
        let shared = c.lease.shared.clone();
        let task = lock(&shared.task).take().unwrap();
        drop(events);
        drop(c);
        task.await.unwrap();
        assert_eq!(drops.load(Ordering::SeqCst), 2);
        assert_eq!(shared.control().state, ConnectionState::Closed);
        let (unpolled, unpolled_events, unpolled_drops) = fixture(Default::default());
        let task = lock(&unpolled.lease.shared.task).take().unwrap();
        task.abort();
        assert!(task.await.unwrap_err().is_cancelled());
        assert!(matches!(
            unpolled.closed().await.reason.kind,
            ErrorKind::DriverFailed
        ));
        assert_eq!(unpolled_drops.load(Ordering::SeqCst), 2);
        drop(unpolled_events);
        let (c, events, drops) = fixture(Default::default());
        tokio::task::yield_now().await;
        let task = lock(&c.lease.shared.task).take().unwrap();
        task.abort();
        assert!(task.await.unwrap_err().is_cancelled());
        let report = c.closed().await;
        assert!(matches!(report.reason.kind, ErrorKind::DriverFailed));
        assert_eq!(drops.load(Ordering::SeqCst), 2);
        drop(events);
    }
    #[tokio::test]
    async fn poisoned_control_converges_instead_of_reopening() {
        let (c, events, _) = fixture(Default::default());
        let shared = c.lease.shared.clone();
        let _ = std::panic::catch_unwind(|| {
            let _guard = shared.control.lock().unwrap();
            panic!("故障注入");
        });
        assert_eq!(c.state(), ConnectionState::Closing);
        let report = c.close().await;
        assert!(matches!(report.reason.kind, ErrorKind::DriverFailed));
        drop(events);
    }
    #[tokio::test]
    async fn id_sequence_exhaustion_keeps_existing_request() {
        let (c, events, _) = fixture(Default::default());
        drop(events);
        let shared = &c.lease.shared;
        shared.control().next_request = i64::from(i32::MAX);
        let mut last = Box::pin(c.root_session().send::<Echo>(Payload { value: 1 }));
        assert!(poll!(&mut last).is_pending());
        assert_eq!(last.request_id().unwrap().get(), i32::MAX);
        assert!(matches!(
            c.root_session()
                .send::<Echo>(Payload { value: 2 })
                .await
                .unwrap_err()
                .kind,
            ErrorKind::IdExhausted
        ));
        drop(last);
        shared.control().seq = u64::MAX;
        assert!(matches!(
            receive(shared, Bytes::from_static(br#"{"method":"Echo.changed"}"#))
                .unwrap_err()
                .kind,
            ErrorKind::ResourceLimit {
                resource: "receive_seq"
            }
        ));
        joined(&c).await;
    }

    async fn response_session_mismatch_retirement_matrix() {
        let (c, events, _) = fixture(Default::default());
        drop(events);
        let child = c
            .adopt_session(SessionId::new("child".into()).unwrap())
            .unwrap();

        // Driver 先读到迟到响应时，上层还来不及根据 detach 关闭 Session；只终结原请求。
        let mut request = Box::pin(child.send::<Echo>(Payload { value: 1 }));
        assert!(poll!(&mut request).is_pending());
        let request_id = request.request_id().unwrap();
        receive(
            &c.lease.shared,
            Bytes::from(format!(
                "{{\"id\":{},\"result\":{{\"value\":999}}}}",
                request_id.get()
            )),
        )
        .unwrap();
        let error = request.await.unwrap_err();
        assert!(matches!(error.kind, ErrorKind::ResponseSessionMismatch));
        let context = error.context.expect("错路由请求保留结构化上下文");
        assert_eq!(context.request_id, Some(request_id));
        assert_eq!(context.session_key, child.key());
        assert_eq!(context.submission, Submission::PossiblySent);
        assert_eq!(context.receive_seq, Some(ReceiveSeq(1)));
        assert_eq!(c.state(), ConnectionState::Open);
        assert!(c.lease.shared.control().pending.is_empty());

        // 上层先处理 detach 时，Session::close 已移除 pending；更晚的未知响应同样不能破坏连接。
        let mut closing_request = Box::pin(child.send::<Echo>(Payload { value: 2 }));
        assert!(poll!(&mut closing_request).is_pending());
        let closing_id = closing_request.request_id().unwrap();
        child.close();
        let error = closing_request.await.unwrap_err();
        assert!(matches!(error.kind, ErrorKind::SessionClosed));
        receive(
            &c.lease.shared,
            Bytes::from(format!(
                "{{\"id\":{},\"result\":{{\"value\":2}}}}",
                closing_id.get()
            )),
        )
        .unwrap();
        assert_eq!(c.state(), ConnectionState::Open);

        // 两种退役顺序之后，独立根路由仍可发送并接受严格匹配响应。
        let mut root = Box::pin(c.root_session().send::<Echo>(Payload { value: 3 }));
        assert!(poll!(&mut root).is_pending());
        let root_id = root.request_id().unwrap();
        receive(
            &c.lease.shared,
            Bytes::from(format!(
                "{{\"id\":{},\"result\":{{\"value\":3}}}}",
                root_id.get()
            )),
        )
        .unwrap();
        assert_eq!(root.await.unwrap(), Payload { value: 3 });
        joined(&c).await;
    }

    #[tokio::test(flavor = "current_thread")]
    async fn response_session_mismatch_retirement_current_thread() {
        response_session_mismatch_retirement_matrix().await;
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn response_session_mismatch_retirement_multi_thread() {
        response_session_mismatch_retirement_matrix().await;
    }
    #[derive(Default)]
    struct WriteLog {
        sent: Vec<Bytes>,
        flushes: Vec<(usize, usize)>,
        batch_frames: usize,
        batch_bytes: usize,
    }
    struct LoggedWriter {
        log: Arc<Mutex<WriteLog>>,
        flush_gate: Arc<tokio::sync::Semaphore>,
    }
    impl MessageWriter for LoggedWriter {
        async fn send(&mut self, bytes: Bytes) -> std::result::Result<(), TransportError> {
            let mut log = lock(&self.log);
            log.batch_frames += 1;
            log.batch_bytes += bytes.len();
            log.sent.push(bytes);
            Ok(())
        }
        async fn flush(&mut self) -> std::result::Result<(), TransportError> {
            {
                let mut log = lock(&self.log);
                let entry = (log.batch_frames, log.batch_bytes);
                log.flushes.push(entry);
                log.batch_frames = 0;
                log.batch_bytes = 0;
            }
            self.flush_gate.acquire().await.unwrap().forget();
            Ok(())
        }
        async fn close(&mut self) -> std::result::Result<(), TransportError> {
            Ok(())
        }
    }
    fn queue_raw(c: &Connection, size: usize, class: TrafficClass) -> Pin<Box<RawRequest>> {
        let params = RawJson::from_bytes(Bytes::from(format!(
            "{{\"text\":\"{}\"}}",
            "x".repeat(size)
        )))
        .unwrap();
        let mut request = Box::pin(c.root_session().send_raw(
            MethodName::new("Echo.raw".into()).unwrap(),
            params,
            RequestOptions {
                traffic_class: class,
                ..Default::default()
            },
        ));
        let mut cx = Context::from_waker(Waker::noop());
        assert!(request.as_mut().poll(&mut cx).is_pending());
        request
    }
    #[tokio::test]
    async fn writer_flush_owns_slots_and_obeys_frame_and_byte_limits() {
        let mut options = RuntimeOptions::default();
        options.limits.normal.outbound_slots = 64;
        let (c, events, _) = fixture(options);
        drop(events);
        let shared = c.lease.shared.clone();
        let requests = (0..33)
            .map(|_| queue_raw(&c, 1, TrafficClass::Normal))
            .collect::<Vec<_>>();
        let log = Arc::new(Mutex::new(WriteLog::default()));
        let gate = Arc::new(tokio::sync::Semaphore::new(0));
        let mut writer = LoggedWriter {
            log: log.clone(),
            flush_gate: gate.clone(),
        };
        {
            let mut batch = Vec::new();
            let mut writing = Box::pin(write_loop(&mut writer, &shared, &mut batch));
            assert!(poll!(&mut writing).is_pending());
            assert_eq!(lock(&log).flushes[0].0, 32);
            assert_eq!(shared.control().counts[0].slots, 33);
            gate.add_permits(1);
            assert!(poll!(&mut writing).is_pending());
            assert_eq!(lock(&log).flushes[1].0, 1);
            assert_eq!(shared.control().counts[0].slots, 1);
            gate.add_permits(1);
            assert!(poll!(&mut writing).is_pending());
            assert_eq!(shared.control().counts[0].slots, 0);
        }
        drop(requests);
        let requests = vec![
            queue_raw(&c, 40 << 10, TrafficClass::Normal),
            queue_raw(&c, 40 << 10, TrafficClass::Normal),
            queue_raw(&c, 70 << 10, TrafficClass::Normal),
            queue_raw(&c, 1, TrafficClass::Normal),
        ];
        *lock(&log) = WriteLog::default();
        {
            let mut batch = Vec::new();
            let mut writing = Box::pin(write_loop(&mut writer, &shared, &mut batch));
            for expected in 1..=4 {
                assert!(poll!(&mut writing).is_pending());
                let log = lock(&log);
                assert_eq!(log.flushes.len(), expected);
                assert_eq!(log.flushes[expected - 1].0, 1);
                assert_eq!(shared.control().counts[0].slots, 5 - expected);
                drop(log);
                gate.add_permits(1);
            }
            assert!(poll!(&mut writing).is_pending());
            assert_eq!(shared.control().counts[0].slots, 0);
        }
        drop(requests);
        joined(&c).await;
    }
    #[tokio::test]
    async fn control_capacity_is_independent_and_fair_eight_to_one() {
        let mut options = RuntimeOptions::default();
        options.limits.normal.requests = 2;
        options.limits.normal.outbound_slots = 2;
        options.limits.control.requests = 20;
        options.limits.control.outbound_slots = 20;
        let (c, events, _) = fixture(options);
        drop(events);
        let shared = &c.lease.shared;
        let mut requests = vec![
            queue_raw(&c, 1, TrafficClass::Normal),
            queue_raw(&c, 1, TrafficClass::Normal),
        ];
        requests.extend((0..17).map(|_| queue_raw(&c, 1, TrafficClass::Control)));
        let mut order = Vec::new();
        while let Some(frame) = shared.pop() {
            order.push(frame._permit.class);
            drop(frame);
        }
        assert_eq!(
            order,
            [vec![1; 8], vec![0], vec![1; 8], vec![0, 1]].concat()
        );
        assert_eq!(shared.control().counts[0].slots, 0);
        assert_eq!(shared.control().counts[1].slots, 0);
        drop(requests);
        joined(&c).await;
    }
    struct AlwaysReader {
        reads: Arc<AtomicUsize>,
    }
    impl MessageReader for AlwaysReader {
        async fn recv(&mut self) -> std::result::Result<Option<Bytes>, TransportError> {
            self.reads.fetch_add(1, Ordering::SeqCst);
            Ok(Some(Bytes::from_static(br#"{"id":1,"result":{}}"#)))
        }
    }
    #[tokio::test]
    async fn immediately_ready_read_and_write_loops_yield() {
        let mut options = RuntimeOptions::default();
        options.limits.normal.outbound_slots = 600;
        let (c, events, _) = fixture(options);
        drop(events);
        let shared = c.lease.shared.clone();
        let requests = (0..600)
            .map(|_| queue_raw(&c, 1, TrafficClass::Normal))
            .collect::<Vec<_>>();
        let log = Arc::new(Mutex::new(WriteLog::default()));
        let mut writer = LoggedWriter {
            log: log.clone(),
            flush_gate: Arc::new(tokio::sync::Semaphore::new(600)),
        };
        {
            let mut batch = Vec::new();
            let mut writing = Box::pin(write_loop(&mut writer, &shared, &mut batch));
            assert!(poll!(&mut writing).is_pending());
            let count = lock(&log).sent.len();
            assert!(count > 0 && count <= 256);
        }
        let reads = Arc::new(AtomicUsize::new(0));
        let mut reader = AlwaysReader {
            reads: reads.clone(),
        };
        {
            let mut reading = Box::pin(read_loop(&mut reader, &shared));
            assert!(poll!(&mut reading).is_pending());
            let count = reads.load(Ordering::SeqCst);
            assert!(count > 0 && count <= 256);
        }
        drop(requests);
        joined(&c).await;
    }
    #[tokio::test]
    async fn poisoned_subscription_converges_connection() {
        let (c, mut events, _) = fixture(Default::default());
        let subscriber = events.subscriber.clone();
        let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let _guard = subscriber.state.lock().unwrap();
            panic!("订阅锁故障注入");
        }));
        let error = events.next().await.unwrap().unwrap_err();
        assert!(matches!(error.kind, ErrorKind::DriverFailed));
        assert_eq!(c.state(), ConnectionState::Closing);
        joined(&c).await;
    }
    struct PanicEncode;
    impl serde::Serialize for PanicEncode {
        fn serialize<S: serde::Serializer>(&self, _: S) -> std::result::Result<S::Ok, S::Error> {
            panic!("调用者编码故障注入")
        }
    }
    struct EncodeCommand;
    impl Command for EncodeCommand {
        type Params = PanicEncode;
        type Response = Payload;
        const METHOD: &'static str = "Echo.encode";
    }
    #[derive(Debug)]
    struct PanicDecode;
    impl<'de> serde::Deserialize<'de> for PanicDecode {
        fn deserialize<D: serde::Deserializer<'de>>(_: D) -> std::result::Result<Self, D::Error> {
            panic!("调用者解码故障注入")
        }
    }
    struct DecodeCommand;
    impl Command for DecodeCommand {
        type Params = Payload;
        type Response = PanicDecode;
        const METHOD: &'static str = "Echo.decode";
    }
    async fn user_panics_cleanup_case() {
        let (c, events, _) = fixture(Default::default());
        drop(events);
        let shared = c.lease.shared.clone();
        let mut cx = Context::from_waker(Waker::noop());
        let mut encode = Box::pin(c.root_session().send::<EncodeCommand>(PanicEncode));
        assert!(
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| encode
                .as_mut()
                .poll(&mut cx)))
            .is_err()
        );
        drop(encode);
        assert_eq!(shared.control().counts[0].requests, 0);
        assert_eq!(shared.control().counts[0].slots, 0);
        assert_eq!(shared.control().counts[0].bytes, 0);
        let mut decode = Box::pin(c.root_session().send::<DecodeCommand>(Payload { value: 1 }));
        assert!(decode.as_mut().poll(&mut cx).is_pending());
        response(&shared, decode.request_id().unwrap().get(), 2);
        assert!(
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| decode
                .as_mut()
                .poll(&mut cx)))
            .is_err()
        );
        drop(decode);
        assert_eq!(shared.control().counts[0].requests, 0);
        assert_eq!(shared.wire_budget.used(), 0);
        assert_eq!(c.state(), ConnectionState::Open);
        joined(&c).await;
    }
    #[tokio::test]
    async fn user_panics_cleanup_current_thread() {
        user_panics_cleanup_case().await;
    }
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn user_panics_cleanup_multi_thread() {
        user_panics_cleanup_case().await;
    }
    #[tokio::test(start_paused = true)]
    async fn dropping_close_waiter_does_not_cancel_shared_shutdown() {
        let (c, events, drops) = fixture(Default::default());
        let other = c.clone();
        let mut close = Box::pin(c.close());
        assert!(poll!(&mut close).is_pending());
        drop(close);
        let report = other.closed().await;
        assert!(report.driver_resources_released);
        assert_eq!(drops.load(Ordering::SeqCst), 2);
        assert_eq!(c.close().await.transport_end, report.transport_end);
        let task = lock(&c.lease.shared.task).take().unwrap();
        task.await.unwrap();
        drop(events);
    }
    #[tokio::test]
    async fn invalid_runtime_limits_reject_before_driver() {
        for options in [
            RuntimeOptions {
                initial_events: SubscriptionOptions {
                    max_events: 0,
                    max_bytes: 1,
                },
                ..Default::default()
            },
            RuntimeOptions {
                limits: Limits {
                    max_json_depth: 129,
                    ..Default::default()
                },
                ..Default::default()
            },
        ] {
            assert!(validate_options(&options).is_err());
        }
        assert_eq!(TrafficClass::Normal.index(), 0);
        assert_eq!(TrafficClass::Control.index(), 1);
    }
    struct EncodingBarrier {
        entered: Arc<std::sync::Barrier>,
        resume: Arc<std::sync::Barrier>,
    }
    impl serde::Serialize for EncodingBarrier {
        fn serialize<S: serde::Serializer>(
            &self,
            serializer: S,
        ) -> std::result::Result<S::Ok, S::Error> {
            self.entered.wait();
            self.resume.wait();
            Payload { value: 1 }.serialize(serializer)
        }
    }
    struct BarrierCommand;
    impl Command for BarrierCommand {
        type Params = EncodingBarrier;
        type Response = Payload;
        const METHOD: &'static str = "Echo.barrier";
    }
    async fn close_during_encoding_case() {
        for close_connection in [false, true] {
            let (c, events, _) = fixture(Default::default());
            drop(events);
            let child = c
                .adopt_session(SessionId::new("child".into()).unwrap())
                .unwrap();
            let entered = Arc::new(std::sync::Barrier::new(2));
            let resume = Arc::new(std::sync::Barrier::new(2));
            let closer = if close_connection {
                c.root_session()
            } else {
                child.clone()
            };
            let other_entered = entered.clone();
            let other_resume = resume.clone();
            let thread = std::thread::spawn(move || {
                other_entered.wait();
                closer.close();
                other_resume.wait();
            });
            let mut request =
                Box::pin(child.send::<BarrierCommand>(EncodingBarrier { entered, resume }));
            let result = request.as_mut().await.unwrap_err();
            thread.join().unwrap();
            assert_eq!(request.submission(), Submission::NotSubmitted);
            assert!(matches!(
                result.kind,
                ErrorKind::ConnectionClosed | ErrorKind::SessionClosed
            ));
            assert!(c.lease.shared.control().pending.is_empty());
            assert!(
                c.lease
                    .shared
                    .control()
                    .queues
                    .iter()
                    .all(VecDeque::is_empty)
            );
            assert_eq!(c.lease.shared.control().counts[0].requests, 0);
            assert_eq!(c.lease.shared.control().counts[0].slots, 0);
            assert_eq!(c.lease.shared.control().counts[0].bytes, 0);
            joined(&c).await;
        }
    }
    #[tokio::test]
    async fn close_during_encoding_current_thread() {
        close_during_encoding_case().await;
    }
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn close_during_encoding_multi_thread() {
        close_during_encoding_case().await;
    }
    #[tokio::test(start_paused = true)]
    async fn granted_waiter_timeout_returns_both_permits_and_advances_fifo() {
        let mut options = RuntimeOptions::default();
        options.limits.normal.requests = 1;
        options.limits.normal.outbound_slots = 1;
        let (c, events, _) = fixture(options);
        drop(events);
        let mut first = queue_raw(&c, 1, TrafficClass::Normal);
        let mut expiring = Box::pin(c.root_session().send_with::<Echo>(
            Payload { value: 2 },
            RequestOptions {
                timeout: Timeout::After(Duration::from_secs(1)),
                ..Default::default()
            },
        ));
        assert!(poll!(&mut expiring).is_pending());
        let mut next = Box::pin(c.root_session().send::<Echo>(Payload { value: 3 }));
        assert!(poll!(&mut next).is_pending());
        response(&c.lease.shared, first.request_id().unwrap().get(), 1);
        assert!(first.as_mut().await.is_ok());
        drop(c.lease.shared.pop());
        assert_eq!(
            c.lease
                .shared
                .control()
                .waiters
                .values()
                .filter(|w| w.granted)
                .count(),
            1
        );
        tokio::time::advance(Duration::from_secs(1)).await;
        assert!(matches!(
            expiring.await.unwrap_err().kind,
            ErrorKind::Timeout
        ));
        assert!(poll!(&mut next).is_pending());
        assert!(next.request_id().is_some());
        drop(next);
        joined(&c).await;
    }
    #[tokio::test]
    async fn admission_pending_and_outbound_byte_limits_remain_bounded() {
        let mut options = RuntimeOptions::default();
        options.limits.normal.requests = 1;
        options.limits.normal.outbound_slots = 1;
        options.limits.normal.admission_waiters = 1;
        let (c, events, _) = fixture(options);
        drop(events);
        let active = queue_raw(&c, 1, TrafficClass::Normal);
        let mut waiting = Box::pin(c.root_session().send::<Echo>(Payload { value: 2 }));
        assert!(poll!(&mut waiting).is_pending());
        let mut rejected = Box::pin(c.root_session().send::<Echo>(Payload { value: 3 }));
        assert!(matches!(
            poll!(&mut rejected),
            Poll::Ready(Err(Error {
                kind: ErrorKind::AdmissionFull,
                ..
            }))
        ));
        {
            let control = c.lease.shared.control();
            assert_eq!(control.pending.len(), 1);
            assert_eq!(control.waiters.len(), 1);
            assert_eq!(control.counts[0].waiters, 1);
        }
        drop(active);
        drop(c.lease.shared.pop());
        assert_eq!(
            c.lease
                .shared
                .control()
                .waiters
                .values()
                .filter(|waiter| waiter.granted)
                .count(),
            1
        );
        drop(waiting);
        assert!(c.lease.shared.control().waiters.is_empty());
        joined(&c).await;

        let mut options = RuntimeOptions::default();
        options.limits.normal.requests = 2;
        options.limits.normal.outbound_slots = 2;
        options.limits.normal.outbound_bytes = 180;
        options.limits.normal.max_command_bytes = 180;
        let (c, events, _) = fixture(options);
        drop(events);
        let first = queue_raw(&c, 100, TrafficClass::Normal);
        let params =
            RawJson::from_bytes(Bytes::from(format!("{{\"text\":\"{}\"}}", "x".repeat(100))))
                .unwrap();
        let mut over_budget = Box::pin(c.root_session().send_raw(
            MethodName::new("Echo.raw".into()).unwrap(),
            params,
            Default::default(),
        ));
        assert!(matches!(
            poll!(&mut over_budget),
            Poll::Ready(Err(Error {
                kind: ErrorKind::ResourceLimit {
                    resource: "outbound_bytes"
                },
                ..
            }))
        ));
        {
            let control = c.lease.shared.control();
            assert_eq!(control.pending.len(), 1);
            assert_eq!(control.queues[0].len(), 1);
            assert_eq!(control.counts[0].requests, 1);
            assert_eq!(control.counts[0].slots, 1);
            assert!(control.counts[0].bytes <= 180);
        }
        drop(first);
        joined(&c).await;
    }
    #[derive(Clone, Copy)]
    enum ActiveFailure {
        Read,
        Write,
    }
    struct ActiveFailureReader {
        mode: ActiveFailure,
        drops: Arc<AtomicUsize>,
    }
    struct ActiveFailureWriter {
        mode: ActiveFailure,
        drops: Arc<AtomicUsize>,
    }
    struct ActiveFailureTransport {
        mode: ActiveFailure,
        drops: Arc<AtomicUsize>,
    }
    impl Transport for ActiveFailureTransport {
        type Reader = ActiveFailureReader;
        type Writer = ActiveFailureWriter;
        fn split(self) -> (Self::Reader, Self::Writer) {
            (
                ActiveFailureReader {
                    mode: self.mode,
                    drops: self.drops.clone(),
                },
                ActiveFailureWriter {
                    mode: self.mode,
                    drops: self.drops,
                },
            )
        }
    }
    impl Drop for ActiveFailureReader {
        fn drop(&mut self) {
            self.drops.fetch_add(1, Ordering::SeqCst);
        }
    }
    impl Drop for ActiveFailureWriter {
        fn drop(&mut self) {
            self.drops.fetch_add(1, Ordering::SeqCst);
        }
    }
    impl MessageReader for ActiveFailureReader {
        async fn recv(&mut self) -> std::result::Result<Option<Bytes>, TransportError> {
            match self.mode {
                ActiveFailure::Read => Err(TransportError::new(io::Error::other("读故障注入"))),
                ActiveFailure::Write => std::future::pending().await,
            }
        }
    }
    impl MessageWriter for ActiveFailureWriter {
        async fn send(&mut self, _: Bytes) -> std::result::Result<(), TransportError> {
            match self.mode {
                ActiveFailure::Read => Ok(()),
                ActiveFailure::Write => Err(TransportError::new(io::Error::other("写故障注入"))),
            }
        }
        async fn flush(&mut self) -> std::result::Result<(), TransportError> {
            Ok(())
        }
        async fn close(&mut self) -> std::result::Result<(), TransportError> {
            Ok(())
        }
    }
    #[tokio::test(start_paused = true)]
    async fn active_read_and_write_failures_preserve_first_reason_and_release_transport() {
        for mode in [ActiveFailure::Read, ActiveFailure::Write] {
            let drops = Arc::new(AtomicUsize::new(0));
            let (c, events) = Connection::from_transport(
                ActiveFailureTransport {
                    mode,
                    drops: drops.clone(),
                },
                RuntimeOptions {
                    close_timeout: Duration::ZERO,
                    ..Default::default()
                },
            )
            .unwrap();
            let request = if matches!(mode, ActiveFailure::Write) {
                let mut request = Box::pin(c.root_session().send::<Echo>(Payload { value: 1 }));
                assert!(poll!(&mut request).is_pending());
                Some(request)
            } else {
                None
            };
            let report = c.closed().await;
            assert!(matches!(report.reason.kind, ErrorKind::Transport));
            assert!(report.driver_resources_released);
            assert!(report.supplemental_errors.len() <= 1);
            assert_eq!(drops.load(Ordering::SeqCst), 2);
            if let Some(request) = request {
                assert!(matches!(
                    request.await.unwrap_err().kind,
                    ErrorKind::Transport
                ));
            }
            let task = lock(&c.lease.shared.task).take().unwrap();
            task.await.unwrap();
            drop(events);
        }
    }
    struct PanicReader(Arc<AtomicUsize>);
    struct PanicTransport(Arc<AtomicUsize>);
    impl Drop for PanicReader {
        fn drop(&mut self) {
            self.0.fetch_add(1, Ordering::SeqCst);
        }
    }
    impl Transport for PanicTransport {
        type Reader = PanicReader;
        type Writer = IdleWriter;
        fn split(self) -> (Self::Reader, Self::Writer) {
            (PanicReader(self.0.clone()), IdleWriter(self.0))
        }
    }
    impl MessageReader for PanicReader {
        async fn recv(&mut self) -> std::result::Result<Option<Bytes>, TransportError> {
            panic!("Driver 读方向 panic 故障注入")
        }
    }
    #[tokio::test]
    async fn driver_panic_runs_guard_and_reports_released_transport() {
        let drops = Arc::new(AtomicUsize::new(0));
        let (c, events) =
            Connection::from_transport(PanicTransport(drops.clone()), Default::default()).unwrap();
        let task = lock(&c.lease.shared.task).take().unwrap();
        assert!(task.await.unwrap_err().is_panic());
        let report = c.closed().await;
        assert!(matches!(report.reason.kind, ErrorKind::DriverFailed));
        assert_eq!(report.transport_end, TransportEnd::Failed);
        assert!(report.driver_resources_released);
        assert_eq!(drops.load(Ordering::SeqCst), 2);
        drop(events);
    }
    #[derive(Clone, Copy)]
    enum CloseMode {
        Fail,
        Pending,
    }
    struct CloseState {
        closing: std::sync::atomic::AtomicBool,
        notify: Notify,
        observed: Notify,
        drops: AtomicUsize,
    }
    struct CloseReader {
        state: Arc<CloseState>,
        mode: CloseMode,
    }
    struct CloseWriter {
        state: Arc<CloseState>,
        mode: CloseMode,
    }
    struct CloseTransport {
        state: Arc<CloseState>,
        read: CloseMode,
        write: CloseMode,
    }
    impl Transport for CloseTransport {
        type Reader = CloseReader;
        type Writer = CloseWriter;
        fn split(self) -> (Self::Reader, Self::Writer) {
            (
                CloseReader {
                    state: self.state.clone(),
                    mode: self.read,
                },
                CloseWriter {
                    state: self.state,
                    mode: self.write,
                },
            )
        }
    }
    impl Drop for CloseReader {
        fn drop(&mut self) {
            self.state.drops.fetch_add(1, Ordering::SeqCst);
        }
    }
    impl Drop for CloseWriter {
        fn drop(&mut self) {
            self.state.drops.fetch_add(1, Ordering::SeqCst);
        }
    }
    impl MessageReader for CloseReader {
        async fn recv(&mut self) -> std::result::Result<Option<Bytes>, TransportError> {
            let notification = self.state.notify.notified();
            tokio::pin!(notification);
            notification.as_mut().enable();
            if !self.state.closing.load(Ordering::SeqCst) {
                notification.await;
            }
            match self.mode {
                CloseMode::Fail => {
                    self.state.observed.notify_one();
                    Err(TransportError::new(io::Error::other("关闭读故障")))
                }
                CloseMode::Pending => std::future::pending().await,
            }
        }
    }
    impl MessageWriter for CloseWriter {
        async fn send(&mut self, _: Bytes) -> std::result::Result<(), TransportError> {
            Ok(())
        }
        async fn flush(&mut self) -> std::result::Result<(), TransportError> {
            Ok(())
        }
        async fn close(&mut self) -> std::result::Result<(), TransportError> {
            self.state.closing.store(true, Ordering::SeqCst);
            self.state.notify.notify_waiters();
            match self.mode {
                CloseMode::Fail => {
                    self.state.observed.notify_one();
                    Err(TransportError::new(io::Error::other("关闭写故障")))
                }
                CloseMode::Pending => std::future::pending().await,
            }
        }
    }
    #[tokio::test(start_paused = true)]
    async fn shutdown_keeps_observed_error_while_other_direction_times_out() {
        for (read, write) in [
            (CloseMode::Fail, CloseMode::Pending),
            (CloseMode::Pending, CloseMode::Fail),
        ] {
            let state = Arc::new(CloseState {
                closing: std::sync::atomic::AtomicBool::new(false),
                notify: Notify::new(),
                observed: Notify::new(),
                drops: AtomicUsize::new(0),
            });
            let (c, events) = Connection::from_transport(
                CloseTransport {
                    state: state.clone(),
                    read,
                    write,
                },
                RuntimeOptions {
                    close_timeout: Duration::from_secs(5),
                    ..Default::default()
                },
            )
            .unwrap();
            let mut closing = Box::pin(c.close());
            assert!(poll!(&mut closing).is_pending());
            state.observed.notified().await;
            assert_eq!(c.lease.shared.control().supplemental_errors.len(), 1);
            assert!(poll!(&mut closing).is_pending());
            tokio::time::advance(Duration::from_secs(5)).await;
            let report = closing.await;
            assert!(matches!(report.reason.kind, ErrorKind::ConnectionClosed));
            assert_eq!(report.transport_end, TransportEnd::Aborted);
            assert_eq!(report.supplemental_errors.len(), 1);
            assert!(matches!(
                report.supplemental_errors[0].kind,
                ErrorKind::Transport
            ));
            assert!(std::error::Error::source(&report.supplemental_errors[0]).is_some());
            assert_eq!(state.drops.load(Ordering::SeqCst), 2);
            let task = lock(&c.lease.shared.task).take().unwrap();
            task.await.unwrap();
            drop(events);
        }
    }
    #[tokio::test(start_paused = true)]
    async fn both_shutdown_failures_are_bounded_and_preserve_first_reason() {
        let state = Arc::new(CloseState {
            closing: std::sync::atomic::AtomicBool::new(false),
            notify: Notify::new(),
            observed: Notify::new(),
            drops: AtomicUsize::new(0),
        });
        let (c, events) = Connection::from_transport(
            CloseTransport {
                state: state.clone(),
                read: CloseMode::Fail,
                write: CloseMode::Fail,
            },
            Default::default(),
        )
        .unwrap();
        let report = c.close().await;
        assert!(matches!(report.reason.kind, ErrorKind::ConnectionClosed));
        assert_eq!(report.transport_end, TransportEnd::Failed);
        assert_eq!(report.supplemental_errors.len(), 2);
        assert!(
            report
                .supplemental_errors
                .iter()
                .all(|error| matches!(error.kind, ErrorKind::Transport))
        );
        assert_eq!(state.drops.load(Ordering::SeqCst), 2);
        assert_eq!(c.close().await.supplemental_errors.len(), 2);
        let task = lock(&c.lease.shared.task).take().unwrap();
        task.await.unwrap();
        drop(events);
    }
    #[tokio::test]
    async fn interrupted_writer_keeps_batch_permits_until_transport_is_dropped() {
        let (c, events, _) = fixture(Default::default());
        drop(events);
        let request = queue_raw(&c, 100, TrafficClass::Normal);
        let shared = c.lease.shared.clone();
        let log = Arc::new(Mutex::new(WriteLog::default()));
        let mut writer = LoggedWriter {
            log,
            flush_gate: Arc::new(tokio::sync::Semaphore::new(0)),
        };
        let mut batch = Vec::new();
        {
            let mut writing = Box::pin(write_loop(&mut writer, &shared, &mut batch));
            assert!(poll!(&mut writing).is_pending());
        }
        assert_eq!(shared.control().counts[0].slots, 1);
        assert!(shared.control().counts[0].bytes > 0);
        assert_eq!(batch.len(), 1);
        drop(writer);
        batch.clear();
        assert_eq!(shared.control().counts[0].slots, 0);
        assert_eq!(shared.control().counts[0].bytes, 0);
        drop(request);
        joined(&c).await;
    }
    #[tokio::test]
    async fn typed_decode_failure_removes_exact_subscription_index() {
        let (c, events, _) = fixture(Default::default());
        drop(events);
        let session = c.root_session();
        for _ in 0..100 {
            let mut typed = session.events::<Changed>(Default::default()).unwrap();
            receive(
                &c.lease.shared,
                Bytes::from_static(br#"{"method":"Echo.changed","params":{"value":"bad"}}"#),
            )
            .unwrap();
            assert!(typed.next().await.unwrap().is_err());
            drop(typed);
            assert!(c.lease.shared.control().exact.is_empty());
        }
        joined(&c).await;
    }
    struct PartialState {
        shared: Mutex<Option<Weak<Shared>>>,
        entered: Notify,
        dropped_slots: AtomicUsize,
    }
    struct PartialWriter(Arc<PartialState>);
    struct PartialTransport(Arc<PartialState>);
    impl Drop for PartialWriter {
        fn drop(&mut self) {
            if let Some(shared) = lock(&self.0.shared).as_ref().and_then(Weak::upgrade) {
                self.0
                    .dropped_slots
                    .store(shared.control().counts[0].slots, Ordering::SeqCst);
            }
        }
    }
    impl Transport for PartialTransport {
        type Reader = IdleReader;
        type Writer = PartialWriter;
        fn split(self) -> (Self::Reader, Self::Writer) {
            (
                IdleReader(Arc::new(AtomicUsize::new(0))),
                PartialWriter(self.0),
            )
        }
    }
    impl MessageWriter for PartialWriter {
        async fn send(&mut self, _: Bytes) -> std::result::Result<(), TransportError> {
            self.0.entered.notify_one();
            std::future::pending().await
        }
        async fn flush(&mut self) -> std::result::Result<(), TransportError> {
            std::future::pending().await
        }
        async fn close(&mut self) -> std::result::Result<(), TransportError> {
            std::future::pending().await
        }
    }
    #[tokio::test(start_paused = true)]
    async fn driver_shutdown_drops_transport_before_inflight_send_permit() {
        let state = Arc::new(PartialState {
            shared: Mutex::new(None),
            entered: Notify::new(),
            dropped_slots: AtomicUsize::new(0),
        });
        let (c, events) = Connection::from_transport(
            PartialTransport(state.clone()),
            RuntimeOptions {
                close_timeout: Duration::from_secs(5),
                ..Default::default()
            },
        )
        .unwrap();
        *lock(&state.shared) = Some(Arc::downgrade(&c.lease.shared));
        let request = queue_raw(&c, 1, TrafficClass::Normal);
        state.entered.notified().await;
        let child = c
            .adopt_session(SessionId::new("retired".into()).unwrap())
            .unwrap();
        child.close();
        let mut close = Box::pin(c.close());
        assert!(poll!(&mut close).is_pending());
        assert_eq!(c.lease.shared.control().counts[0].slots, 1);
        tokio::time::advance(Duration::from_secs(5)).await;
        let report = close.await;
        assert!(report.driver_resources_released);
        assert_eq!(state.dropped_slots.load(Ordering::SeqCst), 1);
        assert_eq!(c.lease.shared.control().counts[0].slots, 0);
        assert!(c.lease.shared.control().routes.is_empty());
        assert!(c.lease.shared.control().retired.is_empty());
        let task = lock(&c.lease.shared.task).take().unwrap();
        task.await.unwrap();
        drop(request);
        drop(events);
    }
    #[tokio::test]
    async fn protocol_errors_without_data_keep_queued_frame_budget() {
        for (count, message_len) in [(1, 512 << 10), (32, 8 << 10)] {
            let (c, events, _) = fixture(Default::default());
            drop(events);
            let shared = &c.lease.shared;
            let mut requests = Vec::new();
            let mut sizes = Vec::new();
            let mut expected = 0;
            for _ in 0..count {
                let mut request = Box::pin(c.root_session().send::<Echo>(Payload { value: 1 }));
                assert!(poll!(&mut request).is_pending());
                let id = request.request_id().unwrap().get();
                let bytes = Bytes::from(format!(
                    "{{\"id\":{id},\"error\":{{\"code\":-1,\"message\":\"{}\"}}}}",
                    "x".repeat(message_len)
                ));
                let len = bytes.len();
                receive(shared, bytes).unwrap();
                expected += len;
                assert_eq!(shared.wire_budget.used(), expected);
                requests.push(request);
                sizes.push(len);
            }
            assert!(shared.control().pending.is_empty());
            assert_eq!(shared.control().counts[0].requests, count);
            while let Some(request) = requests.pop() {
                drop(request);
                expected -= sizes.pop().unwrap();
                assert_eq!(shared.wire_budget.used(), expected);
            }
            assert_eq!(shared.control().counts[0].requests, 0);
            joined(&c).await;
        }
    }
    #[tokio::test]
    async fn delivered_protocol_error_clones_share_one_frame_permit() {
        let (c, events, _) = fixture(Default::default());
        drop(events);
        let shared = &c.lease.shared;
        let mut request = Box::pin(c.root_session().send::<Echo>(Payload { value: 1 }));
        assert!(poll!(&mut request).is_pending());
        let bytes = Bytes::from(format!(
            "{{\"id\":{},\"error\":{{\"code\":-1,\"message\":\"{}\"}}}}",
            request.request_id().unwrap().get(),
            "x".repeat(64 << 10)
        ));
        let len = bytes.len();
        receive(shared, bytes).unwrap();
        let error = request.await.unwrap_err();
        assert!(matches!(error.kind, ErrorKind::Protocol { data: None, .. }));
        assert_eq!(shared.wire_budget.used(), len);
        let cloned = error.clone();
        drop(error);
        assert_eq!(shared.wire_budget.used(), len);
        drop(cloned);
        assert_eq!(shared.wire_budget.used(), 0);
        joined(&c).await;
    }
    #[tokio::test]
    async fn retained_protocol_error_exhausts_wire_budget() {
        let mut options = RuntimeOptions::default();
        options.limits.wire_bytes = 100 << 10;
        let (c, events, _) = fixture(options);
        drop(events);
        let shared = &c.lease.shared;
        let mut first = Box::pin(c.root_session().send::<Echo>(Payload { value: 1 }));
        let mut second = Box::pin(c.root_session().send::<Echo>(Payload { value: 2 }));
        assert!(poll!(&mut first).is_pending());
        assert!(poll!(&mut second).is_pending());
        for (index, request) in [&first, &second].into_iter().enumerate() {
            let bytes = Bytes::from(format!(
                "{{\"id\":{},\"error\":{{\"code\":-1,\"message\":\"{}\"}}}}",
                request.request_id().unwrap().get(),
                "x".repeat(64 << 10)
            ));
            let result = receive(shared, bytes);
            if index == 0 {
                assert!(result.is_ok());
            } else {
                assert!(matches!(
                    result.unwrap_err().kind,
                    ErrorKind::ResourceLimit { .. }
                ));
            }
        }
        assert!(shared.wire_budget.used() > 64 << 10);
        assert!(matches!(
            second.await.unwrap_err().kind,
            ErrorKind::ResourceLimit { .. }
        ));
        drop(first);
        assert_eq!(shared.wire_budget.used(), 0);
        joined(&c).await;
    }
    struct StressReader(tokio::sync::mpsc::Receiver<Bytes>);
    struct StressWriter(Option<tokio::sync::mpsc::Sender<Bytes>>);
    struct StressTransport {
        input: tokio::sync::mpsc::Receiver<Bytes>,
        output: tokio::sync::mpsc::Sender<Bytes>,
    }
    impl Transport for StressTransport {
        type Reader = StressReader;
        type Writer = StressWriter;
        fn split(self) -> (Self::Reader, Self::Writer) {
            (StressReader(self.input), StressWriter(Some(self.output)))
        }
    }
    impl MessageReader for StressReader {
        async fn recv(&mut self) -> std::result::Result<Option<Bytes>, TransportError> {
            Ok(self.0.recv().await)
        }
    }
    impl MessageWriter for StressWriter {
        async fn send(&mut self, bytes: Bytes) -> std::result::Result<(), TransportError> {
            self.0
                .as_ref()
                .unwrap()
                .send(bytes)
                .await
                .map_err(|error| TransportError::new(io::Error::other(error)))
        }
        async fn flush(&mut self) -> std::result::Result<(), TransportError> {
            Ok(())
        }
        async fn close(&mut self) -> std::result::Result<(), TransportError> {
            self.0.take();
            Ok(())
        }
    }
    #[derive(serde::Deserialize)]
    struct StressCommand {
        id: i32,
        params: Payload,
        #[serde(rename = "sessionId")]
        session_id: Option<String>,
    }

    struct StressReport {
        iterations: usize,
        cancelled: usize,
        subscriptions: usize,
        elapsed: Duration,
        initial_rss_bytes: Option<u64>,
        peak_rss_bytes: Option<u64>,
        final_rss_bytes: Option<u64>,
    }

    #[cfg(target_os = "linux")]
    fn current_rss_bytes() -> Option<u64> {
        let status = std::fs::read_to_string("/proc/self/status").ok()?;
        let kib = status.lines().find_map(|line| {
            let value = line.strip_prefix("VmRSS:")?.split_whitespace().next()?;
            value.parse::<u64>().ok()
        })?;
        kib.checked_mul(1024)
    }

    #[cfg(not(target_os = "linux"))]
    fn current_rss_bytes() -> Option<u64> {
        None
    }

    fn assert_stress_quiescent(shared: &Shared, active_routes: usize) {
        let control = shared.control();
        assert!(control.pending.is_empty());
        assert!(control.waiters.is_empty());
        assert!(control.queues.iter().all(VecDeque::is_empty));
        assert!(control.counts.iter().all(|counts| {
            counts.requests == 0 && counts.slots == 0 && counts.bytes == 0 && counts.waiters == 0
        }));
        assert_eq!(control.subscription_count, 0);
        assert!(control.subscriptions.is_empty());
        assert!(control.all.is_empty());
        assert!(control.wild.is_empty());
        assert!(control.exact.is_empty());
        assert_eq!(control.routes.len(), active_routes);
        assert_eq!(shared.wire_budget.used(), 0);
    }

    async fn mixed_lifecycle_stress(
        minimum_iterations: usize,
        minimum_duration: Duration,
    ) -> StressReport {
        let started = std::time::Instant::now();
        let initial_rss_bytes = current_rss_bytes();
        let mut peak_rss_bytes = initial_rss_bytes;
        let (out_tx, mut out_rx) = tokio::sync::mpsc::channel::<Bytes>(4);
        let (in_tx, in_rx) = tokio::sync::mpsc::channel::<Bytes>(4);
        let (cancel_seen_tx, mut cancel_seen_rx) = tokio::sync::mpsc::channel::<()>(1);
        let (cancel_release_tx, mut cancel_release_rx) = tokio::sync::mpsc::channel::<()>(1);
        let endpoint = tokio::spawn(async move {
            let mut count = 0usize;
            while let Some(bytes) = out_rx.recv().await {
                let command: StressCommand = serde_json::from_slice(&bytes).unwrap();
                let route = command
                    .session_id
                    .map(|id| format!(",\"sessionId\":{}", serde_json::to_string(&id).unwrap()))
                    .unwrap_or_default();
                let value = command.params.value;
                if value % 4 == 1 {
                    cancel_seen_tx.send(()).await.unwrap();
                    cancel_release_rx.recv().await.unwrap();
                }
                in_tx
                    .send(Bytes::from(format!(
                        "{{\"method\":\"Echo.changed\",\"params\":{{\"value\":{value}}}{route}}}"
                    )))
                    .await
                    .unwrap();
                in_tx
                    .send(Bytes::from(format!(
                        "{{\"id\":{},\"result\":{{\"value\":{value}}}{route}}}",
                        command.id
                    )))
                    .await
                    .unwrap();
                count += 1;
            }
            count
        });
        let (connection, events) = Connection::from_transport(
            StressTransport {
                input: in_rx,
                output: out_tx,
            },
            Default::default(),
        )
        .unwrap();
        drop(events);
        let shared = connection.lease.shared.clone();
        let sessions = [
            connection.root_session(),
            connection
                .adopt_session(SessionId::new("stress-a".into()).unwrap())
                .unwrap(),
            connection
                .adopt_session(SessionId::new("stress-b".into()).unwrap())
                .unwrap(),
            connection
                .adopt_session(SessionId::new("stress-c".into()).unwrap())
                .unwrap(),
        ];
        let mut iterations = 0usize;
        let mut cancelled = 0usize;
        let mut subscriptions = 0usize;
        while iterations < minimum_iterations || started.elapsed() < minimum_duration {
            let session = &sessions[iterations % sessions.len()];
            let value = u32::try_from(iterations % u32::MAX as usize).unwrap();
            match iterations % 4 {
                0 => assert_eq!(
                    session.send::<Echo>(Payload { value }).await.unwrap().value,
                    value
                ),
                1 => {
                    let mut request = Box::pin(session.send::<Echo>(Payload { value }));
                    assert!(poll!(&mut request).is_pending());
                    cancel_seen_rx.recv().await.unwrap();
                    assert_eq!(request.submission(), Submission::PossiblySent);
                    drop(request);
                    cancel_release_tx.send(()).await.unwrap();
                    cancelled += 1;
                }
                2 => {
                    let mut events = session.events::<Changed>(Default::default()).unwrap();
                    assert_eq!(
                        session.send::<Echo>(Payload { value }).await.unwrap().value,
                        value
                    );
                    assert_eq!(events.next().await.unwrap().unwrap().params.value, value);
                    drop(events);
                    subscriptions += 1;
                }
                _ => {
                    drop(session.raw_events(Default::default()).unwrap());
                    assert_eq!(
                        session.send::<Echo>(Payload { value }).await.unwrap().value,
                        value
                    );
                    subscriptions += 1;
                }
            }
            iterations += 1;
            if iterations.is_multiple_of(4096) {
                assert_stress_quiescent(&shared, 3);
                if let Some(rss) = current_rss_bytes() {
                    peak_rss_bytes = Some(peak_rss_bytes.unwrap_or(rss).max(rss));
                }
            }
        }
        assert_stress_quiescent(&shared, 3);
        for session in sessions.iter().skip(1) {
            session.close();
        }
        assert_eq!(shared.control().retired.len(), 3);
        joined(&connection).await;
        assert_eq!(endpoint.await.unwrap(), iterations);
        assert!(shared.control().routes.is_empty());
        assert!(shared.control().retired.is_empty());
        assert_eq!(shared.wire_budget.used(), 0);
        let final_rss_bytes = current_rss_bytes();
        if let Some(rss) = final_rss_bytes {
            peak_rss_bytes = Some(peak_rss_bytes.unwrap_or(rss).max(rss));
        }
        StressReport {
            iterations,
            cancelled,
            subscriptions,
            elapsed: started.elapsed(),
            initial_rss_bytes,
            peak_rss_bytes,
            final_rss_bytes,
        }
    }

    fn print_stress_report(kind: &str, report: &StressReport) {
        println!(
            "YDP_STRESS_REPORT={}",
            serde_json::json!({
                "kind": kind,
                "iterations": report.iterations,
                "cancelled": report.cancelled,
                "subscriptions": report.subscriptions,
                "elapsed_ms": report.elapsed.as_millis(),
                "initial_rss_bytes": report.initial_rss_bytes,
                "peak_rss_bytes": report.peak_rss_bytes,
                "final_rss_bytes": report.final_rss_bytes,
                "resources_released": true,
            })
        );
    }

    #[tokio::test]
    async fn hundred_thousand_mixed_lifecycles_return_all_internal_resources() {
        let report = tokio::time::timeout(
            Duration::from_secs(120),
            mixed_lifecycle_stress(100_000, Duration::ZERO),
        )
        .await
        .expect("压力测试超时保护触发，需调查进展停滞");
        print_stress_report("deterministic-100000", &report);
    }

    #[tokio::test]
    async fn slow_consumer_large_messages_and_unresponsive_endpoint_stay_bounded() {
        let large_padding = "x".repeat(128 << 10);
        let first_event = Bytes::from(format!(
            "{{\"method\":\"Echo.changed\",\"params\":{{\"value\":1,\"padding\":\"{large_padding}\"}}}}"
        ));
        let second_event = Bytes::from(format!(
            "{{\"method\":\"Echo.changed\",\"params\":{{\"value\":2,\"padding\":\"{large_padding}\"}}}}"
        ));
        let first_len = first_event.len();
        let mut options = RuntimeOptions {
            initial_events: SubscriptionOptions {
                max_events: 1,
                max_bytes: first_len + 1,
            },
            ..Default::default()
        };
        options.limits.wire_bytes = first_len * 2;
        let (connection, mut slow_events, _) = fixture(options);
        let shared = connection.lease.shared.clone();

        // IdleTransport 永不返回响应，模拟永久无响应端点；Request 的外部 Drop 只取消本地等待。
        let mut unanswered = Box::pin(connection.root_session().send::<Echo>(Payload { value: 7 }));
        assert!(poll!(&mut unanswered).is_pending());
        assert!(unanswered.request_id().is_some());
        assert_eq!(shared.control().pending.len(), 1);

        // 首条大事件由未消费的全连接流持有，第二条触发独立终态而不阻塞 Reader。
        receive(&shared, first_event).unwrap();
        assert_eq!(shared.wire_budget.used(), first_len);
        assert_eq!(shared.control().subscription_count, 1);
        receive(&shared, second_event).unwrap();
        assert!(shared.wire_budget.used() <= first_len);

        drop(unanswered);
        assert!(shared.control().pending.is_empty());
        assert!(matches!(
            slow_events.next().await.unwrap().unwrap_err().kind,
            ErrorKind::EventBufferFull
        ));
        assert!(slow_events.next().await.is_none());
        drop(slow_events);
        // 端点已经接收命令但永久不响应；测试线程接管并释放尚未由 Driver 写出的帧。
        drop(shared.pop());
        assert_stress_quiescent(&shared, 0);

        joined(&connection).await;
        assert!(shared.control().routes.is_empty());
        assert!(shared.control().retired.is_empty());
        assert_eq!(shared.wire_budget.used(), 0);
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    #[ignore = "需要显式执行的 30 分钟资源长测"]
    async fn thirty_minute_mixed_lifecycles_records_resource_trend() {
        let seconds = std::env::var("YDP_SOAK_SECONDS")
            .ok()
            .and_then(|value| value.parse::<u64>().ok())
            .unwrap_or(30 * 60);
        let duration = Duration::from_secs(seconds);
        let report = tokio::time::timeout(
            duration.saturating_add(Duration::from_secs(120)),
            mixed_lifecycle_stress(100_000, duration),
        )
        .await
        .expect("资源长测超时保护触发，需调查进展停滞");
        print_stress_report("soak", &report);
    }
}
