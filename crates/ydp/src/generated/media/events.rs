///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Media.event.playerCreated/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct PlayerCreatedParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Media.event.playerCreated/parameters/player。
    #[serde(rename = "player")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub player: crate::media::types::Player,
}
impl<'de> serde::Deserialize<'de> for PlayerCreatedParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = PlayerCreatedParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Media.event.playerCreated/parameters/player。
                    #[serde(rename = "player")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub player: crate::media::types::Player,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(PlayerCreatedParams {
                    player: fields.player,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Media.event.playerCreated。
///
///协议原文：
///
///<pre>Called whenever a player is created, or when a new agent joins and receives
///a list of active players. If an agent is restored, it will receive one
///event for each active player.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct PlayerCreated;
impl ydp_core::Event for PlayerCreated {
    type Params = PlayerCreatedParams;
    const METHOD: &'static str = "Media.playerCreated";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Media.event.playerErrorsRaised/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct PlayerErrorsRaisedParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Media.event.playerErrorsRaised/parameters/playerId。
    #[serde(rename = "playerId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub player_id: crate::media::types::PlayerId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Media.event.playerErrorsRaised/parameters/errors。
    #[serde(rename = "errors")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub errors: Vec<crate::media::types::PlayerError>,
}
impl<'de> serde::Deserialize<'de> for PlayerErrorsRaisedParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = PlayerErrorsRaisedParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Media.event.playerErrorsRaised/parameters/playerId。
                    #[serde(rename = "playerId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub player_id: crate::media::types::PlayerId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Media.event.playerErrorsRaised/parameters/errors。
                    #[serde(rename = "errors")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub errors: Vec<crate::media::types::PlayerError>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(PlayerErrorsRaisedParams {
                    player_id: fields.player_id,
                    errors: fields.errors,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Media.event.playerErrorsRaised。
///
///协议原文：
///
///<pre>Send a list of any errors that need to be delivered.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct PlayerErrorsRaised;
impl ydp_core::Event for PlayerErrorsRaised {
    type Params = PlayerErrorsRaisedParams;
    const METHOD: &'static str = "Media.playerErrorsRaised";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Media.event.playerEventsAdded/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct PlayerEventsAddedParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Media.event.playerEventsAdded/parameters/playerId。
    #[serde(rename = "playerId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub player_id: crate::media::types::PlayerId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Media.event.playerEventsAdded/parameters/events。
    #[serde(rename = "events")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub events: Vec<crate::media::types::PlayerEvent>,
}
impl<'de> serde::Deserialize<'de> for PlayerEventsAddedParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = PlayerEventsAddedParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Media.event.playerEventsAdded/parameters/playerId。
                    #[serde(rename = "playerId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub player_id: crate::media::types::PlayerId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Media.event.playerEventsAdded/parameters/events。
                    #[serde(rename = "events")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub events: Vec<crate::media::types::PlayerEvent>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(PlayerEventsAddedParams {
                    player_id: fields.player_id,
                    events: fields.events,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Media.event.playerEventsAdded。
///
///协议原文：
///
///<pre>Send events as a list, allowing them to be batched on the browser for less
///congestion. If batched, events must ALWAYS be in chronological order.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct PlayerEventsAdded;
impl ydp_core::Event for PlayerEventsAdded {
    type Params = PlayerEventsAddedParams;
    const METHOD: &'static str = "Media.playerEventsAdded";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Media.event.playerMessagesLogged/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct PlayerMessagesLoggedParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Media.event.playerMessagesLogged/parameters/playerId。
    #[serde(rename = "playerId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub player_id: crate::media::types::PlayerId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Media.event.playerMessagesLogged/parameters/messages。
    #[serde(rename = "messages")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub messages: Vec<crate::media::types::PlayerMessage>,
}
impl<'de> serde::Deserialize<'de> for PlayerMessagesLoggedParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = PlayerMessagesLoggedParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Media.event.playerMessagesLogged/parameters/playerId。
                    #[serde(rename = "playerId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub player_id: crate::media::types::PlayerId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Media.event.playerMessagesLogged/parameters/messages。
                    #[serde(rename = "messages")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub messages: Vec<crate::media::types::PlayerMessage>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(PlayerMessagesLoggedParams {
                    player_id: fields.player_id,
                    messages: fields.messages,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Media.event.playerMessagesLogged。
///
///协议原文：
///
///<pre>Send a list of any messages that need to be delivered.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct PlayerMessagesLogged;
impl ydp_core::Event for PlayerMessagesLogged {
    type Params = PlayerMessagesLoggedParams;
    const METHOD: &'static str = "Media.playerMessagesLogged";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Media.event.playerPropertiesChanged/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct PlayerPropertiesChangedParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Media.event.playerPropertiesChanged/parameters/playerId。
    #[serde(rename = "playerId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub player_id: crate::media::types::PlayerId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Media.event.playerPropertiesChanged/parameters/properties。
    #[serde(rename = "properties")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub properties: Vec<crate::media::types::PlayerProperty>,
}
impl<'de> serde::Deserialize<'de> for PlayerPropertiesChangedParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = PlayerPropertiesChangedParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Media.event.playerPropertiesChanged/parameters/playerId。
                    #[serde(rename = "playerId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub player_id: crate::media::types::PlayerId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Media.event.playerPropertiesChanged/parameters/properties。
                    #[serde(rename = "properties")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub properties: Vec<crate::media::types::PlayerProperty>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(PlayerPropertiesChangedParams {
                    player_id: fields.player_id,
                    properties: fields.properties,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Media.event.playerPropertiesChanged。
///
///协议原文：
///
///<pre>This can be called multiple times, and can be used to set / override /
///remove player properties. A null propValue indicates removal.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct PlayerPropertiesChanged;
impl ydp_core::Event for PlayerPropertiesChanged {
    type Params = PlayerPropertiesChangedParams;
    const METHOD: &'static str = "Media.playerPropertiesChanged";
}
