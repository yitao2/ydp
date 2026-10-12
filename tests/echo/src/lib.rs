//! 独立 Echo 协议验收包；生成绑定只依赖 ydp-core 与序列化基础库。
#![forbid(unsafe_code)]

/// 由固定 Echo 输入离线生成的绑定；不包含手写命令 marker。
pub mod generated;
/// 在绑定包根导出生成的 Domain 与扩展 trait，保持生成路径稳定。
pub use generated::*;
