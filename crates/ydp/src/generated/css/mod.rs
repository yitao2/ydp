///命令 marker、完整参数与成功响应。
pub mod commands;
///事件 marker 和拥有型事件参数。
pub mod events;
///命名协议类型；不包含连接状态。
pub mod types;
///薄 Domain 视图，仅借用 Session；创建不发送数据。协议来源：CSS。
///
///协议原文：
///
///<pre>This domain exposes CSS read/write operations. All CSS objects (stylesheets, rules, and styles)
///have an associated `id` used in subsequent operations on the related object. Each object type has
///a specific `id` structure, and those are not interchangeable between objects of different kinds.
///CSS objects can be loaded using the `get*ForNode()` calls (which accept a DOM node id). A client
///can also keep track of stylesheets via the `styleSheetAdded`/`styleSheetRemoved` events and
///subsequently load the required stylesheet contents using the `getStyleSheet[Text]()` methods.</pre>
///
///此项为 experimental，端点可能不支持。
pub struct Domain<'a> {
    ///借用的通信路由，生命周期不超过 Session。
    pub(super) session: &'a ydp_core::Session,
}
impl<'a> Domain<'a> {
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：CSS.command.addRule。
    ///
    ///协议原文：
    ///
    ///<pre>Inserts a new rule with the given `ruleText` in a stylesheet with given `styleSheetId`, at the
    ///position specified by `location`.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::css::commands::AddRuleParams) -> ydp_core::Result<()> {
    ///let response = session.css().add_rule(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn add_rule(
        &self,
        params: commands::AddRuleParams,
    ) -> ydp_core::Request<commands::AddRule> {
        self.session.send::<commands::AddRule>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：CSS.command.collectClassNames。
    ///
    ///协议原文：
    ///
    ///<pre>Returns all class names from specified stylesheet.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::css::commands::CollectClassNamesParams) -> ydp_core::Result<()> {
    ///let response = session.css().collect_class_names(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn collect_class_names(
        &self,
        params: commands::CollectClassNamesParams,
    ) -> ydp_core::Request<commands::CollectClassNames> {
        self.session.send::<commands::CollectClassNames>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：CSS.command.createStyleSheet。
    ///
    ///协议原文：
    ///
    ///<pre>Creates a new special "via-inspector" stylesheet in the frame with given `frameId`.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::css::commands::CreateStyleSheetParams) -> ydp_core::Result<()> {
    ///let response = session.css().create_style_sheet(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn create_style_sheet(
        &self,
        params: commands::CreateStyleSheetParams,
    ) -> ydp_core::Request<commands::CreateStyleSheet> {
        self.session.send::<commands::CreateStyleSheet>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：CSS.command.disable。
    ///
    ///协议原文：
    ///
    ///<pre>Disables the CSS agent for the given page.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::css::commands::DisableParams) -> ydp_core::Result<()> {
    ///let response = session.css().disable(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn disable(&self, params: commands::DisableParams) -> ydp_core::Request<commands::Disable> {
        self.session.send::<commands::Disable>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：CSS.command.enable。
    ///
    ///协议原文：
    ///
    ///<pre>Enables the CSS agent for the given page. Clients should not assume that the CSS agent has been
    ///enabled until the result of this command is received.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::css::commands::EnableParams) -> ydp_core::Result<()> {
    ///let response = session.css().enable(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn enable(&self, params: commands::EnableParams) -> ydp_core::Request<commands::Enable> {
        self.session.send::<commands::Enable>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：CSS.command.forcePseudoState。
    ///
    ///协议原文：
    ///
    ///<pre>Ensures that the given node will have specified pseudo-classes whenever its style is computed by
    ///the browser.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::css::commands::ForcePseudoStateParams) -> ydp_core::Result<()> {
    ///let response = session.css().force_pseudo_state(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn force_pseudo_state(
        &self,
        params: commands::ForcePseudoStateParams,
    ) -> ydp_core::Request<commands::ForcePseudoState> {
        self.session.send::<commands::ForcePseudoState>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：CSS.command.forceStartingStyle。
    ///
    ///协议原文：
    ///
    ///<pre>Ensures that the given node is in its starting-style state.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::css::commands::ForceStartingStyleParams) -> ydp_core::Result<()> {
    ///let response = session.css().force_starting_style(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn force_starting_style(
        &self,
        params: commands::ForceStartingStyleParams,
    ) -> ydp_core::Request<commands::ForceStartingStyle> {
        self.session.send::<commands::ForceStartingStyle>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：CSS.command.getAnimatedStylesForNode。
    ///
    ///协议原文：
    ///
    ///<pre>Returns the styles coming from animations &amp; transitions
    ///including the animation &amp; transition styles coming from inheritance chain.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::css::commands::GetAnimatedStylesForNodeParams) -> ydp_core::Result<()> {
    ///let response = session.css().get_animated_styles_for_node(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn get_animated_styles_for_node(
        &self,
        params: commands::GetAnimatedStylesForNodeParams,
    ) -> ydp_core::Request<commands::GetAnimatedStylesForNode> {
        self.session
            .send::<commands::GetAnimatedStylesForNode>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：CSS.command.getBackgroundColors。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::css::commands::GetBackgroundColorsParams) -> ydp_core::Result<()> {
    ///let response = session.css().get_background_colors(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn get_background_colors(
        &self,
        params: commands::GetBackgroundColorsParams,
    ) -> ydp_core::Request<commands::GetBackgroundColors> {
        self.session.send::<commands::GetBackgroundColors>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：CSS.command.getComputedStyleForNode。
    ///
    ///协议原文：
    ///
    ///<pre>Returns the computed style for a DOM node identified by `nodeId`.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::css::commands::GetComputedStyleForNodeParams) -> ydp_core::Result<()> {
    ///let response = session.css().get_computed_style_for_node(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn get_computed_style_for_node(
        &self,
        params: commands::GetComputedStyleForNodeParams,
    ) -> ydp_core::Request<commands::GetComputedStyleForNode> {
        self.session
            .send::<commands::GetComputedStyleForNode>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：CSS.command.getEnvironmentVariables。
    ///
    ///协议原文：
    ///
    ///<pre>Returns the values of the default UA-defined environment variables used in env()</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::css::commands::GetEnvironmentVariablesParams) -> ydp_core::Result<()> {
    ///let response = session.css().get_environment_variables(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn get_environment_variables(
        &self,
        params: commands::GetEnvironmentVariablesParams,
    ) -> ydp_core::Request<commands::GetEnvironmentVariables> {
        self.session
            .send::<commands::GetEnvironmentVariables>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：CSS.command.getInlineStylesForNode。
    ///
    ///协议原文：
    ///
    ///<pre>Returns the styles defined inline (explicitly in the "style" attribute and implicitly, using DOM
    ///attributes) for a DOM node identified by `nodeId`.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::css::commands::GetInlineStylesForNodeParams) -> ydp_core::Result<()> {
    ///let response = session.css().get_inline_styles_for_node(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn get_inline_styles_for_node(
        &self,
        params: commands::GetInlineStylesForNodeParams,
    ) -> ydp_core::Request<commands::GetInlineStylesForNode> {
        self.session
            .send::<commands::GetInlineStylesForNode>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：CSS.command.getLayersForNode。
    ///
    ///协议原文：
    ///
    ///<pre>Returns all layers parsed by the rendering engine for the tree scope of a node.
    ///Given a DOM element identified by nodeId, getLayersForNode returns the root
    ///layer for the nearest ancestor document or shadow root. The layer root contains
    ///the full layer tree for the tree scope and their ordering.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::css::commands::GetLayersForNodeParams) -> ydp_core::Result<()> {
    ///let response = session.css().get_layers_for_node(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn get_layers_for_node(
        &self,
        params: commands::GetLayersForNodeParams,
    ) -> ydp_core::Request<commands::GetLayersForNode> {
        self.session.send::<commands::GetLayersForNode>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：CSS.command.getLocationForSelector。
    ///
    ///协议原文：
    ///
    ///<pre>Given a CSS selector text and a style sheet ID, getLocationForSelector
    ///returns an array of locations of the CSS selector in the style sheet.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::css::commands::GetLocationForSelectorParams) -> ydp_core::Result<()> {
    ///let response = session.css().get_location_for_selector(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn get_location_for_selector(
        &self,
        params: commands::GetLocationForSelectorParams,
    ) -> ydp_core::Request<commands::GetLocationForSelector> {
        self.session
            .send::<commands::GetLocationForSelector>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：CSS.command.getLonghandProperties。此项为 experimental，端点可能不支持。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::css::commands::GetLonghandPropertiesParams) -> ydp_core::Result<()> {
    ///let response = session.css().get_longhand_properties(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn get_longhand_properties(
        &self,
        params: commands::GetLonghandPropertiesParams,
    ) -> ydp_core::Request<commands::GetLonghandProperties> {
        self.session.send::<commands::GetLonghandProperties>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：CSS.command.getMatchedStylesForNode。
    ///
    ///协议原文：
    ///
    ///<pre>Returns requested styles for a DOM node identified by `nodeId`.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::css::commands::GetMatchedStylesForNodeParams) -> ydp_core::Result<()> {
    ///let response = session.css().get_matched_styles_for_node(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn get_matched_styles_for_node(
        &self,
        params: commands::GetMatchedStylesForNodeParams,
    ) -> ydp_core::Request<commands::GetMatchedStylesForNode> {
        self.session
            .send::<commands::GetMatchedStylesForNode>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：CSS.command.getMediaQueries。
    ///
    ///协议原文：
    ///
    ///<pre>Returns all media queries parsed by the rendering engine.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::css::commands::GetMediaQueriesParams) -> ydp_core::Result<()> {
    ///let response = session.css().get_media_queries(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn get_media_queries(
        &self,
        params: commands::GetMediaQueriesParams,
    ) -> ydp_core::Request<commands::GetMediaQueries> {
        self.session.send::<commands::GetMediaQueries>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：CSS.command.getPlatformFontsForNode。
    ///
    ///协议原文：
    ///
    ///<pre>Requests information about platform fonts which we used to render child TextNodes in the given
    ///node.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::css::commands::GetPlatformFontsForNodeParams) -> ydp_core::Result<()> {
    ///let response = session.css().get_platform_fonts_for_node(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn get_platform_fonts_for_node(
        &self,
        params: commands::GetPlatformFontsForNodeParams,
    ) -> ydp_core::Request<commands::GetPlatformFontsForNode> {
        self.session
            .send::<commands::GetPlatformFontsForNode>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：CSS.command.getStyleSheetText。
    ///
    ///协议原文：
    ///
    ///<pre>Returns the current textual content for a stylesheet.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::css::commands::GetStyleSheetTextParams) -> ydp_core::Result<()> {
    ///let response = session.css().get_style_sheet_text(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn get_style_sheet_text(
        &self,
        params: commands::GetStyleSheetTextParams,
    ) -> ydp_core::Request<commands::GetStyleSheetText> {
        self.session.send::<commands::GetStyleSheetText>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：CSS.command.resolveValues。
    ///
    ///协议原文：
    ///
    ///<pre>Resolve the specified values in the context of the provided element.
    ///For example, a value of '1em' is evaluated according to the computed
    ///'font-size' of the element and a value 'calc(1px + 2px)' will be
    ///resolved to '3px'.
    ///If the `propertyName` was specified the `values` are resolved as if
    ///they were property's declaration. If a value cannot be parsed according
    ///to the provided property syntax, the value is parsed using combined
    ///syntax as if null `propertyName` was provided. If the value cannot be
    ///resolved even then, return the provided value without any changes.
    ///Note: this function currently does not resolve CSS random() function,
    ///it returns unmodified random() function parts.`</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::css::commands::ResolveValuesParams) -> ydp_core::Result<()> {
    ///let response = session.css().resolve_values(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn resolve_values(
        &self,
        params: commands::ResolveValuesParams,
    ) -> ydp_core::Request<commands::ResolveValues> {
        self.session.send::<commands::ResolveValues>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：CSS.command.setContainerQueryConditionText。此项为 experimental，端点可能不支持。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::css::commands::SetContainerQueryConditionTextParams) -> ydp_core::Result<()> {
    ///let response = session.css().set_container_query_condition_text(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn set_container_query_condition_text(
        &self,
        params: commands::SetContainerQueryConditionTextParams,
    ) -> ydp_core::Request<commands::SetContainerQueryConditionText> {
        self.session
            .send::<commands::SetContainerQueryConditionText>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：CSS.command.setContainerQueryText。
    ///
    ///协议原文：
    ///
    ///<pre>Modifies the expression of a container query.
    ///Deprecated. Use setContainerQueryConditionText instead.</pre>
    ///
    ///此项为 experimental，端点可能不支持。此项已 deprecated，保留用于旧端点兼容。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::css::commands::SetContainerQueryTextParams) -> ydp_core::Result<()> {
    ///let response = session.css().set_container_query_text(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn set_container_query_text(
        &self,
        params: commands::SetContainerQueryTextParams,
    ) -> ydp_core::Request<commands::SetContainerQueryText> {
        self.session.send::<commands::SetContainerQueryText>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：CSS.command.setEffectivePropertyValueForNode。
    ///
    ///协议原文：
    ///
    ///<pre>Find a rule with the given active property for the given node and set the new value for this
    ///property</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::css::commands::SetEffectivePropertyValueForNodeParams) -> ydp_core::Result<()> {
    ///let response = session.css().set_effective_property_value_for_node(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn set_effective_property_value_for_node(
        &self,
        params: commands::SetEffectivePropertyValueForNodeParams,
    ) -> ydp_core::Request<commands::SetEffectivePropertyValueForNode> {
        self.session
            .send::<commands::SetEffectivePropertyValueForNode>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：CSS.command.setKeyframeKey。
    ///
    ///协议原文：
    ///
    ///<pre>Modifies the keyframe rule key text.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::css::commands::SetKeyframeKeyParams) -> ydp_core::Result<()> {
    ///let response = session.css().set_keyframe_key(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn set_keyframe_key(
        &self,
        params: commands::SetKeyframeKeyParams,
    ) -> ydp_core::Request<commands::SetKeyframeKey> {
        self.session.send::<commands::SetKeyframeKey>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：CSS.command.setLocalFontsEnabled。
    ///
    ///协议原文：
    ///
    ///<pre>Enables/disables rendering of local CSS fonts (enabled by default).</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::css::commands::SetLocalFontsEnabledParams) -> ydp_core::Result<()> {
    ///let response = session.css().set_local_fonts_enabled(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn set_local_fonts_enabled(
        &self,
        params: commands::SetLocalFontsEnabledParams,
    ) -> ydp_core::Request<commands::SetLocalFontsEnabled> {
        self.session.send::<commands::SetLocalFontsEnabled>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：CSS.command.setMediaText。
    ///
    ///协议原文：
    ///
    ///<pre>Modifies the rule selector.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::css::commands::SetMediaTextParams) -> ydp_core::Result<()> {
    ///let response = session.css().set_media_text(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn set_media_text(
        &self,
        params: commands::SetMediaTextParams,
    ) -> ydp_core::Request<commands::SetMediaText> {
        self.session.send::<commands::SetMediaText>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：CSS.command.setNavigationText。
    ///
    ///协议原文：
    ///
    ///<pre>Modifies the expression of a navigation at-rule.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::css::commands::SetNavigationTextParams) -> ydp_core::Result<()> {
    ///let response = session.css().set_navigation_text(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn set_navigation_text(
        &self,
        params: commands::SetNavigationTextParams,
    ) -> ydp_core::Request<commands::SetNavigationText> {
        self.session.send::<commands::SetNavigationText>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：CSS.command.setPropertyRulePropertyName。
    ///
    ///协议原文：
    ///
    ///<pre>Modifies the property rule property name.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::css::commands::SetPropertyRulePropertyNameParams) -> ydp_core::Result<()> {
    ///let response = session.css().set_property_rule_property_name(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn set_property_rule_property_name(
        &self,
        params: commands::SetPropertyRulePropertyNameParams,
    ) -> ydp_core::Request<commands::SetPropertyRulePropertyName> {
        self.session
            .send::<commands::SetPropertyRulePropertyName>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：CSS.command.setRuleSelector。
    ///
    ///协议原文：
    ///
    ///<pre>Modifies the rule selector.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::css::commands::SetRuleSelectorParams) -> ydp_core::Result<()> {
    ///let response = session.css().set_rule_selector(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn set_rule_selector(
        &self,
        params: commands::SetRuleSelectorParams,
    ) -> ydp_core::Request<commands::SetRuleSelector> {
        self.session.send::<commands::SetRuleSelector>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：CSS.command.setScopeText。
    ///
    ///协议原文：
    ///
    ///<pre>Modifies the expression of a scope at-rule.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::css::commands::SetScopeTextParams) -> ydp_core::Result<()> {
    ///let response = session.css().set_scope_text(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn set_scope_text(
        &self,
        params: commands::SetScopeTextParams,
    ) -> ydp_core::Request<commands::SetScopeText> {
        self.session.send::<commands::SetScopeText>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：CSS.command.setStyleSheetText。
    ///
    ///协议原文：
    ///
    ///<pre>Sets the new stylesheet text.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::css::commands::SetStyleSheetTextParams) -> ydp_core::Result<()> {
    ///let response = session.css().set_style_sheet_text(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn set_style_sheet_text(
        &self,
        params: commands::SetStyleSheetTextParams,
    ) -> ydp_core::Request<commands::SetStyleSheetText> {
        self.session.send::<commands::SetStyleSheetText>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：CSS.command.setStyleTexts。
    ///
    ///协议原文：
    ///
    ///<pre>Applies specified style edits one after another in the given order.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::css::commands::SetStyleTextsParams) -> ydp_core::Result<()> {
    ///let response = session.css().set_style_texts(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn set_style_texts(
        &self,
        params: commands::SetStyleTextsParams,
    ) -> ydp_core::Request<commands::SetStyleTexts> {
        self.session.send::<commands::SetStyleTexts>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：CSS.command.setSupportsText。
    ///
    ///协议原文：
    ///
    ///<pre>Modifies the expression of a supports at-rule.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::css::commands::SetSupportsTextParams) -> ydp_core::Result<()> {
    ///let response = session.css().set_supports_text(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn set_supports_text(
        &self,
        params: commands::SetSupportsTextParams,
    ) -> ydp_core::Request<commands::SetSupportsText> {
        self.session.send::<commands::SetSupportsText>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：CSS.command.startRuleUsageTracking。
    ///
    ///协议原文：
    ///
    ///<pre>Enables the selector recording.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::css::commands::StartRuleUsageTrackingParams) -> ydp_core::Result<()> {
    ///let response = session.css().start_rule_usage_tracking(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn start_rule_usage_tracking(
        &self,
        params: commands::StartRuleUsageTrackingParams,
    ) -> ydp_core::Request<commands::StartRuleUsageTracking> {
        self.session
            .send::<commands::StartRuleUsageTracking>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：CSS.command.stopRuleUsageTracking。
    ///
    ///协议原文：
    ///
    ///<pre>Stop tracking rule usage and return the list of rules that were used since last call to
    ///`takeCoverageDelta` (or since start of coverage instrumentation).</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::css::commands::StopRuleUsageTrackingParams) -> ydp_core::Result<()> {
    ///let response = session.css().stop_rule_usage_tracking(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn stop_rule_usage_tracking(
        &self,
        params: commands::StopRuleUsageTrackingParams,
    ) -> ydp_core::Request<commands::StopRuleUsageTracking> {
        self.session.send::<commands::StopRuleUsageTracking>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：CSS.command.takeComputedStyleUpdates。
    ///
    ///协议原文：
    ///
    ///<pre>Polls the next batch of computed style updates.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::css::commands::TakeComputedStyleUpdatesParams) -> ydp_core::Result<()> {
    ///let response = session.css().take_computed_style_updates(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn take_computed_style_updates(
        &self,
        params: commands::TakeComputedStyleUpdatesParams,
    ) -> ydp_core::Request<commands::TakeComputedStyleUpdates> {
        self.session
            .send::<commands::TakeComputedStyleUpdates>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：CSS.command.takeCoverageDelta。
    ///
    ///协议原文：
    ///
    ///<pre>Obtain list of rules that became used since last call to this method (or since start of coverage
    ///instrumentation).</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::css::commands::TakeCoverageDeltaParams) -> ydp_core::Result<()> {
    ///let response = session.css().take_coverage_delta(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn take_coverage_delta(
        &self,
        params: commands::TakeCoverageDeltaParams,
    ) -> ydp_core::Request<commands::TakeCoverageDelta> {
        self.session.send::<commands::TakeCoverageDelta>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：CSS.command.trackComputedStyleUpdates。
    ///
    ///协议原文：
    ///
    ///<pre>Starts tracking the given computed styles for updates. The specified array of properties
    ///replaces the one previously specified. Pass empty array to disable tracking.
    ///Use takeComputedStyleUpdates to retrieve the list of nodes that had properties modified.
    ///The changes to computed style properties are only tracked for nodes pushed to the front-end
    ///by the DOM agent. If no changes to the tracked properties occur after the node has been pushed
    ///to the front-end, no updates will be issued for the node.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::css::commands::TrackComputedStyleUpdatesParams) -> ydp_core::Result<()> {
    ///let response = session.css().track_computed_style_updates(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn track_computed_style_updates(
        &self,
        params: commands::TrackComputedStyleUpdatesParams,
    ) -> ydp_core::Request<commands::TrackComputedStyleUpdates> {
        self.session
            .send::<commands::TrackComputedStyleUpdates>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：CSS.command.trackComputedStyleUpdatesForNode。
    ///
    ///协议原文：
    ///
    ///<pre>Starts tracking the given node for the computed style updates
    ///and whenever the computed style is updated for node, it queues
    ///a `computedStyleUpdated` event with throttling.
    ///There can only be 1 node tracked for computed style updates
    ///so passing a new node id removes tracking from the previous node.
    ///Pass `undefined` to disable tracking.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::css::commands::TrackComputedStyleUpdatesForNodeParams) -> ydp_core::Result<()> {
    ///let response = session.css().track_computed_style_updates_for_node(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn track_computed_style_updates_for_node(
        &self,
        params: commands::TrackComputedStyleUpdatesForNodeParams,
    ) -> ydp_core::Request<commands::TrackComputedStyleUpdatesForNode> {
        self.session
            .send::<commands::TrackComputedStyleUpdatesForNode>(params)
    }
}
