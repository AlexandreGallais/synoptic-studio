//! Tests of the JSON reader.

use hooks::json::{Value, parse, quote};

#[test]
fn reads_every_kind_of_value() {
    let value = parse(r#" {"a": [1, -2.5e1, true, false, null], "b": {"c": "d"}} "#).expect("JSON");

    assert_eq!(
        value.get("a"),
        &Value::Array(vec![
            Value::Number(1.0),
            Value::Number(-25.0),
            Value::Bool(true),
            Value::Bool(false),
            Value::Null,
        ])
    );
    assert_eq!(value.get("b").get("c").text(), "d");
}

#[test]
fn reads_escapes_and_characters_beyond_the_basic_plane() {
    let value = parse(r#""a\"b\\c\/d\n\t\r\b\f é \u00e9 \ud83d\ude00 😀""#).expect("JSON");

    assert_eq!(value.text(), "a\"b\\c/d\n\t\r\u{8}\u{c} é é 😀 😀");
}

#[test]
fn reads_empty_containers() {
    assert_eq!(parse("[]"), Some(Value::Array(Vec::new())));
    assert_eq!(parse("{ }"), Some(Value::Object(Vec::new())));
}

#[test]
fn refuses_what_is_not_one_json_value() {
    for text in [
        "",
        "{",
        "[1,]",
        "[1 2]",
        "{\"a\" 1}",
        "{a: 1}",
        "\"open",
        "nul",
        "1 2",
        "\"\\x\"",
        "\"\\ud83d\\u0041\"",
        "\"\\u12\"",
        "-",
    ] {
        assert_eq!(parse(text), None, "{text}");
    }
}

#[test]
fn gives_null_and_empty_for_what_is_missing() {
    let value = parse(r#"{"a": 1, "a": 2}"#).expect("JSON");

    assert_eq!(value.get("a"), &Value::Number(1.0));
    assert_eq!(value.get("z"), &Value::Null);
    assert_eq!(value.get("a").get("b"), &Value::Null);
    assert_eq!(value.get("a").text(), "");
    assert_eq!(value.get("a").items(), &[] as &[Value]);
}

#[test]
fn quotes_a_text_that_reads_back_the_same() {
    let text = "say \"hi\"\\\n\t\r\u{1} — é";

    assert_eq!(quote(text), r#""say \"hi\"\\\n\t\r\u0001 — é""#);
    assert_eq!(
        parse(&quote(text)).map(|value| value.text().to_owned()),
        Some(text.to_owned())
    );
}
