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
