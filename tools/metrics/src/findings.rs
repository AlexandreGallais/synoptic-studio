//! The findings of the project's rules the agent met in its tool results (EN-107), the count
//! EN-093 made by hand for `the records' lint_rules`: each hook's refusal by its event and
//! reason; in a shell command's output, each lint rule's finding as `ESLint`, oxlint, the project's
//! linter and markdownlint report it, each TypeScript error code, each Clippy lint and each check
//! `check:all` failed. The metrics report counts them per epic, so that the next review of the
//! rules reads them instead of searching the transcripts.

use std::collections::HashMap;

use crate::format::grouped;
use crate::tables::markdown_table;

/// A rule's findings in one tool result.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Finding {
    /// The result's time, epoch milliseconds.
    pub time: i64,
    /// The result's Git branch.
    pub branch: String,
    /// What reported it: `hook`, `lint`, `type`, `clippy` or `check`.
    pub kind: String,
    /// The rule: `PreToolUse:Bash: <reason>`, `unicorn/no-null`, `TS2322`, `clippy::too_many_lines`,
    /// `test`.
    pub rule: String,
    /// How many times the result reports it.
    pub count: i64,
}

/// What follows a hook's event and tool in the error Claude Code returns for a refusal:
/// `PreToolUse:Bash hook error: <its message>`.
const HOOK_ERROR: &str = " hook error: ";

/// Characters of a hook's reason kept.
const REASON_SHOWN: usize = 100;

/// A text without its terminal styles (`ESC [ … m` and the other control sequences).
#[must_use]
pub fn plain(text: &str) -> String {
    let mut plain = String::with_capacity(text.len());
    let mut characters = text.chars().peekable();

    while let Some(character) = characters.next() {
        if character == '\u{1b}' && characters.peek() == Some(&'[') {
            characters.next();
            for code in characters.by_ref() {
                if code.is_ascii_alphabetic() {
                    break;
                }
            }
        } else {
            plain.push(character);
        }
    }
    plain
}

/// The rule of a hook's refusal: its event and tool, then the first sentence or clause of its
/// message, up to `: `, `. ` or the line's end; `None` when the result is not a refusal.
#[must_use]
pub fn hook_refusal(text: &str) -> Option<String> {
    let at = text.find(HOOK_ERROR)?;
    let hook = &text[..at];

    if hook.is_empty() || !hook.contains(':') || hook.contains(char::is_whitespace) {
        return None;
    }

    let message = &text[at + HOOK_ERROR.len()..];
    let end = [": ", ". ", "\n"]
        .iter()
        .filter_map(|stop| message.find(stop))
        .min()
        .unwrap_or(message.len());
    // Without its code quotes, which would end the table's code span.
    let reason: String = message[..end]
        .trim()
        .trim_end_matches('.')
        .chars()
        .filter(|character| *character != '`')
        .take(REASON_SHOWN)
        .collect();

    Some(format!("{hook}: {reason}"))
}

/// Whether a text is a rule's name: a letter or `@` first, then letters, digits, `@`, `/`, `_`,
/// `.` or `-`.
fn is_rule_name(text: &str) -> bool {
    text.starts_with(|first: char| first.is_ascii_alphabetic() || first == '@')
        && text
            .chars()
            .all(|character| character.is_ascii_alphanumeric() || "@/_.-".contains(character))
}

/// An oxlint rule written `plugin(rule)` or `plugin(rule):`, named as `.oxlintrc.json` names it:
/// `unicorn/no-null`, `typescript/no-explicit-any`, `id-length` for `ESLint`'s own.
fn oxlint_rule(token: &str) -> Option<String> {
    let inner = token.strip_suffix(':').unwrap_or(token).strip_suffix(')')?;
    let (plugin, rule) = inner.split_once('(')?;

    if !is_rule_name(plugin) || !is_rule_name(rule) {
        return None;
    }

    let plugin = plugin.strip_prefix("eslint-plugin-").unwrap_or(plugin);

    Some(match plugin {
        "eslint" => rule.to_owned(),
        "typescript-eslint" => format!("typescript/{rule}"),
        _ => format!("{plugin}/{rule}"),
    })
}

/// Whether a token is a place written `<path>:<line>:<column>`, a colon after or not.
fn is_place(token: &str) -> bool {
    let mut parts = token.strip_suffix(':').unwrap_or(token).rsplitn(3, ':');
    let numbers = [parts.next(), parts.next()];

    numbers.iter().all(|part| {
        part.is_some_and(|digits| !digits.is_empty() && digits.bytes().all(|b| b.is_ascii_digit()))
    }) && parts.next().is_some_and(|path| !path.is_empty())
}

/// The lint rule a line written `<path>:<line>:<column>` reports: oxlint's
/// `path:1:2: error plugin(rule): …`, markdownlint's `path:1:2 error MD013/line-length …`, the
/// project linter's `path:1:2 unicorn/no-null …`.
fn placed_rule(tokens: &[&str]) -> Option<String> {
    let place = tokens.first().filter(|token| is_place(token))?;
    let has_level = matches!(tokens.get(1), Some(&("error" | "warning")));
    let rule = tokens.get(if has_level { 2 } else { 1 })?;

    if let Some(rule) = oxlint_rule(rule) {
        return has_level.then_some(rule);
    }

    // Without a level, only the project linter's form, its place without a colon after.
    (rule.contains('/') && is_rule_name(rule) && (has_level || !place.ends_with(':')))
        .then(|| (*rule).to_owned())
}

