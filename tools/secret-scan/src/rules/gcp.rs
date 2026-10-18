//! `@secretlint/secretlint-rule-gcp` of `@secretlint/secretlint-rule-preset-recommend` 13.0.7, the
//! only rule that reads a binary file: a `.p12` file whose PKCS #12 MAC verifies under
//! `notasecret`, the password of every GCP service account key in that format; a `.json` file
//! that parses to an object holding `private_key_id` and a `private_key` that looks like a PEM
//! private key. Either finding spans the whole text and names the file, not its content.

use std::sync::LazyLock;

use serde_json::Value;

use crate::js::{SPACE, utf16_len};
use crate::pattern::Pattern;
use crate::pkcs12;
use crate::source::{Finding, Source, file_name};

use super::finding;

/// The rule's id.
const RULE: &str = "@secretlint/secretlint-rule-gcp";

/// The PKCS #12 password of GCP service account keys.
const P12_PASSWORD: &str = "notasecret";

/// The start of a PEM private key in the key's value.
static PRIVATE_KEY: LazyLock<Pattern> = LazyLock::new(|| {
    Pattern::plain(&format!(
        "-----BEGIN[{SPACE}]?(DSA|RSA|EC|PGP|OPENSSH)?[{SPACE}]?PRIVATE KEY"
    ))
});

/// The rule's finding in the file, if any.
pub fn report(source: &Source<'_>, findings: &mut Vec<Finding>) {
    let message = if source.has_extension(".p12") {
        pkcs12::verifies(source.bytes, P12_PASSWORD).then_some((
            "PrivateKeyP12",
            "found GCP Service Account's private key(p12)",
        ))
    } else if source.has_extension(".json") {
        is_service_account_key(&source.content).then_some((
            "PrivateKeyJSON",
            "found GCP Service Account's private key(json)",
        ))
    } else {
        None
    };

    if let Some((id, label)) = message {
        let name = file_name(source.path);

        findings.push(finding(
            RULE,
            id,
            format!("{label}: {name}"),
            vec![name.to_owned()],
            (0, utf16_len(&source.content)),
        ));
    }
}

/// Whether a JSON text is an object with `private_key_id` and a `private_key` whose text, as
/// JavaScript's `String` gives it, holds a PEM private key's start.
fn is_service_account_key(text: &str) -> bool {
    let Ok(Value::Object(object)) = serde_json::from_str::<Value>(text) else {
        return false;
    };

    object.contains_key("private_key_id")
        && object
            .get("private_key")
            .is_some_and(|key| PRIVATE_KEY.test(&js_string(key)))
}

/// A JSON value as JavaScript's `String` writes it: an array joined by commas, its `null`
/// elements empty, an object `[object Object]`.
fn js_string(value: &Value) -> String {
    match value {
        Value::String(text) => text.clone(),
        Value::Array(items) => items
            .iter()
            .map(|item| match item {
                Value::Null => String::new(),
                other => js_string(other),
            })
            .collect::<Vec<_>>()
            .join(","),
        Value::Object(_) => "[object Object]".to_owned(),
        other => other.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_service_account_key_needs_both_fields_and_a_pem_start() {
        let key = r#"{"private_key_id": "1", "private_key": "-----BEGIN RSA PRIVATE KEY-----\n"}"#;

        assert!(is_service_account_key(key));
        assert!(is_service_account_key(
            r#"{"private_key_id": 1, "private_key": ["x", null, "-----BEGIN PRIVATE KEY"]}"#
        ));
        assert!(!is_service_account_key(
            r#"{"private_key": "-----BEGIN PRIVATE KEY"}"#
        ));
        assert!(!is_service_account_key(
            r#"{"private_key_id": "1", "private_key": "secret"}"#
        ));
        assert!(!is_service_account_key(
            r#"{"private_key_id": "1", "private_key": {}}"#
        ));
        assert!(!is_service_account_key(
            r#"["private_key_id", "private_key"]"#
        ));
        assert!(!is_service_account_key("not json"));
    }

    #[test]
    fn values_are_written_as_javascripts_string_writes_them() {
        assert_eq!(
            js_string(&serde_json::json!([1, null, "a", [true]])),
            "1,,a,true"
        );
        assert_eq!(js_string(&serde_json::json!({"a": 1})), "[object Object]");
        assert_eq!(js_string(&serde_json::json!(null)), "null");
    }
}
