///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：PWA.command.changeAppUserSettings/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct ChangeAppUserSettingsParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：PWA.command.changeAppUserSettings/parameters/manifestId。
    #[serde(rename = "manifestId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub manifest_id: String,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：PWA.command.changeAppUserSettings/parameters/linkCapturing。
    ///
    ///协议原文：
    ///
    ///<pre>If user allows the links clicked on by the user in the app's scope, or
    ///extended scope if the manifest has scope extensions and the flags
    ///`DesktopPWAsLinkCapturingWithScopeExtensions` and
    ///`WebAppEnableScopeExtensions` are enabled.
    ///
    ///Note, the API does not support resetting the linkCapturing to the
    ///initial value, uninstalling and installing the web app again will reset
    ///it.
    ///
    ///TODO(crbug.com/339453269): Setting this value on ChromeOS is not
    ///supported yet.</pre>
    ///
    ///
    #[serde(rename = "linkCapturing")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub link_capturing: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：PWA.command.changeAppUserSettings/parameters/displayMode。
    #[serde(rename = "displayMode")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub display_mode: Option<crate::pwa::types::DisplayMode>,
}
impl<'de> serde::Deserialize<'de> for ChangeAppUserSettingsParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ChangeAppUserSettingsParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：PWA.command.changeAppUserSettings/parameters/manifestId。
                    #[serde(rename = "manifestId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub manifest_id: String,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：PWA.command.changeAppUserSettings/parameters/linkCapturing。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>If user allows the links clicked on by the user in the app's scope, or
                    ///extended scope if the manifest has scope extensions and the flags
                    ///`DesktopPWAsLinkCapturingWithScopeExtensions` and
                    ///`WebAppEnableScopeExtensions` are enabled.
                    ///
                    ///Note, the API does not support resetting the linkCapturing to the
                    ///initial value, uninstalling and installing the web app again will reset
                    ///it.
                    ///
                    ///TODO(crbug.com/339453269): Setting this value on ChromeOS is not
                    ///supported yet.</pre>
                    ///
                    ///
                    #[serde(rename = "linkCapturing")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub link_capturing: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：PWA.command.changeAppUserSettings/parameters/displayMode。
                    #[serde(rename = "displayMode")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub display_mode: Option<crate::pwa::types::DisplayMode>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(ChangeAppUserSettingsParams {
                    manifest_id: fields.manifest_id,
                    link_capturing: fields.link_capturing,
                    display_mode: fields.display_mode,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：PWA.command.changeAppUserSettings。
///
///协议原文：
///
///<pre>Changes user settings of the web app identified by its manifestId. If the
///app was not installed, this command returns an error. Unset parameters will
///be ignored; unrecognized values will cause an error.
///
///Unlike the ones defined in the manifest files of the web apps, these
///settings are provided by the browser and controlled by the users, they
///impact the way the browser handling the web apps.
///
///See the comment of each parameter.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct ChangeAppUserSettings;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：PWA.command.changeAppUserSettings/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct ChangeAppUserSettingsResponse {}
impl<'de> serde::Deserialize<'de> for ChangeAppUserSettingsResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ChangeAppUserSettingsResponse;
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
                Ok(ChangeAppUserSettingsResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for ChangeAppUserSettings {
    type Params = ChangeAppUserSettingsParams;
    type Response = ChangeAppUserSettingsResponse;
    const METHOD: &'static str = "PWA.changeAppUserSettings";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：PWA.command.getOsAppState/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct GetOsAppStateParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：PWA.command.getOsAppState/parameters/manifestId。
    ///
    ///协议原文：
    ///
    ///<pre>The id from the webapp's manifest file, commonly it's the url of the
    ///site installing the webapp. See
    ///https://web.dev/learn/pwa/web-app-manifest.</pre>
    ///
    ///
    #[serde(rename = "manifestId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub manifest_id: String,
}
impl<'de> serde::Deserialize<'de> for GetOsAppStateParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetOsAppStateParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：PWA.command.getOsAppState/parameters/manifestId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The id from the webapp's manifest file, commonly it's the url of the
                    ///site installing the webapp. See
                    ///https://web.dev/learn/pwa/web-app-manifest.</pre>
                    ///
                    ///
                    #[serde(rename = "manifestId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub manifest_id: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetOsAppStateParams {
                    manifest_id: fields.manifest_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：PWA.command.getOsAppState。
///
///协议原文：
///
///<pre>Returns the following OS state for the given manifest id.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct GetOsAppState;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：PWA.command.getOsAppState/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct GetOsAppStateResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：PWA.command.getOsAppState/returns/badgeCount。
    #[serde(rename = "badgeCount")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub badge_count: i64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：PWA.command.getOsAppState/returns/fileHandlers。
    #[serde(rename = "fileHandlers")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub file_handlers: Vec<crate::pwa::types::FileHandler>,
}
impl<'de> serde::Deserialize<'de> for GetOsAppStateResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetOsAppStateResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：PWA.command.getOsAppState/returns/badgeCount。
                    #[serde(rename = "badgeCount")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub badge_count: i64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：PWA.command.getOsAppState/returns/fileHandlers。
                    #[serde(rename = "fileHandlers")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub file_handlers: Vec<crate::pwa::types::FileHandler>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetOsAppStateResponse {
                    badge_count: fields.badge_count,
                    file_handlers: fields.file_handlers,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for GetOsAppState {
    type Params = GetOsAppStateParams;
    type Response = GetOsAppStateResponse;
    const METHOD: &'static str = "PWA.getOsAppState";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：PWA.command.install/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct InstallParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：PWA.command.install/parameters/manifestId。
    #[serde(rename = "manifestId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub manifest_id: String,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：PWA.command.install/parameters/installUrlOrBundleUrl。
    ///
    ///协议原文：
    ///
    ///<pre>The location of the app or bundle overriding the one derived from the
    ///manifestId.</pre>
    ///
    ///
    #[serde(rename = "installUrlOrBundleUrl")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub install_url_or_bundle_url: Option<String>,
}
impl<'de> serde::Deserialize<'de> for InstallParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = InstallParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：PWA.command.install/parameters/manifestId。
                    #[serde(rename = "manifestId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub manifest_id: String,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：PWA.command.install/parameters/installUrlOrBundleUrl。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The location of the app or bundle overriding the one derived from the
                    ///manifestId.</pre>
                    ///
                    ///
                    #[serde(rename = "installUrlOrBundleUrl")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub install_url_or_bundle_url: Option<String>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(InstallParams {
                    manifest_id: fields.manifest_id,
                    install_url_or_bundle_url: fields.install_url_or_bundle_url,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：PWA.command.install。
