//! The comment filter on the cases of `@secretlint/secretlint-rule-filter-comments` 13.0.7's own
//! snapshots (`packages/@secretlint/secretlint-rule-filter-comments/test/snapshots/` at secretlint's
//! tag `v13.0.7`), which test it with two pattern rules of their own, `SECRET <digit>` and
//! `SECRET <letter>`: here an npm access token stands for the first and a GitHub token for the
//! second, rules of the preset, each built at run time so that no token is in the source. The
//! expected lines are the snapshots'; the edges of the rule's arithmetic follow.

use secret_scan::rules::Options;

/// The npm rule's id, standing for `secret-number`.
const NPM: &str = "@secretlint/secretlint-rule-npm";

/// The GitHub rule's id, standing for `secret-alphabet`.
const GITHUB: &str = "@secretlint/secretlint-rule-github";

/// An npm access token, `npm_` and 36 characters.
fn number() -> String {
    format!("npm_{}", "N".repeat(36))
}

/// A GitHub personal access token, `ghp_` and 36 characters.
fn letter() -> String {
    format!("ghp_{}", "L".repeat(36))
}

/// The rule and the line of each finding of a text.
fn findings(text: &str) -> Vec<(&'static str, usize)> {
    secret_scan::scan("input.txt", text.as_bytes(), Options::default())
        .into_iter()
        .map(|located| (located.finding.rule_id, located.line))
        .collect()
}

#[test]
fn disable_line_ignores_its_own_line() {
    // ng.secretlint-disable-line: line 1 ignored, line 2 found.
    let text = format!(
        "THIS IS {} // secretlint-disable-line\nTHIS IS {}\n",
        letter(),
        letter()
    );

    assert_eq!(findings(&text), [(GITHUB, 2)]);
}

#[test]
fn disable_next_line_ignores_the_next_line() {
    // ng.secretlint-disable-next-line: line 2 ignored, line 3 found.
    let text = format!(
        "// secretlint-disable-next-line\nTHIS IS {}\nTHIS IS {}\n",
        letter(),
        letter()
    );

    assert_eq!(findings(&text), [(GITHUB, 3)]);
}

#[test]
fn disable_ignores_until_enable() {
    // ng.secretlint-disable-secretlint-enable: line 3 ignored, line 7 found.
    let text = format!(
        "// secretlint-disable\n\nTHIS IS {}\n\n// secretlint-enable\n\nTHIS IS {}\n",
        letter(),
        letter()
    );

    assert_eq!(findings(&text), [(GITHUB, 7)]);
}

#[test]
fn disable_with_a_rule_id_ignores_that_rule_only() {
    // ng.secretlint-disable-target-id: the number on line 3 ignored, the letter on line 5 found,
    // and each disable-line ignoring its own rule on lines 9 and 10.
    let text = format!(
        "// secretlint-disable {NPM}\n\nTHIS IS {}\n\nTHIS IS {} <-- DETECT!!\n\n// secretlint-enable {NPM}\n\nTHIS IS {} // secretlint-disable-line {NPM} -- ignore number\nTHIS IS {} // secretlint-disable-line {GITHUB} -- ignore alphabet\n",
        number(),
        letter(),
        number(),
        letter()
    );

    assert_eq!(findings(&text), [(GITHUB, 5)]);
}

#[test]
fn every_kind_of_comment_with_a_reason_ignores_every_rule() {
    // ok.all-ignored and ok.secretlint-disable-all, with both rules.
    let text = format!(
        "// secretlint-disable -- comment\n\nTHIS IS {}\nTHIS IS {}\n\n// secretlint-enable -- comment\n\n\n// secretlint-disable-next-line -- comment\nTHIS IS {}\nTHIS IS {} // secretlint-disable-line -- comment\n",
        letter(),
        number(),
        letter(),
        number()
    );
    let unclosed = format!(
        "// secretlint-disable\n\nTHIS IS {}\nTHIS IS {}\n",
        letter(),
        number()
    );

    assert_eq!(findings(&text), []);
    assert_eq!(findings(&unclosed), []);
}

#[test]
fn html_comments_work_as_well() {
    // ok.ignore-in-markdown.
    let text = format!(
        "<!-- secretlint-disable -->\n\nTHIS IS {}\n\n<!-- secretlint-enable -->\n",
        letter()
    );

    assert_eq!(findings(&text), []);
}

#[test]
fn disable_line_on_a_last_line_without_a_line_break_ignores_nothing() {
    // The line after the last has no index: the range ends at NaN, which contains nothing.
    let text = format!("THIS IS {} // secretlint-disable-line", letter());

    assert_eq!(findings(&text), [(GITHUB, 1)]);
}

#[test]
fn disable_next_line_on_the_last_line_ignores_nothing() {
    // The next line of line 2 is line 3, past the last: nothing ignored, the token on line 1
    // still found.
    let text = format!("{}\n// secretlint-disable-next-line", letter());

    assert_eq!(findings(&text), [(GITHUB, 1)]);
}
