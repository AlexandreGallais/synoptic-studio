//! The commit message checked as commitlint 21.2.3 checks it with `@commitlint/config-conventional`
//! and the project's rules of `commitlint.config.ts` (EN-108, `REF-COMMITLINT`).
//!
//! Ported from `@commitlint/cli` 21.2.3 (MIT, Copyright (c) 2016 - present Mario Nebl): `lint`,
//! `is-ignored` (its defaults), `ensure` (`max-line-length` and its URL exemption), `rules` (one
//! function per rule below, each with its source file), `parse`, and `config-conventional`'s rule
//! levels; the parser follows `conventional-commits-parser` 7.1.3 (MIT, Copyright © the
//! conventional-changelog team) with the options of `conventional-changelog-conventionalcommits`
//! 10.4.1 (ISC, the same holders): its header patterns and note keywords. The ignored versions
//! follow `semver` 7.8.5 (ISC, Copyright (c) Isaac Z. Schlueter and Contributors), `valid`. Lengths
//! are counted in UTF-16 units, as JavaScript's `length`.

use std::fmt::Write as _;
use std::sync::LazyLock;

use regex::Regex;

use crate::case::{Case, is_in_case, starts_with_cased_letter};

/// The project's scopes: one per layer of the library, plus the areas of the project. The one list
/// the commit-msg hook and the CI check (`docs/tooling/git-workflow.md`, Commit messages).
pub const SCOPES: [&str; 14] = [
    "adr",
    "backlog",
    "ci",
    "deps",
    "docs",
    "geometry",
    "interaction",
    "io",
    "math",
    "model",
    "playground",
    "render",
    "routing",
    "tooling",
];

/// `config-conventional`'s types.
pub const TYPES: [&str; 11] = [
    "build", "chore", "ci", "docs", "feat", "fix", "perf", "refactor", "revert", "style", "test",
];

/// The longest header, body line and footer line, in UTF-16 units.
pub const MAX_LINE_LENGTH: usize = 100;

/// The cases a subject may not be in.
pub const SUBJECT_CASES: [Case; 4] = [Case::Sentence, Case::Start, Case::Pascal, Case::Upper];

/// JavaScript's `.`: any character but a line terminator.
const DOT: &str = r"[^\n\r\x{2028}\x{2029}]";

/// A pattern of the port, compiled once.
fn pattern(text: &str) -> Regex {
    Regex::new(&text.replace("DOT", DOT)).unwrap_or_else(|_| unreachable!("a valid pattern"))
}

/// The breaking header, `type(scope)!: subject`.
static BREAKING_HEADER: LazyLock<Regex> =
    LazyLock::new(|| pattern(r"^([A-Za-z0-9_]*)(?:\((DOT*)\))?!: (DOT*)$"));
/// The header, `type(scope): subject`, `!` allowed.
static HEADER: LazyLock<Regex> =
    LazyLock::new(|| pattern(r"^([A-Za-z0-9_]*)(?:\((DOT*)\))?!?: (DOT*)$"));
/// A note, `BREAKING CHANGE: …`.
static NOTE: LazyLock<Regex> =
    LazyLock::new(|| pattern(r"(?i)^(?:\*\s+)?(BREAKING CHANGE|BREAKING-CHANGE):\s*(DOT*)"));
/// A footer's token at a line's start, `Refs: …` or `Fixes #1` (the Git trailer convention).
static FOOTER_TOKEN: LazyLock<Regex> =
    LazyLock::new(|| pattern(r"(?i)^(?:BREAKING CHANGE|[A-Za-z0-9_-]+)(?::\s+|\s+(?:#))DOT+"));
/// A line holding a URL, exempt from the line length (`max-line-length.js`).
static URL: LazyLock<Regex> = LazyLock::new(|| pattern(r"(?-u:\b)https?://\S+"));
/// A GPG line, left out of the parsed message.
static GPG: LazyLock<Regex> = LazyLock::new(|| pattern(r"^\s*gpg:"));
/// The scope's delimiters: `/`, `\` and `, `.
static SCOPE_DELIMITER: LazyLock<Regex> = LazyLock::new(|| pattern(r"/|\\|, ?"));

