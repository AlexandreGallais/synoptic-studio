//! `@secretlint/secretlint-rule-npm` of `@secretlint/secretlint-rule-preset-recommend` 13.0.7: a
//! GitHub token in an `x-oauth-basic` URL of `package.json` or `package-lock.json`; an
//! `_authToken=` of `.npmrc` that is not a `${…}` variable; and an npm access token anywhere. The
//! first two ranges start at the match and span the token's length, as the rule gives them.

use std::sync::LazyLock;

use crate::js::DOT;
use crate::pattern::Pattern;
use crate::source::{Finding, Source};

use super::finding;

/// The rule's id.
const RULE: &str = "@secretlint/secretlint-rule-npm";

/// A token before `:x-oauth-basic@github.com/`, the `.` of `github.com` any character.
static X_OAUTH: LazyLock<Pattern> = LazyLock::new(|| {
    Pattern::plain(&format!(
        "https?://(?P<token>{DOT}{{1,256}}):x-oauth-basic@github{DOT}com/"
    ))
});

/// `_authToken=` and the rest of the line, unless it starts with `$`.
static AUTH_TOKEN: LazyLock<Pattern> =
    LazyLock::new(|| Pattern::plain(&format!("_authToken=(?P<token>[^$]{DOT}*)")));

/// An npm access token: `npm_` and 36 characters.
static ACCESS_TOKEN: LazyLock<Pattern> =
    LazyLock::new(|| Pattern::bounded("npm_[A-Za-z0-9_]{36}", "A-Za-z0-9_"));

/// The rule's findings: by the file's name first, then the access tokens.
pub fn report(source: &Source<'_>, findings: &mut Vec<Finding>) {
    let path = source.path;

    #[expect(
        clippy::case_sensitive_file_extension_comparisons,
        reason = "the rule compares the path's end as JavaScript's `endsWith`, case included"
    )]
    let npmrc = path.ends_with(".npmrc");

    if path.ends_with("package.json") || path.ends_with("package-lock.json") {
        for found in X_OAUTH.find_all(&source.content) {
            let token = found.group("token").unwrap_or_default();

            findings.push(finding(
                RULE,
                "PackageJSON_xOauthToken",
                format!("found GitHub Token: {token}"),
                vec![token.to_owned()],
                source.range(found.start, token),
            ));
        }
    } else if npmrc {
        for found in AUTH_TOKEN.find_all(&source.content) {
            let token = found.group("token").unwrap_or_default();

            findings.push(finding(
                RULE,
                "Npmrc_authToken",
                format!("found npmrc authToken: {token}"),
                vec![token.to_owned()],
                source.range(found.start, token),
            ));
        }
    }
    for found in ACCESS_TOKEN.find_all(&source.content) {
        let token = &source.content[found.start..found.end];

        findings.push(finding(
            RULE,
            "NPM_ACCESS_TOKEN",
            format!("found npm access token: {token}"),
            vec![token.to_owned()],
            source.range(found.start, token),
        ));
    }
}
