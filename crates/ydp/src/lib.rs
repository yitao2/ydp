//! 固定官方 CDP 快照的生成绑定及异步通信核心。
//!
//! 协议描述保留上游原文；生成一致性与 SDK 发布验收分别检查。
//! 普通构建不运行生成器、不读取 schema、不访问网络。
#![forbid(unsafe_code)]

/// 与自定义绑定共用的协议通信核心；不维护浏览器语义状态。
pub use ydp_core as core;
/// 通用连接、Session、请求和事件契约；与独立 Vendor 绑定使用相同运行时。
pub use ydp_core::*;

/// 固定官方输入生成的全部协议域；描述保留上游原文。
pub mod generated;
/// 官方 Domain 与薄 Session 扩展；无自动 enable 或浏览器启动行为。
pub use generated::*;