/// commitlint's ignored messages (`is-ignored/defaults.js`), the version tested apart.
static IGNORED: LazyLock<[Regex; 9]> = LazyLock::new(|| {
    [
        pattern(
            r"(?mR)^((Merge pull request)|(Merge (DOT*?) into (DOT*?)|(Merge branch (DOT*?)))(?:\r?\n)*$)",
        ),
        pattern(r"(?mR)^(Merge tag (DOT*?))(?:\r?\n)*$"),
        pattern(r"^(R|r)evert (DOT*)"),
        pattern(r"^(R|r)eapply (DOT*)"),
        pattern(r"^(amend|fixup|squash)!"),
        pattern(r"^(Merged (DOT*?)(in|into) (DOT*)|Merged PR (DOT*): (DOT*))"),
        pattern(r"^Merge remote-tracking branch(\s*)(DOT*)"),
        pattern(r"^Automatic merge(DOT*)"),
        pattern(r"^Auto-merged (DOT*?) into (DOT*)"),
    ]
});
/// The `chore(…):` before a version (`isSemver`).
static CHORE_PREFIX: LazyLock<Regex> = LazyLock::new(|| pattern(r"^chore(\([^)]+\))?:"));
/// A CI skip marker in brackets.
static SKIP_BRACKETS: LazyLock<Regex> =
    LazyLock::new(|| pattern(r"(?i)\[(skip|ci)(-|\s)(ci|skip)\]"));
/// A CI skip marker in parentheses.
static SKIP_PARENTHESES: LazyLock<Regex> =
    LazyLock::new(|| pattern(r"(?i)\((skip|ci)(-|\s)(ci|skip)\)"));
/// A valid version, node-semver's `FULL`: Semantic Versioning 2.0.0, a `v` allowed first.
static VERSION: LazyLock<Regex> = LazyLock::new(|| {
    let number = "(0|[1-9][0-9]*)";
    let identifier = "(?:0|[1-9][0-9]*|[0-9]*[a-zA-Z-][a-zA-Z0-9-]*)";

    pattern(&format!(
        r"^v?{number}\.{number}\.{number}(?:-{identifier}(?:\.{identifier})*)?(?:\+[a-zA-Z0-9-]+(?:\.[a-zA-Z0-9-]+)*)?$"
    ))
});
/// The length of a text as JavaScript counts it, in UTF-16 units.
fn length(text: &str) -> usize {
    text.encode_utf16().count()
}

/// JavaScript's `trim()`, `trimStart()` and `trimEnd()` also remove U+FEFF.
fn is_js_space(character: char) -> bool {
    character.is_whitespace() || character == '\u{feff}'
}

/// The lines of a text split at `\r?\n` (`to-lines`).
fn lines(text: &str) -> Vec<&str> {
    text.split('\n')
        .map(|line| line.strip_suffix('\r').unwrap_or(line))
        .collect()
}

/// A commit message as the parser reads it.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Commit {
    /// The message as given.
    pub raw: String,
    /// Its first line, `None` when empty.
    pub header: Option<String>,
    /// The header's type, `None` when empty or not matched.
    pub kind: Option<String>,
    /// The header's scope.
    pub scope: Option<String>,
    /// The header's subject.
    pub subject: Option<String>,
    /// The lines before the first footer.
    pub body: Option<String>,
    /// The footers, from the first line holding a token or a note.
    pub footer: Option<String>,
}

/// `appendLine`: a line added after a text, the text dropped when empty.
fn append(text: Option<String>, line: &str) -> String {
    match text.filter(|text| !text.is_empty()) {
        Some(text) => format!("{text}\n{line}"),
        None => line.to_owned(),
    }
}

/// `trimNewLines`: the line feeds and carriage returns around a text removed.
fn trim_new_lines(text: &str) -> String {
    text.trim_matches(['\r', '\n']).to_owned()
}

/// The parser's walk through the lines after the header.
struct Walk<'a> {
    lines: Vec<&'a str>,
    index: usize,
    body: Option<String>,
    footer: Option<String>,
}

