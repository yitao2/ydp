///命令 marker、完整参数与成功响应。
pub mod commands;
///事件 marker 和拥有型事件参数。
pub mod events;
///命名协议类型；不包含连接状态。
pub mod types;
///薄 Domain 视图，仅借用 Session；创建不发送数据。协议来源：Input。
pub struct Domain<'a> {
    ///借用的通信路由，生命周期不超过 Session。
    pub(super) session: &'a ydp_core::Session,
}
impl<'a> Domain<'a> {
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Input.command.cancelDragging。
    ///
    ///协议原文：
    ///
    ///<pre>Cancels any active dragging in the page.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::input::commands::CancelDraggingParams) -> ydp_core::Result<()> {
    ///let response = session.input().cancel_dragging(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn cancel_dragging(
        &self,
        params: commands::CancelDraggingParams,
    ) -> ydp_core::Request<commands::CancelDragging> {
        self.session.send::<commands::CancelDragging>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Input.command.dispatchDragEvent。
    ///
    ///协议原文：
    ///
    ///<pre>Dispatches a drag event into the page.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::input::commands::DispatchDragEventParams) -> ydp_core::Result<()> {
    ///let response = session.input().dispatch_drag_event(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn dispatch_drag_event(
        &self,
        params: commands::DispatchDragEventParams,
    ) -> ydp_core::Request<commands::DispatchDragEvent> {
        self.session.send::<commands::DispatchDragEvent>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Input.command.dispatchKeyEvent。
    ///
    ///协议原文：
    ///
    ///<pre>Dispatches a key event to the page.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::input::commands::DispatchKeyEventParams) -> ydp_core::Result<()> {
    ///let response = session.input().dispatch_key_event(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn dispatch_key_event(
        &self,
        params: commands::DispatchKeyEventParams,
    ) -> ydp_core::Request<commands::DispatchKeyEvent> {
        self.session.send::<commands::DispatchKeyEvent>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Input.command.dispatchMouseEvent。
    ///
    ///协议原文：
    ///
    ///<pre>Dispatches a mouse event to the page.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::input::commands::DispatchMouseEventParams) -> ydp_core::Result<()> {
    ///let response = session.input().dispatch_mouse_event(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn dispatch_mouse_event(
        &self,
        params: commands::DispatchMouseEventParams,
    ) -> ydp_core::Request<commands::DispatchMouseEvent> {
        self.session.send::<commands::DispatchMouseEvent>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Input.command.dispatchTouchEvent。
    ///
    ///协议原文：
    ///
    ///<pre>Dispatches a touch event to the page.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::input::commands::DispatchTouchEventParams) -> ydp_core::Result<()> {
    ///let response = session.input().dispatch_touch_event(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn dispatch_touch_event(
        &self,
        params: commands::DispatchTouchEventParams,
    ) -> ydp_core::Request<commands::DispatchTouchEvent> {
        self.session.send::<commands::DispatchTouchEvent>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Input.command.emulateTouchFromMouseEvent。
    ///
    ///协议原文：
    ///
    ///<pre>Emulates touch event from the mouse event parameters.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::input::commands::EmulateTouchFromMouseEventParams) -> ydp_core::Result<()> {
    ///let response = session.input().emulate_touch_from_mouse_event(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn emulate_touch_from_mouse_event(
        &self,
        params: commands::EmulateTouchFromMouseEventParams,
    ) -> ydp_core::Request<commands::EmulateTouchFromMouseEvent> {
        self.session
            .send::<commands::EmulateTouchFromMouseEvent>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Input.command.imeSetComposition。
    ///
    ///协议原文：
    ///
    ///<pre>This method sets the current candidate text for IME.
    ///Use imeCommitComposition to commit the final text.
    ///Use imeSetComposition with empty string as text to cancel composition.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::input::commands::ImeSetCompositionParams) -> ydp_core::Result<()> {
    ///let response = session.input().ime_set_composition(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn ime_set_composition(
        &self,
        params: commands::ImeSetCompositionParams,
    ) -> ydp_core::Request<commands::ImeSetComposition> {
        self.session.send::<commands::ImeSetComposition>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Input.command.insertText。
    ///
    ///协议原文：
    ///
    ///<pre>This method emulates inserting text that doesn't come from a key press,
    ///for example an emoji keyboard or an IME.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::input::commands::InsertTextParams) -> ydp_core::Result<()> {
    ///let response = session.input().insert_text(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn insert_text(
        &self,
        params: commands::InsertTextParams,
    ) -> ydp_core::Request<commands::InsertText> {
        self.session.send::<commands::InsertText>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Input.command.setIgnoreInputEvents。
    ///
    ///协议原文：
    ///
    ///<pre>Ignores input events (useful while auditing page).</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::input::commands::SetIgnoreInputEventsParams) -> ydp_core::Result<()> {
    ///let response = session.input().set_ignore_input_events(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn set_ignore_input_events(
        &self,
        params: commands::SetIgnoreInputEventsParams,
    ) -> ydp_core::Request<commands::SetIgnoreInputEvents> {
        self.session.send::<commands::SetIgnoreInputEvents>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Input.command.setInterceptDrags。
    ///
    ///协议原文：
    ///
    ///<pre>Prevents default drag and drop behavior and instead emits `Input.dragIntercepted` events.
    ///Drag and drop behavior can be directly controlled via `Input.dispatchDragEvent`.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::input::commands::SetInterceptDragsParams) -> ydp_core::Result<()> {
    ///let response = session.input().set_intercept_drags(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn set_intercept_drags(
        &self,
        params: commands::SetInterceptDragsParams,
    ) -> ydp_core::Request<commands::SetInterceptDrags> {
        self.session.send::<commands::SetInterceptDrags>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Input.command.synthesizePinchGesture。
    ///
    ///协议原文：
    ///
    ///<pre>Synthesizes a pinch gesture over a time period by issuing appropriate touch events.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::input::commands::SynthesizePinchGestureParams) -> ydp_core::Result<()> {
    ///let response = session.input().synthesize_pinch_gesture(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn synthesize_pinch_gesture(
        &self,
        params: commands::SynthesizePinchGestureParams,
    ) -> ydp_core::Request<commands::SynthesizePinchGesture> {
        self.session
            .send::<commands::SynthesizePinchGesture>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Input.command.synthesizeScrollGesture。
    ///
    ///协议原文：
    ///
    ///<pre>Synthesizes a scroll gesture over a time period by issuing appropriate touch events.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::input::commands::SynthesizeScrollGestureParams) -> ydp_core::Result<()> {
    ///let response = session.input().synthesize_scroll_gesture(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn synthesize_scroll_gesture(
        &self,
        params: commands::SynthesizeScrollGestureParams,
    ) -> ydp_core::Request<commands::SynthesizeScrollGesture> {
        self.session
            .send::<commands::SynthesizeScrollGesture>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Input.command.synthesizeTapGesture。
    ///
    ///协议原文：
    ///
    ///<pre>Synthesizes a tap gesture over a time period by issuing appropriate touch events.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::input::commands::SynthesizeTapGestureParams) -> ydp_core::Result<()> {
    ///let response = session.input().synthesize_tap_gesture(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn synthesize_tap_gesture(
        &self,
        params: commands::SynthesizeTapGestureParams,
    ) -> ydp_core::Request<commands::SynthesizeTapGesture> {
        self.session.send::<commands::SynthesizeTapGesture>(params)
    }
}
