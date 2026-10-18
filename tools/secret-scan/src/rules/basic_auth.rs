//! `@secretlint/secretlint-rule-basicauth` of `@secretlint/secretlint-rule-preset-recommend`
//! 13.0.7: a `user:password@host` in an HTTP or FTP URL, unless the user or the password looks
//! like a placeholder: too short, a `${…}`-like variable, holding a common placeholder such as
//! `password` or `localhost`, or a password of one character repeated.

use std::sync::LazyLock;

use crate::js::{BOUNDARY, is_one_unit_repeated, utf16_len};
use crate::pattern::Pattern;
use crate::source::{Finding, Source};

use super::finding;

/// The rule's id.
const RULE: &str = "@secretlint/secretlint-rule-basicauth";

/// The placeholders a user or a password holding one of them, in any case, is taken for.
const PLACEHOLDERS: [&str; 15] = [
    "localhost",
    "127.0.0.1",
    "password",
    "mypassword",
    "myusername",
    "{password}",
    "${password}",
    "{{password}}",
    "{username}",
    "${username}",
    "{{username}}",
    "YOUR_PASSWORD",
    "YOUR_USERNAME",
    "REPLACE_WITH_PASSWORD",
    "REPLACE_WITH_USERNAME",
];

/// A URL with credentials.
static URL: LazyLock<Pattern> = LazyLock::new(|| {
    Pattern::plain(&format!(
        "(?P<protocol>(?:https?|ftp|ftps))://(?P<user>[-a-zA-Z0-9_]{{1,256}}):(?P<password>[-a-zA-Z0-9_]{{1,256}})@[-a-zA-Z0-9%._+~#=]{{1,256}}\\.[a-zA-Z0-9()]{{1,6}}{BOUNDARY}"
    ))
});

/// The variable-like patterns, `${var}`, `{{var}}`, `{var}`, `%VAR%` and `$VAR`.
static VARIABLES: LazyLock<Vec<Pattern>> = LazyLock::new(|| {
    [
        r"\$\{[^}]{1,50}\}",
        r"\{\{[^}]{1,50}\}\}",
        r"\{[^}]{1,50}\}",
        "%[A-Z_]{1,30}%",
        r"\$[A-Z_]{1,30}",
    ]
    .into_iter()
    .map(Pattern::plain)
    .collect()
});

/// Whether a text holds one of the variable-like patterns.
fn is_variable_like(text: &str) -> bool {
    VARIABLES.iter().any(|pattern| pattern.test(text))
}

/// Whether a text holds a placeholder, compared in lower case.
fn holds_placeholder(text: &str) -> bool {
    let lower = text.to_lowercase();

    PLACEHOLDERS
        .iter()
        .any(|placeholder| lower.contains(&placeholder.to_lowercase()))
}

/// Whether a user looks real: two characters or more, no variable, no placeholder.
fn is_real_user(user: &str) -> bool {
    utf16_len(user) >= 2 && !is_variable_like(user) && !holds_placeholder(user)
}

/// Whether a password looks real: four characters or more, no variable, no placeholder, not one
/// character repeated.
fn is_real_password(password: &str) -> bool {
    utf16_len(password) >= 4
        && !is_variable_like(password)
        && !holds_placeholder(password)
        && !is_one_unit_repeated(password)
}

/// The rule's findings.
pub fn report(source: &Source<'_>, findings: &mut Vec<Finding>) {
    for found in URL.find_all(&source.content) {
        let user = found.group("user").unwrap_or_default();
        let password = found.group("password").unwrap_or_default();

        if is_real_user(user) && is_real_password(password) {
            let credential = &source.content[found.start..found.end];

            findings.push(finding(
                RULE,
                "BasicAuth",
                format!("found basic auth credential: {credential}"),
                vec![credential.to_owned()],
                source.range(found.start, credential),
            ));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn placeholders_and_variables_are_not_real() {
        assert!(is_real_user("deploy"));
        assert!(!is_real_user("a"));
        assert!(!is_real_user("MyUserName"));
        assert!(!is_real_user("${USER}"));
        assert!(is_real_password("s3cr3tV4lue"));
        assert!(!is_real_password("abc"));
        assert!(!is_real_password("xxxx"));
        assert!(!is_real_password("my-Password-1"));
        assert!(!is_real_password("%PASS%"));
        assert!(!is_real_password("$PASS"));
        assert!(!is_real_password("{{pass}}"));
    }
}
