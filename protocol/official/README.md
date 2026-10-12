# 固定官方输入

源自 [ChromeDevTools/devtools-protocol](https://github.com/ChromeDevTools/devtools-protocol/tree/90778954a4820558eb0a98194e89000d40087ba4)，revision `90778954a4820558eb0a98194e89000d40087ba4`，与设计研究采用的 revision 相同。[snapshot.json](snapshot.json) 固定全部 JSON、PDL/include 和上游 [LICENSE](LICENSE) 的原始字节摘要。

`protocol.toml` 和 `protocol-pdl.toml` 分别验证同一快照的 JSON/PDL 输入。原始英文描述保留在输入和生成文档中，不由生成器在线翻译，也不要求中文翻译目录齐全。`docs_zh` 仅为可选兼容配置，已提供条目仍校验摘要和非空文本。正式官方绑定须通过全量生成/编译、输入摘要校验及两种输入的 wire 语义等价；正式生成通过不等于 SDK 完成运行时、上层集成、fuzz/soak 和平台发布验收。

源仓库 revision、schema 版本与实测 Chromium 版本分别记录，不能把 `1.3` 当作精确浏览器版本。
