//! 生成契约的序列化基础设施；不保存连接或协议域状态。
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// 无参数命令的对象参数；编码为 `{}`，接收时容忍新增字段。
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize)]
pub struct EmptyParams {}

/// 无返回字段的成功对象；接收时容忍端点新增结果字段。
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize)]
pub struct EmptyResponse {}

struct EmptyObjectVisitor;
impl<'de> serde::de::Visitor<'de> for EmptyObjectVisitor {
    type Value = ();
    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("JSON 对象")
    }
    fn visit_map<M: serde::de::MapAccess<'de>>(self, mut map: M) -> Result<(), M::Error> {
        while map
            .next_entry::<serde::de::IgnoredAny, serde::de::IgnoredAny>()?
            .is_some()
        {}
        Ok(())
    }
}
impl<'de> Deserialize<'de> for EmptyParams {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer
            .deserialize_map(EmptyObjectVisitor)
            .map(|()| Self {})
    }
}
impl<'de> Deserialize<'de> for EmptyResponse {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer
            .deserialize_map(EmptyObjectVisitor)
            .map(|()| Self {})
    }
}

/// 显式区分 JSON null 与有效值；字段是否缺失由生成结构控制。
#[derive(Debug, Clone, PartialEq)]
pub enum Nullable<T> {
    /// 显式 JSON null，不代表字段缺失。
    Null,
    /// 持有符合底层类型的值。
    Value(T),
}
impl<T: Serialize> Serialize for Nullable<T> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Null => serializer.serialize_none(),
            Self::Value(v) => v.serialize(serializer),
        }
    }
}
impl<'de, T: Deserialize<'de>> Deserialize<'de> for Nullable<T> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Option::<T>::deserialize(deserializer).map(|value| match value {
            Some(v) => Self::Value(v),
            None => Self::Null,
        })
    }
}

/// 已出现的字段直接解码为 `T` 并包装 Some，保留 optional any 的 null。
/// 配合字段上的 `default`，仅字段缺失才得到 None；类型错误向上传播。
/// ```
/// #[derive(serde::Deserialize)]
/// struct Data {
///     #[serde(default, deserialize_with = "ydp_core::binding_support::presence")]
///     value: Option<serde_json::Value>,
/// }
/// assert_eq!(serde_json::from_str::<Data>(r#"{"value":null}"#)?.value, Some(serde_json::Value::Null));
/// # Ok::<(), serde_json::Error>(())
/// ```
pub fn presence<'de, D: Deserializer<'de>, T: Deserialize<'de>>(
    deserializer: D,
) -> Result<Option<T>, D::Error> {
    T::deserialize(deserializer).map(Some)
}

/// 必需字段的反序列化入口；serde 的字段适配器确保缺失时报错。
/// 不添加默认值，null 是否有效由 `T` 决定。
/// ```
/// #[derive(serde::Deserialize)]
/// struct Data {
///     #[serde(deserialize_with = "ydp_core::binding_support::required")]
///     value: ydp_core::binding_support::Nullable<String>,
/// }
/// assert!(serde_json::from_str::<Data>("{}").is_err());
/// let data: Data = serde_json::from_str(r#"{"value":null}"#)?;
/// assert_eq!(data.value, ydp_core::binding_support::Nullable::Null);
/// # Ok::<(), serde_json::Error>(())
/// ```
pub fn required<'de, D: Deserializer<'de>, T: Deserialize<'de>>(
    deserializer: D,
) -> Result<T, D::Error> {
    T::deserialize(deserializer)
}

/// 只持有有限 f64 的协议 number；不提供整数精度或十进制精确性保证。
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct FiniteF64(f64);
impl FiniteF64 {
    /// 读取底层有限数值，不转移所有权、不执行任何通信。
    /// ```
    /// let number = ydp_core::FiniteF64::try_from(1.5)?;
    /// assert_eq!(number.get(), 1.5);
    /// # Ok::<(), ydp_core::Error>(())
    /// ```
    pub fn get(self) -> f64 {
        self.0
    }
}
impl TryFrom<f64> for FiniteF64 {
    type Error = crate::Error;
    /// 验证输入有限；NaN 或正负无穷返回 InvalidArgument。
    /// ```
    /// assert!(ydp_core::FiniteF64::try_from(f64::NAN).is_err());
    /// assert!(ydp_core::FiniteF64::try_from(0.0).is_ok());
    /// ```
    fn try_from(value: f64) -> Result<Self, Self::Error> {
        if value.is_finite() {
            Ok(Self(value))
        } else {
            Err(crate::Error::new(crate::ErrorKind::InvalidArgument))
        }
    }
}
impl Serialize for FiniteF64 {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        if self.0.is_finite() {
            serializer.serialize_f64(self.0)
        } else {
            Err(serde::ser::Error::custom("协议 number 必须有限"))
        }
    }
}
impl<'de> Deserialize<'de> for FiniteF64 {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = f64::deserialize(deserializer)?;
        Self::try_from(value).map_err(|_| serde::de::Error::custom("协议 number 必须有限"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[derive(Deserialize)]
    struct Shape {
        #[serde(default, deserialize_with = "presence")]
        optional: Option<String>,
        #[serde(default, deserialize_with = "presence")]
        any: Option<serde_json::Value>,
        #[serde(deserialize_with = "required")]
        nullable: Nullable<String>,
    }
    #[test]
    fn presence_is_not_null() {
        assert!(serde_json::from_str::<Shape>(r#"{"nullable":null,"optional":null}"#).is_err());
        assert!(serde_json::from_str::<Shape>("{}").is_err());
        let value: Shape = serde_json::from_str(r#"{"nullable":null,"any":null}"#).unwrap();
        assert_eq!(value.optional, None);
        assert_eq!(value.any, Some(serde_json::Value::Null));
        assert_eq!(value.nullable, Nullable::Null);
    }
    #[test]
    fn empty_objects_and_finite_numbers() {
        assert_eq!(serde_json::to_string(&EmptyParams {}).unwrap(), "{}");
        assert!(serde_json::from_str::<EmptyResponse>(r#"{"future":1}"#).is_ok());
        for invalid in ["null", "[]", "1"] {
            assert!(serde_json::from_str::<EmptyResponse>(invalid).is_err());
        }
        for invalid in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
            assert!(FiniteF64::try_from(invalid).is_err());
        }
        assert!(serde_json::from_str::<FiniteF64>("1e400").is_err());
    }
}
