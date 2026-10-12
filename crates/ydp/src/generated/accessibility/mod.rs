///命令 marker、完整参数与成功响应。
pub mod commands;
///事件 marker 和拥有型事件参数。
pub mod events;
///命名协议类型；不包含连接状态。
pub mod types;
///薄 Domain 视图，仅借用 Session；创建不发送数据。协议来源：Accessibility。此项为 experimental，端点可能不支持。
pub struct Domain<'a> {
    ///借用的通信路由，生命周期不超过 Session。
    pub(super) session: &'a ydp_core::Session,
}
impl<'a> Domain<'a> {
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Accessibility.command.disable。
    ///
    ///协议原文：
    ///
    ///<pre>Disables the accessibility domain.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::accessibility::commands::DisableParams) -> ydp_core::Result<()> {
    ///let response = session.accessibility().disable(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn disable(&self, params: commands::DisableParams) -> ydp_core::Request<commands::Disable> {
        self.session.send::<commands::Disable>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Accessibility.command.enable。
    ///
    ///协议原文：
    ///
    ///<pre>Enables the accessibility domain which causes `AXNodeId`s to remain consistent between method calls.
    ///This turns on accessibility for the page, which can impact performance until accessibility is disabled.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::accessibility::commands::EnableParams) -> ydp_core::Result<()> {
    ///let response = session.accessibility().enable(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn enable(&self, params: commands::EnableParams) -> ydp_core::Request<commands::Enable> {
        self.session.send::<commands::Enable>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Accessibility.command.getAXNodeAndAncestors。
    ///
    ///协议原文：
    ///
    ///<pre>Fetches a node and all ancestors up to and including the root.
    ///Requires `enable()` to have been called previously.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::accessibility::commands::GetAxNodeAndAncestorsParams) -> ydp_core::Result<()> {
    ///let response = session.accessibility().get_ax_node_and_ancestors(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn get_ax_node_and_ancestors(
        &self,
        params: commands::GetAxNodeAndAncestorsParams,
    ) -> ydp_core::Request<commands::GetAxNodeAndAncestors> {
        self.session.send::<commands::GetAxNodeAndAncestors>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Accessibility.command.getChildAXNodes。
    ///
    ///协议原文：
    ///
    ///<pre>Fetches a particular accessibility node by AXNodeId.
    ///Requires `enable()` to have been called previously.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::accessibility::commands::GetChildAxNodesParams) -> ydp_core::Result<()> {
    ///let response = session.accessibility().get_child_ax_nodes(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn get_child_ax_nodes(
        &self,
        params: commands::GetChildAxNodesParams,
    ) -> ydp_core::Request<commands::GetChildAxNodes> {
        self.session.send::<commands::GetChildAxNodes>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Accessibility.command.getFullAXTree。
    ///
    ///协议原文：
    ///
    ///<pre>Fetches the entire accessibility tree for the root Document</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::accessibility::commands::GetFullAxTreeParams) -> ydp_core::Result<()> {
    ///let response = session.accessibility().get_full_ax_tree(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn get_full_ax_tree(
        &self,
        params: commands::GetFullAxTreeParams,
    ) -> ydp_core::Request<commands::GetFullAxTree> {
        self.session.send::<commands::GetFullAxTree>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Accessibility.command.getPartialAXTree。
    ///
    ///协议原文：
    ///
    ///<pre>Fetches the accessibility node and partial accessibility tree for this DOM node, if it exists.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::accessibility::commands::GetPartialAxTreeParams) -> ydp_core::Result<()> {
    ///let response = session.accessibility().get_partial_ax_tree(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn get_partial_ax_tree(
        &self,
        params: commands::GetPartialAxTreeParams,
    ) -> ydp_core::Request<commands::GetPartialAxTree> {
        self.session.send::<commands::GetPartialAxTree>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Accessibility.command.getRootAXNode。
    ///
    ///协议原文：
    ///
    ///<pre>Fetches the root node.
    ///Requires `enable()` to have been called previously.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::accessibility::commands::GetRootAxNodeParams) -> ydp_core::Result<()> {
    ///let response = session.accessibility().get_root_ax_node(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn get_root_ax_node(
        &self,
        params: commands::GetRootAxNodeParams,
    ) -> ydp_core::Request<commands::GetRootAxNode> {
        self.session.send::<commands::GetRootAxNode>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Accessibility.command.queryAXTree。
    ///
    ///协议原文：
    ///
    ///<pre>Query a DOM node's accessibility subtree for accessible name and role.
    ///This command computes the name and role for all nodes in the subtree, including those that are
    ///ignored for accessibility, and returns those that match the specified name and role. If no DOM
    ///node is specified, or the DOM node does not exist, the command returns an error. If neither
    ///`accessibleName` or `role` is specified, it returns all the accessibility nodes in the subtree.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::accessibility::commands::QueryAxTreeParams) -> ydp_core::Result<()> {
    ///let response = session.accessibility().query_ax_tree(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn query_ax_tree(
        &self,
        params: commands::QueryAxTreeParams,
    ) -> ydp_core::Request<commands::QueryAxTree> {
        self.session.send::<commands::QueryAxTree>(params)
    }
}
