//! `@secretlint/secretlint-rule-1password` of `@secretlint/secretlint-rule-preset-recommend`
//! 13.0.7: a 1Password service account token, `ops_` and base64 of a JSON object or array, decoded
//! as `atob` decodes it (`-` and `_` read as `+` and `/`) and parsed, each byte one character as
//! `atob`'s binary string holds it.

use std::sync::LazyLock;

use serde_json::Value;

use crate::base64;
use crate::pattern::Pattern;
use crate::source::{Finding, Source};

use super::finding;

/// The rule's id.
const RULE: &str = "@secretlint/secretlint-rule-1password";

/// `ops_` and base64 starting `ey` (`{"`) and ending `fQ` (`}`), its padding included.
static TOKEN: LazyLock<Pattern> = LazyLock::new(|| {
    Pattern::bounded(
        "ops_(?P<base64>ey[A-Za-z0-9+/=]{100,1280}fQ={0,2})",
        "A-Za-z0-9+/=",
    )
});

/// Whether base64 decodes to a JSON object or array.
fn is_token(base64: &str) -> bool {
    let normalized = base64.replace('-', "+").replace('_', "/");
    let Some(bytes) = base64::decode(&normalized) else {
        return false;
    };
    let text: String = bytes.into_iter().map(char::from).collect();

    matches!(
        serde_json::from_str::<Value>(&text),
        Ok(Value::Object(_) | Value::Array(_))
    )
}

/// The rule's findings.
pub fn report(source: &Source<'_>, findings: &mut Vec<Finding>) {
    for found in TOKEN.find_all(&source.content) {
        if is_token(found.group("base64").unwrap_or_default()) {
            let token = &source.content[found.start..found.end];

            findings.push(finding(
                RULE,
                "OPS_TOKEN",
                format!("found 1Password Service Account token: {token}"),
                vec![token.to_owned()],
                source.range(found.start, token),
            ));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_token_is_base64_of_json_object_or_array() {
        // eyJhIjoxfQ== is {"a":1}; WzFd is [1]; MQ== is 1, a number; eyJh is {"a, cut.
        assert!(is_token("eyJhIjoxfQ=="));
        assert!(is_token("WzFd"));
        assert!(!is_token("MQ=="));
        assert!(!is_token("eyJh"));
        assert!(!is_token("e"));
    }
}
