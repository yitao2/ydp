///命令 marker、完整参数与成功响应。
pub mod commands;
///事件 marker 和拥有型事件参数。
pub mod events;
///命名协议类型；不包含连接状态。
pub mod types;
///薄 Domain 视图，仅借用 Session；创建不发送数据。协议来源：DOM。
///
///协议原文：
///
///<pre>This domain exposes DOM read/write operations. Each DOM Node is represented with its mirror object
///that has an `id`. This `id` can be used to get additional information on the Node, resolve it into
///the JavaScript object wrapper, etc. It is important that client receives DOM events only for the
///nodes that are known to the client. Backend keeps track of the nodes that were sent to the client
///and never sends the same node twice. It is client's responsibility to collect information about
///the nodes that were sent to the client. Note that `iframe` owner elements will return
///corresponding document elements as their child nodes.</pre>
///
///
pub struct Domain<'a> {
    ///借用的通信路由，生命周期不超过 Session。
    pub(super) session: &'a ydp_core::Session,
}
impl<'a> Domain<'a> {
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：DOM.command.collectClassNamesFromSubtree。
    ///
    ///协议原文：
    ///
    ///<pre>Collects class names for the node with given id and all of it's child nodes.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::dom::commands::CollectClassNamesFromSubtreeParams) -> ydp_core::Result<()> {
    ///let response = session.dom().collect_class_names_from_subtree(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn collect_class_names_from_subtree(
        &self,
        params: commands::CollectClassNamesFromSubtreeParams,
    ) -> ydp_core::Request<commands::CollectClassNamesFromSubtree> {
        self.session
            .send::<commands::CollectClassNamesFromSubtree>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：DOM.command.copyTo。
    ///
    ///协议原文：
    ///
    ///<pre>Creates a deep copy of the specified node and places it into the target container before the
    ///given anchor.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::dom::commands::CopyToParams) -> ydp_core::Result<()> {
    ///let response = session.dom().copy_to(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn copy_to(&self, params: commands::CopyToParams) -> ydp_core::Request<commands::CopyTo> {
        self.session.send::<commands::CopyTo>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：DOM.command.describeNode。
    ///
    ///协议原文：
    ///
    ///<pre>Describes node given its id, does not require domain to be enabled. Does not start tracking any
    ///objects, can be used for automation.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::dom::commands::DescribeNodeParams) -> ydp_core::Result<()> {
    ///let response = session.dom().describe_node(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn describe_node(
        &self,
        params: commands::DescribeNodeParams,
    ) -> ydp_core::Request<commands::DescribeNode> {
        self.session.send::<commands::DescribeNode>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：DOM.command.disable。
    ///
    ///协议原文：
    ///
    ///<pre>Disables DOM agent for the given page.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::dom::commands::DisableParams) -> ydp_core::Result<()> {
    ///let response = session.dom().disable(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn disable(&self, params: commands::DisableParams) -> ydp_core::Request<commands::Disable> {
        self.session.send::<commands::Disable>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：DOM.command.discardSearchResults。
    ///
    ///协议原文：
    ///
    ///<pre>Discards search results from the session with the given id. `getSearchResults` should no longer
    ///be called for that search.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::dom::commands::DiscardSearchResultsParams) -> ydp_core::Result<()> {
    ///let response = session.dom().discard_search_results(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn discard_search_results(
        &self,
        params: commands::DiscardSearchResultsParams,
    ) -> ydp_core::Request<commands::DiscardSearchResults> {
        self.session.send::<commands::DiscardSearchResults>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：DOM.command.enable。
    ///
    ///协议原文：
    ///
    ///<pre>Enables DOM agent for the given page.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::dom::commands::EnableParams) -> ydp_core::Result<()> {
    ///let response = session.dom().enable(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn enable(&self, params: commands::EnableParams) -> ydp_core::Request<commands::Enable> {
        self.session.send::<commands::Enable>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：DOM.command.focus。
    ///
    ///协议原文：
    ///
    ///<pre>Focuses the given element.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::dom::commands::FocusParams) -> ydp_core::Result<()> {
    ///let response = session.dom().focus(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn focus(&self, params: commands::FocusParams) -> ydp_core::Request<commands::Focus> {
        self.session.send::<commands::Focus>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：DOM.command.forceShowInterest。
    ///
    ///协议原文：
    ///
    ///<pre>When enabling, this API forces an element to gain interest in its target,
    ///keeping interest active until disabled.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::dom::commands::ForceShowInterestParams) -> ydp_core::Result<()> {
    ///let response = session.dom().force_show_interest(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn force_show_interest(
        &self,
        params: commands::ForceShowInterestParams,
    ) -> ydp_core::Request<commands::ForceShowInterest> {
        self.session.send::<commands::ForceShowInterest>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：DOM.command.forceShowPopover。
    ///
    ///协议原文：
    ///
    ///<pre>When enabling, this API force-opens the popover identified by nodeId
    ///and keeps it open until disabled.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::dom::commands::ForceShowPopoverParams) -> ydp_core::Result<()> {
    ///let response = session.dom().force_show_popover(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn force_show_popover(
        &self,
        params: commands::ForceShowPopoverParams,
    ) -> ydp_core::Request<commands::ForceShowPopover> {
        self.session.send::<commands::ForceShowPopover>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：DOM.command.getAnchorElement。
    ///
    ///协议原文：
    ///
    ///<pre>Returns the target anchor element of the given anchor query according to
    ///https://www.w3.org/TR/css-anchor-position-1/#target.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::dom::commands::GetAnchorElementParams) -> ydp_core::Result<()> {
    ///let response = session.dom().get_anchor_element(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn get_anchor_element(
        &self,
        params: commands::GetAnchorElementParams,
    ) -> ydp_core::Request<commands::GetAnchorElement> {
        self.session.send::<commands::GetAnchorElement>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：DOM.command.getAttributes。
    ///
    ///协议原文：
    ///
    ///<pre>Returns attributes for the specified node.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::dom::commands::GetAttributesParams) -> ydp_core::Result<()> {
    ///let response = session.dom().get_attributes(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn get_attributes(
        &self,
        params: commands::GetAttributesParams,
    ) -> ydp_core::Request<commands::GetAttributes> {
        self.session.send::<commands::GetAttributes>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：DOM.command.getBoxModel。
    ///
    ///协议原文：
    ///
    ///<pre>Returns boxes for the given node.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::dom::commands::GetBoxModelParams) -> ydp_core::Result<()> {
    ///let response = session.dom().get_box_model(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn get_box_model(
        &self,
        params: commands::GetBoxModelParams,
    ) -> ydp_core::Request<commands::GetBoxModel> {
        self.session.send::<commands::GetBoxModel>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：DOM.command.getContainerForNode。
    ///
    ///协议原文：
    ///
    ///<pre>Returns the query container of the given node based on container query
    ///conditions: containerName, physical and logical axes, and whether it queries
    ///scroll-state or anchored elements. If no axes are provided and
    ///queriesScrollState is false, the style container is returned, which is the
    ///direct parent or the closest element with a matching container-name.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::dom::commands::GetContainerForNodeParams) -> ydp_core::Result<()> {
    ///let response = session.dom().get_container_for_node(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn get_container_for_node(
        &self,
        params: commands::GetContainerForNodeParams,
    ) -> ydp_core::Request<commands::GetContainerForNode> {
        self.session.send::<commands::GetContainerForNode>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：DOM.command.getContentQuads。
    ///
    ///协议原文：
    ///
    ///<pre>Returns quads that describe node position on the page. This method
    ///might return multiple quads for inline nodes.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::dom::commands::GetContentQuadsParams) -> ydp_core::Result<()> {
    ///let response = session.dom().get_content_quads(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn get_content_quads(
        &self,
        params: commands::GetContentQuadsParams,
    ) -> ydp_core::Request<commands::GetContentQuads> {
        self.session.send::<commands::GetContentQuads>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：DOM.command.getDetachedDomNodes。
    ///
    ///协议原文：
    ///
    ///<pre>Returns list of detached nodes</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::dom::commands::GetDetachedDomNodesParams) -> ydp_core::Result<()> {
    ///let response = session.dom().get_detached_dom_nodes(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn get_detached_dom_nodes(
        &self,
        params: commands::GetDetachedDomNodesParams,
    ) -> ydp_core::Request<commands::GetDetachedDomNodes> {
        self.session.send::<commands::GetDetachedDomNodes>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：DOM.command.getDocument。
    ///
    ///协议原文：
    ///
    ///<pre>Returns the root DOM node (and optionally the subtree) to the caller.
    ///Implicitly enables the DOM domain events for the current target.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::dom::commands::GetDocumentParams) -> ydp_core::Result<()> {
    ///let response = session.dom().get_document(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn get_document(
        &self,
        params: commands::GetDocumentParams,
    ) -> ydp_core::Request<commands::GetDocument> {
        self.session.send::<commands::GetDocument>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：DOM.command.getElementByRelation。
    ///
    ///协议原文：
    ///
    ///<pre>Returns the NodeId of the matched element according to certain relations.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::dom::commands::GetElementByRelationParams) -> ydp_core::Result<()> {
    ///let response = session.dom().get_element_by_relation(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn get_element_by_relation(
        &self,
        params: commands::GetElementByRelationParams,
    ) -> ydp_core::Request<commands::GetElementByRelation> {
        self.session.send::<commands::GetElementByRelation>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：DOM.command.getFileInfo。
    ///
    ///协议原文：
    ///
    ///<pre>Returns file information for the given
    ///File wrapper.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::dom::commands::GetFileInfoParams) -> ydp_core::Result<()> {
    ///let response = session.dom().get_file_info(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn get_file_info(
        &self,
        params: commands::GetFileInfoParams,
    ) -> ydp_core::Request<commands::GetFileInfo> {
        self.session.send::<commands::GetFileInfo>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：DOM.command.getFlattenedDocument。
    ///
    ///协议原文：
    ///
    ///<pre>Returns the root DOM node (and optionally the subtree) to the caller.
    ///Deprecated, as it is not designed to work well with the rest of the DOM agent.
    ///Use DOMSnapshot.captureSnapshot instead.</pre>
    ///
    ///此项已 deprecated，保留用于旧端点兼容。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::dom::commands::GetFlattenedDocumentParams) -> ydp_core::Result<()> {
    ///let response = session.dom().get_flattened_document(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn get_flattened_document(
        &self,
        params: commands::GetFlattenedDocumentParams,
    ) -> ydp_core::Request<commands::GetFlattenedDocument> {
        self.session.send::<commands::GetFlattenedDocument>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：DOM.command.getFrameOwner。
    ///
    ///协议原文：
    ///
    ///<pre>Returns iframe node that owns iframe with the given domain.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::dom::commands::GetFrameOwnerParams) -> ydp_core::Result<()> {
    ///let response = session.dom().get_frame_owner(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn get_frame_owner(
        &self,
        params: commands::GetFrameOwnerParams,
    ) -> ydp_core::Request<commands::GetFrameOwner> {
        self.session.send::<commands::GetFrameOwner>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：DOM.command.getNodeForLocation。
    ///
    ///协议原文：
    ///
    ///<pre>Returns node id at given location. Depending on whether DOM domain is enabled, nodeId is
    ///either returned or not.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::dom::commands::GetNodeForLocationParams) -> ydp_core::Result<()> {
    ///let response = session.dom().get_node_for_location(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn get_node_for_location(
        &self,
        params: commands::GetNodeForLocationParams,
    ) -> ydp_core::Request<commands::GetNodeForLocation> {
        self.session.send::<commands::GetNodeForLocation>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：DOM.command.getNodeStackTraces。
    ///
    ///协议原文：
    ///
    ///<pre>Gets stack traces associated with a Node. As of now, only provides stack trace for Node creation.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::dom::commands::GetNodeStackTracesParams) -> ydp_core::Result<()> {
    ///let response = session.dom().get_node_stack_traces(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn get_node_stack_traces(
        &self,
        params: commands::GetNodeStackTracesParams,
    ) -> ydp_core::Request<commands::GetNodeStackTraces> {
        self.session.send::<commands::GetNodeStackTraces>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：DOM.command.getNodesForSubtreeByStyle。
    ///
    ///协议原文：
    ///
    ///<pre>Finds nodes with a given computed style in a subtree.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::dom::commands::GetNodesForSubtreeByStyleParams) -> ydp_core::Result<()> {
    ///let response = session.dom().get_nodes_for_subtree_by_style(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn get_nodes_for_subtree_by_style(
        &self,
        params: commands::GetNodesForSubtreeByStyleParams,
    ) -> ydp_core::Request<commands::GetNodesForSubtreeByStyle> {
        self.session
            .send::<commands::GetNodesForSubtreeByStyle>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：DOM.command.getOuterHTML。
    ///
    ///协议原文：
    ///
    ///<pre>Returns node's HTML markup.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::dom::commands::GetOuterHtmlParams) -> ydp_core::Result<()> {
    ///let response = session.dom().get_outer_html(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn get_outer_html(
        &self,
        params: commands::GetOuterHtmlParams,
    ) -> ydp_core::Request<commands::GetOuterHtml> {
        self.session.send::<commands::GetOuterHtml>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：DOM.command.getQueryingDescendantsForContainer。
    ///
    ///协议原文：
    ///
    ///<pre>Returns the descendants of a container query container that have
    ///container queries against this container.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::dom::commands::GetQueryingDescendantsForContainerParams) -> ydp_core::Result<()> {
    ///let response = session.dom().get_querying_descendants_for_container(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn get_querying_descendants_for_container(
        &self,
        params: commands::GetQueryingDescendantsForContainerParams,
    ) -> ydp_core::Request<commands::GetQueryingDescendantsForContainer> {
        self.session
            .send::<commands::GetQueryingDescendantsForContainer>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：DOM.command.getRelayoutBoundary。
    ///
    ///协议原文：
    ///
    ///<pre>Returns the id of the nearest ancestor that is a relayout boundary.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::dom::commands::GetRelayoutBoundaryParams) -> ydp_core::Result<()> {
    ///let response = session.dom().get_relayout_boundary(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn get_relayout_boundary(
        &self,
        params: commands::GetRelayoutBoundaryParams,
    ) -> ydp_core::Request<commands::GetRelayoutBoundary> {
        self.session.send::<commands::GetRelayoutBoundary>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：DOM.command.getSearchResults。
    ///
    ///协议原文：
    ///
    ///<pre>Returns search results from given `fromIndex` to given `toIndex` from the search with the given
    ///identifier.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::dom::commands::GetSearchResultsParams) -> ydp_core::Result<()> {
    ///let response = session.dom().get_search_results(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn get_search_results(
        &self,
        params: commands::GetSearchResultsParams,
    ) -> ydp_core::Request<commands::GetSearchResults> {
        self.session.send::<commands::GetSearchResults>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：DOM.command.getTopLayerElements。
    ///
    ///协议原文：
    ///
    ///<pre>Returns NodeIds of current top layer elements.
    ///Top layer is rendered closest to the user within a viewport, therefore its elements always
    ///appear on top of all other content.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::dom::commands::GetTopLayerElementsParams) -> ydp_core::Result<()> {
    ///let response = session.dom().get_top_layer_elements(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn get_top_layer_elements(
        &self,
        params: commands::GetTopLayerElementsParams,
    ) -> ydp_core::Request<commands::GetTopLayerElements> {
        self.session.send::<commands::GetTopLayerElements>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：DOM.command.hideHighlight。
    ///
    ///协议原文：
    ///
    ///<pre>Hides any highlight.</pre>
    ///
    ///此命令由协议重定向到 Overlay 域处理。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::dom::commands::HideHighlightParams) -> ydp_core::Result<()> {
    ///let response = session.dom().hide_highlight(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn hide_highlight(
        &self,
        params: commands::HideHighlightParams,
    ) -> ydp_core::Request<commands::HideHighlight> {
        self.session.send::<commands::HideHighlight>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：DOM.command.highlightNode。
    ///
    ///协议原文：
    ///
    ///<pre>Highlights DOM node.</pre>
    ///
    ///此命令由协议重定向到 Overlay 域处理。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::dom::commands::HighlightNodeParams) -> ydp_core::Result<()> {
    ///let response = session.dom().highlight_node(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn highlight_node(
        &self,
        params: commands::HighlightNodeParams,
    ) -> ydp_core::Request<commands::HighlightNode> {
        self.session.send::<commands::HighlightNode>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：DOM.command.highlightRect。
    ///
    ///协议原文：
    ///
    ///<pre>Highlights given rectangle.</pre>
    ///
    ///此命令由协议重定向到 Overlay 域处理。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::dom::commands::HighlightRectParams) -> ydp_core::Result<()> {
    ///let response = session.dom().highlight_rect(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn highlight_rect(
        &self,
        params: commands::HighlightRectParams,
    ) -> ydp_core::Request<commands::HighlightRect> {
        self.session.send::<commands::HighlightRect>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：DOM.command.markUndoableState。
    ///
    ///协议原文：
    ///
    ///<pre>Marks last undoable state.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::dom::commands::MarkUndoableStateParams) -> ydp_core::Result<()> {
    ///let response = session.dom().mark_undoable_state(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn mark_undoable_state(
        &self,
        params: commands::MarkUndoableStateParams,
    ) -> ydp_core::Request<commands::MarkUndoableState> {
        self.session.send::<commands::MarkUndoableState>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：DOM.command.moveTo。
    ///
    ///协议原文：
    ///
    ///<pre>Moves node into the new container, places it before the given anchor.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::dom::commands::MoveToParams) -> ydp_core::Result<()> {
    ///let response = session.dom().move_to(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn move_to(&self, params: commands::MoveToParams) -> ydp_core::Request<commands::MoveTo> {
        self.session.send::<commands::MoveTo>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：DOM.command.performSearch。
    ///
    ///协议原文：
    ///
    ///<pre>Searches for a given string in the DOM tree. Use `getSearchResults` to access search results or
    ///`cancelSearch` to end this search session.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::dom::commands::PerformSearchParams) -> ydp_core::Result<()> {
    ///let response = session.dom().perform_search(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn perform_search(
        &self,
        params: commands::PerformSearchParams,
    ) -> ydp_core::Request<commands::PerformSearch> {
        self.session.send::<commands::PerformSearch>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：DOM.command.pushNodeByPathToFrontend。
    ///
    ///协议原文：
    ///
    ///<pre>Requests that the node is sent to the caller given its path. // FIXME, use XPath</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::dom::commands::PushNodeByPathToFrontendParams) -> ydp_core::Result<()> {
    ///let response = session.dom().push_node_by_path_to_frontend(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn push_node_by_path_to_frontend(
        &self,
        params: commands::PushNodeByPathToFrontendParams,
    ) -> ydp_core::Request<commands::PushNodeByPathToFrontend> {
        self.session
            .send::<commands::PushNodeByPathToFrontend>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：DOM.command.pushNodesByBackendIdsToFrontend。
    ///
    ///协议原文：
    ///
    ///<pre>Requests that a batch of nodes is sent to the caller given their backend node ids.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::dom::commands::PushNodesByBackendIdsToFrontendParams) -> ydp_core::Result<()> {
    ///let response = session.dom().push_nodes_by_backend_ids_to_frontend(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn push_nodes_by_backend_ids_to_frontend(
        &self,
        params: commands::PushNodesByBackendIdsToFrontendParams,
    ) -> ydp_core::Request<commands::PushNodesByBackendIdsToFrontend> {
        self.session
            .send::<commands::PushNodesByBackendIdsToFrontend>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：DOM.command.querySelector。
    ///
    ///协议原文：
    ///
    ///<pre>Executes `querySelector` on a given node.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::dom::commands::QuerySelectorParams) -> ydp_core::Result<()> {
    ///let response = session.dom().query_selector(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn query_selector(
        &self,
        params: commands::QuerySelectorParams,
    ) -> ydp_core::Request<commands::QuerySelector> {
        self.session.send::<commands::QuerySelector>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：DOM.command.querySelectorAll。
    ///
    ///协议原文：
    ///
    ///<pre>Executes `querySelectorAll` on a given node.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::dom::commands::QuerySelectorAllParams) -> ydp_core::Result<()> {
    ///let response = session.dom().query_selector_all(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn query_selector_all(
        &self,
        params: commands::QuerySelectorAllParams,
    ) -> ydp_core::Request<commands::QuerySelectorAll> {
        self.session.send::<commands::QuerySelectorAll>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：DOM.command.redo。
    ///
    ///协议原文：
    ///
    ///<pre>Re-does the last undone action.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::dom::commands::RedoParams) -> ydp_core::Result<()> {
    ///let response = session.dom().redo(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn redo(&self, params: commands::RedoParams) -> ydp_core::Request<commands::Redo> {
        self.session.send::<commands::Redo>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：DOM.command.removeAttribute。
    ///
    ///协议原文：
    ///
    ///<pre>Removes attribute with given name from an element with given id.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::dom::commands::RemoveAttributeParams) -> ydp_core::Result<()> {
    ///let response = session.dom().remove_attribute(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn remove_attribute(
        &self,
        params: commands::RemoveAttributeParams,
    ) -> ydp_core::Request<commands::RemoveAttribute> {
        self.session.send::<commands::RemoveAttribute>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：DOM.command.removeNode。
    ///
    ///协议原文：
    ///
    ///<pre>Removes node with given id.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::dom::commands::RemoveNodeParams) -> ydp_core::Result<()> {
    ///let response = session.dom().remove_node(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn remove_node(
        &self,
        params: commands::RemoveNodeParams,
    ) -> ydp_core::Request<commands::RemoveNode> {
        self.session.send::<commands::RemoveNode>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：DOM.command.requestChildNodes。
    ///
    ///协议原文：
    ///
    ///<pre>Requests that children of the node with given id are returned to the caller in form of
    ///`setChildNodes` events where not only immediate children are retrieved, but all children down to
    ///the specified depth.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::dom::commands::RequestChildNodesParams) -> ydp_core::Result<()> {
    ///let response = session.dom().request_child_nodes(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn request_child_nodes(
        &self,
        params: commands::RequestChildNodesParams,
    ) -> ydp_core::Request<commands::RequestChildNodes> {
        self.session.send::<commands::RequestChildNodes>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：DOM.command.requestNode。
    ///
    ///协议原文：
    ///
    ///<pre>Requests that the node is sent to the caller given the JavaScript node object reference. All
    ///nodes that form the path from the node to the root are also sent to the client as a series of
    ///`setChildNodes` notifications.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::dom::commands::RequestNodeParams) -> ydp_core::Result<()> {
    ///let response = session.dom().request_node(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn request_node(
        &self,
        params: commands::RequestNodeParams,
    ) -> ydp_core::Request<commands::RequestNode> {
        self.session.send::<commands::RequestNode>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：DOM.command.resolveNode。
    ///
    ///协议原文：
    ///
    ///<pre>Resolves the JavaScript node object for a given NodeId or BackendNodeId.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::dom::commands::ResolveNodeParams) -> ydp_core::Result<()> {
    ///let response = session.dom().resolve_node(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn resolve_node(
        &self,
        params: commands::ResolveNodeParams,
    ) -> ydp_core::Request<commands::ResolveNode> {
        self.session.send::<commands::ResolveNode>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：DOM.command.scrollIntoViewIfNeeded。
    ///
    ///协议原文：
    ///
    ///<pre>Scrolls the specified rect of the given node into view if not already visible.
    ///Note: exactly one between nodeId, backendNodeId and objectId should be passed
    ///to identify the node.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::dom::commands::ScrollIntoViewIfNeededParams) -> ydp_core::Result<()> {
    ///let response = session.dom().scroll_into_view_if_needed(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn scroll_into_view_if_needed(
        &self,
        params: commands::ScrollIntoViewIfNeededParams,
    ) -> ydp_core::Request<commands::ScrollIntoViewIfNeeded> {
        self.session
            .send::<commands::ScrollIntoViewIfNeeded>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：DOM.command.setAttributeValue。
    ///
    ///协议原文：
    ///
    ///<pre>Sets attribute for an element with given id.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::dom::commands::SetAttributeValueParams) -> ydp_core::Result<()> {
    ///let response = session.dom().set_attribute_value(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn set_attribute_value(
        &self,
        params: commands::SetAttributeValueParams,
    ) -> ydp_core::Request<commands::SetAttributeValue> {
        self.session.send::<commands::SetAttributeValue>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：DOM.command.setAttributesAsText。
    ///
    ///协议原文：
    ///
    ///<pre>Sets attributes on element with given id. This method is useful when user edits some existing
    ///attribute value and types in several attribute name/value pairs.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::dom::commands::SetAttributesAsTextParams) -> ydp_core::Result<()> {
    ///let response = session.dom().set_attributes_as_text(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn set_attributes_as_text(
        &self,
        params: commands::SetAttributesAsTextParams,
    ) -> ydp_core::Request<commands::SetAttributesAsText> {
        self.session.send::<commands::SetAttributesAsText>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：DOM.command.setFileInputFiles。
    ///
    ///协议原文：
    ///
    ///<pre>Sets files for the given file input element.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::dom::commands::SetFileInputFilesParams) -> ydp_core::Result<()> {
    ///let response = session.dom().set_file_input_files(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn set_file_input_files(
        &self,
        params: commands::SetFileInputFilesParams,
    ) -> ydp_core::Request<commands::SetFileInputFiles> {
        self.session.send::<commands::SetFileInputFiles>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：DOM.command.setInspectedNode。
    ///
    ///协议原文：
    ///
    ///<pre>Enables console to refer to the node with given id via $x (see Command Line API for more details
    ///$x functions).</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::dom::commands::SetInspectedNodeParams) -> ydp_core::Result<()> {
    ///let response = session.dom().set_inspected_node(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn set_inspected_node(
        &self,
        params: commands::SetInspectedNodeParams,
    ) -> ydp_core::Request<commands::SetInspectedNode> {
        self.session.send::<commands::SetInspectedNode>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：DOM.command.setNodeName。
    ///
    ///协议原文：
    ///
    ///<pre>Sets node name for a node with given id.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::dom::commands::SetNodeNameParams) -> ydp_core::Result<()> {
    ///let response = session.dom().set_node_name(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn set_node_name(
        &self,
        params: commands::SetNodeNameParams,
    ) -> ydp_core::Request<commands::SetNodeName> {
        self.session.send::<commands::SetNodeName>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：DOM.command.setNodeStackTracesEnabled。
    ///
    ///协议原文：
    ///
    ///<pre>Sets if stack traces should be captured for Nodes. See `Node.getNodeStackTraces`. Default is disabled.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::dom::commands::SetNodeStackTracesEnabledParams) -> ydp_core::Result<()> {
    ///let response = session.dom().set_node_stack_traces_enabled(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn set_node_stack_traces_enabled(
        &self,
        params: commands::SetNodeStackTracesEnabledParams,
    ) -> ydp_core::Request<commands::SetNodeStackTracesEnabled> {
        self.session
            .send::<commands::SetNodeStackTracesEnabled>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：DOM.command.setNodeValue。
    ///
    ///协议原文：
    ///
    ///<pre>Sets node value for a node with given id.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::dom::commands::SetNodeValueParams) -> ydp_core::Result<()> {
    ///let response = session.dom().set_node_value(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn set_node_value(
        &self,
        params: commands::SetNodeValueParams,
    ) -> ydp_core::Request<commands::SetNodeValue> {
        self.session.send::<commands::SetNodeValue>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：DOM.command.setOuterHTML。
    ///
    ///协议原文：
    ///
    ///<pre>Sets node HTML markup, returns new node id.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::dom::commands::SetOuterHtmlParams) -> ydp_core::Result<()> {
    ///let response = session.dom().set_outer_html(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn set_outer_html(
        &self,
        params: commands::SetOuterHtmlParams,
    ) -> ydp_core::Request<commands::SetOuterHtml> {
        self.session.send::<commands::SetOuterHtml>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：DOM.command.undo。
    ///
    ///协议原文：
    ///
    ///<pre>Undoes the last performed action.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::dom::commands::UndoParams) -> ydp_core::Result<()> {
    ///let response = session.dom().undo(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn undo(&self, params: commands::UndoParams) -> ydp_core::Request<commands::Undo> {
        self.session.send::<commands::Undo>(params)
    }
}