/// The lint rule a line of `ESLint`'s report names: `  12:5  error  Message  rule-name`.
fn eslint_rule(line: &str, tokens: &[&str]) -> Option<String> {
    let place = tokens.first()?;
    let rule = tokens.last()?;
    let is_place = place.split_once(':').is_some_and(|(row, column)| {
        [row, column]
            .iter()
            .all(|part| !part.is_empty() && part.bytes().all(|b| b.is_ascii_digit()))
    });

    (line.starts_with(char::is_whitespace)
        && is_place
        && matches!(tokens.get(1), Some(&("error" | "warning")))
        && tokens.len() >= 4
        && is_rule_name(rule)
        && !rule.contains(|c: char| c.is_ascii_uppercase())
        && line.trim_end().ends_with(&format!("  {rule}")))
    .then(|| (*rule).to_owned())
}

/// The TypeScript error code a line names: `error TS2322:`.
fn type_error(line: &str) -> Option<String> {
    line.match_indices("error TS").find_map(|(at, found)| {
        let digits: String = line[at + found.len()..]
            .chars()
            .take_while(char::is_ascii_digit)
            .collect();

        (!digits.is_empty() && line[at + found.len() + digits.len()..].starts_with(':'))
            .then(|| format!("TS{digits}"))
    })
}

/// The Clippy lint a line's link names: `…/rust-clippy/…/index.html#too_many_lines`.
fn clippy_lint(line: &str) -> Option<String> {
    let at = line.find("rust-clippy/")?;
    let anchor = line[at..].split_once("index.html#")?.1;
    let name: String = anchor
        .chars()
        .take_while(|c| c.is_ascii_alphanumeric() || *c == '_')
        .collect();

    (!name.is_empty()).then(|| format!("clippy::{name}"))
}

/// The check a line of `check:all` says failed: `✗ test 26.2 s`.
fn failed_check(tokens: &[&str]) -> Option<String> {
    match tokens {
        ["✗", name, seconds, "s"] if seconds.parse::<f64>().is_ok() => Some((*name).to_owned()),
        _ => None,
    }
}

/// The finding a line of a command's output reports, as its kind and rule.
#[must_use]
pub fn line_finding(line: &str) -> Option<(&'static str, String)> {
    let tokens: Vec<&str> = line.split_whitespace().collect();

    if let Some(rule) = type_error(line) {
        return Some(("type", rule));
    }
    if let Some(lint) = clippy_lint(line) {
        return Some(("clippy", lint));
    }
    if let Some(check) = failed_check(&tokens) {
        return Some(("check", check));
    }
    if let (Some(&("×" | "⚠" | "!")), Some(rule)) = (tokens.first(), tokens.get(1)) {
        return oxlint_rule(rule).map(|rule| ("lint", rule));
    }

    placed_rule(&tokens)
        .or_else(|| eslint_rule(line, &tokens))
        .map(|rule| ("lint", rule))
}

/// The findings of one tool result, each rule once with its count, in the order first met: a
/// refusal of a hook, whatever the tool; the lines of a shell command's output.
#[must_use]
pub fn result_findings(text: &str, is_error: bool, is_shell: bool) -> Vec<(String, String, i64)> {
    if is_error && let Some(rule) = hook_refusal(text) {
        return vec![("hook".to_owned(), rule, 1)];
    }
    if !is_shell {
        return Vec::new();
    }

    let mut found: Vec<(String, String, i64)> = Vec::new();

    for line in plain(text).lines() {
        if let Some((kind, rule)) = line_finding(line) {
            match found
                .iter_mut()
                .find(|(known, name, _)| known == kind && *name == rule)
            {
                Some(entry) => entry.2 += 1,
                None => found.push((kind.to_owned(), rule, 1)),
            }
        }
    }
    found
}

/// A row of the findings' table: the kind, the rule, its findings, and those of each epic.
type Row<'a> = (&'a str, &'a str, i64, Vec<(&'a str, i64)>);

/// The table of the findings: each rule with its kind, its count and its count per epic, the most
/// found first, then by kind and rule; the epics of a rule by count, then by name.
#[must_use]
pub fn findings_table(findings: &[(String, String, String, i64)]) -> String {
    let mut rules: HashMap<(&str, &str), HashMap<&str, i64>> = HashMap::new();

    for (epic, kind, rule, count) in findings {
        *rules
            .entry((kind, rule))
            .or_default()
            .entry(epic)
            .or_default() += count;
    }

    let mut rows: Vec<Row<'_>> = rules
        .into_iter()
        .map(|((kind, rule), epics)| {
            let mut epics: Vec<(&str, i64)> = epics.into_iter().collect();

            epics.sort_by(|left, right| right.1.cmp(&left.1).then(left.0.cmp(right.0)));
            (
                kind,
                rule,
                epics.iter().map(|(_, count)| count).sum(),
                epics,
            )
        })
        .collect();

    rows.sort_by(|left, right| {
        right
            .2
            .cmp(&left.2)
            .then(left.0.cmp(right.0))
            .then(left.1.cmp(right.1))
    });

    markdown_table(
        &["Kind", "Rule", "Findings", "Per epic"],
        &rows
            .iter()
            .map(|(kind, rule, total, epics)| {
                vec![
                    (*kind).to_owned(),
                    format!("`{rule}`"),
                    grouped(*total),
                    epics
                        .iter()
                        .map(|(epic, count)| format!("{epic} {}", grouped(*count)))
                        .collect::<Vec<_>>()
                        .join(" · "),
                ]
            })
            .collect::<Vec<_>>(),
    )
}
