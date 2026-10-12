///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Runtime.command.addBinding/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct AddBindingParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Runtime.command.addBinding/parameters/name。
    #[serde(rename = "name")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub name: String,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.command.addBinding/parameters/executionContextId。
    ///
    ///协议原文：
    ///
    ///<pre>If specified, the binding would only be exposed to the specified
    ///execution context. If omitted and `executionContextName` is not set,
    ///the binding is exposed to all execution contexts of the target.
    ///This parameter is mutually exclusive with `executionContextName`.
    ///Deprecated in favor of `executionContextName` due to an unclear use case
    ///and bugs in implementation (crbug.com/1169639). `executionContextId` will be
    ///removed in the future.</pre>
    ///
    ///此项为 experimental，端点可能不支持。此项已 deprecated，保留用于旧端点兼容。
    #[serde(rename = "executionContextId")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub execution_context_id: Option<crate::runtime::types::ExecutionContextId>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.command.addBinding/parameters/executionContextName。
    ///
    ///协议原文：
    ///
    ///<pre>If specified, the binding is exposed to the executionContext with
    ///matching name, even for contexts created after the binding is added.
    ///See also `ExecutionContext.name` and `worldName` parameter to
    ///`Page.addScriptToEvaluateOnNewDocument`.
    ///This parameter is mutually exclusive with `executionContextId`.</pre>
    ///
    ///
    #[serde(rename = "executionContextName")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub execution_context_name: Option<String>,
}
impl<'de> serde::Deserialize<'de> for AddBindingParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = AddBindingParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Runtime.command.addBinding/parameters/name。
                    #[serde(rename = "name")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub name: String,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.command.addBinding/parameters/executionContextId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>If specified, the binding would only be exposed to the specified
                    ///execution context. If omitted and `executionContextName` is not set,
                    ///the binding is exposed to all execution contexts of the target.
                    ///This parameter is mutually exclusive with `executionContextName`.
                    ///Deprecated in favor of `executionContextName` due to an unclear use case
                    ///and bugs in implementation (crbug.com/1169639). `executionContextId` will be
                    ///removed in the future.</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。此项已 deprecated，保留用于旧端点兼容。
                    #[serde(rename = "executionContextId")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub execution_context_id: Option<crate::runtime::types::ExecutionContextId>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.command.addBinding/parameters/executionContextName。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>If specified, the binding is exposed to the executionContext with
                    ///matching name, even for contexts created after the binding is added.
                    ///See also `ExecutionContext.name` and `worldName` parameter to
                    ///`Page.addScriptToEvaluateOnNewDocument`.
                    ///This parameter is mutually exclusive with `executionContextId`.</pre>
                    ///
                    ///
                    #[serde(rename = "executionContextName")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub execution_context_name: Option<String>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(AddBindingParams {
                    name: fields.name,
                    execution_context_id: fields.execution_context_id,
                    execution_context_name: fields.execution_context_name,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Runtime.command.addBinding。
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
#[derive(Debug, Clone, Copy)]
pub struct AddBinding;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Runtime.command.addBinding/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct AddBindingResponse {}
impl<'de> serde::Deserialize<'de> for AddBindingResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = AddBindingResponse;
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
                Ok(AddBindingResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for AddBinding {
    type Params = AddBindingParams;
    type Response = AddBindingResponse;
    const METHOD: &'static str = "Runtime.addBinding";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Runtime.command.awaitPromise/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct AwaitPromiseParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Runtime.command.awaitPromise/parameters/promiseObjectId。
    ///
    ///协议原文：
    ///
    ///<pre>Identifier of the promise.</pre>
    ///
    ///
    #[serde(rename = "promiseObjectId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub promise_object_id: crate::runtime::types::RemoteObjectId,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.command.awaitPromise/parameters/returnByValue。
    ///
    ///协议原文：
    ///
    ///<pre>Whether the result is expected to be a JSON object that should be sent by value.</pre>
    ///
    ///
    #[serde(rename = "returnByValue")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub return_by_value: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.command.awaitPromise/parameters/generatePreview。
    ///
    ///协议原文：
    ///
    ///<pre>Whether preview should be generated for the result.</pre>
    ///
    ///
    #[serde(rename = "generatePreview")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub generate_preview: Option<bool>,
}
impl<'de> serde::Deserialize<'de> for AwaitPromiseParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = AwaitPromiseParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Runtime.command.awaitPromise/parameters/promiseObjectId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Identifier of the promise.</pre>
                    ///
                    ///
                    #[serde(rename = "promiseObjectId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub promise_object_id: crate::runtime::types::RemoteObjectId,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.command.awaitPromise/parameters/returnByValue。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Whether the result is expected to be a JSON object that should be sent by value.</pre>
                    ///
                    ///
                    #[serde(rename = "returnByValue")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub return_by_value: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.command.awaitPromise/parameters/generatePreview。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Whether preview should be generated for the result.</pre>
                    ///
                    ///
                    #[serde(rename = "generatePreview")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub generate_preview: Option<bool>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(AwaitPromiseParams {
                    promise_object_id: fields.promise_object_id,
                    return_by_value: fields.return_by_value,
                    generate_preview: fields.generate_preview,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Runtime.command.awaitPromise。
///
///协议原文：
///
///<pre>Add handler to promise with given promise object id.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct AwaitPromise;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Runtime.command.awaitPromise/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct AwaitPromiseResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Runtime.command.awaitPromise/returns/result。
    ///
    ///协议原文：
    ///
    ///<pre>Promise result. Will contain rejected value if promise was rejected.</pre>
    ///
    ///
    #[serde(rename = "result")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub result: crate::runtime::types::RemoteObject,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.command.awaitPromise/returns/exceptionDetails。
    ///
    ///协议原文：
    ///
    ///<pre>Exception details if stack strace is available.</pre>
    ///
    ///
    #[serde(rename = "exceptionDetails")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub exception_details: Option<crate::runtime::types::ExceptionDetails>,
}
impl<'de> serde::Deserialize<'de> for AwaitPromiseResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = AwaitPromiseResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Runtime.command.awaitPromise/returns/result。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Promise result. Will contain rejected value if promise was rejected.</pre>
                    ///
                    ///
                    #[serde(rename = "result")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub result: crate::runtime::types::RemoteObject,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.command.awaitPromise/returns/exceptionDetails。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Exception details if stack strace is available.</pre>
                    ///
                    ///
                    #[serde(rename = "exceptionDetails")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub exception_details: Option<crate::runtime::types::ExceptionDetails>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(AwaitPromiseResponse {
                    result: fields.result,
                    exception_details: fields.exception_details,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for AwaitPromise {
    type Params = AwaitPromiseParams;
    type Response = AwaitPromiseResponse;
    const METHOD: &'static str = "Runtime.awaitPromise";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Runtime.command.callFunctionOn/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct CallFunctionOnParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Runtime.command.callFunctionOn/parameters/functionDeclaration。
    ///
    ///协议原文：
    ///
    ///<pre>Declaration of the function to call.</pre>
    ///
    ///
    #[serde(rename = "functionDeclaration")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub function_declaration: String,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.command.callFunctionOn/parameters/objectId。
    ///
    ///协议原文：
    ///
    ///<pre>Identifier of the object to call function on. Either objectId or executionContextId should
    ///be specified.</pre>
    ///
    ///
    #[serde(rename = "objectId")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub object_id: Option<crate::runtime::types::RemoteObjectId>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.command.callFunctionOn/parameters/arguments。
    ///
    ///协议原文：
    ///
    ///<pre>Call arguments. All call arguments must belong to the same JavaScript world as the target
    ///object.</pre>
    ///
    ///
    #[serde(rename = "arguments")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub arguments: Option<Vec<crate::runtime::types::CallArgument>>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.command.callFunctionOn/parameters/silent。
    ///
    ///协议原文：
    ///
    ///<pre>In silent mode exceptions thrown during evaluation are not reported and do not pause
    ///execution. Overrides `setPauseOnException` state.</pre>
    ///
    ///
    #[serde(rename = "silent")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub silent: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.command.callFunctionOn/parameters/returnByValue。
    ///
    ///协议原文：
    ///
    ///<pre>Whether the result is expected to be a JSON object which should be sent by value.
    ///Can be overriden by `serializationOptions`.</pre>
    ///
    ///
    #[serde(rename = "returnByValue")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub return_by_value: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.command.callFunctionOn/parameters/generatePreview。
    ///
    ///协议原文：
    ///
    ///<pre>Whether preview should be generated for the result.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "generatePreview")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub generate_preview: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.command.callFunctionOn/parameters/userGesture。
    ///
    ///协议原文：
    ///
    ///<pre>Whether execution should be treated as initiated by user in the UI.</pre>
    ///
    ///
    #[serde(rename = "userGesture")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub user_gesture: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.command.callFunctionOn/parameters/awaitPromise。
    ///
    ///协议原文：
    ///
    ///<pre>Whether execution should `await` for resulting value and return once awaited promise is
    ///resolved.</pre>
    ///
    ///
    #[serde(rename = "awaitPromise")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub await_promise: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.command.callFunctionOn/parameters/executionContextId。
    ///
    ///协议原文：
    ///
    ///<pre>Specifies execution context which global object will be used to call function on. Either
    ///executionContextId or objectId should be specified.</pre>
    ///
    ///
    #[serde(rename = "executionContextId")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub execution_context_id: Option<crate::runtime::types::ExecutionContextId>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.command.callFunctionOn/parameters/objectGroup。
    ///
    ///协议原文：
    ///
    ///<pre>Symbolic group name that can be used to release multiple objects. If objectGroup is not
    ///specified and objectId is, objectGroup will be inherited from object.</pre>
    ///
    ///
    #[serde(rename = "objectGroup")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub object_group: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.command.callFunctionOn/parameters/throwOnSideEffect。
    ///
    ///协议原文：
    ///
    ///<pre>Whether to throw an exception if side effect cannot be ruled out during evaluation.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "throwOnSideEffect")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub throw_on_side_effect: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.command.callFunctionOn/parameters/uniqueContextId。
    ///
    ///协议原文：
    ///
    ///<pre>An alternative way to specify the execution context to call function on.
    ///Compared to contextId that may be reused across processes, this is guaranteed to be
    ///system-unique, so it can be used to prevent accidental function call
    ///in context different than intended (e.g. as a result of navigation across process
    ///boundaries).
    ///This is mutually exclusive with `executionContextId`.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "uniqueContextId")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub unique_context_id: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.command.callFunctionOn/parameters/serializationOptions。
    ///
    ///协议原文：
    ///
    ///<pre>Specifies the result serialization. If provided, overrides
    ///`generatePreview` and `returnByValue`.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "serializationOptions")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub serialization_options: Option<crate::runtime::types::SerializationOptions>,
}
impl<'de> serde::Deserialize<'de> for CallFunctionOnParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CallFunctionOnParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Runtime.command.callFunctionOn/parameters/functionDeclaration。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Declaration of the function to call.</pre>
                    ///
                    ///
                    #[serde(rename = "functionDeclaration")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub function_declaration: String,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.command.callFunctionOn/parameters/objectId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Identifier of the object to call function on. Either objectId or executionContextId should
                    ///be specified.</pre>
                    ///
                    ///
                    #[serde(rename = "objectId")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub object_id: Option<crate::runtime::types::RemoteObjectId>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.command.callFunctionOn/parameters/arguments。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Call arguments. All call arguments must belong to the same JavaScript world as the target
                    ///object.</pre>
                    ///
                    ///
                    #[serde(rename = "arguments")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub arguments: Option<Vec<crate::runtime::types::CallArgument>>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.command.callFunctionOn/parameters/silent。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>In silent mode exceptions thrown during evaluation are not reported and do not pause
                    ///execution. Overrides `setPauseOnException` state.</pre>
                    ///
                    ///
                    #[serde(rename = "silent")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub silent: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.command.callFunctionOn/parameters/returnByValue。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Whether the result is expected to be a JSON object which should be sent by value.
                    ///Can be overriden by `serializationOptions`.</pre>
                    ///
                    ///
                    #[serde(rename = "returnByValue")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub return_by_value: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.command.callFunctionOn/parameters/generatePreview。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Whether preview should be generated for the result.</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "generatePreview")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub generate_preview: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.command.callFunctionOn/parameters/userGesture。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Whether execution should be treated as initiated by user in the UI.</pre>
                    ///
                    ///
                    #[serde(rename = "userGesture")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub user_gesture: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.command.callFunctionOn/parameters/awaitPromise。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Whether execution should `await` for resulting value and return once awaited promise is
                    ///resolved.</pre>
                    ///
                    ///
                    #[serde(rename = "awaitPromise")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub await_promise: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.command.callFunctionOn/parameters/executionContextId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Specifies execution context which global object will be used to call function on. Either
                    ///executionContextId or objectId should be specified.</pre>
                    ///
                    ///
                    #[serde(rename = "executionContextId")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub execution_context_id: Option<crate::runtime::types::ExecutionContextId>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.command.callFunctionOn/parameters/objectGroup。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Symbolic group name that can be used to release multiple objects. If objectGroup is not
                    ///specified and objectId is, objectGroup will be inherited from object.</pre>
                    ///
                    ///
                    #[serde(rename = "objectGroup")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub object_group: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.command.callFunctionOn/parameters/throwOnSideEffect。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Whether to throw an exception if side effect cannot be ruled out during evaluation.</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "throwOnSideEffect")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub throw_on_side_effect: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.command.callFunctionOn/parameters/uniqueContextId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>An alternative way to specify the execution context to call function on.
                    ///Compared to contextId that may be reused across processes, this is guaranteed to be
                    ///system-unique, so it can be used to prevent accidental function call
                    ///in context different than intended (e.g. as a result of navigation across process
                    ///boundaries).
                    ///This is mutually exclusive with `executionContextId`.</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "uniqueContextId")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub unique_context_id: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.command.callFunctionOn/parameters/serializationOptions。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Specifies the result serialization. If provided, overrides
                    ///`generatePreview` and `returnByValue`.</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "serializationOptions")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub serialization_options: Option<crate::runtime::types::SerializationOptions>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(CallFunctionOnParams {
                    function_declaration: fields.function_declaration,
                    object_id: fields.object_id,
                    arguments: fields.arguments,
                    silent: fields.silent,
                    return_by_value: fields.return_by_value,
                    generate_preview: fields.generate_preview,
                    user_gesture: fields.user_gesture,
                    await_promise: fields.await_promise,
                    execution_context_id: fields.execution_context_id,
                    object_group: fields.object_group,
                    throw_on_side_effect: fields.throw_on_side_effect,
                    unique_context_id: fields.unique_context_id,
                    serialization_options: fields.serialization_options,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Runtime.command.callFunctionOn。
