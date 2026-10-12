///命令 marker、完整参数与成功响应。
pub mod commands;
///事件 marker 和拥有型事件参数。
pub mod events;
///命名协议类型；不包含连接状态。
pub mod types;
///薄 Domain 视图，仅借用 Session；创建不发送数据。协议来源：Debugger。
///
///协议原文：
///
///<pre>Debugger domain exposes JavaScript debugging capabilities. It allows setting and removing
///breakpoints, stepping through execution, exploring stack traces, etc.</pre>
///
///
pub struct Domain<'a> {
    ///借用的通信路由，生命周期不超过 Session。
    pub(super) session: &'a ydp_core::Session,
}
impl<'a> Domain<'a> {
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Debugger.command.continueToLocation。
    ///
    ///协议原文：
    ///
    ///<pre>Continues execution until specific location is reached.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::debugger::commands::ContinueToLocationParams) -> ydp_core::Result<()> {
    ///let response = session.debugger().continue_to_location(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn continue_to_location(
        &self,
        params: commands::ContinueToLocationParams,
    ) -> ydp_core::Request<commands::ContinueToLocation> {
        self.session.send::<commands::ContinueToLocation>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Debugger.command.disable。
    ///
    ///协议原文：
    ///
    ///<pre>Disables debugger for given page.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::debugger::commands::DisableParams) -> ydp_core::Result<()> {
    ///let response = session.debugger().disable(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn disable(&self, params: commands::DisableParams) -> ydp_core::Request<commands::Disable> {
        self.session.send::<commands::Disable>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Debugger.command.disassembleWasmModule。此项为 experimental，端点可能不支持。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::debugger::commands::DisassembleWasmModuleParams) -> ydp_core::Result<()> {
    ///let response = session.debugger().disassemble_wasm_module(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn disassemble_wasm_module(
        &self,
        params: commands::DisassembleWasmModuleParams,
    ) -> ydp_core::Request<commands::DisassembleWasmModule> {
        self.session.send::<commands::DisassembleWasmModule>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Debugger.command.enable。
    ///
    ///协议原文：
    ///
    ///<pre>Enables debugger for the given page. Clients should not assume that the debugging has been
    ///enabled until the result for this command is received.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::debugger::commands::EnableParams) -> ydp_core::Result<()> {
    ///let response = session.debugger().enable(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn enable(&self, params: commands::EnableParams) -> ydp_core::Request<commands::Enable> {
        self.session.send::<commands::Enable>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Debugger.command.evaluateOnCallFrame。
    ///
    ///协议原文：
    ///
    ///<pre>Evaluates expression on a given call frame.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::debugger::commands::EvaluateOnCallFrameParams) -> ydp_core::Result<()> {
    ///let response = session.debugger().evaluate_on_call_frame(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn evaluate_on_call_frame(
        &self,
        params: commands::EvaluateOnCallFrameParams,
    ) -> ydp_core::Request<commands::EvaluateOnCallFrame> {
        self.session.send::<commands::EvaluateOnCallFrame>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Debugger.command.getPossibleBreakpoints。
    ///
    ///协议原文：
    ///
    ///<pre>Returns possible locations for breakpoint. scriptId in start and end range locations should be
    ///the same.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::debugger::commands::GetPossibleBreakpointsParams) -> ydp_core::Result<()> {
    ///let response = session.debugger().get_possible_breakpoints(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn get_possible_breakpoints(
        &self,
        params: commands::GetPossibleBreakpointsParams,
    ) -> ydp_core::Request<commands::GetPossibleBreakpoints> {
        self.session
            .send::<commands::GetPossibleBreakpoints>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Debugger.command.getScriptSource。
    ///
    ///协议原文：
    ///
    ///<pre>Returns source for the script with given id.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::debugger::commands::GetScriptSourceParams) -> ydp_core::Result<()> {
    ///let response = session.debugger().get_script_source(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn get_script_source(
        &self,
        params: commands::GetScriptSourceParams,
    ) -> ydp_core::Request<commands::GetScriptSource> {
        self.session.send::<commands::GetScriptSource>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Debugger.command.getStackTrace。
    ///
    ///协议原文：
    ///
    ///<pre>Returns stack trace with given `stackTraceId`.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::debugger::commands::GetStackTraceParams) -> ydp_core::Result<()> {
    ///let response = session.debugger().get_stack_trace(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn get_stack_trace(
        &self,
        params: commands::GetStackTraceParams,
    ) -> ydp_core::Request<commands::GetStackTrace> {
        self.session.send::<commands::GetStackTrace>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Debugger.command.getWasmBytecode。
    ///
    ///协议原文：
    ///
    ///<pre>This command is deprecated. Use getScriptSource instead.</pre>
    ///
    ///此项已 deprecated，保留用于旧端点兼容。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::debugger::commands::GetWasmBytecodeParams) -> ydp_core::Result<()> {
    ///let response = session.debugger().get_wasm_bytecode(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn get_wasm_bytecode(
        &self,
        params: commands::GetWasmBytecodeParams,
    ) -> ydp_core::Request<commands::GetWasmBytecode> {
        self.session.send::<commands::GetWasmBytecode>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Debugger.command.nextWasmDisassemblyChunk。
    ///
    ///协议原文：
    ///
    ///<pre>Disassemble the next chunk of lines for the module corresponding to the
    ///stream. If disassembly is complete, this API will invalidate the streamId
    ///and return an empty chunk. Any subsequent calls for the now invalid stream
    ///will return errors.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::debugger::commands::NextWasmDisassemblyChunkParams) -> ydp_core::Result<()> {
    ///let response = session.debugger().next_wasm_disassembly_chunk(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn next_wasm_disassembly_chunk(
        &self,
        params: commands::NextWasmDisassemblyChunkParams,
    ) -> ydp_core::Request<commands::NextWasmDisassemblyChunk> {
        self.session
            .send::<commands::NextWasmDisassemblyChunk>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Debugger.command.pause。
    ///
    ///协议原文：
    ///
    ///<pre>Stops on the next JavaScript statement.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::debugger::commands::PauseParams) -> ydp_core::Result<()> {
    ///let response = session.debugger().pause(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn pause(&self, params: commands::PauseParams) -> ydp_core::Request<commands::Pause> {
        self.session.send::<commands::Pause>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Debugger.command.pauseOnAsyncCall。此项为 experimental，端点可能不支持。此项已 deprecated，保留用于旧端点兼容。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::debugger::commands::PauseOnAsyncCallParams) -> ydp_core::Result<()> {
    ///let response = session.debugger().pause_on_async_call(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn pause_on_async_call(
        &self,
        params: commands::PauseOnAsyncCallParams,
    ) -> ydp_core::Request<commands::PauseOnAsyncCall> {
        self.session.send::<commands::PauseOnAsyncCall>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Debugger.command.removeBreakpoint。
    ///
    ///协议原文：
    ///
    ///<pre>Removes JavaScript breakpoint.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::debugger::commands::RemoveBreakpointParams) -> ydp_core::Result<()> {
    ///let response = session.debugger().remove_breakpoint(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn remove_breakpoint(
        &self,
        params: commands::RemoveBreakpointParams,
    ) -> ydp_core::Request<commands::RemoveBreakpoint> {
        self.session.send::<commands::RemoveBreakpoint>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Debugger.command.restartFrame。
    ///
    ///协议原文：
    ///
    ///<pre>Restarts particular call frame from the beginning. The old, deprecated
    ///behavior of `restartFrame` is to stay paused and allow further CDP commands
    ///after a restart was scheduled. This can cause problems with restarting, so
    ///we now continue execution immediatly after it has been scheduled until we
    ///reach the beginning of the restarted frame.
    ///
    ///To stay back-wards compatible, `restartFrame` now expects a `mode`
    ///parameter to be present. If the `mode` parameter is missing, `restartFrame`
    ///errors out.
    ///
    ///The various return values are deprecated and `callFrames` is always empty.
    ///Use the call frames from the `Debugger#paused` events instead, that fires
    ///once V8 pauses at the beginning of the restarted function.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::debugger::commands::RestartFrameParams) -> ydp_core::Result<()> {
    ///let response = session.debugger().restart_frame(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn restart_frame(
        &self,
        params: commands::RestartFrameParams,
    ) -> ydp_core::Request<commands::RestartFrame> {
        self.session.send::<commands::RestartFrame>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Debugger.command.resume。
    ///
    ///协议原文：
    ///
    ///<pre>Resumes JavaScript execution.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::debugger::commands::ResumeParams) -> ydp_core::Result<()> {
    ///let response = session.debugger().resume(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn resume(&self, params: commands::ResumeParams) -> ydp_core::Request<commands::Resume> {
        self.session.send::<commands::Resume>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Debugger.command.searchInContent。
    ///
    ///协议原文：
    ///
    ///<pre>Searches for given string in script content.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::debugger::commands::SearchInContentParams) -> ydp_core::Result<()> {
    ///let response = session.debugger().search_in_content(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn search_in_content(
        &self,
        params: commands::SearchInContentParams,
    ) -> ydp_core::Request<commands::SearchInContent> {
        self.session.send::<commands::SearchInContent>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Debugger.command.setAsyncCallStackDepth。
    ///
    ///协议原文：
    ///
    ///<pre>Enables or disables async call stacks tracking.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::debugger::commands::SetAsyncCallStackDepthParams) -> ydp_core::Result<()> {
    ///let response = session.debugger().set_async_call_stack_depth(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn set_async_call_stack_depth(
        &self,
        params: commands::SetAsyncCallStackDepthParams,
    ) -> ydp_core::Request<commands::SetAsyncCallStackDepth> {
        self.session
            .send::<commands::SetAsyncCallStackDepth>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Debugger.command.setBlackboxExecutionContexts。
    ///
    ///协议原文：
    ///
    ///<pre>Replace previous blackbox execution contexts with passed ones. Forces backend to skip
    ///stepping/pausing in scripts in these execution contexts. VM will try to leave blackboxed script by
    ///performing 'step in' several times, finally resorting to 'step out' if unsuccessful.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::debugger::commands::SetBlackboxExecutionContextsParams) -> ydp_core::Result<()> {
    ///let response = session.debugger().set_blackbox_execution_contexts(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn set_blackbox_execution_contexts(
        &self,
        params: commands::SetBlackboxExecutionContextsParams,
    ) -> ydp_core::Request<commands::SetBlackboxExecutionContexts> {
        self.session
            .send::<commands::SetBlackboxExecutionContexts>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Debugger.command.setBlackboxPatterns。
    ///
    ///协议原文：
    ///
    ///<pre>Replace previous blackbox patterns with passed ones. Forces backend to skip stepping/pausing in
    ///scripts with url matching one of the patterns. VM will try to leave blackboxed script by
    ///performing 'step in' several times, finally resorting to 'step out' if unsuccessful.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::debugger::commands::SetBlackboxPatternsParams) -> ydp_core::Result<()> {
    ///let response = session.debugger().set_blackbox_patterns(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn set_blackbox_patterns(
        &self,
        params: commands::SetBlackboxPatternsParams,
    ) -> ydp_core::Request<commands::SetBlackboxPatterns> {
        self.session.send::<commands::SetBlackboxPatterns>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Debugger.command.setBlackboxedRanges。
    ///
    ///协议原文：
    ///
    ///<pre>Makes backend skip steps in the script in blackboxed ranges. VM will try leave blacklisted
    ///scripts by performing 'step in' several times, finally resorting to 'step out' if unsuccessful.
    ///Positions array contains positions where blackbox state is changed. First interval isn't
    ///blackboxed. Array should be sorted.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::debugger::commands::SetBlackboxedRangesParams) -> ydp_core::Result<()> {
    ///let response = session.debugger().set_blackboxed_ranges(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn set_blackboxed_ranges(
        &self,
        params: commands::SetBlackboxedRangesParams,
    ) -> ydp_core::Request<commands::SetBlackboxedRanges> {
        self.session.send::<commands::SetBlackboxedRanges>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Debugger.command.setBreakpoint。
    ///
    ///协议原文：
    ///
    ///<pre>Sets JavaScript breakpoint at a given location.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::debugger::commands::SetBreakpointParams) -> ydp_core::Result<()> {
    ///let response = session.debugger().set_breakpoint(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn set_breakpoint(
        &self,
        params: commands::SetBreakpointParams,
    ) -> ydp_core::Request<commands::SetBreakpoint> {
        self.session.send::<commands::SetBreakpoint>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Debugger.command.setBreakpointByUrl。
    ///
    ///协议原文：
    ///
    ///<pre>Sets JavaScript breakpoint at given location specified either by URL or URL regex. Once this
    ///command is issued, all existing parsed scripts will have breakpoints resolved and returned in
    ///`locations` property. Further matching script parsing will result in subsequent
    ///`breakpointResolved` events issued. This logical breakpoint will survive page reloads.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::debugger::commands::SetBreakpointByUrlParams) -> ydp_core::Result<()> {
    ///let response = session.debugger().set_breakpoint_by_url(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn set_breakpoint_by_url(
        &self,
        params: commands::SetBreakpointByUrlParams,
    ) -> ydp_core::Request<commands::SetBreakpointByUrl> {
        self.session.send::<commands::SetBreakpointByUrl>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Debugger.command.setBreakpointOnFunctionCall。
    ///
    ///协议原文：
    ///
    ///<pre>Sets JavaScript breakpoint before each call to the given function.
    ///If another function was created from the same source as a given one,
    ///calling it will also trigger the breakpoint.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::debugger::commands::SetBreakpointOnFunctionCallParams) -> ydp_core::Result<()> {
    ///let response = session.debugger().set_breakpoint_on_function_call(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn set_breakpoint_on_function_call(
        &self,
        params: commands::SetBreakpointOnFunctionCallParams,
    ) -> ydp_core::Request<commands::SetBreakpointOnFunctionCall> {
        self.session
            .send::<commands::SetBreakpointOnFunctionCall>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Debugger.command.setBreakpointsActive。
    ///
    ///协议原文：
    ///
    ///<pre>Activates / deactivates all breakpoints on the page.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::debugger::commands::SetBreakpointsActiveParams) -> ydp_core::Result<()> {
    ///let response = session.debugger().set_breakpoints_active(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn set_breakpoints_active(
        &self,
        params: commands::SetBreakpointsActiveParams,
    ) -> ydp_core::Request<commands::SetBreakpointsActive> {
        self.session.send::<commands::SetBreakpointsActive>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Debugger.command.setInstrumentationBreakpoint。
    ///
    ///协议原文：
    ///
    ///<pre>Sets instrumentation breakpoint.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::debugger::commands::SetInstrumentationBreakpointParams) -> ydp_core::Result<()> {
    ///let response = session.debugger().set_instrumentation_breakpoint(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn set_instrumentation_breakpoint(
        &self,
        params: commands::SetInstrumentationBreakpointParams,
    ) -> ydp_core::Request<commands::SetInstrumentationBreakpoint> {
        self.session
            .send::<commands::SetInstrumentationBreakpoint>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Debugger.command.setPauseOnExceptions。
    ///
    ///协议原文：
    ///
    ///<pre>Defines pause on exceptions state. Can be set to stop on all exceptions, uncaught exceptions,
    ///or caught exceptions, no exceptions. Initial pause on exceptions state is `none`.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::debugger::commands::SetPauseOnExceptionsParams) -> ydp_core::Result<()> {
    ///let response = session.debugger().set_pause_on_exceptions(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn set_pause_on_exceptions(
        &self,
        params: commands::SetPauseOnExceptionsParams,
    ) -> ydp_core::Request<commands::SetPauseOnExceptions> {
        self.session.send::<commands::SetPauseOnExceptions>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Debugger.command.setReturnValue。
    ///
    ///协议原文：
    ///
    ///<pre>Changes return value in top frame. Available only at return break position.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::debugger::commands::SetReturnValueParams) -> ydp_core::Result<()> {
    ///let response = session.debugger().set_return_value(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn set_return_value(
        &self,
        params: commands::SetReturnValueParams,
    ) -> ydp_core::Request<commands::SetReturnValue> {
        self.session.send::<commands::SetReturnValue>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Debugger.command.setScriptSource。
    ///
    ///协议原文：
    ///
    ///<pre>Live edit is no longer supported and this command always fails with a "no longer available" error.</pre>
    ///
    ///此项已 deprecated，保留用于旧端点兼容。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::debugger::commands::SetScriptSourceParams) -> ydp_core::Result<()> {
    ///let response = session.debugger().set_script_source(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn set_script_source(
        &self,
        params: commands::SetScriptSourceParams,
    ) -> ydp_core::Request<commands::SetScriptSource> {
        self.session.send::<commands::SetScriptSource>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Debugger.command.setSkipAllPauses。
    ///
    ///协议原文：
    ///
    ///<pre>Makes page not interrupt on any pauses (breakpoint, exception, dom exception etc).</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::debugger::commands::SetSkipAllPausesParams) -> ydp_core::Result<()> {
    ///let response = session.debugger().set_skip_all_pauses(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn set_skip_all_pauses(
        &self,
        params: commands::SetSkipAllPausesParams,
    ) -> ydp_core::Request<commands::SetSkipAllPauses> {
        self.session.send::<commands::SetSkipAllPauses>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Debugger.command.setVariableValue。
    ///
    ///协议原文：
    ///
    ///<pre>Changes value of variable in a callframe. Object-based scopes are not supported and must be
    ///mutated manually.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::debugger::commands::SetVariableValueParams) -> ydp_core::Result<()> {
    ///let response = session.debugger().set_variable_value(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn set_variable_value(
        &self,
        params: commands::SetVariableValueParams,
    ) -> ydp_core::Request<commands::SetVariableValue> {
        self.session.send::<commands::SetVariableValue>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Debugger.command.stepInto。
    ///
    ///协议原文：
    ///
    ///<pre>Steps into the function call.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::debugger::commands::StepIntoParams) -> ydp_core::Result<()> {
    ///let response = session.debugger().step_into(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn step_into(
        &self,
        params: commands::StepIntoParams,
    ) -> ydp_core::Request<commands::StepInto> {
        self.session.send::<commands::StepInto>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Debugger.command.stepOut。
    ///
    ///协议原文：
    ///
    ///<pre>Steps out of the function call.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::debugger::commands::StepOutParams) -> ydp_core::Result<()> {
    ///let response = session.debugger().step_out(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn step_out(
        &self,
        params: commands::StepOutParams,
    ) -> ydp_core::Request<commands::StepOut> {
        self.session.send::<commands::StepOut>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Debugger.command.stepOver。
    ///
    ///协议原文：
    ///
    ///<pre>Steps over the statement.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::debugger::commands::StepOverParams) -> ydp_core::Result<()> {
    ///let response = session.debugger().step_over(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn step_over(
        &self,
        params: commands::StepOverParams,
    ) -> ydp_core::Request<commands::StepOver> {
        self.session.send::<commands::StepOver>(params)
    }
}
