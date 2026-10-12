///命令 marker、完整参数与成功响应。
pub mod commands;
///事件 marker 和拥有型事件参数。
pub mod events;
///命名协议类型；不包含连接状态。
pub mod types;
///薄 Domain 视图，仅借用 Session；创建不发送数据。协议来源：Runtime。
///
///协议原文：
///
///<pre>Runtime domain exposes JavaScript runtime by means of remote evaluation and mirror objects.
///Evaluation results are returned as mirror object that expose object type, string representation
///and unique identifier that can be used for further object reference. Original objects are
///maintained in memory unless they are either explicitly released or are released along with the
///other objects in their object group.</pre>
///
///
pub struct Domain<'a> {
    ///借用的通信路由，生命周期不超过 Session。
    pub(super) session: &'a ydp_core::Session,
}
impl<'a> Domain<'a> {
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Runtime.command.addBinding。
    ///
    ///协议原文：
    ///
    ///<pre>If executionContextId is empty, adds binding with the given name on the
    ///global objects of all inspected contexts, including those created later,
    ///bindings survive reloads.
    ///Binding function takes exactly one argument, this argument should be string,
    ///in case of any other input, function throws an exception.
    ///Each binding function call produces Runtime.bindingCalled notification.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::runtime::commands::AddBindingParams) -> ydp_core::Result<()> {
    ///let response = session.runtime().add_binding(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn add_binding(
        &self,
        params: commands::AddBindingParams,
    ) -> ydp_core::Request<commands::AddBinding> {
        self.session.send::<commands::AddBinding>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Runtime.command.awaitPromise。
    ///
    ///协议原文：
    ///
    ///<pre>Add handler to promise with given promise object id.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::runtime::commands::AwaitPromiseParams) -> ydp_core::Result<()> {
    ///let response = session.runtime().await_promise(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn await_promise(
        &self,
        params: commands::AwaitPromiseParams,
    ) -> ydp_core::Request<commands::AwaitPromise> {
        self.session.send::<commands::AwaitPromise>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Runtime.command.callFunctionOn。
    ///
    ///协议原文：
    ///
    ///<pre>Calls function with given declaration on the given object. Object group of the result is
    ///inherited from the target object.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::runtime::commands::CallFunctionOnParams) -> ydp_core::Result<()> {
    ///let response = session.runtime().call_function_on(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn call_function_on(
        &self,
        params: commands::CallFunctionOnParams,
    ) -> ydp_core::Request<commands::CallFunctionOn> {
        self.session.send::<commands::CallFunctionOn>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Runtime.command.compileScript。
    ///
    ///协议原文：
    ///
    ///<pre>Compiles expression.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::runtime::commands::CompileScriptParams) -> ydp_core::Result<()> {
    ///let response = session.runtime().compile_script(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn compile_script(
        &self,
        params: commands::CompileScriptParams,
    ) -> ydp_core::Request<commands::CompileScript> {
        self.session.send::<commands::CompileScript>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Runtime.command.disable。
    ///
    ///协议原文：
    ///
    ///<pre>Disables reporting of execution contexts creation.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::runtime::commands::DisableParams) -> ydp_core::Result<()> {
    ///let response = session.runtime().disable(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn disable(&self, params: commands::DisableParams) -> ydp_core::Request<commands::Disable> {
        self.session.send::<commands::Disable>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Runtime.command.discardConsoleEntries。
    ///
    ///协议原文：
    ///
    ///<pre>Discards collected exceptions and console API calls.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::runtime::commands::DiscardConsoleEntriesParams) -> ydp_core::Result<()> {
    ///let response = session.runtime().discard_console_entries(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn discard_console_entries(
        &self,
        params: commands::DiscardConsoleEntriesParams,
    ) -> ydp_core::Request<commands::DiscardConsoleEntries> {
        self.session.send::<commands::DiscardConsoleEntries>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Runtime.command.enable。
    ///
    ///协议原文：
    ///
    ///<pre>Enables reporting of execution contexts creation by means of `executionContextCreated` event.
    ///When the reporting gets enabled the event will be sent immediately for each existing execution
    ///context.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::runtime::commands::EnableParams) -> ydp_core::Result<()> {
    ///let response = session.runtime().enable(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn enable(&self, params: commands::EnableParams) -> ydp_core::Request<commands::Enable> {
        self.session.send::<commands::Enable>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Runtime.command.evaluate。
    ///
    ///协议原文：
    ///
    ///<pre>Evaluates expression on global object.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::runtime::commands::EvaluateParams) -> ydp_core::Result<()> {
    ///let response = session.runtime().evaluate(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn evaluate(
        &self,
        params: commands::EvaluateParams,
    ) -> ydp_core::Request<commands::Evaluate> {
        self.session.send::<commands::Evaluate>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Runtime.command.getExceptionDetails。
    ///
    ///协议原文：
    ///
    ///<pre>This method tries to lookup and populate exception details for a
    ///JavaScript Error object.
    ///Note that the stackTrace portion of the resulting exceptionDetails will
    ///only be populated if the Runtime domain was enabled at the time when the
    ///Error was thrown.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::runtime::commands::GetExceptionDetailsParams) -> ydp_core::Result<()> {
    ///let response = session.runtime().get_exception_details(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn get_exception_details(
        &self,
        params: commands::GetExceptionDetailsParams,
    ) -> ydp_core::Request<commands::GetExceptionDetails> {
        self.session.send::<commands::GetExceptionDetails>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Runtime.command.getHeapUsage。
    ///
    ///协议原文：
    ///
    ///<pre>Returns the JavaScript heap usage.
    ///It is the total usage of the corresponding isolate not scoped to a particular Runtime.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::runtime::commands::GetHeapUsageParams) -> ydp_core::Result<()> {
    ///let response = session.runtime().get_heap_usage(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn get_heap_usage(
        &self,
        params: commands::GetHeapUsageParams,
    ) -> ydp_core::Request<commands::GetHeapUsage> {
        self.session.send::<commands::GetHeapUsage>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Runtime.command.getIsolateId。
    ///
    ///协议原文：
    ///
    ///<pre>Returns the isolate id.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::runtime::commands::GetIsolateIdParams) -> ydp_core::Result<()> {
    ///let response = session.runtime().get_isolate_id(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn get_isolate_id(
        &self,
        params: commands::GetIsolateIdParams,
    ) -> ydp_core::Request<commands::GetIsolateId> {
        self.session.send::<commands::GetIsolateId>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Runtime.command.getProperties。
    ///
    ///协议原文：
    ///
    ///<pre>Returns properties of a given object. Object group of the result is inherited from the target
    ///object.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::runtime::commands::GetPropertiesParams) -> ydp_core::Result<()> {
    ///let response = session.runtime().get_properties(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn get_properties(
        &self,
        params: commands::GetPropertiesParams,
    ) -> ydp_core::Request<commands::GetProperties> {
        self.session.send::<commands::GetProperties>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Runtime.command.globalLexicalScopeNames。
    ///
    ///协议原文：
    ///
    ///<pre>Returns all let, const and class variables from global scope.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::runtime::commands::GlobalLexicalScopeNamesParams) -> ydp_core::Result<()> {
    ///let response = session.runtime().global_lexical_scope_names(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn global_lexical_scope_names(
        &self,
        params: commands::GlobalLexicalScopeNamesParams,
    ) -> ydp_core::Request<commands::GlobalLexicalScopeNames> {
        self.session
            .send::<commands::GlobalLexicalScopeNames>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Runtime.command.queryObjects。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::runtime::commands::QueryObjectsParams) -> ydp_core::Result<()> {
    ///let response = session.runtime().query_objects(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn query_objects(
        &self,
        params: commands::QueryObjectsParams,
    ) -> ydp_core::Request<commands::QueryObjects> {
        self.session.send::<commands::QueryObjects>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Runtime.command.releaseObject。
    ///
    ///协议原文：
    ///
    ///<pre>Releases remote object with given id.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::runtime::commands::ReleaseObjectParams) -> ydp_core::Result<()> {
    ///let response = session.runtime().release_object(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn release_object(
        &self,
        params: commands::ReleaseObjectParams,
    ) -> ydp_core::Request<commands::ReleaseObject> {
        self.session.send::<commands::ReleaseObject>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Runtime.command.releaseObjectGroup。
    ///
    ///协议原文：
    ///
    ///<pre>Releases all remote objects that belong to a given group.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::runtime::commands::ReleaseObjectGroupParams) -> ydp_core::Result<()> {
    ///let response = session.runtime().release_object_group(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn release_object_group(
        &self,
        params: commands::ReleaseObjectGroupParams,
    ) -> ydp_core::Request<commands::ReleaseObjectGroup> {
        self.session.send::<commands::ReleaseObjectGroup>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Runtime.command.removeBinding。
    ///
    ///协议原文：
    ///
    ///<pre>This method does not remove binding function from global object but
    ///unsubscribes current runtime agent from Runtime.bindingCalled notifications.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::runtime::commands::RemoveBindingParams) -> ydp_core::Result<()> {
    ///let response = session.runtime().remove_binding(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn remove_binding(
        &self,
        params: commands::RemoveBindingParams,
    ) -> ydp_core::Request<commands::RemoveBinding> {
        self.session.send::<commands::RemoveBinding>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Runtime.command.runIfWaitingForDebugger。
    ///
    ///协议原文：
    ///
    ///<pre>Tells inspected instance to run if it was waiting for debugger to attach.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::runtime::commands::RunIfWaitingForDebuggerParams) -> ydp_core::Result<()> {
    ///let response = session.runtime().run_if_waiting_for_debugger(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn run_if_waiting_for_debugger(
        &self,
        params: commands::RunIfWaitingForDebuggerParams,
    ) -> ydp_core::Request<commands::RunIfWaitingForDebugger> {
        self.session
            .send::<commands::RunIfWaitingForDebugger>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Runtime.command.runScript。
    ///
    ///协议原文：
    ///
    ///<pre>Runs script with given id in a given context.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::runtime::commands::RunScriptParams) -> ydp_core::Result<()> {
    ///let response = session.runtime().run_script(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn run_script(
        &self,
        params: commands::RunScriptParams,
    ) -> ydp_core::Request<commands::RunScript> {
        self.session.send::<commands::RunScript>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Runtime.command.setAsyncCallStackDepth。
    ///
    ///协议原文：
    ///
    ///<pre>Enables or disables async call stacks tracking.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::runtime::commands::SetAsyncCallStackDepthParams) -> ydp_core::Result<()> {
    ///let response = session.runtime().set_async_call_stack_depth(params).await?;
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
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Runtime.command.setCustomObjectFormatterEnabled。此项为 experimental，端点可能不支持。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::runtime::commands::SetCustomObjectFormatterEnabledParams) -> ydp_core::Result<()> {
    ///let response = session.runtime().set_custom_object_formatter_enabled(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn set_custom_object_formatter_enabled(
        &self,
        params: commands::SetCustomObjectFormatterEnabledParams,
    ) -> ydp_core::Request<commands::SetCustomObjectFormatterEnabled> {
        self.session
            .send::<commands::SetCustomObjectFormatterEnabled>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Runtime.command.setMaxCallStackSizeToCapture。此项为 experimental，端点可能不支持。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::runtime::commands::SetMaxCallStackSizeToCaptureParams) -> ydp_core::Result<()> {
    ///let response = session.runtime().set_max_call_stack_size_to_capture(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn set_max_call_stack_size_to_capture(
        &self,
        params: commands::SetMaxCallStackSizeToCaptureParams,
    ) -> ydp_core::Request<commands::SetMaxCallStackSizeToCapture> {
        self.session
            .send::<commands::SetMaxCallStackSizeToCapture>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Runtime.command.terminateExecution。
    ///
    ///协议原文：
    ///
    ///<pre>Terminate current or next JavaScript execution.
    ///Will cancel the termination when the outer-most script execution ends.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::runtime::commands::TerminateExecutionParams) -> ydp_core::Result<()> {
    ///let response = session.runtime().terminate_execution(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn terminate_execution(
        &self,
        params: commands::TerminateExecutionParams,
    ) -> ydp_core::Request<commands::TerminateExecution> {
        self.session.send::<commands::TerminateExecution>(params)
    }
}