impl<'a> Walk<'a> {
    fn current(&self) -> Option<&'a str> {
        self.lines.get(self.index).copied()
    }

    /// `parseNotes`: a note line and the lines after it, up to a footer token, go to the footer.
    fn notes(&mut self) -> bool {
        let Some(line) = self.current() else {
            return false;
        };

        if !NOTE.is_match(line) {
            return false;
        }

        self.footer = Some(append(self.footer.take(), line));
        self.index += 1;

        while self.current().is_some() {
            if self.notes() {
                return true;
            }

            let Some(line) = self.current() else {
                break;
            };
            let is_token = FOOTER_TOKEN.is_match(line);

            self.footer = Some(append(self.footer.take(), line));
            self.index += 1;

            if is_token {
                break;
            }
        }

        true
    }

    /// `parseBodyAndFooter`: a line goes to the body until the first footer token.
    fn body_or_footer(&mut self, is_body: bool) -> bool {
        let Some(line) = self.current() else {
            return is_body;
        };
        let still_body = is_body && !FOOTER_TOKEN.is_match(line);

        if still_body {
            self.body = Some(append(self.body.take(), line));
        } else {
            self.footer = Some(append(self.footer.take(), line));
        }

        self.index += 1;
        still_body
    }
}

/// The text of a regular expression's group, `None` when missing or empty.
fn group(captures: &regex::Captures<'_>, index: usize) -> Option<String> {
    captures
        .get(index)
        .map(|found| found.as_str().to_owned())
        .filter(|text| !text.is_empty())
}

/// Parses a message (`CommitParser.parse`), the lines starting with the comment character left out
/// when one is given, and those after its scissors line; `None` for a blank message.
#[must_use]
pub fn parse(message: &str, comment: Option<&str>) -> Option<Commit> {
    if message.trim_matches(is_js_space).is_empty() {
        return None;
    }

    let trimmed = trim_new_lines(message);
    let raw_lines = lines(&trimmed);
    let kept: Vec<&str> = match comment {
        Some(character) => {
            let scissors =
                format!("{character} ------------------------ >8 ------------------------");
            let end = raw_lines
                .iter()
                .position(|line| *line == scissors)
                .unwrap_or(raw_lines.len());

            raw_lines[..end]
                .iter()
                .filter(|line| !line.starts_with(character) && !GPG.is_match(line))
                .copied()
                .collect()
        }
        None => raw_lines
            .iter()
            .filter(|line| !GPG.is_match(line))
            .copied()
            .collect(),
    };
    let header = kept
        .first()
        .filter(|line| !line.is_empty())
        .map(|line| (*line).to_owned());
    let mut commit = Commit {
        raw: message.to_owned(),
        header: header.clone(),
        ..Commit::default()
    };

    if let Some(header) = &header
        && let Some(captures) = BREAKING_HEADER
            .captures(header)
            .or_else(|| HEADER.captures(header))
    {
        commit.kind = group(&captures, 1);
        commit.scope = group(&captures, 2);
        commit.subject = group(&captures, 3);
    }

    let mut walk = Walk {
        lines: kept,
        index: 1,
        body: None,
        footer: None,
    };
    let mut is_body = true;

    while walk.current().is_some() {
        if walk.notes() {
            is_body = false;
        }

        if !walk.body_or_footer(is_body) {
            is_body = false;
        }
    }

    commit.body = walk
        .body
        .filter(|body| !body.is_empty())
        .map(|body| trim_new_lines(&body));
    commit.footer = walk
        .footer
        .filter(|footer| !footer.is_empty())
        .map(|footer| trim_new_lines(&footer));
    Some(commit)
}

/// Whether a version is valid, as node-semver's `valid` reads it.
fn is_version(text: &str) -> bool {
    length(text) <= 256
        && VERSION.captures(text).is_some_and(|captures| {
            (1..=3).all(|index| {
                captures.get(index).is_some_and(|number| {
                    number.as_str().parse::<u64>().is_ok_and(|n| n < (1 << 53))
                })
            })
        })
}

/// Whether the first line, a `chore(…):` and a CI skip marker removed, is a version (`isSemver`).
fn is_version_commit(message: &str) -> bool {
    let first = message.split('\n').next().unwrap_or_default();
    let stripped = CHORE_PREFIX.replace(first, "");
    let stripped = SKIP_BRACKETS.replace(&stripped, "");
    let stripped = SKIP_PARENTHESES.replace(&stripped, "");

    is_version(stripped.trim_matches(is_js_space))
}

/// Whether commitlint skips a message: its default ignores, then the project's (Dependabot's; the
/// release commit `chore: release x.y.z` of `tools/release` is checked as any other, EN-112), the
/// message's end trimmed first.
#[must_use]
pub fn is_ignored(message: &str) -> bool {
    let message = message.trim_end_matches(is_js_space);

    IGNORED.iter().any(|ignored| ignored.is_match(message))
        || is_version_commit(message)
        || message.contains("Signed-off-by: dependabot[bot]")
}

