///命令 marker、完整参数与成功响应。
pub mod commands;
///事件 marker 和拥有型事件参数。
pub mod events;
///命名协议类型；不包含连接状态。
pub mod types;
///薄 Domain 视图，仅借用 Session；创建不发送数据。协议来源：Overlay。
///
///协议原文：
///
///<pre>This domain provides various functionality related to drawing atop the inspected page.</pre>
///
///此项为 experimental，端点可能不支持。
pub struct Domain<'a> {
    ///借用的通信路由，生命周期不超过 Session。
    pub(super) session: &'a ydp_core::Session,
}
impl<'a> Domain<'a> {
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Overlay.command.disable。
    ///
    ///协议原文：
    ///
    ///<pre>Disables domain notifications.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::overlay::commands::DisableParams) -> ydp_core::Result<()> {
    ///let response = session.overlay().disable(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn disable(&self, params: commands::DisableParams) -> ydp_core::Request<commands::Disable> {
        self.session.send::<commands::Disable>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Overlay.command.enable。
    ///
    ///协议原文：
    ///
    ///<pre>Enables domain notifications.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::overlay::commands::EnableParams) -> ydp_core::Result<()> {
    ///let response = session.overlay().enable(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn enable(&self, params: commands::EnableParams) -> ydp_core::Request<commands::Enable> {
        self.session.send::<commands::Enable>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Overlay.command.getGridHighlightObjectsForTest。
    ///
    ///协议原文：
    ///
    ///<pre>For Persistent Grid testing.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::overlay::commands::GetGridHighlightObjectsForTestParams) -> ydp_core::Result<()> {
    ///let response = session.overlay().get_grid_highlight_objects_for_test(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn get_grid_highlight_objects_for_test(
        &self,
        params: commands::GetGridHighlightObjectsForTestParams,
    ) -> ydp_core::Request<commands::GetGridHighlightObjectsForTest> {
        self.session
            .send::<commands::GetGridHighlightObjectsForTest>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Overlay.command.getHighlightObjectForTest。
    ///
    ///协议原文：
    ///
    ///<pre>For testing.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::overlay::commands::GetHighlightObjectForTestParams) -> ydp_core::Result<()> {
    ///let response = session.overlay().get_highlight_object_for_test(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn get_highlight_object_for_test(
        &self,
        params: commands::GetHighlightObjectForTestParams,
    ) -> ydp_core::Request<commands::GetHighlightObjectForTest> {
        self.session
            .send::<commands::GetHighlightObjectForTest>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Overlay.command.getSourceOrderHighlightObjectForTest。
    ///
    ///协议原文：
    ///
    ///<pre>For Source Order Viewer testing.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::overlay::commands::GetSourceOrderHighlightObjectForTestParams) -> ydp_core::Result<()> {
    ///let response = session.overlay().get_source_order_highlight_object_for_test(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn get_source_order_highlight_object_for_test(
        &self,
        params: commands::GetSourceOrderHighlightObjectForTestParams,
    ) -> ydp_core::Request<commands::GetSourceOrderHighlightObjectForTest> {
        self.session
            .send::<commands::GetSourceOrderHighlightObjectForTest>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Overlay.command.hideHighlight。
    ///
    ///协议原文：
    ///
    ///<pre>Hides any highlight.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::overlay::commands::HideHighlightParams) -> ydp_core::Result<()> {
    ///let response = session.overlay().hide_highlight(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn hide_highlight(
        &self,
        params: commands::HideHighlightParams,
    ) -> ydp_core::Request<commands::HideHighlight> {
        self.session.send::<commands::HideHighlight>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Overlay.command.highlightFrame。
    ///
    ///协议原文：
    ///
    ///<pre>Highlights owner element of the frame with given id.
    ///Deprecated: Doesn't work reliably and cannot be fixed due to process
    ///separation (the owner node might be in a different process). Determine
    ///the owner node in the client and use highlightNode.</pre>
    ///
    ///此项已 deprecated，保留用于旧端点兼容。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::overlay::commands::HighlightFrameParams) -> ydp_core::Result<()> {
    ///let response = session.overlay().highlight_frame(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn highlight_frame(
        &self,
        params: commands::HighlightFrameParams,
    ) -> ydp_core::Request<commands::HighlightFrame> {
        self.session.send::<commands::HighlightFrame>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Overlay.command.highlightNode。
    ///
    ///协议原文：
    ///
    ///<pre>Highlights DOM node with given id or with the given JavaScript object wrapper. Either nodeId or
    ///objectId must be specified.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::overlay::commands::HighlightNodeParams) -> ydp_core::Result<()> {
    ///let response = session.overlay().highlight_node(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn highlight_node(
        &self,
        params: commands::HighlightNodeParams,
    ) -> ydp_core::Request<commands::HighlightNode> {
        self.session.send::<commands::HighlightNode>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Overlay.command.highlightQuad。
    ///
    ///协议原文：
    ///
    ///<pre>Highlights given quad. Coordinates are absolute with respect to the main frame viewport.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::overlay::commands::HighlightQuadParams) -> ydp_core::Result<()> {
    ///let response = session.overlay().highlight_quad(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn highlight_quad(
        &self,
        params: commands::HighlightQuadParams,
    ) -> ydp_core::Request<commands::HighlightQuad> {
        self.session.send::<commands::HighlightQuad>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Overlay.command.highlightRect。
    ///
    ///协议原文：
    ///
    ///<pre>Highlights given rectangle. Coordinates are absolute with respect to the main frame viewport.
    ///Issue: the method does not handle device pixel ratio (DPR) correctly.
    ///The coordinates currently have to be adjusted by the client
    ///if DPR is not 1 (see crbug.com/437807128).</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::overlay::commands::HighlightRectParams) -> ydp_core::Result<()> {
    ///let response = session.overlay().highlight_rect(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn highlight_rect(
        &self,
        params: commands::HighlightRectParams,
    ) -> ydp_core::Request<commands::HighlightRect> {
        self.session.send::<commands::HighlightRect>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Overlay.command.highlightSourceOrder。
    ///
    ///协议原文：
    ///
    ///<pre>Highlights the source order of the children of the DOM node with given id or with the given
    ///JavaScript object wrapper. Either nodeId or objectId must be specified.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::overlay::commands::HighlightSourceOrderParams) -> ydp_core::Result<()> {
    ///let response = session.overlay().highlight_source_order(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn highlight_source_order(
        &self,
        params: commands::HighlightSourceOrderParams,
    ) -> ydp_core::Request<commands::HighlightSourceOrder> {
        self.session.send::<commands::HighlightSourceOrder>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Overlay.command.setInspectMode。
    ///
    ///协议原文：
    ///
    ///<pre>Enters the 'inspect' mode. In this mode, elements that user is hovering over are highlighted.
    ///Backend then generates 'inspectNodeRequested' event upon element selection.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::overlay::commands::SetInspectModeParams) -> ydp_core::Result<()> {
    ///let response = session.overlay().set_inspect_mode(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn set_inspect_mode(
        &self,
        params: commands::SetInspectModeParams,
    ) -> ydp_core::Request<commands::SetInspectMode> {
        self.session.send::<commands::SetInspectMode>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Overlay.command.setPausedInDebuggerMessage。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::overlay::commands::SetPausedInDebuggerMessageParams) -> ydp_core::Result<()> {
    ///let response = session.overlay().set_paused_in_debugger_message(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn set_paused_in_debugger_message(
        &self,
        params: commands::SetPausedInDebuggerMessageParams,
    ) -> ydp_core::Request<commands::SetPausedInDebuggerMessage> {
        self.session
            .send::<commands::SetPausedInDebuggerMessage>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Overlay.command.setShowAdHighlights。
    ///
    ///协议原文：
    ///
    ///<pre>Highlights owner element of all frames detected to be ads.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::overlay::commands::SetShowAdHighlightsParams) -> ydp_core::Result<()> {
    ///let response = session.overlay().set_show_ad_highlights(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn set_show_ad_highlights(
        &self,
        params: commands::SetShowAdHighlightsParams,
    ) -> ydp_core::Request<commands::SetShowAdHighlights> {
        self.session.send::<commands::SetShowAdHighlights>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Overlay.command.setShowContainerQueryOverlays。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::overlay::commands::SetShowContainerQueryOverlaysParams) -> ydp_core::Result<()> {
    ///let response = session.overlay().set_show_container_query_overlays(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn set_show_container_query_overlays(
        &self,
        params: commands::SetShowContainerQueryOverlaysParams,
    ) -> ydp_core::Request<commands::SetShowContainerQueryOverlays> {
        self.session
            .send::<commands::SetShowContainerQueryOverlays>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Overlay.command.setShowDebugBorders。
    ///
    ///协议原文：
    ///
    ///<pre>Requests that backend shows debug borders on layers</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::overlay::commands::SetShowDebugBordersParams) -> ydp_core::Result<()> {
    ///let response = session.overlay().set_show_debug_borders(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn set_show_debug_borders(
        &self,
        params: commands::SetShowDebugBordersParams,
    ) -> ydp_core::Request<commands::SetShowDebugBorders> {
        self.session.send::<commands::SetShowDebugBorders>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Overlay.command.setShowDisplayCutout。
    ///
    ///协议原文：
    ///
    ///<pre>Add a display cutout overlay.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::overlay::commands::SetShowDisplayCutoutParams) -> ydp_core::Result<()> {
    ///let response = session.overlay().set_show_display_cutout(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn set_show_display_cutout(
        &self,
        params: commands::SetShowDisplayCutoutParams,
    ) -> ydp_core::Request<commands::SetShowDisplayCutout> {
        self.session.send::<commands::SetShowDisplayCutout>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Overlay.command.setShowFPSCounter。
    ///
    ///协议原文：
    ///
    ///<pre>Requests that backend shows the FPS counter</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::overlay::commands::SetShowFpsCounterParams) -> ydp_core::Result<()> {
    ///let response = session.overlay().set_show_fps_counter(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn set_show_fps_counter(
        &self,
        params: commands::SetShowFpsCounterParams,
    ) -> ydp_core::Request<commands::SetShowFpsCounter> {
        self.session.send::<commands::SetShowFpsCounter>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Overlay.command.setShowFlexOverlays。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::overlay::commands::SetShowFlexOverlaysParams) -> ydp_core::Result<()> {
    ///let response = session.overlay().set_show_flex_overlays(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn set_show_flex_overlays(
        &self,
        params: commands::SetShowFlexOverlaysParams,
    ) -> ydp_core::Request<commands::SetShowFlexOverlays> {
        self.session.send::<commands::SetShowFlexOverlays>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Overlay.command.setShowGridOverlays。
    ///
    ///协议原文：
    ///
    ///<pre>Highlight multiple elements with the CSS Grid overlay.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::overlay::commands::SetShowGridOverlaysParams) -> ydp_core::Result<()> {
    ///let response = session.overlay().set_show_grid_overlays(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn set_show_grid_overlays(
        &self,
        params: commands::SetShowGridOverlaysParams,
    ) -> ydp_core::Request<commands::SetShowGridOverlays> {
        self.session.send::<commands::SetShowGridOverlays>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Overlay.command.setShowHinge。
    ///
    ///协议原文：
    ///
    ///<pre>Add a dual screen device hinge</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::overlay::commands::SetShowHingeParams) -> ydp_core::Result<()> {
    ///let response = session.overlay().set_show_hinge(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn set_show_hinge(
        &self,
        params: commands::SetShowHingeParams,
    ) -> ydp_core::Request<commands::SetShowHinge> {
        self.session.send::<commands::SetShowHinge>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Overlay.command.setShowHitTestBorders。
    ///
    ///协议原文：
    ///
    ///<pre>Deprecated, no longer has any effect.</pre>
    ///
    ///此项已 deprecated，保留用于旧端点兼容。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::overlay::commands::SetShowHitTestBordersParams) -> ydp_core::Result<()> {
    ///let response = session.overlay().set_show_hit_test_borders(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn set_show_hit_test_borders(
        &self,
        params: commands::SetShowHitTestBordersParams,
    ) -> ydp_core::Request<commands::SetShowHitTestBorders> {
        self.session.send::<commands::SetShowHitTestBorders>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Overlay.command.setShowInspectedElementAnchor。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::overlay::commands::SetShowInspectedElementAnchorParams) -> ydp_core::Result<()> {
    ///let response = session.overlay().set_show_inspected_element_anchor(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn set_show_inspected_element_anchor(
        &self,
        params: commands::SetShowInspectedElementAnchorParams,
    ) -> ydp_core::Request<commands::SetShowInspectedElementAnchor> {
        self.session
            .send::<commands::SetShowInspectedElementAnchor>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Overlay.command.setShowIsolatedElements。
    ///
    ///协议原文：
    ///
    ///<pre>Show elements in isolation mode with overlays.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::overlay::commands::SetShowIsolatedElementsParams) -> ydp_core::Result<()> {
    ///let response = session.overlay().set_show_isolated_elements(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn set_show_isolated_elements(
        &self,
        params: commands::SetShowIsolatedElementsParams,
    ) -> ydp_core::Request<commands::SetShowIsolatedElements> {
        self.session
            .send::<commands::SetShowIsolatedElements>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Overlay.command.setShowLayoutShiftRegions。
    ///
    ///协议原文：
    ///
    ///<pre>Requests that backend shows layout shift regions</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::overlay::commands::SetShowLayoutShiftRegionsParams) -> ydp_core::Result<()> {
    ///let response = session.overlay().set_show_layout_shift_regions(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn set_show_layout_shift_regions(
        &self,
        params: commands::SetShowLayoutShiftRegionsParams,
    ) -> ydp_core::Request<commands::SetShowLayoutShiftRegions> {
        self.session
            .send::<commands::SetShowLayoutShiftRegions>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Overlay.command.setShowPaintRects。
    ///
    ///协议原文：
    ///
    ///<pre>Requests that backend shows paint rectangles</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::overlay::commands::SetShowPaintRectsParams) -> ydp_core::Result<()> {
    ///let response = session.overlay().set_show_paint_rects(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn set_show_paint_rects(
        &self,
        params: commands::SetShowPaintRectsParams,
    ) -> ydp_core::Request<commands::SetShowPaintRects> {
        self.session.send::<commands::SetShowPaintRects>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Overlay.command.setShowScrollBottleneckRects。
    ///
    ///协议原文：
    ///
    ///<pre>Requests that backend shows scroll bottleneck rects</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::overlay::commands::SetShowScrollBottleneckRectsParams) -> ydp_core::Result<()> {
    ///let response = session.overlay().set_show_scroll_bottleneck_rects(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn set_show_scroll_bottleneck_rects(
        &self,
        params: commands::SetShowScrollBottleneckRectsParams,
    ) -> ydp_core::Request<commands::SetShowScrollBottleneckRects> {
        self.session
            .send::<commands::SetShowScrollBottleneckRects>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Overlay.command.setShowScrollSnapOverlays。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::overlay::commands::SetShowScrollSnapOverlaysParams) -> ydp_core::Result<()> {
    ///let response = session.overlay().set_show_scroll_snap_overlays(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn set_show_scroll_snap_overlays(
        &self,
        params: commands::SetShowScrollSnapOverlaysParams,
    ) -> ydp_core::Request<commands::SetShowScrollSnapOverlays> {
        self.session
            .send::<commands::SetShowScrollSnapOverlays>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Overlay.command.setShowViewportSizeOnResize。
    ///
    ///协议原文：
    ///
    ///<pre>Paints viewport size upon main frame resize.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::overlay::commands::SetShowViewportSizeOnResizeParams) -> ydp_core::Result<()> {
    ///let response = session.overlay().set_show_viewport_size_on_resize(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn set_show_viewport_size_on_resize(
        &self,
        params: commands::SetShowViewportSizeOnResizeParams,
    ) -> ydp_core::Request<commands::SetShowViewportSizeOnResize> {
        self.session
            .send::<commands::SetShowViewportSizeOnResize>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Overlay.command.setShowWebVitals。
    ///
    ///协议原文：
    ///
    ///<pre>Deprecated, no longer has any effect.</pre>
    ///
    ///此项已 deprecated，保留用于旧端点兼容。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::overlay::commands::SetShowWebVitalsParams) -> ydp_core::Result<()> {
    ///let response = session.overlay().set_show_web_vitals(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn set_show_web_vitals(
        &self,
        params: commands::SetShowWebVitalsParams,
    ) -> ydp_core::Request<commands::SetShowWebVitals> {
        self.session.send::<commands::SetShowWebVitals>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Overlay.command.setShowWindowControlsOverlay。
    ///
    ///协议原文：
    ///
    ///<pre>Show Window Controls Overlay for PWA</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::overlay::commands::SetShowWindowControlsOverlayParams) -> ydp_core::Result<()> {
    ///let response = session.overlay().set_show_window_controls_overlay(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn set_show_window_controls_overlay(
        &self,
        params: commands::SetShowWindowControlsOverlayParams,
    ) -> ydp_core::Request<commands::SetShowWindowControlsOverlay> {
        self.session
            .send::<commands::SetShowWindowControlsOverlay>(params)
    }
}
