//! `@secretlint/secretlint-rule-database-connection-string` of
//! `@secretlint/secretlint-rule-preset-recommend` 13.0.7: a MongoDB, MySQL or PostgreSQL URI with
//! a user and a password, unless they look like an example: the password the user's, shorter
//! than four characters, a `${…}`-like variable, exactly a common placeholder, or one character
//! repeated. The scheme is matched in any ASCII case (the `i` flag without `u`).

use std::sync::LazyLock;

use crate::js::{SPACE, is_one_unit_repeated, utf16_len};
use crate::pattern::Pattern;
use crate::source::{Finding, Source};

use super::finding;

/// The rule's id.
const RULE: &str = "@secretlint/secretlint-rule-database-connection-string";

/// The placeholders a password equal to one of them, in any case, is taken for.
const PLACEHOLDERS: [&str; 23] = [
    "localhost",
    "127.0.0.1",
    "example.com",
    "example.org",
    "test.com",
    "password",
    "username",
    "secret",
    "mypassword",
    "myusername",
    "root",
    "admin",
    "user",
    "pass",
    "{password}",
    "${password}",
    "{{password}}",
    "{username}",
    "${username}",
    "{{username}}",
    "YOUR_PASSWORD",
    "YOUR_USERNAME",
    "REPLACE_WITH_PASSWORD",
];

/// A URI after its scheme: credentials, host, database and options, as the three patterns share.
fn uri(scheme: &str) -> Pattern {
    Pattern::plain(&format!(
        "(?i-u:{scheme})://(?:(?P<username>[^:/{SPACE}]{{1,100}}):(?P<password>[^@/{SPACE}]{{1,200}})@)?(?P<host>[^/{SPACE}?]{{1,100}})(?:/(?P<database>[^?{SPACE}]{{0,100}}))?(?:\\?(?P<options>[^{SPACE}]{{0,500}}))?"
    ))
}

/// The three URIs, in the rule's order, each with its message.
static URIS: LazyLock<[(Pattern, &str, &str); 3]> = LazyLock::new(|| {
    [
        (
            uri(r"mongodb(?:\+srv)?"),
            "MongoDBConnection",
            "found MongoDB connection string",
        ),
        (
            uri("(?:jdbc:)?mysql(?:x)?"),
            "MySQLConnection",
            "found MySQL connection string",
        ),
        (
            uri("postgres(?:ql)?"),
            "PostgreSQLConnection",
            "found PostgreSQL connection string",
        ),
    ]
});

/// The variable-like patterns, `${var}`, `{{var}}`, `{var}`, `%VAR%`, `$VAR` and `$(var)`.
static VARIABLES: LazyLock<Vec<Pattern>> = LazyLock::new(|| {
    [
        r"\$\{[^}]{1,50}\}",
        r"\{\{[^}]{1,50}\}\}",
        r"\{[^}]{1,50}\}",
        "%[A-Z_]{1,30}%",
        r"\$[A-Z_]{1,30}",
        r"\$\([^)]{1,50}\)",
    ]
    .into_iter()
    .map(Pattern::plain)
    .collect()
});

/// Whether the credentials look real.
fn is_real(username: &str, password: &str) -> bool {
    let lower = password.to_lowercase();

    username != password
        && utf16_len(password) >= 4
        && !VARIABLES.iter().any(|pattern| pattern.test(password))
        && !PLACEHOLDERS
            .iter()
            .any(|placeholder| lower == placeholder.to_lowercase())
        && !is_one_unit_repeated(password)
}

/// The rule's findings, MongoDB's, then MySQL's, then PostgreSQL's.
pub fn report(source: &Source<'_>, findings: &mut Vec<Finding>) {
    for (pattern, id, label) in URIS.iter() {
        for found in pattern.find_all(&source.content) {
            let (Some(username), Some(password)) =
                (found.group("username"), found.group("password"))
            else {
                continue;
            };

            if is_real(username, password) {
                let uri = &source.content[found.start..found.end];

                findings.push(finding(
                    RULE,
                    id,
                    format!("{label}: {uri}"),
                    vec![uri.to_owned()],
                    source.range(found.start, uri),
                ));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_credentials_are_not_real() {
        assert!(is_real("app", "Zq8vLx2"));
        assert!(!is_real("same", "same"));
        assert!(!is_real("app", "abc"));
        assert!(!is_real("app", "Admin"));
        assert!(!is_real("app", "$(cat key)"));
        assert!(!is_real("app", "1111"));
        assert!(is_real("app", "admin1"));
    }
}