///
///协议原文：
///
///<pre>Calls function with given declaration on the given object. Object group of the result is
///inherited from the target object.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct CallFunctionOn;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Runtime.command.callFunctionOn/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct CallFunctionOnResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Runtime.command.callFunctionOn/returns/result。
    ///
    ///协议原文：
    ///
    ///<pre>Call result.</pre>
    ///
    ///
    #[serde(rename = "result")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub result: crate::runtime::types::RemoteObject,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.command.callFunctionOn/returns/exceptionDetails。
    ///
    ///协议原文：
    ///
    ///<pre>Exception details.</pre>
    ///
    ///
    #[serde(rename = "exceptionDetails")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub exception_details: Option<crate::runtime::types::ExceptionDetails>,
}
impl<'de> serde::Deserialize<'de> for CallFunctionOnResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CallFunctionOnResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Runtime.command.callFunctionOn/returns/result。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Call result.</pre>
                    ///
                    ///
                    #[serde(rename = "result")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub result: crate::runtime::types::RemoteObject,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.command.callFunctionOn/returns/exceptionDetails。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Exception details.</pre>
                    ///
                    ///
                    #[serde(rename = "exceptionDetails")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub exception_details: Option<crate::runtime::types::ExceptionDetails>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(CallFunctionOnResponse {
                    result: fields.result,
                    exception_details: fields.exception_details,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for CallFunctionOn {
    type Params = CallFunctionOnParams;
    type Response = CallFunctionOnResponse;
    const METHOD: &'static str = "Runtime.callFunctionOn";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Runtime.command.compileScript/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct CompileScriptParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Runtime.command.compileScript/parameters/expression。
    ///
    ///协议原文：
    ///
    ///<pre>Expression to compile.</pre>
    ///
    ///
    #[serde(rename = "expression")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub expression: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Runtime.command.compileScript/parameters/sourceURL。
    ///
    ///协议原文：
    ///
    ///<pre>Source url to be set for the script.</pre>
    ///
    ///
    #[serde(rename = "sourceURL")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub source_url: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Runtime.command.compileScript/parameters/persistScript。
    ///
    ///协议原文：
    ///
    ///<pre>Specifies whether the compiled script should be persisted.</pre>
    ///
    ///
    #[serde(rename = "persistScript")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub persist_script: bool,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.command.compileScript/parameters/executionContextId。
    ///
    ///协议原文：
    ///
    ///<pre>Specifies in which execution context to perform script run. If the parameter is omitted the
    ///evaluation will be performed in the context of the inspected page.</pre>
    ///
    ///
    #[serde(rename = "executionContextId")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub execution_context_id: Option<crate::runtime::types::ExecutionContextId>,
}
impl<'de> serde::Deserialize<'de> for CompileScriptParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CompileScriptParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Runtime.command.compileScript/parameters/expression。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Expression to compile.</pre>
                    ///
                    ///
                    #[serde(rename = "expression")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub expression: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Runtime.command.compileScript/parameters/sourceURL。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Source url to be set for the script.</pre>
                    ///
                    ///
                    #[serde(rename = "sourceURL")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub source_url: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Runtime.command.compileScript/parameters/persistScript。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Specifies whether the compiled script should be persisted.</pre>
                    ///
                    ///
                    #[serde(rename = "persistScript")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub persist_script: bool,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.command.compileScript/parameters/executionContextId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Specifies in which execution context to perform script run. If the parameter is omitted the
                    ///evaluation will be performed in the context of the inspected page.</pre>
                    ///
                    ///
                    #[serde(rename = "executionContextId")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub execution_context_id: Option<crate::runtime::types::ExecutionContextId>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(CompileScriptParams {
                    expression: fields.expression,
                    source_url: fields.source_url,
                    persist_script: fields.persist_script,
                    execution_context_id: fields.execution_context_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Runtime.command.compileScript。