/// A rule's finding: its name and message.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Finding {
    /// The rule, `subject-case`.
    pub rule: &'static str,
    /// What it asks, as commitlint words it.
    pub message: String,
}

/// What the rules found in a message: errors refuse it, warnings are printed.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Report {
    /// The header, to name the input.
    pub header: String,
    /// The errors.
    pub errors: Vec<Finding>,
    /// The warnings.
    pub warnings: Vec<Finding>,
}

/// `body-leading-blank.js`: a body starts after a blank line.
#[must_use]
pub fn body_leading_blank(commit: &Commit) -> bool {
    commit.body.is_none() || lines(&commit.raw).get(1) == Some(&"")
}

/// `footer-leading-blank.js`: the footer starts after a blank line, the line before the first
/// raw line equal to the footer's first, read as JavaScript's `slice(index - 1)[0]`.
#[must_use]
pub fn footer_leading_blank(commit: &Commit) -> bool {
    let Some(footer) = &commit.footer else {
        return true;
    };
    let raw = lines(&commit.raw);
    let first = lines(footer).first().copied().unwrap_or_default();
    let offset = raw
        .iter()
        .position(|line| *line == first)
        .map_or(-1, |index| i64::try_from(index).unwrap_or(i64::MAX));
    let start = offset - 1;
    let count = i64::try_from(raw.len()).unwrap_or(i64::MAX);
    let from = if start < 0 {
        (count + start).max(0)
    } else {
        start
    };

    usize::try_from(from).ok().and_then(|index| raw.get(index)) == Some(&"")
}

/// `max-line-length.js`: every line within the length, a line holding a URL exempt.
#[must_use]
pub fn max_line_length(text: &str, max: usize) -> bool {
    lines(text)
        .iter()
        .all(|line| URL.is_match(line) || length(line) <= max)
}

/// `header-max-length.js`: a header, within the length; a missing header fails.
#[must_use]
pub fn header_within(commit: &Commit, max: usize) -> bool {
    commit
        .header
        .as_deref()
        .is_some_and(|header| length(header) <= max)
}

/// `header-trim.js`: the header neither starts nor ends with white space; its message otherwise.
#[must_use]
pub fn header_trim(commit: &Commit) -> Option<&'static str> {
    let header = commit.header.as_deref()?;
    let starts = header.starts_with(is_js_space);
    let ends = header.ends_with(is_js_space);

    match (starts, ends) {
        (true, true) => Some("header must not be surrounded by whitespace"),
        (true, false) => Some("header must not start with whitespace"),
        (false, true) => Some("header must not end with whitespace"),
        (false, false) => None,
    }
}

/// `subject-case.js` with `never`: the cases the subject is in, among those given; none when it
/// does not start with a cased letter.
#[must_use]
pub fn subject_cases(commit: &Commit, cases: &[Case]) -> Vec<Case> {
    match commit.subject.as_deref() {
        Some(subject) if starts_with_cased_letter(subject) => cases
            .iter()
            .copied()
            .filter(|&case| is_in_case(subject, case))
            .collect(),
        _ => Vec::new(),
    }
}

/// `subject-full-stop.js` with `never .`: whether the header ends with a full stop, an ellipsis
/// and a header ending at its colon excepted.
#[must_use]
pub fn ends_with_full_stop(commit: &Commit) -> bool {
    let Some(header) = commit.header.as_deref() else {
        return false;
    };
    let units: Vec<u16> = header.encode_utf16().collect();
    let colon = units.iter().position(|&unit| unit == u16::from(b':'));

    if colon.is_some_and(|at| at > 0 && at == units.len() - 1) {
        return false;
    }

    header.ends_with('.') && !header.ends_with("...")
}

/// `scope-enum.js` with `always`: each part of the scope, or the whole scope, in the list.
#[must_use]
pub fn scope_in(scope: &str, scopes: &[&str]) -> bool {
    SCOPE_DELIMITER
        .split(scope)
        .all(|part| scopes.contains(&part))
        || scopes.contains(&scope)
}

/// A list as commitlint writes it in a message, `[a, b]`.
fn listed(items: &[&str]) -> String {
    format!("[{}]", items.join(", "))
}