///
///协议原文：
///
///<pre>Installs the given manifest identity, optionally using the given installUrlOrBundleUrl
///
///IWA-specific install description:
///manifestId corresponds to isolated-app:// + web_package::SignedWebBundleId
///
///File installation mode:
///The installUrlOrBundleUrl can be either file:// or http(s):// pointing
///to a signed web bundle (.swbn). In this case SignedWebBundleId must correspond to
///The .swbn file's signing key.
///
///Dev proxy installation mode:
///installUrlOrBundleUrl must be http(s):// that serves dev mode IWA.
///web_package::SignedWebBundleId must be of type dev proxy.
///
///The advantage of dev proxy mode is that all changes to IWA
///automatically will be reflected in the running app without
///reinstallation.
///
///To generate bundle id for proxy mode:
///1. Generate 32 random bytes.
///2. Add a specific suffix at the end following the documentation
///   https://github.com/WICG/isolated-web-apps/blob/main/Scheme.md#suffix
///3. Encode the entire sequence using Base32 without padding.
///
///If Chrome is not in IWA dev
///mode, the installation will fail, regardless of the state of the allowlist.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct Install;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：PWA.command.install/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct InstallResponse {}
impl<'de> serde::Deserialize<'de> for InstallResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = InstallResponse;
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
                Ok(InstallResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for Install {
    type Params = InstallParams;
    type Response = InstallResponse;
    const METHOD: &'static str = "PWA.install";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：PWA.command.launch/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct LaunchParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：PWA.command.launch/parameters/manifestId。
    #[serde(rename = "manifestId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub manifest_id: String,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：PWA.command.launch/parameters/url。
    #[serde(rename = "url")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub url: Option<String>,
}
impl<'de> serde::Deserialize<'de> for LaunchParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = LaunchParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：PWA.command.launch/parameters/manifestId。
                    #[serde(rename = "manifestId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub manifest_id: String,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：PWA.command.launch/parameters/url。
                    #[serde(rename = "url")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub url: Option<String>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(LaunchParams {
                    manifest_id: fields.manifest_id,
                    url: fields.url,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：PWA.command.launch。
///
///协议原文：
///
///<pre>Launches the installed web app, or an url in the same web app instead of the
///default start url if it is provided. Returns a page Target.TargetID which
///can be used to attach to via Target.attachToTarget or similar APIs.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct Launch;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：PWA.command.launch/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct LaunchResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：PWA.command.launch/returns/targetId。
    ///
    ///协议原文：
    ///
    ///<pre>ID of the tab target created as a result.</pre>
    ///
    ///
    #[serde(rename = "targetId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub target_id: crate::target::types::TargetId,
}
impl<'de> serde::Deserialize<'de> for LaunchResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = LaunchResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：PWA.command.launch/returns/targetId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>ID of the tab target created as a result.</pre>
                    ///
                    ///
                    #[serde(rename = "targetId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub target_id: crate::target::types::TargetId,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(LaunchResponse {
                    target_id: fields.target_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for Launch {
    type Params = LaunchParams;
    type Response = LaunchResponse;
    const METHOD: &'static str = "PWA.launch";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：PWA.command.launchFilesInApp/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct LaunchFilesInAppParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：PWA.command.launchFilesInApp/parameters/manifestId。
    #[serde(rename = "manifestId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub manifest_id: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：PWA.command.launchFilesInApp/parameters/files。
    #[serde(rename = "files")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub files: Vec<String>,
}
impl<'de> serde::Deserialize<'de> for LaunchFilesInAppParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = LaunchFilesInAppParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：PWA.command.launchFilesInApp/parameters/manifestId。
                    #[serde(rename = "manifestId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub manifest_id: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：PWA.command.launchFilesInApp/parameters/files。
                    #[serde(rename = "files")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub files: Vec<String>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(LaunchFilesInAppParams {
                    manifest_id: fields.manifest_id,
                    files: fields.files,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：PWA.command.launchFilesInApp。
