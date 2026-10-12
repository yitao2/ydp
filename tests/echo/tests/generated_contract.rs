//! 独立生成源码的值语义测试，不用手写 Serde 类型替代生成器输出。
use ydp_core::binding_support::Nullable;
use ydp_echo_tests::echo::{
    commands::{EchoParams, EchoResponse},
    events::EchoedParams,
    types::Record,
};

#[test]
fn r38_generated_missing_null_and_value_are_distinct() {
    let missing_optional: Record =
        serde_json::from_str(r#"{"text":"required","maybe":null}"#).unwrap();
    assert_eq!(missing_optional.text, "required");
    assert_eq!(missing_optional.note, None);
    assert_eq!(missing_optional.extra, None);
    assert_eq!(missing_optional.maybe, Nullable::Null);
    let explicit_any: Record =
        serde_json::from_str(r#"{"text":"required","maybe":"value","extra":null}"#).unwrap();
    assert_eq!(explicit_any.extra, Some(serde_json::Value::Null));
    assert_eq!(explicit_any.maybe, Nullable::Value("value".into()));
    let encoded: serde_json::Value =
        serde_json::from_str(&serde_json::to_string(&explicit_any).unwrap()).unwrap();
    assert!(encoded.get("note").is_none());
    assert_eq!(encoded["extra"], serde_json::Value::Null);
    for invalid in [
        r#"{"text":"required"}"#,
        r#"{"maybe":null}"#,
        r#"{"text":null,"maybe":null}"#,
        r#"{"text":"required","maybe":null,"note":null}"#,
        r#"{"text":"required","maybe":1}"#,
        r#"["required",null,null,null]"#,
    ] {
        assert!(
            serde_json::from_str::<Record>(invalid).is_err(),
            "{invalid}"
        );
    }
}

#[test]
fn r37_generated_objects_reject_arrays_and_accept_unknown_fields() {
    for invalid in ["[]", r#"["text"]"#, "null", "{}", r#"{"text":null}"#] {
        assert!(
            serde_json::from_str::<EchoParams>(invalid).is_err(),
            "{invalid}"
        );
        assert!(
            serde_json::from_str::<EchoResponse>(invalid).is_err(),
            "{invalid}"
        );
        assert!(
            serde_json::from_str::<EchoedParams>(invalid).is_err(),
            "{invalid}"
        );
    }
    assert_eq!(
        serde_json::from_str::<EchoResponse>(r#"{"text":"ok","future":42}"#)
            .unwrap()
            .text,
        "ok"
    );
}

#[test]
fn r46_generated_markers_preserve_wire_names() {
    use ydp_core::{Command, Event};
    assert_eq!(ydp_echo_tests::echo::commands::Echo::METHOD, "Echo.echo");
    assert_eq!(ydp_echo_tests::echo::events::Echoed::METHOD, "Echo.echoed");
}

#[test]
fn r37_r38_named_nullable_and_closed_maps_preserve_shapes() {
    use ydp_echo_tests::echo::types::{Closed, MaybeMode, MaybeRecord};
    for invalid in ["[]", "[null]", "null", r#"{"x":1}"#] {
        assert!(
            serde_json::from_str::<Closed>(invalid).is_err(),
            "{invalid}"
        );
    }
    let closed: Closed = serde_json::from_str("{}").unwrap();
    assert_eq!(serde_json::to_string(&closed).unwrap(), "{}");
    let object: MaybeRecord = serde_json::from_str("null").unwrap();
    let enumeration: MaybeMode = serde_json::from_str("null").unwrap();
    assert_eq!(serde_json::to_string(&object).unwrap(), "null");
    assert_eq!(serde_json::to_string(&enumeration).unwrap(), "null");
    assert!(serde_json::from_str::<MaybeRecord>(r#"{"text":"present"}"#).is_ok());
    assert!(serde_json::from_str::<MaybeRecord>("{}").is_err());
    assert!(serde_json::from_str::<MaybeMode>("1").is_err());
}

#[test]
fn r37_r39_all_wire_shapes_and_numeric_boundaries_round_trip() {
    use ydp_echo_tests::echo::types::WireShapes;

    let source = format!(
        r#"{{"stringValue":"文本","integerValue":{},"numberValue":1.0000000000000002,"booleanValue":true,"anyValue":[null,{{"nested":7}}],"binaryValue":"AAECAw==","openMap":{{"text":"value","flag":false}},"integerMap":{{"min":{},"max":{}}},"emptyObject":{{}},"unknown":42}}"#,
        i64::MIN,
        i64::MIN,
        i64::MAX
    );
    let decoded: WireShapes = serde_json::from_str(&source).unwrap();
    assert_eq!(decoded.string_value, "文本");
    assert_eq!(decoded.integer_value, i64::MIN);
    assert_eq!(
        decoded.number_value.get().to_bits(),
        1.0000000000000002_f64.to_bits()
    );
    assert!(decoded.boolean_value);
    assert_eq!(decoded.any_value[1]["nested"], 7);
    assert_eq!(decoded.binary_value, "AAECAw==");
    assert_eq!(decoded.open_map["text"], "value");
    assert_eq!(decoded.integer_map["min"], i64::MIN);
    assert_eq!(decoded.integer_map["max"], i64::MAX);

    let encoded = serde_json::to_value(decoded).unwrap();
    assert_eq!(encoded["integerValue"], i64::MIN);
    assert_eq!(
        encoded["numberValue"].as_f64().unwrap().to_bits(),
        1.0000000000000002_f64.to_bits()
    );
    assert_eq!(encoded["anyValue"][0], serde_json::Value::Null);
    assert_eq!(encoded["binaryValue"], "AAECAw==");
    assert_eq!(encoded["integerMap"]["max"], i64::MAX);
    assert_eq!(encoded["emptyObject"], serde_json::json!({}));
    assert!(encoded.get("unknown").is_none());

    assert!(serde_json::from_str::<i64>("9223372036854775808").is_err());
    assert!(serde_json::from_str::<i64>("-9223372036854775809").is_err());
}

#[test]
fn r40_named_strings_and_known_unknown_enum_round_trip() {
    use ydp_echo_tests::echo::types::{Label, Mode};
    let label = Label::new("标签".into());
    assert_eq!(label.as_str(), "标签");
    assert_eq!(label.into_inner(), "标签");
    let known = Mode::from("Unknown".to_owned());
    assert!(matches!(known, Mode::KnownUnknown));
    assert_eq!(known.as_str(), "Unknown");
    let unknown: Mode = serde_json::from_str(r#""upstream-new-value""#).unwrap();
    assert!(matches!(unknown, Mode::Unknown(_)));
    assert_eq!(
        serde_json::to_string(&unknown).unwrap(),
        r#""upstream-new-value""#
    );
}
