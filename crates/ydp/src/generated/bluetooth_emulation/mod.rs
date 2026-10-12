///命令 marker、完整参数与成功响应。
pub mod commands;
///事件 marker 和拥有型事件参数。
pub mod events;
///命名协议类型；不包含连接状态。
pub mod types;
///薄 Domain 视图，仅借用 Session；创建不发送数据。协议来源：BluetoothEmulation。
///
///协议原文：
///
///<pre>This domain allows configuring virtual Bluetooth devices to test
///the web-bluetooth API.</pre>
///
///此项为 experimental，端点可能不支持。
pub struct Domain<'a> {
    ///借用的通信路由，生命周期不超过 Session。
    pub(super) session: &'a ydp_core::Session,
}
impl<'a> Domain<'a> {
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：BluetoothEmulation.command.addCharacteristic。
    ///
    ///协议原文：
    ///
    ///<pre>Adds a characteristic with |characteristicUuid| and |properties| to the
    ///service represented by |serviceId|.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::bluetooth_emulation::commands::AddCharacteristicParams) -> ydp_core::Result<()> {
    ///let response = session.bluetooth_emulation().add_characteristic(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn add_characteristic(
        &self,
        params: commands::AddCharacteristicParams,
    ) -> ydp_core::Request<commands::AddCharacteristic> {
        self.session.send::<commands::AddCharacteristic>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：BluetoothEmulation.command.addDescriptor。
    ///
    ///协议原文：
    ///
    ///<pre>Adds a descriptor with |descriptorUuid| to the characteristic respresented
    ///by |characteristicId|.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::bluetooth_emulation::commands::AddDescriptorParams) -> ydp_core::Result<()> {
    ///let response = session.bluetooth_emulation().add_descriptor(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn add_descriptor(
        &self,
        params: commands::AddDescriptorParams,
    ) -> ydp_core::Request<commands::AddDescriptor> {
        self.session.send::<commands::AddDescriptor>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：BluetoothEmulation.command.addService。
    ///
    ///协议原文：
    ///
    ///<pre>Adds a service with |serviceUuid| to the peripheral with |address|.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::bluetooth_emulation::commands::AddServiceParams) -> ydp_core::Result<()> {
    ///let response = session.bluetooth_emulation().add_service(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn add_service(
        &self,
        params: commands::AddServiceParams,
    ) -> ydp_core::Request<commands::AddService> {
        self.session.send::<commands::AddService>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：BluetoothEmulation.command.disable。
    ///
    ///协议原文：
    ///
    ///<pre>Disable the BluetoothEmulation domain.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::bluetooth_emulation::commands::DisableParams) -> ydp_core::Result<()> {
    ///let response = session.bluetooth_emulation().disable(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn disable(&self, params: commands::DisableParams) -> ydp_core::Request<commands::Disable> {
        self.session.send::<commands::Disable>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：BluetoothEmulation.command.enable。
    ///
    ///协议原文：
    ///
    ///<pre>Enable the BluetoothEmulation domain.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::bluetooth_emulation::commands::EnableParams) -> ydp_core::Result<()> {
    ///let response = session.bluetooth_emulation().enable(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn enable(&self, params: commands::EnableParams) -> ydp_core::Request<commands::Enable> {
        self.session.send::<commands::Enable>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：BluetoothEmulation.command.removeCharacteristic。
    ///
    ///协议原文：
    ///
    ///<pre>Removes the characteristic respresented by |characteristicId| from the
    ///simulated central.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::bluetooth_emulation::commands::RemoveCharacteristicParams) -> ydp_core::Result<()> {
    ///let response = session.bluetooth_emulation().remove_characteristic(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn remove_characteristic(
        &self,
        params: commands::RemoveCharacteristicParams,
    ) -> ydp_core::Request<commands::RemoveCharacteristic> {
        self.session.send::<commands::RemoveCharacteristic>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：BluetoothEmulation.command.removeDescriptor。
    ///
    ///协议原文：
    ///
    ///<pre>Removes the descriptor with |descriptorId| from the simulated central.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::bluetooth_emulation::commands::RemoveDescriptorParams) -> ydp_core::Result<()> {
    ///let response = session.bluetooth_emulation().remove_descriptor(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn remove_descriptor(
        &self,
        params: commands::RemoveDescriptorParams,
    ) -> ydp_core::Request<commands::RemoveDescriptor> {
        self.session.send::<commands::RemoveDescriptor>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：BluetoothEmulation.command.removeService。
    ///
    ///协议原文：
    ///
    ///<pre>Removes the service respresented by |serviceId| from the simulated central.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::bluetooth_emulation::commands::RemoveServiceParams) -> ydp_core::Result<()> {
    ///let response = session.bluetooth_emulation().remove_service(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn remove_service(
        &self,
        params: commands::RemoveServiceParams,
    ) -> ydp_core::Request<commands::RemoveService> {
        self.session.send::<commands::RemoveService>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：BluetoothEmulation.command.setSimulatedCentralState。
    ///
    ///协议原文：
    ///
    ///<pre>Set the state of the simulated central.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::bluetooth_emulation::commands::SetSimulatedCentralStateParams) -> ydp_core::Result<()> {
    ///let response = session.bluetooth_emulation().set_simulated_central_state(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn set_simulated_central_state(
        &self,
        params: commands::SetSimulatedCentralStateParams,
    ) -> ydp_core::Request<commands::SetSimulatedCentralState> {
        self.session
            .send::<commands::SetSimulatedCentralState>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：BluetoothEmulation.command.simulateAdvertisement。
    ///
    ///协议原文：
    ///
    ///<pre>Simulates an advertisement packet described in |entry| being received by
    ///the central.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::bluetooth_emulation::commands::SimulateAdvertisementParams) -> ydp_core::Result<()> {
    ///let response = session.bluetooth_emulation().simulate_advertisement(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn simulate_advertisement(
        &self,
        params: commands::SimulateAdvertisementParams,
    ) -> ydp_core::Request<commands::SimulateAdvertisement> {
        self.session.send::<commands::SimulateAdvertisement>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：BluetoothEmulation.command.simulateCharacteristicOperationResponse。
    ///
    ///协议原文：
    ///
    ///<pre>Simulates the response from the characteristic with |characteristicId| for a
    ///characteristic operation of |type|. The |code| value follows the Error
    ///Codes from Bluetooth Core Specification Vol 3 Part F 3.4.1.1 Error Response.
    ///The |data| is expected to exist when simulating a successful read operation
    ///response.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::bluetooth_emulation::commands::SimulateCharacteristicOperationResponseParams) -> ydp_core::Result<()> {
    ///let response = session.bluetooth_emulation().simulate_characteristic_operation_response(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn simulate_characteristic_operation_response(
        &self,
        params: commands::SimulateCharacteristicOperationResponseParams,
    ) -> ydp_core::Request<commands::SimulateCharacteristicOperationResponse> {
        self.session
            .send::<commands::SimulateCharacteristicOperationResponse>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：BluetoothEmulation.command.simulateDescriptorOperationResponse。
    ///
    ///协议原文：
    ///
    ///<pre>Simulates the response from the descriptor with |descriptorId| for a
    ///descriptor operation of |type|. The |code| value follows the Error
    ///Codes from Bluetooth Core Specification Vol 3 Part F 3.4.1.1 Error Response.
    ///The |data| is expected to exist when simulating a successful read operation
    ///response.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::bluetooth_emulation::commands::SimulateDescriptorOperationResponseParams) -> ydp_core::Result<()> {
    ///let response = session.bluetooth_emulation().simulate_descriptor_operation_response(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn simulate_descriptor_operation_response(
        &self,
        params: commands::SimulateDescriptorOperationResponseParams,
    ) -> ydp_core::Request<commands::SimulateDescriptorOperationResponse> {
        self.session
            .send::<commands::SimulateDescriptorOperationResponse>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：BluetoothEmulation.command.simulateGATTDisconnection。
    ///
    ///协议原文：
    ///
    ///<pre>Simulates a GATT disconnection from the peripheral with |address|.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::bluetooth_emulation::commands::SimulateGattDisconnectionParams) -> ydp_core::Result<()> {
    ///let response = session.bluetooth_emulation().simulate_gatt_disconnection(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn simulate_gatt_disconnection(
        &self,
        params: commands::SimulateGattDisconnectionParams,
    ) -> ydp_core::Request<commands::SimulateGattDisconnection> {
        self.session
            .send::<commands::SimulateGattDisconnection>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：BluetoothEmulation.command.simulateGATTOperationResponse。
    ///
    ///协议原文：
    ///
    ///<pre>Simulates the response code from the peripheral with |address| for a
    ///GATT operation of |type|. The |code| value follows the HCI Error Codes from
    ///Bluetooth Core Specification Vol 2 Part D 1.3 List Of Error Codes.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::bluetooth_emulation::commands::SimulateGattOperationResponseParams) -> ydp_core::Result<()> {
    ///let response = session.bluetooth_emulation().simulate_gatt_operation_response(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn simulate_gatt_operation_response(
        &self,
        params: commands::SimulateGattOperationResponseParams,
    ) -> ydp_core::Request<commands::SimulateGattOperationResponse> {
        self.session
            .send::<commands::SimulateGattOperationResponse>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：BluetoothEmulation.command.simulatePreconnectedPeripheral。
    ///
    ///协议原文：
    ///
    ///<pre>Simulates a peripheral with |address|, |name| and |knownServiceUuids|
    ///that has already been connected to the system.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::bluetooth_emulation::commands::SimulatePreconnectedPeripheralParams) -> ydp_core::Result<()> {
    ///let response = session.bluetooth_emulation().simulate_preconnected_peripheral(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn simulate_preconnected_peripheral(
        &self,
        params: commands::SimulatePreconnectedPeripheralParams,
    ) -> ydp_core::Request<commands::SimulatePreconnectedPeripheral> {
        self.session
            .send::<commands::SimulatePreconnectedPeripheral>(params)
    }
}
