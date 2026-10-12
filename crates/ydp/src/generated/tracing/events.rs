///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Tracing.event.bufferUsage/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct BufferUsageParams {
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Tracing.event.bufferUsage/parameters/percentFull。
    ///
    ///协议原文：
    ///
    ///<pre>A number in range [0..1] that indicates the used size of event buffer as a fraction of its
    ///total size.</pre>
    ///
    ///
    #[serde(rename = "percentFull")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub percent_full: Option<ydp_core::FiniteF64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Tracing.event.bufferUsage/parameters/eventCount。
    ///
    ///协议原文：
    ///
    ///<pre>An approximate number of events in the trace log.</pre>
    ///
    ///
    #[serde(rename = "eventCount")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub event_count: Option<ydp_core::FiniteF64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Tracing.event.bufferUsage/parameters/value。
    ///
    ///协议原文：
    ///
    ///<pre>A number in range [0..1] that indicates the used size of event buffer as a fraction of its
    ///total size.</pre>
    ///
    ///
    #[serde(rename = "value")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub value: Option<ydp_core::FiniteF64>,
}
impl<'de> serde::Deserialize<'de> for BufferUsageParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = BufferUsageParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Tracing.event.bufferUsage/parameters/percentFull。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>A number in range [0..1] that indicates the used size of event buffer as a fraction of its
                    ///total size.</pre>
                    ///
                    ///
                    #[serde(rename = "percentFull")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub percent_full: Option<ydp_core::FiniteF64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Tracing.event.bufferUsage/parameters/eventCount。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>An approximate number of events in the trace log.</pre>
                    ///
                    ///
                    #[serde(rename = "eventCount")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub event_count: Option<ydp_core::FiniteF64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Tracing.event.bufferUsage/parameters/value。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>A number in range [0..1] that indicates the used size of event buffer as a fraction of its
                    ///total size.</pre>
                    ///
                    ///
                    #[serde(rename = "value")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub value: Option<ydp_core::FiniteF64>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(BufferUsageParams {
                    percent_full: fields.percent_full,
                    event_count: fields.event_count,
                    value: fields.value,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Tracing.event.bufferUsage。此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct BufferUsage;
impl ydp_core::Event for BufferUsage {
    type Params = BufferUsageParams;
    const METHOD: &'static str = "Tracing.bufferUsage";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Tracing.event.dataCollected/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct DataCollectedParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Tracing.event.dataCollected/parameters/value。
    #[serde(rename = "value")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub value: Vec<std::collections::BTreeMap<String, serde_json::Value>>,
}
impl<'de> serde::Deserialize<'de> for DataCollectedParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = DataCollectedParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Tracing.event.dataCollected/parameters/value。
                    #[serde(rename = "value")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub value: Vec<std::collections::BTreeMap<String, serde_json::Value>>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(DataCollectedParams {
                    value: fields.value,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Tracing.event.dataCollected。
///
///协议原文：
///
///<pre>Contains a bucket of collected trace events. When tracing is stopped collected events will be
///sent as a sequence of dataCollected events followed by tracingComplete event.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct DataCollected;
impl ydp_core::Event for DataCollected {
    type Params = DataCollectedParams;
    const METHOD: &'static str = "Tracing.dataCollected";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Tracing.event.tracingComplete/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct TracingCompleteParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Tracing.event.tracingComplete/parameters/dataLossOccurred。
    ///
    ///协议原文：
    ///
    ///<pre>Indicates whether some trace data is known to have been lost, e.g. because the trace ring
    ///buffer wrapped around.</pre>
    ///
    ///
    #[serde(rename = "dataLossOccurred")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub data_loss_occurred: bool,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Tracing.event.tracingComplete/parameters/stream。
    ///
    ///协议原文：
    ///
    ///<pre>A handle of the stream that holds resulting trace data.</pre>
    ///
    ///
    #[serde(rename = "stream")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub stream: Option<crate::io::types::StreamHandle>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Tracing.event.tracingComplete/parameters/traceFormat。
    ///
    ///协议原文：
    ///
    ///<pre>Trace data format of returned stream.</pre>
    ///
    ///
    #[serde(rename = "traceFormat")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub trace_format: Option<crate::tracing::types::StreamFormat>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Tracing.event.tracingComplete/parameters/streamCompression。
    ///
    ///协议原文：
    ///
    ///<pre>Compression format of returned stream.</pre>
    ///
    ///
    #[serde(rename = "streamCompression")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub stream_compression: Option<crate::tracing::types::StreamCompression>,
}
impl<'de> serde::Deserialize<'de> for TracingCompleteParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = TracingCompleteParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Tracing.event.tracingComplete/parameters/dataLossOccurred。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Indicates whether some trace data is known to have been lost, e.g. because the trace ring
                    ///buffer wrapped around.</pre>
                    ///
                    ///
                    #[serde(rename = "dataLossOccurred")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub data_loss_occurred: bool,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Tracing.event.tracingComplete/parameters/stream。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>A handle of the stream that holds resulting trace data.</pre>
                    ///
                    ///
                    #[serde(rename = "stream")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub stream: Option<crate::io::types::StreamHandle>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Tracing.event.tracingComplete/parameters/traceFormat。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Trace data format of returned stream.</pre>
                    ///
                    ///
                    #[serde(rename = "traceFormat")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub trace_format: Option<crate::tracing::types::StreamFormat>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Tracing.event.tracingComplete/parameters/streamCompression。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Compression format of returned stream.</pre>
                    ///
                    ///
                    #[serde(rename = "streamCompression")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub stream_compression: Option<crate::tracing::types::StreamCompression>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(TracingCompleteParams {
                    data_loss_occurred: fields.data_loss_occurred,
                    stream: fields.stream,
                    trace_format: fields.trace_format,
                    stream_compression: fields.stream_compression,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Tracing.event.tracingComplete。
///
///协议原文：
///
///<pre>Signals that tracing is stopped and there is no trace buffers pending flush, all data were
///delivered via dataCollected events.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct TracingComplete;
impl ydp_core::Event for TracingComplete {
    type Params = TracingCompleteParams;
    const METHOD: &'static str = "Tracing.tracingComplete";
}