///
///协议原文：
///
///<pre>Compiles expression.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct CompileScript;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Runtime.command.compileScript/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct CompileScriptResponse {
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.command.compileScript/returns/scriptId。
    ///
    ///协议原文：
    ///
    ///<pre>Id of the script.</pre>
    ///
    ///
    #[serde(rename = "scriptId")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub script_id: Option<crate::runtime::types::ScriptId>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.command.compileScript/returns/exceptionDetails。
    ///
    ///协议原文：
    ///
    ///<pre>Exception details.</pre>
    ///
    ///
    #[serde(rename = "exceptionDetails")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub exception_details: Option<crate::runtime::types::ExceptionDetails>,
}
impl<'de> serde::Deserialize<'de> for CompileScriptResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CompileScriptResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.command.compileScript/returns/scriptId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Id of the script.</pre>
                    ///
                    ///
                    #[serde(rename = "scriptId")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub script_id: Option<crate::runtime::types::ScriptId>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.command.compileScript/returns/exceptionDetails。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Exception details.</pre>
                    ///
                    ///
                    #[serde(rename = "exceptionDetails")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub exception_details: Option<crate::runtime::types::ExceptionDetails>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(CompileScriptResponse {
                    script_id: fields.script_id,
                    exception_details: fields.exception_details,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for CompileScript {
    type Params = CompileScriptParams;
    type Response = CompileScriptResponse;
    const METHOD: &'static str = "Runtime.compileScript";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Runtime.command.disable/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct DisableParams {}
impl<'de> serde::Deserialize<'de> for DisableParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = DisableParams;
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
                Ok(DisableParams {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Runtime.command.disable。
///
///协议原文：
///
///<pre>Disables reporting of execution contexts creation.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct Disable;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Runtime.command.disable/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct DisableResponse {}
impl<'de> serde::Deserialize<'de> for DisableResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = DisableResponse;
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
                Ok(DisableResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for Disable {
    type Params = DisableParams;
    type Response = DisableResponse;
    const METHOD: &'static str = "Runtime.disable";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Runtime.command.discardConsoleEntries/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct DiscardConsoleEntriesParams {}
impl<'de> serde::Deserialize<'de> for DiscardConsoleEntriesParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = DiscardConsoleEntriesParams;
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
                Ok(DiscardConsoleEntriesParams {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Runtime.command.discardConsoleEntries。
///
///协议原文：
///
///<pre>Discards collected exceptions and console API calls.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct DiscardConsoleEntries;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Runtime.command.discardConsoleEntries/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct DiscardConsoleEntriesResponse {}
impl<'de> serde::Deserialize<'de> for DiscardConsoleEntriesResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = DiscardConsoleEntriesResponse;
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
                Ok(DiscardConsoleEntriesResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for DiscardConsoleEntries {
    type Params = DiscardConsoleEntriesParams;
    type Response = DiscardConsoleEntriesResponse;
    const METHOD: &'static str = "Runtime.discardConsoleEntries";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Runtime.command.enable/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct EnableParams {}
impl<'de> serde::Deserialize<'de> for EnableParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = EnableParams;
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
                Ok(EnableParams {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Runtime.command.enable。
///
///协议原文：
///
///<pre>Enables reporting of execution contexts creation by means of `executionContextCreated` event.
///When the reporting gets enabled the event will be sent immediately for each existing execution
///context.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct Enable;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Runtime.command.enable/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct EnableResponse {}
impl<'de> serde::Deserialize<'de> for EnableResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = EnableResponse;
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
                Ok(EnableResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for Enable {
    type Params = EnableParams;
    type Response = EnableResponse;
    const METHOD: &'static str = "Runtime.enable";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Runtime.command.evaluate/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct EvaluateParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Runtime.command.evaluate/parameters/expression。
    ///
    ///协议原文：
    ///
    ///<pre>Expression to evaluate.</pre>
    ///
    ///
    #[serde(rename = "expression")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub expression: String,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.command.evaluate/parameters/objectGroup。
    ///
    ///协议原文：
    ///
    ///<pre>Symbolic group name that can be used to release multiple objects.</pre>
    ///
    ///
    #[serde(rename = "objectGroup")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub object_group: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.command.evaluate/parameters/includeCommandLineAPI。
    ///
    ///协议原文：
    ///
    ///<pre>Determines whether Command Line API should be available during the evaluation.</pre>
    ///
    ///
    #[serde(rename = "includeCommandLineAPI")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub include_command_line_api: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.command.evaluate/parameters/silent。
    ///
    ///协议原文：
    ///
    ///<pre>In silent mode exceptions thrown during evaluation are not reported and do not pause
    ///execution. Overrides `setPauseOnException` state.</pre>
    ///
    ///
    #[serde(rename = "silent")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub silent: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.command.evaluate/parameters/contextId。
    ///
    ///协议原文：
    ///
    ///<pre>Specifies in which execution context to perform evaluation. If the parameter is omitted the
    ///evaluation will be performed in the context of the inspected page.
    ///This is mutually exclusive with `uniqueContextId`, which offers an
    ///alternative way to identify the execution context that is more reliable
    ///in a multi-process environment.</pre>
    ///
    ///
    #[serde(rename = "contextId")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub context_id: Option<crate::runtime::types::ExecutionContextId>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.command.evaluate/parameters/returnByValue。
    ///
    ///协议原文：
    ///
    ///<pre>Whether the result is expected to be a JSON object that should be sent by value.</pre>
    ///
    ///
    #[serde(rename = "returnByValue")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub return_by_value: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.command.evaluate/parameters/generatePreview。
    ///
    ///协议原文：
    ///
    ///<pre>Whether preview should be generated for the result.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "generatePreview")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub generate_preview: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.command.evaluate/parameters/userGesture。
    ///
    ///协议原文：
    ///
    ///<pre>Whether execution should be treated as initiated by user in the UI.</pre>
    ///
    ///
    #[serde(rename = "userGesture")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub user_gesture: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.command.evaluate/parameters/awaitPromise。
    ///
    ///协议原文：
    ///
    ///<pre>Whether execution should `await` for resulting value and return once awaited promise is
    ///resolved.</pre>
    ///
    ///
    #[serde(rename = "awaitPromise")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub await_promise: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.command.evaluate/parameters/throwOnSideEffect。
    ///
    ///协议原文：
    ///
    ///<pre>Whether to throw an exception if side effect cannot be ruled out during evaluation.
    ///This implies `disableBreaks` below.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "throwOnSideEffect")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub throw_on_side_effect: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.command.evaluate/parameters/timeout。
    ///
    ///协议原文：
    ///
    ///<pre>Terminate execution after timing out (number of milliseconds).</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "timeout")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub timeout: Option<crate::runtime::types::TimeDelta>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.command.evaluate/parameters/disableBreaks。
    ///
    ///协议原文：
    ///
    ///<pre>Disable breakpoints during execution.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "disableBreaks")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub disable_breaks: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.command.evaluate/parameters/replMode。
    ///
    ///协议原文：
    ///
    ///<pre>Setting this flag to true enables `let` re-declaration and top-level `await`.
    ///Note that `let` variables can only be re-declared if they originate from
    ///`replMode` themselves.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "replMode")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub repl_mode: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.command.evaluate/parameters/allowUnsafeEvalBlockedByCSP。
    ///
    ///协议原文：
    ///
    ///<pre>The Content Security Policy (CSP) for the target might block 'unsafe-eval'
    ///which includes eval(), Function(), setTimeout() and setInterval()
    ///when called with non-callable arguments. This flag bypasses CSP for this
    ///evaluation and allows unsafe-eval. Defaults to true.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "allowUnsafeEvalBlockedByCSP")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub allow_unsafe_eval_blocked_by_csp: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.command.evaluate/parameters/uniqueContextId。
    ///
    ///协议原文：
    ///
    ///<pre>An alternative way to specify the execution context to evaluate in.
    ///Compared to contextId that may be reused across processes, this is guaranteed to be
    ///system-unique, so it can be used to prevent accidental evaluation of the expression
    ///in context different than intended (e.g. as a result of navigation across process
    ///boundaries).
    ///This is mutually exclusive with `contextId`.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "uniqueContextId")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub unique_context_id: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.command.evaluate/parameters/serializationOptions。
    ///
    ///协议原文：
    ///
    ///<pre>Specifies the result serialization. If provided, overrides
    ///`generatePreview` and `returnByValue`.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "serializationOptions")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub serialization_options: Option<crate::runtime::types::SerializationOptions>,
}
impl<'de> serde::Deserialize<'de> for EvaluateParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = EvaluateParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Runtime.command.evaluate/parameters/expression。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Expression to evaluate.</pre>
                    ///
                    ///
                    #[serde(rename = "expression")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub expression: String,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.command.evaluate/parameters/objectGroup。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Symbolic group name that can be used to release multiple objects.</pre>
                    ///
                    ///
                    #[serde(rename = "objectGroup")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub object_group: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.command.evaluate/parameters/includeCommandLineAPI。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Determines whether Command Line API should be available during the evaluation.</pre>
                    ///
                    ///
                    #[serde(rename = "includeCommandLineAPI")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub include_command_line_api: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.command.evaluate/parameters/silent。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>In silent mode exceptions thrown during evaluation are not reported and do not pause
                    ///execution. Overrides `setPauseOnException` state.</pre>
                    ///
                    ///
                    #[serde(rename = "silent")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub silent: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.command.evaluate/parameters/contextId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Specifies in which execution context to perform evaluation. If the parameter is omitted the
                    ///evaluation will be performed in the context of the inspected page.
                    ///This is mutually exclusive with `uniqueContextId`, which offers an
                    ///alternative way to identify the execution context that is more reliable
                    ///in a multi-process environment.</pre>
                    ///
                    ///
                    #[serde(rename = "contextId")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub context_id: Option<crate::runtime::types::ExecutionContextId>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.command.evaluate/parameters/returnByValue。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Whether the result is expected to be a JSON object that should be sent by value.</pre>
                    ///
                    ///
                    #[serde(rename = "returnByValue")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub return_by_value: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.command.evaluate/parameters/generatePreview。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Whether preview should be generated for the result.</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "generatePreview")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub generate_preview: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.command.evaluate/parameters/userGesture。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Whether execution should be treated as initiated by user in the UI.</pre>
                    ///
                    ///
                    #[serde(rename = "userGesture")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub user_gesture: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.command.evaluate/parameters/awaitPromise。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Whether execution should `await` for resulting value and return once awaited promise is
                    ///resolved.</pre>
                    ///
                    ///
                    #[serde(rename = "awaitPromise")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub await_promise: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.command.evaluate/parameters/throwOnSideEffect。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Whether to throw an exception if side effect cannot be ruled out during evaluation.
                    ///This implies `disableBreaks` below.</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "throwOnSideEffect")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub throw_on_side_effect: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.command.evaluate/parameters/timeout。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Terminate execution after timing out (number of milliseconds).</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "timeout")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub timeout: Option<crate::runtime::types::TimeDelta>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.command.evaluate/parameters/disableBreaks。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Disable breakpoints during execution.</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "disableBreaks")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub disable_breaks: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.command.evaluate/parameters/replMode。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Setting this flag to true enables `let` re-declaration and top-level `await`.
                    ///Note that `let` variables can only be re-declared if they originate from
                    ///`replMode` themselves.</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "replMode")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub repl_mode: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.command.evaluate/parameters/allowUnsafeEvalBlockedByCSP。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The Content Security Policy (CSP) for the target might block 'unsafe-eval'
                    ///which includes eval(), Function(), setTimeout() and setInterval()
                    ///when called with non-callable arguments. This flag bypasses CSP for this
                    ///evaluation and allows unsafe-eval. Defaults to true.</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "allowUnsafeEvalBlockedByCSP")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub allow_unsafe_eval_blocked_by_csp: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.command.evaluate/parameters/uniqueContextId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>An alternative way to specify the execution context to evaluate in.
                    ///Compared to contextId that may be reused across processes, this is guaranteed to be
                    ///system-unique, so it can be used to prevent accidental evaluation of the expression
                    ///in context different than intended (e.g. as a result of navigation across process
                    ///boundaries).
                    ///This is mutually exclusive with `contextId`.</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "uniqueContextId")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub unique_context_id: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.command.evaluate/parameters/serializationOptions。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Specifies the result serialization. If provided, overrides
                    ///`generatePreview` and `returnByValue`.</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "serializationOptions")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub serialization_options: Option<crate::runtime::types::SerializationOptions>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(EvaluateParams {
                    expression: fields.expression,
                    object_group: fields.object_group,
                    include_command_line_api: fields.include_command_line_api,
                    silent: fields.silent,
                    context_id: fields.context_id,
                    return_by_value: fields.return_by_value,
                    generate_preview: fields.generate_preview,
                    user_gesture: fields.user_gesture,
                    await_promise: fields.await_promise,
                    throw_on_side_effect: fields.throw_on_side_effect,
                    timeout: fields.timeout,
                    disable_breaks: fields.disable_breaks,
                    repl_mode: fields.repl_mode,
                    allow_unsafe_eval_blocked_by_csp: fields.allow_unsafe_eval_blocked_by_csp,
                    unique_context_id: fields.unique_context_id,
                    serialization_options: fields.serialization_options,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Runtime.command.evaluate。
///
///协议原文：
///
///<pre>Evaluates expression on global object.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct Evaluate;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Runtime.command.evaluate/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct EvaluateResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Runtime.command.evaluate/returns/result。
    ///
    ///协议原文：
    ///
    ///<pre>Evaluation result.</pre>
    ///
    ///
    #[serde(rename = "result")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub result: crate::runtime::types::RemoteObject,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.command.evaluate/returns/exceptionDetails。
    ///
    ///协议原文：
    ///
    ///<pre>Exception details.</pre>
    ///
    ///
    #[serde(rename = "exceptionDetails")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub exception_details: Option<crate::runtime::types::ExceptionDetails>,
}
impl<'de> serde::Deserialize<'de> for EvaluateResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = EvaluateResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Runtime.command.evaluate/returns/result。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Evaluation result.</pre>
                    ///
                    ///
                    #[serde(rename = "result")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub result: crate::runtime::types::RemoteObject,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.command.evaluate/returns/exceptionDetails。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Exception details.</pre>
                    ///
                    ///
                    #[serde(rename = "exceptionDetails")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub exception_details: Option<crate::runtime::types::ExceptionDetails>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(EvaluateResponse {
                    result: fields.result,
                    exception_details: fields.exception_details,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for Evaluate {
    type Params = EvaluateParams;
    type Response = EvaluateResponse;
    const METHOD: &'static str = "Runtime.evaluate";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Runtime.command.getExceptionDetails/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct GetExceptionDetailsParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Runtime.command.getExceptionDetails/parameters/errorObjectId。
    ///
    ///协议原文：
    ///
    ///<pre>The error object for which to resolve the exception details.</pre>
    ///
    ///
    #[serde(rename = "errorObjectId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub error_object_id: crate::runtime::types::RemoteObjectId,
}
impl<'de> serde::Deserialize<'de> for GetExceptionDetailsParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetExceptionDetailsParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Runtime.command.getExceptionDetails/parameters/errorObjectId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The error object for which to resolve the exception details.</pre>
                    ///
                    ///
                    #[serde(rename = "errorObjectId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub error_object_id: crate::runtime::types::RemoteObjectId,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetExceptionDetailsParams {
                    error_object_id: fields.error_object_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Runtime.command.getExceptionDetails。
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
#[derive(Debug, Clone, Copy)]
pub struct GetExceptionDetails;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Runtime.command.getExceptionDetails/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct GetExceptionDetailsResponse {
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.command.getExceptionDetails/returns/exceptionDetails。
    #[serde(rename = "exceptionDetails")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub exception_details: Option<crate::runtime::types::ExceptionDetails>,
}
impl<'de> serde::Deserialize<'de> for GetExceptionDetailsResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetExceptionDetailsResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.command.getExceptionDetails/returns/exceptionDetails。
                    #[serde(rename = "exceptionDetails")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub exception_details: Option<crate::runtime::types::ExceptionDetails>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetExceptionDetailsResponse {
                    exception_details: fields.exception_details,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for GetExceptionDetails {
    type Params = GetExceptionDetailsParams;
    type Response = GetExceptionDetailsResponse;
    const METHOD: &'static str = "Runtime.getExceptionDetails";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Runtime.command.getHeapUsage/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct GetHeapUsageParams {}
impl<'de> serde::Deserialize<'de> for GetHeapUsageParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetHeapUsageParams;
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
                Ok(GetHeapUsageParams {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Runtime.command.getHeapUsage。
///
///协议原文：
///
///<pre>Returns the JavaScript heap usage.
///It is the total usage of the corresponding isolate not scoped to a particular Runtime.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct GetHeapUsage;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Runtime.command.getHeapUsage/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct GetHeapUsageResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Runtime.command.getHeapUsage/returns/usedSize。
    ///
    ///协议原文：
    ///
    ///<pre>Used JavaScript heap size in bytes.</pre>
    ///
    ///
    #[serde(rename = "usedSize")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub used_size: ydp_core::FiniteF64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Runtime.command.getHeapUsage/returns/totalSize。
    ///
    ///协议原文：
    ///
    ///<pre>Allocated JavaScript heap size in bytes.</pre>
    ///
    ///
    #[serde(rename = "totalSize")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub total_size: ydp_core::FiniteF64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Runtime.command.getHeapUsage/returns/embedderHeapUsedSize。
    ///
    ///协议原文：
    ///
    ///<pre>Used size in bytes in the embedder's garbage-collected heap.</pre>
    ///
    ///
    #[serde(rename = "embedderHeapUsedSize")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub embedder_heap_used_size: ydp_core::FiniteF64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Runtime.command.getHeapUsage/returns/backingStorageSize。
    ///
    ///协议原文：
    ///
    ///<pre>Size in bytes of backing storage for array buffers and external strings.</pre>
    ///
    ///
    #[serde(rename = "backingStorageSize")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub backing_storage_size: ydp_core::FiniteF64,
}
impl<'de> serde::Deserialize<'de> for GetHeapUsageResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetHeapUsageResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Runtime.command.getHeapUsage/returns/usedSize。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Used JavaScript heap size in bytes.</pre>
                    ///
                    ///
                    #[serde(rename = "usedSize")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub used_size: ydp_core::FiniteF64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Runtime.command.getHeapUsage/returns/totalSize。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Allocated JavaScript heap size in bytes.</pre>
                    ///
                    ///
                    #[serde(rename = "totalSize")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub total_size: ydp_core::FiniteF64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Runtime.command.getHeapUsage/returns/embedderHeapUsedSize。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Used size in bytes in the embedder's garbage-collected heap.</pre>
                    ///
                    ///
                    #[serde(rename = "embedderHeapUsedSize")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub embedder_heap_used_size: ydp_core::FiniteF64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Runtime.command.getHeapUsage/returns/backingStorageSize。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Size in bytes of backing storage for array buffers and external strings.</pre>
                    ///
                    ///
                    #[serde(rename = "backingStorageSize")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub backing_storage_size: ydp_core::FiniteF64,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetHeapUsageResponse {
                    used_size: fields.used_size,
                    total_size: fields.total_size,
                    embedder_heap_used_size: fields.embedder_heap_used_size,
                    backing_storage_size: fields.backing_storage_size,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for GetHeapUsage {
    type Params = GetHeapUsageParams;
    type Response = GetHeapUsageResponse;
    const METHOD: &'static str = "Runtime.getHeapUsage";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Runtime.command.getIsolateId/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct GetIsolateIdParams {}
impl<'de> serde::Deserialize<'de> for GetIsolateIdParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetIsolateIdParams;
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
                Ok(GetIsolateIdParams {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Runtime.command.getIsolateId。
///
///协议原文：
///
///<pre>Returns the isolate id.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct GetIsolateId;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Runtime.command.getIsolateId/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct GetIsolateIdResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Runtime.command.getIsolateId/returns/id。
    ///
    ///协议原文：
    ///
    ///<pre>The isolate id.</pre>
    ///
    ///
    #[serde(rename = "id")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub id: String,
}
impl<'de> serde::Deserialize<'de> for GetIsolateIdResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetIsolateIdResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Runtime.command.getIsolateId/returns/id。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The isolate id.</pre>
                    ///
                    ///
                    #[serde(rename = "id")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub id: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetIsolateIdResponse { id: fields.id })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for GetIsolateId {
    type Params = GetIsolateIdParams;
    type Response = GetIsolateIdResponse;
    const METHOD: &'static str = "Runtime.getIsolateId";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Runtime.command.getProperties/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct GetPropertiesParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Runtime.command.getProperties/parameters/objectId。
    ///
    ///协议原文：
    ///
    ///<pre>Identifier of the object to return properties for.</pre>
    ///
    ///
    #[serde(rename = "objectId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub object_id: crate::runtime::types::RemoteObjectId,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.command.getProperties/parameters/ownProperties。
    ///
    ///协议原文：
    ///
    ///<pre>If true, returns properties belonging only to the element itself, not to its prototype
    ///chain.</pre>
    ///
    ///
    #[serde(rename = "ownProperties")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub own_properties: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.command.getProperties/parameters/accessorPropertiesOnly。
    ///
    ///协议原文：
    ///
    ///<pre>If true, returns accessor properties (with getter/setter) only; internal properties are not
    ///returned either.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "accessorPropertiesOnly")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub accessor_properties_only: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.command.getProperties/parameters/generatePreview。
    ///
    ///协议原文：
    ///
    ///<pre>Whether preview should be generated for the results.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "generatePreview")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub generate_preview: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.command.getProperties/parameters/nonIndexedPropertiesOnly。
    ///
    ///协议原文：
    ///
    ///<pre>If true, returns non-indexed properties only.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "nonIndexedPropertiesOnly")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub non_indexed_properties_only: Option<bool>,
}
impl<'de> serde::Deserialize<'de> for GetPropertiesParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetPropertiesParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Runtime.command.getProperties/parameters/objectId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Identifier of the object to return properties for.</pre>
                    ///
                    ///
                    #[serde(rename = "objectId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub object_id: crate::runtime::types::RemoteObjectId,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.command.getProperties/parameters/ownProperties。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>If true, returns properties belonging only to the element itself, not to its prototype
                    ///chain.</pre>
                    ///
                    ///
                    #[serde(rename = "ownProperties")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub own_properties: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.command.getProperties/parameters/accessorPropertiesOnly。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>If true, returns accessor properties (with getter/setter) only; internal properties are not
                    ///returned either.</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "accessorPropertiesOnly")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub accessor_properties_only: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.command.getProperties/parameters/generatePreview。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Whether preview should be generated for the results.</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "generatePreview")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub generate_preview: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.command.getProperties/parameters/nonIndexedPropertiesOnly。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>If true, returns non-indexed properties only.</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "nonIndexedPropertiesOnly")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub non_indexed_properties_only: Option<bool>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetPropertiesParams {
                    object_id: fields.object_id,
                    own_properties: fields.own_properties,
                    accessor_properties_only: fields.accessor_properties_only,
                    generate_preview: fields.generate_preview,
                    non_indexed_properties_only: fields.non_indexed_properties_only,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Runtime.command.getProperties。
///
///协议原文：
///
///<pre>Returns properties of a given object. Object group of the result is inherited from the target
///object.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct GetProperties;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Runtime.command.getProperties/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct GetPropertiesResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Runtime.command.getProperties/returns/result。
    ///
    ///协议原文：
    ///
    ///<pre>Object properties.</pre>
    ///
    ///
    #[serde(rename = "result")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub result: Vec<crate::runtime::types::PropertyDescriptor>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.command.getProperties/returns/internalProperties。
    ///
    ///协议原文：
    ///
    ///<pre>Internal object properties (only of the element itself).</pre>
    ///
    ///
    #[serde(rename = "internalProperties")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub internal_properties: Option<Vec<crate::runtime::types::InternalPropertyDescriptor>>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.command.getProperties/returns/privateProperties。
    ///
    ///协议原文：
    ///
    ///<pre>Object private properties.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "privateProperties")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub private_properties: Option<Vec<crate::runtime::types::PrivatePropertyDescriptor>>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.command.getProperties/returns/exceptionDetails。
    ///
    ///协议原文：
    ///
    ///<pre>Exception details.</pre>
    ///
    ///
    #[serde(rename = "exceptionDetails")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub exception_details: Option<crate::runtime::types::ExceptionDetails>,
}
impl<'de> serde::Deserialize<'de> for GetPropertiesResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetPropertiesResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Runtime.command.getProperties/returns/result。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Object properties.</pre>
                    ///
                    ///
                    #[serde(rename = "result")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub result: Vec<crate::runtime::types::PropertyDescriptor>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.command.getProperties/returns/internalProperties。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Internal object properties (only of the element itself).</pre>
                    ///
                    ///
                    #[serde(rename = "internalProperties")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub internal_properties:
                        Option<Vec<crate::runtime::types::InternalPropertyDescriptor>>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.command.getProperties/returns/privateProperties。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Object private properties.</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "privateProperties")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub private_properties:
                        Option<Vec<crate::runtime::types::PrivatePropertyDescriptor>>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.command.getProperties/returns/exceptionDetails。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Exception details.</pre>
                    ///
                    ///
                    #[serde(rename = "exceptionDetails")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub exception_details: Option<crate::runtime::types::ExceptionDetails>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetPropertiesResponse {
                    result: fields.result,
                    internal_properties: fields.internal_properties,
                    private_properties: fields.private_properties,
                    exception_details: fields.exception_details,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for GetProperties {
    type Params = GetPropertiesParams;
    type Response = GetPropertiesResponse;
    const METHOD: &'static str = "Runtime.getProperties";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Runtime.command.globalLexicalScopeNames/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct GlobalLexicalScopeNamesParams {
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.command.globalLexicalScopeNames/parameters/executionContextId。
    ///
    ///协议原文：
    ///
    ///<pre>Specifies in which execution context to lookup global scope variables.</pre>
    ///
    ///
    #[serde(rename = "executionContextId")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub execution_context_id: Option<crate::runtime::types::ExecutionContextId>,
}
impl<'de> serde::Deserialize<'de> for GlobalLexicalScopeNamesParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GlobalLexicalScopeNamesParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.command.globalLexicalScopeNames/parameters/executionContextId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Specifies in which execution context to lookup global scope variables.</pre>
                    ///
                    ///
                    #[serde(rename = "executionContextId")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub execution_context_id: Option<crate::runtime::types::ExecutionContextId>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GlobalLexicalScopeNamesParams {
                    execution_context_id: fields.execution_context_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Runtime.command.globalLexicalScopeNames。
///
///协议原文：
///
///<pre>Returns all let, const and class variables from global scope.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct GlobalLexicalScopeNames;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Runtime.command.globalLexicalScopeNames/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct GlobalLexicalScopeNamesResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Runtime.command.globalLexicalScopeNames/returns/names。
    #[serde(rename = "names")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub names: Vec<String>,
}
impl<'de> serde::Deserialize<'de> for GlobalLexicalScopeNamesResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GlobalLexicalScopeNamesResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Runtime.command.globalLexicalScopeNames/returns/names。
                    #[serde(rename = "names")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub names: Vec<String>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GlobalLexicalScopeNamesResponse {
                    names: fields.names,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for GlobalLexicalScopeNames {
    type Params = GlobalLexicalScopeNamesParams;
    type Response = GlobalLexicalScopeNamesResponse;
    const METHOD: &'static str = "Runtime.globalLexicalScopeNames";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Runtime.command.queryObjects/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct QueryObjectsParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Runtime.command.queryObjects/parameters/prototypeObjectId。
    ///
    ///协议原文：
    ///
    ///<pre>Identifier of the prototype to return objects for.</pre>
    ///
    ///
    #[serde(rename = "prototypeObjectId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub prototype_object_id: crate::runtime::types::RemoteObjectId,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.command.queryObjects/parameters/objectGroup。
    ///
    ///协议原文：
    ///
    ///<pre>Symbolic group name that can be used to release the results.</pre>
    ///
    ///
    #[serde(rename = "objectGroup")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub object_group: Option<String>,
}
impl<'de> serde::Deserialize<'de> for QueryObjectsParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = QueryObjectsParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Runtime.command.queryObjects/parameters/prototypeObjectId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Identifier of the prototype to return objects for.</pre>
                    ///
                    ///
                    #[serde(rename = "prototypeObjectId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub prototype_object_id: crate::runtime::types::RemoteObjectId,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.command.queryObjects/parameters/objectGroup。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Symbolic group name that can be used to release the results.</pre>
                    ///
                    ///
                    #[serde(rename = "objectGroup")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub object_group: Option<String>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(QueryObjectsParams {
                    prototype_object_id: fields.prototype_object_id,
                    object_group: fields.object_group,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Runtime.command.queryObjects。
#[derive(Debug, Clone, Copy)]
pub struct QueryObjects;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Runtime.command.queryObjects/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct QueryObjectsResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Runtime.command.queryObjects/returns/objects。
    ///
    ///协议原文：
    ///
    ///<pre>Array with objects.</pre>
    ///
    ///
    #[serde(rename = "objects")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub objects: crate::runtime::types::RemoteObject,
}
impl<'de> serde::Deserialize<'de> for QueryObjectsResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = QueryObjectsResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Runtime.command.queryObjects/returns/objects。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Array with objects.</pre>
                    ///
                    ///
                    #[serde(rename = "objects")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub objects: crate::runtime::types::RemoteObject,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(QueryObjectsResponse {
                    objects: fields.objects,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for QueryObjects {
    type Params = QueryObjectsParams;
    type Response = QueryObjectsResponse;
    const METHOD: &'static str = "Runtime.queryObjects";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Runtime.command.releaseObject/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct ReleaseObjectParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Runtime.command.releaseObject/parameters/objectId。
    ///
    ///协议原文：
    ///
    ///<pre>Identifier of the object to release.</pre>
    ///
    ///
    #[serde(rename = "objectId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub object_id: crate::runtime::types::RemoteObjectId,
}
impl<'de> serde::Deserialize<'de> for ReleaseObjectParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ReleaseObjectParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Runtime.command.releaseObject/parameters/objectId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Identifier of the object to release.</pre>
                    ///
                    ///
                    #[serde(rename = "objectId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub object_id: crate::runtime::types::RemoteObjectId,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(ReleaseObjectParams {
                    object_id: fields.object_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Runtime.command.releaseObject。
///
///协议原文：
///
///<pre>Releases remote object with given id.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct ReleaseObject;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Runtime.command.releaseObject/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct ReleaseObjectResponse {}
impl<'de> serde::Deserialize<'de> for ReleaseObjectResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ReleaseObjectResponse;
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
                Ok(ReleaseObjectResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for ReleaseObject {
    type Params = ReleaseObjectParams;
    type Response = ReleaseObjectResponse;
    const METHOD: &'static str = "Runtime.releaseObject";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Runtime.command.releaseObjectGroup/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct ReleaseObjectGroupParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Runtime.command.releaseObjectGroup/parameters/objectGroup。
    ///
    ///协议原文：
    ///
    ///<pre>Symbolic object group name.</pre>
    ///
    ///
    #[serde(rename = "objectGroup")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub object_group: String,
}
impl<'de> serde::Deserialize<'de> for ReleaseObjectGroupParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ReleaseObjectGroupParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Runtime.command.releaseObjectGroup/parameters/objectGroup。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Symbolic object group name.</pre>
                    ///
                    ///
                    #[serde(rename = "objectGroup")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub object_group: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(ReleaseObjectGroupParams {
                    object_group: fields.object_group,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Runtime.command.releaseObjectGroup。
///
///协议原文：
///
///<pre>Releases all remote objects that belong to a given group.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct ReleaseObjectGroup;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Runtime.command.releaseObjectGroup/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct ReleaseObjectGroupResponse {}
impl<'de> serde::Deserialize<'de> for ReleaseObjectGroupResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ReleaseObjectGroupResponse;
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
                Ok(ReleaseObjectGroupResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for ReleaseObjectGroup {
    type Params = ReleaseObjectGroupParams;
    type Response = ReleaseObjectGroupResponse;
    const METHOD: &'static str = "Runtime.releaseObjectGroup";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Runtime.command.removeBinding/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct RemoveBindingParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Runtime.command.removeBinding/parameters/name。
    #[serde(rename = "name")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub name: String,
}
impl<'de> serde::Deserialize<'de> for RemoveBindingParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = RemoveBindingParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Runtime.command.removeBinding/parameters/name。
                    #[serde(rename = "name")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub name: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(RemoveBindingParams { name: fields.name })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Runtime.command.removeBinding。
///
///协议原文：
///
///<pre>This method does not remove binding function from global object but
///unsubscribes current runtime agent from Runtime.bindingCalled notifications.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct RemoveBinding;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Runtime.command.removeBinding/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct RemoveBindingResponse {}
impl<'de> serde::Deserialize<'de> for RemoveBindingResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = RemoveBindingResponse;
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
                Ok(RemoveBindingResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for RemoveBinding {
    type Params = RemoveBindingParams;
    type Response = RemoveBindingResponse;
    const METHOD: &'static str = "Runtime.removeBinding";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Runtime.command.runIfWaitingForDebugger/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct RunIfWaitingForDebuggerParams {}
impl<'de> serde::Deserialize<'de> for RunIfWaitingForDebuggerParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = RunIfWaitingForDebuggerParams;
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
                Ok(RunIfWaitingForDebuggerParams {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Runtime.command.runIfWaitingForDebugger。
///
///协议原文：
///
///<pre>Tells inspected instance to run if it was waiting for debugger to attach.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct RunIfWaitingForDebugger;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Runtime.command.runIfWaitingForDebugger/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct RunIfWaitingForDebuggerResponse {}
impl<'de> serde::Deserialize<'de> for RunIfWaitingForDebuggerResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = RunIfWaitingForDebuggerResponse;
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
                Ok(RunIfWaitingForDebuggerResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for RunIfWaitingForDebugger {
    type Params = RunIfWaitingForDebuggerParams;
    type Response = RunIfWaitingForDebuggerResponse;
    const METHOD: &'static str = "Runtime.runIfWaitingForDebugger";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Runtime.command.runScript/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct RunScriptParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Runtime.command.runScript/parameters/scriptId。
    ///
    ///协议原文：
    ///
    ///<pre>Id of the script to run.</pre>
    ///
    ///
    #[serde(rename = "scriptId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub script_id: crate::runtime::types::ScriptId,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.command.runScript/parameters/executionContextId。
    ///
    ///协议原文：
    ///
    ///<pre>Specifies in which execution context to perform script run. If the parameter is omitted the
    ///evaluation will be performed in the context of the inspected page.</pre>
    ///
    ///
    #[serde(rename = "executionContextId")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub execution_context_id: Option<crate::runtime::types::ExecutionContextId>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.command.runScript/parameters/objectGroup。
    ///
    ///协议原文：
    ///
    ///<pre>Symbolic group name that can be used to release multiple objects.</pre>
    ///
    ///
    #[serde(rename = "objectGroup")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub object_group: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.command.runScript/parameters/silent。
    ///
    ///协议原文：
    ///
    ///<pre>In silent mode exceptions thrown during evaluation are not reported and do not pause
    ///execution. Overrides `setPauseOnException` state.</pre>
    ///
    ///
    #[serde(rename = "silent")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub silent: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.command.runScript/parameters/includeCommandLineAPI。
    ///
    ///协议原文：
    ///
    ///<pre>Determines whether Command Line API should be available during the evaluation.</pre>
    ///
    ///
    #[serde(rename = "includeCommandLineAPI")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub include_command_line_api: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.command.runScript/parameters/returnByValue。
    ///
    ///协议原文：
    ///
    ///<pre>Whether the result is expected to be a JSON object which should be sent by value.</pre>
    ///
    ///
    #[serde(rename = "returnByValue")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub return_by_value: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.command.runScript/parameters/generatePreview。
    ///
    ///协议原文：
    ///
    ///<pre>Whether preview should be generated for the result.</pre>
    ///
    ///
    #[serde(rename = "generatePreview")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub generate_preview: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.command.runScript/parameters/awaitPromise。
    ///
    ///协议原文：
    ///
    ///<pre>Whether execution should `await` for resulting value and return once awaited promise is
    ///resolved.</pre>
    ///
    ///
    #[serde(rename = "awaitPromise")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub await_promise: Option<bool>,
}
impl<'de> serde::Deserialize<'de> for RunScriptParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = RunScriptParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Runtime.command.runScript/parameters/scriptId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Id of the script to run.</pre>
                    ///
                    ///
                    #[serde(rename = "scriptId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub script_id: crate::runtime::types::ScriptId,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.command.runScript/parameters/executionContextId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Specifies in which execution context to perform script run. If the parameter is omitted the
                    ///evaluation will be performed in the context of the inspected page.</pre>
                    ///
                    ///
                    #[serde(rename = "executionContextId")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub execution_context_id: Option<crate::runtime::types::ExecutionContextId>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.command.runScript/parameters/objectGroup。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Symbolic group name that can be used to release multiple objects.</pre>
                    ///
                    ///
                    #[serde(rename = "objectGroup")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub object_group: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.command.runScript/parameters/silent。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>In silent mode exceptions thrown during evaluation are not reported and do not pause
                    ///execution. Overrides `setPauseOnException` state.</pre>
                    ///
                    ///
                    #[serde(rename = "silent")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub silent: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.command.runScript/parameters/includeCommandLineAPI。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Determines whether Command Line API should be available during the evaluation.</pre>
                    ///
                    ///
                    #[serde(rename = "includeCommandLineAPI")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub include_command_line_api: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.command.runScript/parameters/returnByValue。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Whether the result is expected to be a JSON object which should be sent by value.</pre>
                    ///
                    ///
                    #[serde(rename = "returnByValue")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub return_by_value: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.command.runScript/parameters/generatePreview。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Whether preview should be generated for the result.</pre>
                    ///
                    ///
                    #[serde(rename = "generatePreview")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub generate_preview: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.command.runScript/parameters/awaitPromise。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Whether execution should `await` for resulting value and return once awaited promise is
                    ///resolved.</pre>
                    ///
                    ///
                    #[serde(rename = "awaitPromise")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub await_promise: Option<bool>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(RunScriptParams {
                    script_id: fields.script_id,
                    execution_context_id: fields.execution_context_id,
                    object_group: fields.object_group,
                    silent: fields.silent,
                    include_command_line_api: fields.include_command_line_api,
                    return_by_value: fields.return_by_value,
                    generate_preview: fields.generate_preview,
                    await_promise: fields.await_promise,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Runtime.command.runScript。
///
///协议原文：
///
///<pre>Runs script with given id in a given context.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct RunScript;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Runtime.command.runScript/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct RunScriptResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Runtime.command.runScript/returns/result。
    ///
    ///协议原文：
    ///
    ///<pre>Run result.</pre>
    ///
    ///
    #[serde(rename = "result")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub result: crate::runtime::types::RemoteObject,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.command.runScript/returns/exceptionDetails。
    ///
    ///协议原文：
    ///
    ///<pre>Exception details.</pre>
    ///
    ///
    #[serde(rename = "exceptionDetails")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub exception_details: Option<crate::runtime::types::ExceptionDetails>,
}
impl<'de> serde::Deserialize<'de> for RunScriptResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = RunScriptResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Runtime.command.runScript/returns/result。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Run result.</pre>
                    ///
                    ///
                    #[serde(rename = "result")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub result: crate::runtime::types::RemoteObject,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.command.runScript/returns/exceptionDetails。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Exception details.</pre>
                    ///
                    ///
                    #[serde(rename = "exceptionDetails")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub exception_details: Option<crate::runtime::types::ExceptionDetails>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(RunScriptResponse {
                    result: fields.result,
                    exception_details: fields.exception_details,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for RunScript {
    type Params = RunScriptParams;
    type Response = RunScriptResponse;
    const METHOD: &'static str = "Runtime.runScript";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Runtime.command.setAsyncCallStackDepth/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetAsyncCallStackDepthParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Runtime.command.setAsyncCallStackDepth/parameters/maxDepth。
    ///
    ///协议原文：
    ///
    ///<pre>Maximum depth of async call stacks. Setting to `0` will effectively disable collecting async
    ///call stacks (default).</pre>
    ///
    ///
    #[serde(rename = "maxDepth")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub max_depth: i64,
}
impl<'de> serde::Deserialize<'de> for SetAsyncCallStackDepthParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetAsyncCallStackDepthParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Runtime.command.setAsyncCallStackDepth/parameters/maxDepth。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Maximum depth of async call stacks. Setting to `0` will effectively disable collecting async
                    ///call stacks (default).</pre>
                    ///
                    ///
                    #[serde(rename = "maxDepth")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub max_depth: i64,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SetAsyncCallStackDepthParams {
                    max_depth: fields.max_depth,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Runtime.command.setAsyncCallStackDepth。
///
///协议原文：
///
///<pre>Enables or disables async call stacks tracking.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct SetAsyncCallStackDepth;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Runtime.command.setAsyncCallStackDepth/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetAsyncCallStackDepthResponse {}
impl<'de> serde::Deserialize<'de> for SetAsyncCallStackDepthResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetAsyncCallStackDepthResponse;
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
                Ok(SetAsyncCallStackDepthResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for SetAsyncCallStackDepth {
    type Params = SetAsyncCallStackDepthParams;
    type Response = SetAsyncCallStackDepthResponse;
    const METHOD: &'static str = "Runtime.setAsyncCallStackDepth";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Runtime.command.setCustomObjectFormatterEnabled/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetCustomObjectFormatterEnabledParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Runtime.command.setCustomObjectFormatterEnabled/parameters/enabled。
    #[serde(rename = "enabled")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub enabled: bool,
}
impl<'de> serde::Deserialize<'de> for SetCustomObjectFormatterEnabledParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetCustomObjectFormatterEnabledParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Runtime.command.setCustomObjectFormatterEnabled/parameters/enabled。
                    #[serde(rename = "enabled")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub enabled: bool,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SetCustomObjectFormatterEnabledParams {
                    enabled: fields.enabled,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Runtime.command.setCustomObjectFormatterEnabled。此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct SetCustomObjectFormatterEnabled;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Runtime.command.setCustomObjectFormatterEnabled/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetCustomObjectFormatterEnabledResponse {}
impl<'de> serde::Deserialize<'de> for SetCustomObjectFormatterEnabledResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetCustomObjectFormatterEnabledResponse;
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
                Ok(SetCustomObjectFormatterEnabledResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for SetCustomObjectFormatterEnabled {
    type Params = SetCustomObjectFormatterEnabledParams;
    type Response = SetCustomObjectFormatterEnabledResponse;
    const METHOD: &'static str = "Runtime.setCustomObjectFormatterEnabled";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Runtime.command.setMaxCallStackSizeToCapture/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetMaxCallStackSizeToCaptureParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Runtime.command.setMaxCallStackSizeToCapture/parameters/size。
    #[serde(rename = "size")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub size: i64,
}
impl<'de> serde::Deserialize<'de> for SetMaxCallStackSizeToCaptureParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetMaxCallStackSizeToCaptureParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Runtime.command.setMaxCallStackSizeToCapture/parameters/size。
                    #[serde(rename = "size")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub size: i64,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SetMaxCallStackSizeToCaptureParams { size: fields.size })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Runtime.command.setMaxCallStackSizeToCapture。此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct SetMaxCallStackSizeToCapture;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Runtime.command.setMaxCallStackSizeToCapture/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetMaxCallStackSizeToCaptureResponse {}
impl<'de> serde::Deserialize<'de> for SetMaxCallStackSizeToCaptureResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetMaxCallStackSizeToCaptureResponse;
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
                Ok(SetMaxCallStackSizeToCaptureResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for SetMaxCallStackSizeToCapture {
    type Params = SetMaxCallStackSizeToCaptureParams;
    type Response = SetMaxCallStackSizeToCaptureResponse;
    const METHOD: &'static str = "Runtime.setMaxCallStackSizeToCapture";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Runtime.command.terminateExecution/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct TerminateExecutionParams {}
impl<'de> serde::Deserialize<'de> for TerminateExecutionParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = TerminateExecutionParams;
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
                Ok(TerminateExecutionParams {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Runtime.command.terminateExecution。
///
///协议原文：
///
///<pre>Terminate current or next JavaScript execution.
///Will cancel the termination when the outer-most script execution ends.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct TerminateExecution;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Runtime.command.terminateExecution/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct TerminateExecutionResponse {}
impl<'de> serde::Deserialize<'de> for TerminateExecutionResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = TerminateExecutionResponse;
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
                Ok(TerminateExecutionResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for TerminateExecution {
    type Params = TerminateExecutionParams;
    type Response = TerminateExecutionResponse;
    const METHOD: &'static str = "Runtime.terminateExecution";
}
