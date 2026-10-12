use crate::{Error, ErrorKind, Result};
use std::time::Duration;
/// 命令的编译期描述；序列化与结果解码均在调用者任务执行。
pub trait Command: Send + Sync + 'static {
    /// 拥有型参数；只需 Send，不强制 Sync。
    type Params: serde::Serialize + Send + 'static;
    /// 拥有型成功结果。
    type Response: serde::de::DeserializeOwned + Send + 'static;
    /// 完整 wire 方法名，不在运行时拼接。
    const METHOD: &'static str;
}
/// 事件的编译期描述；参数在消费者 poll 中解码。
pub trait Event: Send + Sync + 'static {
    /// 拥有型事件数据。
    type Params: serde::de::DeserializeOwned + Send + 'static;
    /// 完整 wire 事件名称。
    const METHOD: &'static str;
}
/// 本地连接身份，不写入 wire。
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ConnectionKey(pub(crate) u64);
/// 连接内不可重用的路由身份。
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SessionKey {
    /// 路由所属的本地连接。
    pub connection: ConnectionKey,
    /// 本连接单调递增序号；根为零。
    pub sequence: u64,
}
/// 运行时分配的正 i32 请求标识，在连接存续期间不复用。
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct RequestId(pub(crate) i32);
impl RequestId {
    /// 返回 wire 整数，范围 1 到 i32::MAX。
    /// ```no_run
    /// fn inspect(id: ydp_core::RequestId) { assert!(id.get() > 0); }
    /// ```
    pub fn get(self) -> i32 {
        self.0
    }
}
/// 对每个合法 envelope 递增的连接内接收序号。
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ReceiveSeq(pub(crate) u64);
impl ReceiveSeq {
    /// 返回从一开始的序号；响应会令事件流序号出现空洞。
    /// ```no_run
    /// fn inspect(seq: ydp_core::ReceiveSeq) { assert!(seq.get() > 0); }
    /// ```
    pub fn get(self) -> u64 {
        self.0
    }
}
/// 已校验的拥有型 flattened Session 路由 ID，不是 Target ID。
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SessionId(std::sync::Arc<str>);
impl SessionId {
    /// 验证非空和 1024 UTF-8 字节上限；不发生 attach。
    /// ```
    /// let id = ydp_core::SessionId::new("child".to_owned()).unwrap();
    /// assert_eq!(id.as_str(), "child");
    /// ```
    pub fn new(value: String) -> Result<Self> {
        if value.is_empty() || value.len() > 1024 {
            Err(Error::new(ErrorKind::InvalidArgument))
        } else {
            Ok(Self(value.into()))
        }
    }
    /// 借用原始路由字符串，不分配，不改变会话状态。
    /// ```
    /// assert_eq!(ydp_core::SessionId::new("c".into()).unwrap().as_str(), "c");
    /// ```
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
impl std::borrow::Borrow<str> for SessionId {
    fn borrow(&self) -> &str {
        self.as_str()
    }
}
/// 动态命令的已校验方法名，不强制 Domain 语法。
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MethodName(String);
impl MethodName {
    /// 验证非空、1024 UTF-8 字节上限与无 ASCII 控制字符。
    /// ```
    /// assert!(ydp_core::MethodName::new("Echo.echo".into()).is_ok());
    /// assert!(ydp_core::MethodName::new("\n".into()).is_err());
    /// ```
    pub fn new(value: String) -> Result<Self> {
        if value.is_empty() || value.len() > 1024 || value.bytes().any(|b| b.is_ascii_control()) {
            Err(Error::new(ErrorKind::InvalidArgument))
        } else {
            Ok(Self(value))
        }
    }
    /// 借用完整方法名；不分配。
    /// ```
    /// assert_eq!(ydp_core::MethodName::new("Echo.echo".into()).unwrap().as_str(), "Echo.echo");
    /// ```
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
/// 请求的保守提交证据，不代表远端是否已经执行。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Submission {
    /// 尚未发布到本地 outbox。
    #[default]
    NotSubmitted,
    /// 已发布，可能继续产生远端副作用。
    PossiblySent,
    /// 调用者已消费匹配响应，领域解码仍可能失败。
    Responded,
}
/// 本地合作式期限；不抢占用户同步编解码。
#[derive(Clone, Copy, Debug, Default)]
pub enum Timeout {
    /// 不设置期限，不建立 timer。
    #[default]
    Unlimited,
    /// 从首次 poll 起计时。
    After(Duration),
    /// 使用 Tokio 时钟的绝对期限。
    At(tokio::time::Instant),
}
/// 相互独立保留容量的流量类别。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TrafficClass {
    /// 普通请求，不借用控制容量。
    #[default]
    Normal,
    /// 显式控制/清理流量，不按方法名推断。
    Control,
}
impl TrafficClass {
    /// 返回独立流量账本索引；Normal 为零、Control 为一，内部测试覆盖两类隔离。
    pub(crate) fn index(self) -> usize {
        match self {
            Self::Normal => 0,
            Self::Control => 1,
        }
    }
}
/// 创建请求时固定的本地选项。
#[derive(Clone, Copy, Debug, Default)]
pub struct RequestOptions {
    /// 本次合作式期限，默认 Unlimited。
    pub timeout: Timeout,
    /// 使用的独立容量，默认 Normal。
    pub traffic_class: TrafficClass,
}
/// 单订阅逻辑预算，按完整 backing frame 计字节。
#[derive(Clone, Copy, Debug)]
pub struct SubscriptionOptions {
    /// 排队事件数量，必须大于零。
    pub max_events: usize,
    /// 排队完整帧字节数，必须大于零。
    pub max_bytes: usize,
}
impl Default for SubscriptionOptions {
    fn default() -> Self {
        Self {
            max_events: 256,
            max_bytes: 1 << 20,
        }
    }
}
/// 每个流量类别的独立资源上限。
#[derive(Clone, Copy, Debug)]
pub struct ClassLimits {
    /// 活跃请求，包括已移交而尚未消费的响应许可。
    pub requests: usize,
    /// 等待或已获 grant 但未 poll 的准入记录上限。
    pub admission_waiters: usize,
    /// 编码中、排队和未 flush 帧的总槽位。
    pub outbound_slots: usize,
    /// 编码及出站缓冲合计逻辑字节。
    pub outbound_bytes: usize,
    /// 一条编码命令的最大字节数。
    pub max_command_bytes: usize,
}
/// 连接的有界资源配置；默认值见第 08 篇。
#[derive(Clone, Debug)]
pub struct Limits {
    /// 普通流量独立配额。
    pub normal: ClassLimits,
    /// 控制流量独立配额。
    pub control: ClassLimits,
    /// 已交付或排队 backing frame 的物理字节上限。
    pub wire_bytes: usize,
    /// 一条完整入站消息上限。
    pub max_message_bytes: usize,
    /// JSON 容器深度，范围 1 到 128。
    pub max_json_depth: usize,
    /// 订阅句柄上限，包括终态未 Drop 的句柄。
    pub subscriptions: usize,
    /// 活跃 child Session 记录上限。
    pub active_sessions: usize,
    /// 生命周期不同 child ID 总上限，包括 retired。
    pub session_records: usize,
}
impl Default for Limits {
    fn default() -> Self {
        Self {
            normal: ClassLimits {
                requests: 2048,
                admission_waiters: 4096,
                outbound_slots: 256,
                outbound_bytes: 64 << 20,
                max_command_bytes: 16 << 20,
            },
            control: ClassLimits {
                requests: 32,
                admission_waiters: 64,
                outbound_slots: 8,
                outbound_bytes: 4 << 20,
                max_command_bytes: 256 << 10,
            },
            wire_bytes: 256 << 20,
            max_message_bytes: 128 << 20,
            max_json_depth: 128,
            subscriptions: 64,
            active_sessions: 4096,
            session_records: 65536,
        }
    }
}
/// 已建立传输的运行时配置。
#[derive(Clone, Debug)]
pub struct RuntimeOptions {
    /// 连接资源上限。
    pub limits: Limits,
    /// Driver 开始读取前安装的 All 订阅配额。
    pub initial_events: SubscriptionOptions,
    /// 从请求退出起计算的传输关闭总预算。
    pub close_timeout: Duration,
}
impl Default for RuntimeOptions {
    fn default() -> Self {
        Self {
            limits: Limits::default(),
            initial_events: SubscriptionOptions::default(),
            close_timeout: Duration::from_secs(5),
        }
    }
}
/// 本地连接的单向生命周期。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ConnectionState {
    /// 允许新请求与订阅。
    Open,
    /// 已关闭 gate，正在释放传输。
    Closing,
    /// Driver 拥有的资源已经释放。
    Closed,
}
/// 传输终止方式，不说明浏览器进程是否退出。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TransportEnd {
    /// 关闭操作成功完成。
    Graceful,
    /// 超时或强制取消后销毁传输。
    Aborted,
    /// IO 或 Driver 意外失败。
    Failed,
}
/// 共享关闭结果；用户持有的数据和许可不算已回收。
#[derive(Clone, Debug)]
pub struct CloseReport {
    /// 第一个终结者记录的原因。
    pub reason: Error,
    /// 关闭期间按观察顺序记录的额外传输错误，读写各至多一个、合计至多两个；不覆盖首次原因，不记录参数。
    pub supplemental_errors: Vec<Error>,
    /// 传输结束方式。
    pub transport_end: TransportEnd,
    /// 关闭时取走的 pending 数量。
    pub pending_count: usize,
    /// 关闭时丢弃的尚未写入帧数量。
    pub discarded_outbound: usize,
    /// 传输和 Driver 所有的队列句柄均已释放。
    pub driver_resources_released: bool,
}
