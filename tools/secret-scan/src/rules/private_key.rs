//! `@secretlint/secretlint-rule-privatekey` of `@secretlint/secretlint-rule-preset-recommend`
//! 13.0.7: a PEM private key between its `BEGIN` and `END` lines, whose body, white space and
//! escaped `\n` removed, is base64 of 100 characters or more starting as a DER key (`MI`) or an
//! OpenSSH key (`b3BlbnNzaC1rZXktdjE`) does, so that a placeholder (`...`, `xxx`) is not taken.

use std::sync::LazyLock;

use crate::js::SPACE;
use crate::pattern::Pattern;
use crate::source::{Finding, Source};

use super::finding;

/// The rule's id.
const RULE: &str = "@secretlint/secretlint-rule-privatekey";

/// A PEM block of a private key, its body the group `body`.
static PEM: LazyLock<Pattern> = LazyLock::new(|| {
    let label = "(?:(?:RSA|DSA|EC|OPENSSH|PGP) )?PRIVATE KEY(?: BLOCK)?-----";

    Pattern::plain(&format!(
        "-----BEGIN[ ]?{label}\\n?(?P<body>[A-Za-z0-9+/={SPACE}\\\\]{{100,10000}})\\n?-----END[ ]?{label}"
    )).needing(&["-----BEGIN"])
});

/// Base64 alone, `=` at its end only.
static BASE64: LazyLock<Pattern> = LazyLock::new(|| Pattern::plain("^[A-Za-z0-9+/]+=*$"));

/// One white space character, JavaScript's `\s`.
static WHITE_SPACE: LazyLock<Pattern> = LazyLock::new(|| Pattern::plain(&format!("[{SPACE}]")));

/// The body without its white space nor its escaped line breaks, `\s|\\n` replaced by nothing.
fn stripped(body: &str) -> String {
    let mut text = String::with_capacity(body.len());
    let mut rest = body;

    while let Some(character) = rest.chars().next() {
        let mut buffer = [0; 4];

        if WHITE_SPACE.test(character.encode_utf8(&mut buffer)) {
            rest = &rest[character.len_utf8()..];
        } else if let Some(after) = rest.strip_prefix("\\n") {
            rest = after;
        } else {
            text.push(character);
            rest = &rest[character.len_utf8()..];
        }
    }
    text
}

/// Whether a stripped body is a real key's base64.
fn is_key(base64: &str) -> bool {
    BASE64.test(base64)
        && base64.len() >= 100
        && (base64.starts_with("MI") || base64.starts_with("b3BlbnNzaC1rZXktdjE"))
}

/// The rule's findings.
pub fn report(source: &Source<'_>, findings: &mut Vec<Finding>) {
    for found in PEM.find_all(&source.content) {
        if is_key(&stripped(found.group("body").unwrap_or_default())) {
            let key = &source.content[found.start..found.end];

            findings.push(finding(
                RULE,
                "PrivateKey",
                format!("found private key: {key}"),
                vec![key.to_owned()],
                source.range(found.start, key),
            ));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn white_space_and_escaped_line_breaks_are_removed() {
        // "MI\\nIB A\n=": `\n` escaped (two characters), a space and a line break go.
        assert_eq!(stripped("MI\\nIB A\n="), "MIIBA=");
        assert_eq!(stripped("a\\tb"), "a\\tb");
    }

    #[test]
    fn a_key_is_long_base64_with_a_key_start() {
        let der = format!("MI{}", "A".repeat(98));
        let openssh = format!("b3BlbnNzaC1rZXktdjE{}", "A".repeat(81));

        assert!(is_key(&der));
        assert!(is_key(&format!("{der}==")));
        assert!(is_key(&openssh));
        assert!(!is_key(&format!("MI{}", "A".repeat(97))));
        assert!(!is_key(&format!("XX{}", "A".repeat(98))));
        assert!(!is_key(&format!("MI{}=A", "A".repeat(98))));
    }
}
