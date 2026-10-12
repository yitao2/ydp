///命令 marker、完整参数与成功响应。
pub mod commands;
///事件 marker 和拥有型事件参数。
pub mod events;
///命名协议类型；不包含连接状态。
pub mod types;
///薄 Domain 视图，仅借用 Session；创建不发送数据。协议来源：DeviceOrientation。此项为 experimental，端点可能不支持。
pub struct Domain<'a> {
    ///借用的通信路由，生命周期不超过 Session。
    pub(super) session: &'a ydp_core::Session,
}
impl<'a> Domain<'a> {
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：DeviceOrientation.command.clearDeviceOrientationOverride。
    ///
    ///协议原文：
    ///
    ///<pre>Clears the overridden Device Orientation.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::device_orientation::commands::ClearDeviceOrientationOverrideParams) -> ydp_core::Result<()> {
    ///let response = session.device_orientation().clear_device_orientation_override(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn clear_device_orientation_override(
        &self,
        params: commands::ClearDeviceOrientationOverrideParams,
    ) -> ydp_core::Request<commands::ClearDeviceOrientationOverride> {
        self.session
            .send::<commands::ClearDeviceOrientationOverride>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：DeviceOrientation.command.setDeviceOrientationOverride。
    ///
    ///协议原文：
    ///
    ///<pre>Overrides the Device Orientation.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::device_orientation::commands::SetDeviceOrientationOverrideParams) -> ydp_core::Result<()> {
    ///let response = session.device_orientation().set_device_orientation_override(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn set_device_orientation_override(
        &self,
        params: commands::SetDeviceOrientationOverrideParams,
    ) -> ydp_core::Request<commands::SetDeviceOrientationOverride> {
        self.session
            .send::<commands::SetDeviceOrientationOverride>(params)
    }
}
