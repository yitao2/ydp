//! 独立 Echo 协议验收包；生成绑定只依赖 ydp-core 与序列化基础库。
#![forbid(unsafe_code)]

/// 由固定 Echo 输入离线生成的绑定；不包含手写命令 marker。
///
/// 开放枚举带有 `#[non_exhaustive]`，下游穷尽匹配必须保留通配分支：
///
/// ```compile_fail
/// use ydp_echo_tests::echo::types::Mode;
/// fn classify(value: Mode) {
///     match value {
///         Mode::Ready | Mode::KnownUnknown | Mode::Future | Mode::Unknown(_) => {}
///     }
/// }
/// ```
pub mod generated;
/// 在绑定包根导出生成的 Domain 与扩展 trait，保持生成路径稳定。
pub use generated::*;