/// Lints a message as `commitlint --edit` or `--from … --to …` does with the project's
/// configuration; `None` when it is ignored or blank.
#[must_use]
pub fn lint(message: &str, comment: Option<&str>) -> Option<Report> {
    if is_ignored(message) {
        return None;
    }

    let commit = parse(message, comment)?;

    if commit.header.is_none() && commit.body.is_none() && commit.footer.is_none() {
        return None;
    }

    Some(Report {
        header: commit.header.clone().unwrap_or_default(),
        errors: errors_of(&commit),
        warnings: warnings_of(&commit),
    })
}

/// The errors of a parsed message, each rule of `config-conventional` in its order, `scope-enum`
/// the project's own.
fn errors_of(commit: &Commit) -> Vec<Finding> {
    let mut errors = Vec::new();
    let mut error = |rule: &'static str, message: String| {
        errors.push(Finding { rule, message });
    };

    if commit
        .body
        .as_deref()
        .is_some_and(|body| !max_line_length(body, MAX_LINE_LENGTH))
    {
        error(
            "body-max-line-length",
            format!("body's lines must not be longer than {MAX_LINE_LENGTH} characters"),
        );
    }

    if commit
        .footer
        .as_deref()
        .is_some_and(|footer| !max_line_length(footer, MAX_LINE_LENGTH))
    {
        error(
            "footer-max-line-length",
            format!("footer's lines must not be longer than {MAX_LINE_LENGTH} characters"),
        );
    }

    let header_length = commit.header.as_deref().map(length);

    if !header_within(commit, MAX_LINE_LENGTH) {
        error(
            "header-max-length",
            format!(
                "header must not be longer than {MAX_LINE_LENGTH} characters, current length is {}",
                header_length.map_or_else(|| "undefined".to_owned(), |count| count.to_string())
            ),
        );
    }

    if let Some(message) = header_trim(commit) {
        error("header-trim", message.to_owned());
    }

    let cases = subject_cases(commit, &SUBJECT_CASES);

    if !cases.is_empty() {
        let names: Vec<&str> = cases.iter().map(|case| case.name()).collect();

        error(
            "subject-case",
            format!("subject must not be {}", names.join(", ")),
        );
    }

    if commit.subject.is_none() {
        error("subject-empty", "subject may not be empty".to_owned());
    }

    if ends_with_full_stop(commit) {
        error(
            "subject-full-stop",
            "subject may not end with full stop".to_owned(),
        );
    }

    if commit
        .kind
        .as_deref()
        .is_some_and(|kind| !is_in_case(kind, Case::Lower))
    {
        error("type-case", "type must be lower-case".to_owned());
    }

    if commit.kind.is_none() {
        error("type-empty", "type may not be empty".to_owned());
    }

    if commit
        .kind
        .as_deref()
        .is_some_and(|kind| !TYPES.contains(&kind))
    {
        error(
            "type-enum",
            format!("type must be one of {}", listed(&TYPES)),
        );
    }

    if commit
        .scope
        .as_deref()
        .is_some_and(|scope| !scope_in(scope, &SCOPES))
    {
        error(
            "scope-enum",
            format!("scope must be one of {}", listed(&SCOPES)),
        );
    }

    errors
}

/// The warnings of a parsed message: printed, the commit passes.
fn warnings_of(commit: &Commit) -> Vec<Finding> {
    let mut warnings = Vec::new();

    if !body_leading_blank(commit) {
        warnings.push(Finding {
            rule: "body-leading-blank",
            message: "body must have leading blank line".to_owned(),
        });
    }

    if !footer_leading_blank(commit) {
        warnings.push(Finding {
            rule: "footer-leading-blank",
            message: "footer must have leading blank line".to_owned(),
        });
    }

    warnings
}

/// A report as commitlint's default formatter prints it, empty when there is nothing to say.
#[must_use]
pub fn format_report(report: &Report) -> String {
    if report.errors.is_empty() && report.warnings.is_empty() {
        return String::new();
    }

    let mut text = format!("⧗   input: {}\n", report.header);

    for finding in &report.errors {
        let _ = writeln!(text, "✖   {} [{}]", finding.message, finding.rule);
    }

    for finding in &report.warnings {
        let _ = writeln!(text, "⚠   {} [{}]", finding.message, finding.rule);
    }

    let sign = if report.errors.is_empty() {
        "⚠"
    } else {
        "✖"
    };

    let _ = write!(
        text,
        "\n{sign}   found {} problems, {} warnings\n\
         ⓘ   The rules: docs/tooling/git-workflow.md, Commit messages\n",
        report.errors.len(),
        report.warnings.len()
    );
    text
}
