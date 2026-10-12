///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Browser.command.addMockCamera/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct AddMockCameraParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Browser.command.addMockCamera/parameters/deviceId。
    ///
    ///协议原文：
    ///
    ///<pre>Required non-empty identifier for the mock camera. This is mapped to an
    ///internal virtual-device identifier and is not the MediaDeviceInfo.deviceId
    ///exposed to the page.</pre>
    ///
    ///
    #[serde(rename = "deviceId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub device_id: String,
}
impl<'de> serde::Deserialize<'de> for AddMockCameraParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = AddMockCameraParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Browser.command.addMockCamera/parameters/deviceId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Required non-empty identifier for the mock camera. This is mapped to an
                    ///internal virtual-device identifier and is not the MediaDeviceInfo.deviceId
                    ///exposed to the page.</pre>
                    ///
                    ///
                    #[serde(rename = "deviceId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub device_id: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(AddMockCameraParams {
                    device_id: fields.device_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Browser.command.addMockCamera。
///
///协议原文：
///
///<pre>Adds or updates a mock camera in the shared video capture device list for
///test automation. The mock camera is not scoped to a particular page or
///frame and is removed when the DevTools session that created it disconnects.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct AddMockCamera;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Browser.command.addMockCamera/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct AddMockCameraResponse {}
impl<'de> serde::Deserialize<'de> for AddMockCameraResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = AddMockCameraResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {}
                let _fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(AddMockCameraResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for AddMockCamera {
    type Params = AddMockCameraParams;
    type Response = AddMockCameraResponse;
    const METHOD: &'static str = "Browser.addMockCamera";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Browser.command.addPrivacySandboxEnrollmentOverride/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct AddPrivacySandboxEnrollmentOverrideParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Browser.command.addPrivacySandboxEnrollmentOverride/parameters/url。
    #[serde(rename = "url")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub url: String,
}
impl<'de> serde::Deserialize<'de> for AddPrivacySandboxEnrollmentOverrideParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = AddPrivacySandboxEnrollmentOverrideParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Browser.command.addPrivacySandboxEnrollmentOverride/parameters/url。
                    #[serde(rename = "url")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub url: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(AddPrivacySandboxEnrollmentOverrideParams { url: fields.url })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Browser.command.addPrivacySandboxEnrollmentOverride。
///
///协议原文：
///
///<pre>Allows a site to use privacy sandbox features that require enrollment
///without the site actually being enrolled. Only supported on page targets.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct AddPrivacySandboxEnrollmentOverride;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Browser.command.addPrivacySandboxEnrollmentOverride/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct AddPrivacySandboxEnrollmentOverrideResponse {}
impl<'de> serde::Deserialize<'de> for AddPrivacySandboxEnrollmentOverrideResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = AddPrivacySandboxEnrollmentOverrideResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {}
                let _fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(AddPrivacySandboxEnrollmentOverrideResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for AddPrivacySandboxEnrollmentOverride {
    type Params = AddPrivacySandboxEnrollmentOverrideParams;
    type Response = AddPrivacySandboxEnrollmentOverrideResponse;
    const METHOD: &'static str = "Browser.addPrivacySandboxEnrollmentOverride";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Browser.command.cancelDownload/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct CancelDownloadParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Browser.command.cancelDownload/parameters/guid。
    ///
    ///协议原文：
    ///
    ///<pre>Global unique identifier of the download.</pre>
    ///
    ///
    #[serde(rename = "guid")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub guid: String,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Browser.command.cancelDownload/parameters/browserContextId。
    ///
    ///协议原文：
    ///
    ///<pre>BrowserContext to perform the action in. When omitted, default browser context is used.</pre>
    ///
    ///
    #[serde(rename = "browserContextId")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub browser_context_id: Option<crate::browser::types::BrowserContextId>,
}
impl<'de> serde::Deserialize<'de> for CancelDownloadParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CancelDownloadParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Browser.command.cancelDownload/parameters/guid。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Global unique identifier of the download.</pre>
                    ///
                    ///
                    #[serde(rename = "guid")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub guid: String,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Browser.command.cancelDownload/parameters/browserContextId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>BrowserContext to perform the action in. When omitted, default browser context is used.</pre>
                    ///
                    ///
                    #[serde(rename = "browserContextId")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub browser_context_id: Option<crate::browser::types::BrowserContextId>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(CancelDownloadParams {
                    guid: fields.guid,
                    browser_context_id: fields.browser_context_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Browser.command.cancelDownload。
///
///协议原文：
///
///<pre>Cancel a download if in progress</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct CancelDownload;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Browser.command.cancelDownload/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct CancelDownloadResponse {}
impl<'de> serde::Deserialize<'de> for CancelDownloadResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CancelDownloadResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {}
                let _fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(CancelDownloadResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for CancelDownload {
    type Params = CancelDownloadParams;
    type Response = CancelDownloadResponse;
    const METHOD: &'static str = "Browser.cancelDownload";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Browser.command.close/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct CloseParams {}
impl<'de> serde::Deserialize<'de> for CloseParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CloseParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {}
                let _fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(CloseParams {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Browser.command.close。
///
///协议原文：
///
///<pre>Close browser gracefully.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct Close;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Browser.command.close/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct CloseResponse {}
impl<'de> serde::Deserialize<'de> for CloseResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CloseResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {}
                let _fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(CloseResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for Close {
    type Params = CloseParams;
    type Response = CloseResponse;
    const METHOD: &'static str = "Browser.close";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Browser.command.crash/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct CrashParams {}
impl<'de> serde::Deserialize<'de> for CrashParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CrashParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {}
                let _fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(CrashParams {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Browser.command.crash。
///
///协议原文：
///
///<pre>Crashes browser on the main thread.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct Crash;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Browser.command.crash/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct CrashResponse {}
impl<'de> serde::Deserialize<'de> for CrashResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CrashResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {}
                let _fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(CrashResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for Crash {
    type Params = CrashParams;
    type Response = CrashResponse;
    const METHOD: &'static str = "Browser.crash";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Browser.command.crashGpuProcess/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct CrashGpuProcessParams {}
impl<'de> serde::Deserialize<'de> for CrashGpuProcessParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CrashGpuProcessParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {}
                let _fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(CrashGpuProcessParams {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Browser.command.crashGpuProcess。
///
///协议原文：
///
///<pre>Crashes GPU process.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct CrashGpuProcess;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Browser.command.crashGpuProcess/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct CrashGpuProcessResponse {}
impl<'de> serde::Deserialize<'de> for CrashGpuProcessResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CrashGpuProcessResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {}
                let _fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(CrashGpuProcessResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for CrashGpuProcess {
    type Params = CrashGpuProcessParams;
    type Response = CrashGpuProcessResponse;
    const METHOD: &'static str = "Browser.crashGpuProcess";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Browser.command.executeBrowserCommand/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct ExecuteBrowserCommandParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Browser.command.executeBrowserCommand/parameters/commandId。
    #[serde(rename = "commandId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub command_id: crate::browser::types::BrowserCommandId,
}
impl<'de> serde::Deserialize<'de> for ExecuteBrowserCommandParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ExecuteBrowserCommandParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Browser.command.executeBrowserCommand/parameters/commandId。
                    #[serde(rename = "commandId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub command_id: crate::browser::types::BrowserCommandId,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(ExecuteBrowserCommandParams {
                    command_id: fields.command_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Browser.command.executeBrowserCommand。
///
///协议原文：
///
///<pre>Invoke custom browser commands used by telemetry.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct ExecuteBrowserCommand;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Browser.command.executeBrowserCommand/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct ExecuteBrowserCommandResponse {}
impl<'de> serde::Deserialize<'de> for ExecuteBrowserCommandResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ExecuteBrowserCommandResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {}
                let _fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(ExecuteBrowserCommandResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for ExecuteBrowserCommand {
    type Params = ExecuteBrowserCommandParams;
    type Response = ExecuteBrowserCommandResponse;
    const METHOD: &'static str = "Browser.executeBrowserCommand";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Browser.command.getBrowserCommandLine/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct GetBrowserCommandLineParams {}
impl<'de> serde::Deserialize<'de> for GetBrowserCommandLineParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetBrowserCommandLineParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {}
                let _fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetBrowserCommandLineParams {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Browser.command.getBrowserCommandLine。
///
///协议原文：
///
///<pre>Returns the command line switches for the browser process if, and only if
///--enable-automation is on the commandline.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct GetBrowserCommandLine;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Browser.command.getBrowserCommandLine/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct GetBrowserCommandLineResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Browser.command.getBrowserCommandLine/returns/arguments。
    ///
    ///协议原文：
    ///
    ///<pre>Commandline parameters</pre>
    ///
    ///
    #[serde(rename = "arguments")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub arguments: Vec<String>,
}
impl<'de> serde::Deserialize<'de> for GetBrowserCommandLineResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetBrowserCommandLineResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Browser.command.getBrowserCommandLine/returns/arguments。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Commandline parameters</pre>
                    ///
                    ///
                    #[serde(rename = "arguments")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub arguments: Vec<String>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetBrowserCommandLineResponse {
                    arguments: fields.arguments,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for GetBrowserCommandLine {
    type Params = GetBrowserCommandLineParams;
    type Response = GetBrowserCommandLineResponse;
    const METHOD: &'static str = "Browser.getBrowserCommandLine";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Browser.command.getGlobalPrivacyControl/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct GetGlobalPrivacyControlParams {}
impl<'de> serde::Deserialize<'de> for GetGlobalPrivacyControlParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetGlobalPrivacyControlParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {}
                let _fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetGlobalPrivacyControlParams {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Browser.command.getGlobalPrivacyControl。
///
///协议原文：
///
///<pre>Gets the current globally-applied privacy control status
///See https://www.w3.org/TR/gpc/#get-global-privacy-control</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct GetGlobalPrivacyControl;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Browser.command.getGlobalPrivacyControl/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct GetGlobalPrivacyControlResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Browser.command.getGlobalPrivacyControl/returns/gpc。
    #[serde(rename = "gpc")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub gpc: bool,
}
impl<'de> serde::Deserialize<'de> for GetGlobalPrivacyControlResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetGlobalPrivacyControlResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Browser.command.getGlobalPrivacyControl/returns/gpc。
                    #[serde(rename = "gpc")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub gpc: bool,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetGlobalPrivacyControlResponse { gpc: fields.gpc })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for GetGlobalPrivacyControl {
    type Params = GetGlobalPrivacyControlParams;
    type Response = GetGlobalPrivacyControlResponse;
    const METHOD: &'static str = "Browser.getGlobalPrivacyControl";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Browser.command.getHistogram/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct GetHistogramParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Browser.command.getHistogram/parameters/name。
    ///
    ///协议原文：
    ///
    ///<pre>Requested histogram name.</pre>
    ///
    ///
    #[serde(rename = "name")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub name: String,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Browser.command.getHistogram/parameters/delta。
    ///
    ///协议原文：
    ///
    ///<pre>If true, retrieve delta since last delta call.</pre>
    ///
    ///
    #[serde(rename = "delta")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub delta: Option<bool>,
}
impl<'de> serde::Deserialize<'de> for GetHistogramParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetHistogramParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Browser.command.getHistogram/parameters/name。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Requested histogram name.</pre>
                    ///
                    ///
                    #[serde(rename = "name")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub name: String,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Browser.command.getHistogram/parameters/delta。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>If true, retrieve delta since last delta call.</pre>
                    ///
                    ///
                    #[serde(rename = "delta")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub delta: Option<bool>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetHistogramParams {
                    name: fields.name,
                    delta: fields.delta,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Browser.command.getHistogram。
///
///协议原文：
///
///<pre>Get a Chrome histogram by name.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct GetHistogram;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Browser.command.getHistogram/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct GetHistogramResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Browser.command.getHistogram/returns/histogram。
    ///
    ///协议原文：
    ///
    ///<pre>Histogram.</pre>
    ///
    ///
    #[serde(rename = "histogram")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub histogram: crate::browser::types::Histogram,
}
impl<'de> serde::Deserialize<'de> for GetHistogramResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetHistogramResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Browser.command.getHistogram/returns/histogram。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Histogram.</pre>
                    ///
                    ///
                    #[serde(rename = "histogram")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub histogram: crate::browser::types::Histogram,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetHistogramResponse {
                    histogram: fields.histogram,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for GetHistogram {
    type Params = GetHistogramParams;
    type Response = GetHistogramResponse;
    const METHOD: &'static str = "Browser.getHistogram";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Browser.command.getHistograms/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct GetHistogramsParams {
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Browser.command.getHistograms/parameters/query。
    ///
    ///协议原文：
    ///
    ///<pre>Requested substring in name. Only histograms which have query as a
    ///substring in their name are extracted. An empty or absent query returns
    ///all histograms.</pre>
    ///
    ///
    #[serde(rename = "query")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub query: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Browser.command.getHistograms/parameters/delta。
    ///
    ///协议原文：
    ///
    ///<pre>If true, retrieve delta since last delta call.</pre>
    ///
    ///
    #[serde(rename = "delta")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub delta: Option<bool>,
}
impl<'de> serde::Deserialize<'de> for GetHistogramsParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetHistogramsParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Browser.command.getHistograms/parameters/query。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Requested substring in name. Only histograms which have query as a
                    ///substring in their name are extracted. An empty or absent query returns
                    ///all histograms.</pre>
                    ///
                    ///
                    #[serde(rename = "query")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub query: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Browser.command.getHistograms/parameters/delta。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>If true, retrieve delta since last delta call.</pre>
                    ///
                    ///
                    #[serde(rename = "delta")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub delta: Option<bool>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetHistogramsParams {
                    query: fields.query,
                    delta: fields.delta,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Browser.command.getHistograms。
///
///协议原文：
///
///<pre>Get Chrome histograms.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct GetHistograms;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Browser.command.getHistograms/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct GetHistogramsResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Browser.command.getHistograms/returns/histograms。
    ///
    ///协议原文：
    ///
    ///<pre>Histograms.</pre>
    ///
    ///
    #[serde(rename = "histograms")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub histograms: Vec<crate::browser::types::Histogram>,
}
impl<'de> serde::Deserialize<'de> for GetHistogramsResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetHistogramsResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Browser.command.getHistograms/returns/histograms。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Histograms.</pre>
                    ///
                    ///
                    #[serde(rename = "histograms")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub histograms: Vec<crate::browser::types::Histogram>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetHistogramsResponse {
                    histograms: fields.histograms,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for GetHistograms {
    type Params = GetHistogramsParams;
    type Response = GetHistogramsResponse;
    const METHOD: &'static str = "Browser.getHistograms";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Browser.command.getVersion/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct GetVersionParams {}
impl<'de> serde::Deserialize<'de> for GetVersionParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetVersionParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {}
                let _fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetVersionParams {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Browser.command.getVersion。
///
///协议原文：
///
///<pre>Returns version information.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct GetVersion;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Browser.command.getVersion/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct GetVersionResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Browser.command.getVersion/returns/protocolVersion。
    ///
    ///协议原文：
    ///
    ///<pre>Protocol version.</pre>
    ///
    ///
    #[serde(rename = "protocolVersion")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub protocol_version: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Browser.command.getVersion/returns/product。
    ///
    ///协议原文：
    ///
    ///<pre>Product name.</pre>
    ///
    ///
    #[serde(rename = "product")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub product: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Browser.command.getVersion/returns/revision。
    ///
    ///协议原文：
    ///
    ///<pre>Product revision.</pre>
    ///
    ///
    #[serde(rename = "revision")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub revision: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Browser.command.getVersion/returns/userAgent。
    ///
    ///协议原文：
    ///
    ///<pre>User-Agent.</pre>
    ///
    ///
    #[serde(rename = "userAgent")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub user_agent: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Browser.command.getVersion/returns/jsVersion。
    ///
    ///协议原文：
    ///
    ///<pre>V8 version.</pre>
    ///
    ///
    #[serde(rename = "jsVersion")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub js_version: String,
}
impl<'de> serde::Deserialize<'de> for GetVersionResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetVersionResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Browser.command.getVersion/returns/protocolVersion。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Protocol version.</pre>
                    ///
                    ///
                    #[serde(rename = "protocolVersion")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub protocol_version: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Browser.command.getVersion/returns/product。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Product name.</pre>
                    ///
                    ///
                    #[serde(rename = "product")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub product: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Browser.command.getVersion/returns/revision。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Product revision.</pre>
                    ///
                    ///
                    #[serde(rename = "revision")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub revision: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Browser.command.getVersion/returns/userAgent。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>User-Agent.</pre>
                    ///
                    ///
                    #[serde(rename = "userAgent")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub user_agent: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Browser.command.getVersion/returns/jsVersion。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>V8 version.</pre>
                    ///
                    ///
                    #[serde(rename = "jsVersion")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub js_version: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetVersionResponse {
                    protocol_version: fields.protocol_version,
                    product: fields.product,
                    revision: fields.revision,
                    user_agent: fields.user_agent,
                    js_version: fields.js_version,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for GetVersion {
    type Params = GetVersionParams;
    type Response = GetVersionResponse;
    const METHOD: &'static str = "Browser.getVersion";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Browser.command.getWindowBounds/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct GetWindowBoundsParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Browser.command.getWindowBounds/parameters/windowId。
    ///
    ///协议原文：
    ///
    ///<pre>Browser window id.</pre>
    ///
    ///
    #[serde(rename = "windowId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub window_id: crate::browser::types::WindowId,
}
impl<'de> serde::Deserialize<'de> for GetWindowBoundsParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetWindowBoundsParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Browser.command.getWindowBounds/parameters/windowId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Browser window id.</pre>
                    ///
                    ///
                    #[serde(rename = "windowId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub window_id: crate::browser::types::WindowId,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetWindowBoundsParams {
                    window_id: fields.window_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Browser.command.getWindowBounds。
///
///协议原文：
///
///<pre>Get position and size of the browser window.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct GetWindowBounds;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Browser.command.getWindowBounds/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct GetWindowBoundsResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Browser.command.getWindowBounds/returns/bounds。
    ///
    ///协议原文：
    ///
    ///<pre>Bounds information of the window. When window state is 'minimized', the restored window
    ///position and size are returned.</pre>
    ///
    ///
    #[serde(rename = "bounds")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub bounds: crate::browser::types::Bounds,
}
impl<'de> serde::Deserialize<'de> for GetWindowBoundsResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetWindowBoundsResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Browser.command.getWindowBounds/returns/bounds。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Bounds information of the window. When window state is 'minimized', the restored window
                    ///position and size are returned.</pre>
                    ///
                    ///
                    #[serde(rename = "bounds")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub bounds: crate::browser::types::Bounds,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetWindowBoundsResponse {
                    bounds: fields.bounds,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for GetWindowBounds {
    type Params = GetWindowBoundsParams;
    type Response = GetWindowBoundsResponse;
    const METHOD: &'static str = "Browser.getWindowBounds";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Browser.command.getWindowForTarget/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct GetWindowForTargetParams {
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Browser.command.getWindowForTarget/parameters/targetId。
    ///
    ///协议原文：
    ///
    ///<pre>Devtools agent host id. If called as a part of the session, associated targetId is used.</pre>
    ///
    ///
    #[serde(rename = "targetId")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub target_id: Option<crate::target::types::TargetId>,
}
impl<'de> serde::Deserialize<'de> for GetWindowForTargetParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetWindowForTargetParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Browser.command.getWindowForTarget/parameters/targetId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Devtools agent host id. If called as a part of the session, associated targetId is used.</pre>
                    ///
                    ///
                    #[serde(rename = "targetId")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub target_id: Option<crate::target::types::TargetId>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetWindowForTargetParams {
                    target_id: fields.target_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Browser.command.getWindowForTarget。
///
///协议原文：
///
///<pre>Get the browser window that contains the devtools target.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct GetWindowForTarget;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Browser.command.getWindowForTarget/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct GetWindowForTargetResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Browser.command.getWindowForTarget/returns/windowId。
    ///
    ///协议原文：
    ///
    ///<pre>Browser window id.</pre>
    ///
    ///
    #[serde(rename = "windowId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub window_id: crate::browser::types::WindowId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Browser.command.getWindowForTarget/returns/bounds。
    ///
    ///协议原文：
    ///
    ///<pre>Bounds information of the window. When window state is 'minimized', the restored window
    ///position and size are returned.</pre>
    ///
    ///
    #[serde(rename = "bounds")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub bounds: crate::browser::types::Bounds,
}
impl<'de> serde::Deserialize<'de> for GetWindowForTargetResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetWindowForTargetResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Browser.command.getWindowForTarget/returns/windowId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Browser window id.</pre>
                    ///
                    ///
                    #[serde(rename = "windowId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub window_id: crate::browser::types::WindowId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Browser.command.getWindowForTarget/returns/bounds。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Bounds information of the window. When window state is 'minimized', the restored window
                    ///position and size are returned.</pre>
                    ///
                    ///
                    #[serde(rename = "bounds")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub bounds: crate::browser::types::Bounds,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetWindowForTargetResponse {
                    window_id: fields.window_id,
                    bounds: fields.bounds,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for GetWindowForTarget {
    type Params = GetWindowForTargetParams;
    type Response = GetWindowForTargetResponse;
    const METHOD: &'static str = "Browser.getWindowForTarget";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Browser.command.grantPermissions/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct GrantPermissionsParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Browser.command.grantPermissions/parameters/permissions。
    #[serde(rename = "permissions")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub permissions: Vec<crate::browser::types::PermissionType>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Browser.command.grantPermissions/parameters/origin。
    ///
    ///协议原文：
    ///
    ///<pre>Origin the permission applies to, all origins if not specified.</pre>
    ///
    ///
    #[serde(rename = "origin")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub origin: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Browser.command.grantPermissions/parameters/browserContextId。
    ///
    ///协议原文：
    ///
    ///<pre>BrowserContext to override permissions. When omitted, default browser context is used.</pre>
    ///
    ///
    #[serde(rename = "browserContextId")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub browser_context_id: Option<crate::browser::types::BrowserContextId>,
}
impl<'de> serde::Deserialize<'de> for GrantPermissionsParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GrantPermissionsParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Browser.command.grantPermissions/parameters/permissions。
                    #[serde(rename = "permissions")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub permissions: Vec<crate::browser::types::PermissionType>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Browser.command.grantPermissions/parameters/origin。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Origin the permission applies to, all origins if not specified.</pre>
                    ///
                    ///
                    #[serde(rename = "origin")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub origin: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Browser.command.grantPermissions/parameters/browserContextId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>BrowserContext to override permissions. When omitted, default browser context is used.</pre>
                    ///
                    ///
                    #[serde(rename = "browserContextId")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub browser_context_id: Option<crate::browser::types::BrowserContextId>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GrantPermissionsParams {
                    permissions: fields.permissions,
                    origin: fields.origin,
                    browser_context_id: fields.browser_context_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Browser.command.grantPermissions。
///
///协议原文：
///
///<pre>Grant specific permissions to the given origin and reject all others. Deprecated. Use
///setPermission instead.</pre>
///
///此项为 experimental，端点可能不支持。此项已 deprecated，保留用于旧端点兼容。
#[derive(Debug, Clone, Copy)]
pub struct GrantPermissions;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Browser.command.grantPermissions/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct GrantPermissionsResponse {}
impl<'de> serde::Deserialize<'de> for GrantPermissionsResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GrantPermissionsResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {}
                let _fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GrantPermissionsResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for GrantPermissions {
    type Params = GrantPermissionsParams;
    type Response = GrantPermissionsResponse;
    const METHOD: &'static str = "Browser.grantPermissions";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Browser.command.resetPermissions/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct ResetPermissionsParams {
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Browser.command.resetPermissions/parameters/browserContextId。
    ///
    ///协议原文：
    ///
    ///<pre>BrowserContext to reset permissions. When omitted, default browser context is used.</pre>
    ///
    ///
    #[serde(rename = "browserContextId")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub browser_context_id: Option<crate::browser::types::BrowserContextId>,
}
impl<'de> serde::Deserialize<'de> for ResetPermissionsParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ResetPermissionsParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Browser.command.resetPermissions/parameters/browserContextId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>BrowserContext to reset permissions. When omitted, default browser context is used.</pre>
                    ///
                    ///
                    #[serde(rename = "browserContextId")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub browser_context_id: Option<crate::browser::types::BrowserContextId>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(ResetPermissionsParams {
                    browser_context_id: fields.browser_context_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Browser.command.resetPermissions。
///
///协议原文：
///
///<pre>Reset all permission management for all origins.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct ResetPermissions;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Browser.command.resetPermissions/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct ResetPermissionsResponse {}
impl<'de> serde::Deserialize<'de> for ResetPermissionsResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ResetPermissionsResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {}
                let _fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(ResetPermissionsResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for ResetPermissions {
    type Params = ResetPermissionsParams;
    type Response = ResetPermissionsResponse;
    const METHOD: &'static str = "Browser.resetPermissions";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Browser.command.setContentsSize/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetContentsSizeParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Browser.command.setContentsSize/parameters/windowId。
    ///
    ///协议原文：
    ///
    ///<pre>Browser window id.</pre>
    ///
    ///
    #[serde(rename = "windowId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub window_id: crate::browser::types::WindowId,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Browser.command.setContentsSize/parameters/width。
    ///
    ///协议原文：
    ///
    ///<pre>The window contents width in DIP. Assumes current width if omitted.
    ///Must be specified if 'height' is omitted.</pre>
    ///
    ///
    #[serde(rename = "width")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub width: Option<i64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Browser.command.setContentsSize/parameters/height。
    ///
    ///协议原文：
    ///
    ///<pre>The window contents height in DIP. Assumes current height if omitted.
    ///Must be specified if 'width' is omitted.</pre>
    ///
    ///
    #[serde(rename = "height")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub height: Option<i64>,
}
impl<'de> serde::Deserialize<'de> for SetContentsSizeParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetContentsSizeParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Browser.command.setContentsSize/parameters/windowId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Browser window id.</pre>
                    ///
                    ///
                    #[serde(rename = "windowId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub window_id: crate::browser::types::WindowId,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Browser.command.setContentsSize/parameters/width。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The window contents width in DIP. Assumes current width if omitted.
                    ///Must be specified if 'height' is omitted.</pre>
                    ///
                    ///
                    #[serde(rename = "width")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub width: Option<i64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Browser.command.setContentsSize/parameters/height。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The window contents height in DIP. Assumes current height if omitted.
                    ///Must be specified if 'width' is omitted.</pre>
                    ///
                    ///
                    #[serde(rename = "height")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub height: Option<i64>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SetContentsSizeParams {
                    window_id: fields.window_id,
                    width: fields.width,
                    height: fields.height,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Browser.command.setContentsSize。
///
///协议原文：
///
///<pre>Set size of the browser contents resizing browser window as necessary.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct SetContentsSize;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Browser.command.setContentsSize/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetContentsSizeResponse {}
impl<'de> serde::Deserialize<'de> for SetContentsSizeResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetContentsSizeResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {}
                let _fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SetContentsSizeResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for SetContentsSize {
    type Params = SetContentsSizeParams;
    type Response = SetContentsSizeResponse;
    const METHOD: &'static str = "Browser.setContentsSize";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Browser.command.setDockTile/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct SetDockTileParams {
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Browser.command.setDockTile/parameters/badgeLabel。
    #[serde(rename = "badgeLabel")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub badge_label: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Browser.command.setDockTile/parameters/image。
    ///
    ///协议原文：
    ///
    ///<pre>Png encoded image. (Encoded as a base64 string when passed over JSON)</pre>
    ///
    ///
    #[serde(rename = "image")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub image: Option<String>,
}
impl<'de> serde::Deserialize<'de> for SetDockTileParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetDockTileParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Browser.command.setDockTile/parameters/badgeLabel。
                    #[serde(rename = "badgeLabel")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub badge_label: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Browser.command.setDockTile/parameters/image。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Png encoded image. (Encoded as a base64 string when passed over JSON)</pre>
                    ///
                    ///
                    #[serde(rename = "image")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub image: Option<String>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SetDockTileParams {
                    badge_label: fields.badge_label,
                    image: fields.image,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Browser.command.setDockTile。
///
///协议原文：
///
///<pre>Set dock tile details, platform-specific.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct SetDockTile;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Browser.command.setDockTile/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetDockTileResponse {}
impl<'de> serde::Deserialize<'de> for SetDockTileResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetDockTileResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {}
                let _fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SetDockTileResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for SetDockTile {
    type Params = SetDockTileParams;
    type Response = SetDockTileResponse;
    const METHOD: &'static str = "Browser.setDockTile";
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Browser.command.setDownloadBehavior/parameters/behavior。
///
///协议原文：
///
///<pre>Whether to allow all or deny all download requests, or use default Chrome behavior if
///available (otherwise deny). |allowAndName| allows download and names files according to
///their download guids.</pre>
///
///
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum SetDownloadBehaviorParamsBehavior {
    ///协议值 "deny"。
    Deny,
    ///协议值 "allow"。
    Allow,
    ///协议值 "allowAndName"。
    AllowAndName,
    ///协议值 "default"。
    Default,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl SetDownloadBehaviorParamsBehavior {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::browser::commands::SetDownloadBehaviorParamsBehavior::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::Deny => "deny",
            Self::Allow => "allow",
            Self::AllowAndName => "allowAndName",
            Self::Default => "default",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for SetDownloadBehaviorParamsBehavior {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for SetDownloadBehaviorParamsBehavior {
    fn from(value: String) -> Self {
        match value.as_str() {
            "deny" => SetDownloadBehaviorParamsBehavior::Deny,
            "allow" => SetDownloadBehaviorParamsBehavior::Allow,
            "allowAndName" => SetDownloadBehaviorParamsBehavior::AllowAndName,
            "default" => SetDownloadBehaviorParamsBehavior::Default,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for SetDownloadBehaviorParamsBehavior {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for SetDownloadBehaviorParamsBehavior {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetDownloadBehaviorParamsBehavior;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "deny" => SetDownloadBehaviorParamsBehavior::Deny,
                    "allow" => SetDownloadBehaviorParamsBehavior::Allow,
                    "allowAndName" => SetDownloadBehaviorParamsBehavior::AllowAndName,
                    "default" => SetDownloadBehaviorParamsBehavior::Default,
                    _ => SetDownloadBehaviorParamsBehavior::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(SetDownloadBehaviorParamsBehavior::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Browser.command.setDownloadBehavior/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetDownloadBehaviorParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Browser.command.setDownloadBehavior/parameters/behavior。
    ///
    ///协议原文：
    ///
    ///<pre>Whether to allow all or deny all download requests, or use default Chrome behavior if
    ///available (otherwise deny). |allowAndName| allows download and names files according to
    ///their download guids.</pre>
    ///
    ///
    #[serde(rename = "behavior")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub behavior: SetDownloadBehaviorParamsBehavior,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Browser.command.setDownloadBehavior/parameters/browserContextId。
    ///
    ///协议原文：
    ///
    ///<pre>BrowserContext to set download behavior. When omitted, default browser context is used.</pre>
    ///
    ///
    #[serde(rename = "browserContextId")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub browser_context_id: Option<crate::browser::types::BrowserContextId>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Browser.command.setDownloadBehavior/parameters/downloadPath。
    ///
    ///协议原文：
    ///
    ///<pre>The default path to save downloaded files to. This is required if behavior is set to 'allow'
    ///or 'allowAndName'.</pre>
    ///
    ///
    #[serde(rename = "downloadPath")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub download_path: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Browser.command.setDownloadBehavior/parameters/eventsEnabled。
    ///
    ///协议原文：
    ///
    ///<pre>Whether to emit download events (defaults to false).</pre>
    ///
    ///
    #[serde(rename = "eventsEnabled")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub events_enabled: Option<bool>,
}
impl<'de> serde::Deserialize<'de> for SetDownloadBehaviorParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetDownloadBehaviorParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Browser.command.setDownloadBehavior/parameters/behavior。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Whether to allow all or deny all download requests, or use default Chrome behavior if
                    ///available (otherwise deny). |allowAndName| allows download and names files according to
                    ///their download guids.</pre>
                    ///
                    ///
                    #[serde(rename = "behavior")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub behavior: SetDownloadBehaviorParamsBehavior,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Browser.command.setDownloadBehavior/parameters/browserContextId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>BrowserContext to set download behavior. When omitted, default browser context is used.</pre>
                    ///
                    ///
                    #[serde(rename = "browserContextId")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub browser_context_id: Option<crate::browser::types::BrowserContextId>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Browser.command.setDownloadBehavior/parameters/downloadPath。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The default path to save downloaded files to. This is required if behavior is set to 'allow'
                    ///or 'allowAndName'.</pre>
                    ///
                    ///
                    #[serde(rename = "downloadPath")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub download_path: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Browser.command.setDownloadBehavior/parameters/eventsEnabled。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Whether to emit download events (defaults to false).</pre>
                    ///
                    ///
                    #[serde(rename = "eventsEnabled")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub events_enabled: Option<bool>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SetDownloadBehaviorParams {
                    behavior: fields.behavior,
                    browser_context_id: fields.browser_context_id,
                    download_path: fields.download_path,
                    events_enabled: fields.events_enabled,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Browser.command.setDownloadBehavior。
///
///协议原文：
///
///<pre>Set the behavior when downloading a file.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct SetDownloadBehavior;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Browser.command.setDownloadBehavior/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetDownloadBehaviorResponse {}
impl<'de> serde::Deserialize<'de> for SetDownloadBehaviorResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetDownloadBehaviorResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {}
                let _fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SetDownloadBehaviorResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for SetDownloadBehavior {
    type Params = SetDownloadBehaviorParams;
    type Response = SetDownloadBehaviorResponse;
    const METHOD: &'static str = "Browser.setDownloadBehavior";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Browser.command.setGlobalPrivacyControl/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetGlobalPrivacyControlParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Browser.command.setGlobalPrivacyControl/parameters/gpc。
    #[serde(rename = "gpc")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub gpc: bool,
}
impl<'de> serde::Deserialize<'de> for SetGlobalPrivacyControlParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetGlobalPrivacyControlParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Browser.command.setGlobalPrivacyControl/parameters/gpc。
                    #[serde(rename = "gpc")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub gpc: bool,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SetGlobalPrivacyControlParams { gpc: fields.gpc })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Browser.command.setGlobalPrivacyControl。
///
///协议原文：
///
///<pre>Sets and then gets the current globally-applied privacy control status
///See https://www.w3.org/TR/gpc/#set-global-privacy-control</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct SetGlobalPrivacyControl;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Browser.command.setGlobalPrivacyControl/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetGlobalPrivacyControlResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Browser.command.setGlobalPrivacyControl/returns/gpc。
    #[serde(rename = "gpc")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub gpc: bool,
}
impl<'de> serde::Deserialize<'de> for SetGlobalPrivacyControlResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetGlobalPrivacyControlResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Browser.command.setGlobalPrivacyControl/returns/gpc。
                    #[serde(rename = "gpc")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub gpc: bool,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SetGlobalPrivacyControlResponse { gpc: fields.gpc })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for SetGlobalPrivacyControl {
    type Params = SetGlobalPrivacyControlParams;
    type Response = SetGlobalPrivacyControlResponse;
    const METHOD: &'static str = "Browser.setGlobalPrivacyControl";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Browser.command.setPermission/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetPermissionParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Browser.command.setPermission/parameters/permission。
    ///
    ///协议原文：
    ///
    ///<pre>Descriptor of permission to override.</pre>
    ///
    ///
    #[serde(rename = "permission")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub permission: crate::browser::types::PermissionDescriptor,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Browser.command.setPermission/parameters/setting。
    ///
    ///协议原文：
    ///
    ///<pre>Setting of the permission.</pre>
    ///
    ///
    #[serde(rename = "setting")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub setting: crate::browser::types::PermissionSetting,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Browser.command.setPermission/parameters/origin。
    ///
    ///协议原文：
    ///
    ///<pre>Embedding origin the permission applies to, all origins if not specified.</pre>
    ///
    ///
    #[serde(rename = "origin")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub origin: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Browser.command.setPermission/parameters/embeddedOrigin。
    ///
    ///协议原文：
    ///
    ///<pre>Embedded origin the permission applies to. It is ignored unless the embedding origin is
    ///present and valid. If the embedding origin is provided but the embedded origin isn't, the
    ///embedding origin is used as the embedded origin.</pre>
    ///
    ///
    #[serde(rename = "embeddedOrigin")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub embedded_origin: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Browser.command.setPermission/parameters/browserContextId。
    ///
    ///协议原文：
    ///
    ///<pre>Context to override. When omitted, default browser context is used.</pre>
    ///
    ///
    #[serde(rename = "browserContextId")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub browser_context_id: Option<crate::browser::types::BrowserContextId>,
}
impl<'de> serde::Deserialize<'de> for SetPermissionParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetPermissionParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Browser.command.setPermission/parameters/permission。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Descriptor of permission to override.</pre>
                    ///
                    ///
                    #[serde(rename = "permission")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub permission: crate::browser::types::PermissionDescriptor,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Browser.command.setPermission/parameters/setting。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Setting of the permission.</pre>
                    ///
                    ///
                    #[serde(rename = "setting")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub setting: crate::browser::types::PermissionSetting,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Browser.command.setPermission/parameters/origin。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Embedding origin the permission applies to, all origins if not specified.</pre>
                    ///
                    ///
                    #[serde(rename = "origin")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub origin: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Browser.command.setPermission/parameters/embeddedOrigin。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Embedded origin the permission applies to. It is ignored unless the embedding origin is
                    ///present and valid. If the embedding origin is provided but the embedded origin isn't, the
                    ///embedding origin is used as the embedded origin.</pre>
                    ///
                    ///
                    #[serde(rename = "embeddedOrigin")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub embedded_origin: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Browser.command.setPermission/parameters/browserContextId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Context to override. When omitted, default browser context is used.</pre>
                    ///
                    ///
                    #[serde(rename = "browserContextId")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub browser_context_id: Option<crate::browser::types::BrowserContextId>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SetPermissionParams {
                    permission: fields.permission,
                    setting: fields.setting,
                    origin: fields.origin,
                    embedded_origin: fields.embedded_origin,
                    browser_context_id: fields.browser_context_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Browser.command.setPermission。
///
///协议原文：
///
///<pre>Set permission settings for given embedding and embedded origins.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct SetPermission;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Browser.command.setPermission/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetPermissionResponse {}
impl<'de> serde::Deserialize<'de> for SetPermissionResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetPermissionResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {}
                let _fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SetPermissionResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for SetPermission {
    type Params = SetPermissionParams;
    type Response = SetPermissionResponse;
    const METHOD: &'static str = "Browser.setPermission";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Browser.command.setWindowBounds/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetWindowBoundsParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Browser.command.setWindowBounds/parameters/windowId。
    ///
    ///协议原文：
    ///
    ///<pre>Browser window id.</pre>
    ///
    ///
    #[serde(rename = "windowId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub window_id: crate::browser::types::WindowId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Browser.command.setWindowBounds/parameters/bounds。
    ///
    ///协议原文：
    ///
    ///<pre>New window bounds. The 'minimized', 'maximized' and 'fullscreen' states cannot be combined
    ///with 'left', 'top', 'width' or 'height'. Leaves unspecified fields unchanged.</pre>
    ///
    ///
    #[serde(rename = "bounds")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub bounds: crate::browser::types::Bounds,
}
impl<'de> serde::Deserialize<'de> for SetWindowBoundsParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetWindowBoundsParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Browser.command.setWindowBounds/parameters/windowId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Browser window id.</pre>
                    ///
                    ///
                    #[serde(rename = "windowId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub window_id: crate::browser::types::WindowId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Browser.command.setWindowBounds/parameters/bounds。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>New window bounds. The 'minimized', 'maximized' and 'fullscreen' states cannot be combined
                    ///with 'left', 'top', 'width' or 'height'. Leaves unspecified fields unchanged.</pre>
                    ///
                    ///
                    #[serde(rename = "bounds")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub bounds: crate::browser::types::Bounds,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SetWindowBoundsParams {
                    window_id: fields.window_id,
                    bounds: fields.bounds,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Browser.command.setWindowBounds。
///
///协议原文：
///
///<pre>Set position and/or size of the browser window.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct SetWindowBounds;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Browser.command.setWindowBounds/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetWindowBoundsResponse {}
impl<'de> serde::Deserialize<'de> for SetWindowBoundsResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetWindowBoundsResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {}
                let _fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SetWindowBoundsResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for SetWindowBounds {
    type Params = SetWindowBoundsParams;
    type Response = SetWindowBoundsResponse;
    const METHOD: &'static str = "Browser.setWindowBounds";
}