///
///协议原文：
///
///<pre>Opens one or more local files from an installed web app identified by its
///manifestId. The web app needs to have file handlers registered to process
///the files. The API returns one or more page Target.TargetIDs which can be
///used to attach to via Target.attachToTarget or similar APIs.
///If some files in the parameters cannot be handled by the web app, they will
///be ignored. If none of the files can be handled, this API returns an error.
///If no files are provided as the parameter, this API also returns an error.
///
///According to the definition of the file handlers in the manifest file, one
///Target.TargetID may represent a page handling one or more files. The order
///of the returned Target.TargetIDs is not guaranteed.
///
///TODO(crbug.com/339454034): Check the existences of the input files.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct LaunchFilesInApp;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：PWA.command.launchFilesInApp/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct LaunchFilesInAppResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：PWA.command.launchFilesInApp/returns/targetIds。
    ///
    ///协议原文：
    ///
    ///<pre>IDs of the tab targets created as the result.</pre>
    ///
    ///
    #[serde(rename = "targetIds")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub target_ids: Vec<crate::target::types::TargetId>,
}
impl<'de> serde::Deserialize<'de> for LaunchFilesInAppResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = LaunchFilesInAppResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：PWA.command.launchFilesInApp/returns/targetIds。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>IDs of the tab targets created as the result.</pre>
                    ///
                    ///
                    #[serde(rename = "targetIds")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub target_ids: Vec<crate::target::types::TargetId>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(LaunchFilesInAppResponse {
                    target_ids: fields.target_ids,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for LaunchFilesInApp {
    type Params = LaunchFilesInAppParams;
    type Response = LaunchFilesInAppResponse;
    const METHOD: &'static str = "PWA.launchFilesInApp";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：PWA.command.openCurrentPageInApp/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct OpenCurrentPageInAppParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：PWA.command.openCurrentPageInApp/parameters/manifestId。
    #[serde(rename = "manifestId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub manifest_id: String,
}
impl<'de> serde::Deserialize<'de> for OpenCurrentPageInAppParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = OpenCurrentPageInAppParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：PWA.command.openCurrentPageInApp/parameters/manifestId。
                    #[serde(rename = "manifestId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub manifest_id: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(OpenCurrentPageInAppParams {
                    manifest_id: fields.manifest_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：PWA.command.openCurrentPageInApp。
///
///协议原文：
///
///<pre>Opens the current page in its web app identified by the manifest id, needs
///to be called on a page target. This function returns immediately without
///waiting for the app to finish loading.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct OpenCurrentPageInApp;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：PWA.command.openCurrentPageInApp/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct OpenCurrentPageInAppResponse {}
impl<'de> serde::Deserialize<'de> for OpenCurrentPageInAppResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = OpenCurrentPageInAppResponse;
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
                Ok(OpenCurrentPageInAppResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for OpenCurrentPageInApp {
    type Params = OpenCurrentPageInAppParams;
    type Response = OpenCurrentPageInAppResponse;
    const METHOD: &'static str = "PWA.openCurrentPageInApp";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：PWA.command.uninstall/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct UninstallParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：PWA.command.uninstall/parameters/manifestId。
    #[serde(rename = "manifestId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub manifest_id: String,
}
impl<'de> serde::Deserialize<'de> for UninstallParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = UninstallParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：PWA.command.uninstall/parameters/manifestId。
                    #[serde(rename = "manifestId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub manifest_id: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(UninstallParams {
                    manifest_id: fields.manifest_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：PWA.command.uninstall。
///
///协议原文：
///
///<pre>Uninstalls the given manifest_id and closes any opened app windows.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct Uninstall;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：PWA.command.uninstall/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct UninstallResponse {}
impl<'de> serde::Deserialize<'de> for UninstallResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = UninstallResponse;
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
                Ok(UninstallResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for Uninstall {
    type Params = UninstallParams;
    type Response = UninstallResponse;
    const METHOD: &'static str = "PWA.uninstall";
}
