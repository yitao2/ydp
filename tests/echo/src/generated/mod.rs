///Echo 协议域；所有调用共享同一个 core。
pub mod echo;
///生成的薄 Domain 扩展；只借用 Session，不缓存或解释浏览器状态。
pub trait SessionDomains {
    ///借用 Session 创建 Echo 域视图，不分配远端资源。
    ///
    ///```no_run
    ///use ydp_echo_tests::SessionDomains as _;
    ///# fn view(session:&ydp_core::Session) {
    ///let domain = session.echo();
    ///# }
    ///```
    fn echo(&self) -> echo::Domain<'_>;
}
impl SessionDomains for ydp_core::Session {
    fn echo(&self) -> echo::Domain<'_> {
        echo::Domain { session: self }
    }
}
///只读生成来源信息；不包含 runtime 方法查找表。
pub struct GeneratedMetadata {
    ///生成契约版本。
    pub generated_contract_version: u32,
    ///生成器包版本。
    pub generator_version: &'static str,
    ///绑定 crate 名称。
    pub binding_package: &'static str,
    ///每个输入自己的路径、版本和摘要。
    pub inputs: &'static [SourceMetadata],
    ///配置和说明目录的 SHA-256。
    pub configuration_sha256: &'static str,
}
///一个固定源文件的独立 provenance；不编造浏览器版本。
pub struct SourceMetadata {
    ///manifest 相对路径。
    pub path: &'static str,
    ///原始字节 SHA-256。
    pub sha256: &'static str,
    ///源协议主版本字符串。
    pub version_major: &'static str,
    ///源协议次版本字符串。
    pub version_minor: &'static str,
}
///固定生成快照的只读元数据；所有字符串静态存储，无运行时加载或分配。
pub const GENERATED_METADATA: GeneratedMetadata = GeneratedMetadata {
    generated_contract_version: 1,
    generator_version: "0.1.0-alpha.2",
    binding_package: "ydp_echo_tests",
    inputs: &[SourceMetadata {
        path: "echo.json",
        sha256: "3b31c30f07f40521d13ae52d05fa26d5b7480734e13106c8001b762973c4f083",
        version_major: "1",
        version_minor: "0",
    }],
    configuration_sha256: "25d923147d81a63fef945af6ad7ef5f7a31dbbfa0338497ead795c0eb9e77b40",
};
