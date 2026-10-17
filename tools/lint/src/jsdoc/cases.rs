//! The documented cases of a rule of `eslint-plugin-jsdoc` 65.3.1 run as tests: each case of the
//! plugin's own tests (`test/rules/assertions/`, from which its documentation's examples are
//! written), with what `ESLint` 10.12.0 reported on it under the project's options and the text
//! its fixes left, recorded by EN-083 in `tests/fixtures/jsdoc/<rule>.txt` (a carriage return
//! written `\u{240d}`, so that Git keeps it), the cases neither parser reads left out.

use std::fmt::Write as _;

use crate::finding::{Finding, apply};
use crate::rules::Rule;
use crate::testing::lint;

/// A documented case and what `ESLint` gave on it.
struct Case {
    /// Its number in the fixture.
    number: usize,
    /// The code.
    code: String,
    /// Each finding, `line:column message`.
    findings: Vec<String>,
    /// The code after the fixes, when they change it.
    output: Option<String>,
}

/// The cases of a fixture's text.
fn cases(text: &str) -> Vec<Case> {
    let mut cases = Vec::new();
    for part in text.split("\n@@@ case ").skip(1) {
        let (head, rest) = part.split_once('\n').unwrap_or_default();
        let number = head
            .split(' ')
            .next()
            .and_then(|each| each.parse().ok())
            .unwrap_or_default();
        let (code, rest) = rest.split_once("\n@@@ findings\n").unwrap_or_default();
        let (findings, output) = match rest.split_once("@@@ output\n") {
            Some((findings, output)) => (findings, Some(output)),
            None => (rest, None),
        };
        let findings = findings
            .lines()
            .take_while(|line| !line.starts_with("@@@"))
            .map(|line| line.replace("\\n", "\n"))
            .collect();
        let output = output.map(|output| {
            output
                .strip_suffix("\n@@@ end\n")
                .unwrap_or(output)
                .to_owned()
        });
        cases.push(Case {
            number,
            code: code.to_owned(),
            findings,
            output,
        });
    }
    cases
}

/// The findings in `ESLint`'s order: by line, then column.
fn sorted(mut findings: Vec<Finding>) -> Vec<Finding> {
    findings.sort_by_key(|each| (each.line, each.column));
    findings
}

/// A code with its fixes applied pass after pass, ten at most, as `ESLint`'s `verifyAndFix`.
#[must_use]
pub fn fixed_in_passes(rule: &dyn Rule, path: &str, code: &str) -> String {
    let mut text = code.to_owned();
    for _ in 0..crate::FIX_PASSES {
        let findings = sorted(lint(rule, path, &text));
        let fixes: Vec<_> = findings
            .iter()
            .filter_map(|each| each.fix.as_ref())
            .collect();
        if fixes.is_empty() {
            break;
        }
        let next = apply_first_wins(&text, &fixes);
        if next == text {
            break;
        }
        text = next;
    }
    text
}

/// A text with fixes applied by range, a fix starting at or before the end of the last one
/// applied left for the next pass, as `ESLint`'s `SourceCodeFixer` leaves it.
fn apply_first_wins(text: &str, fixes: &[&crate::finding::Fix]) -> String {
    let mut kept: Vec<&crate::finding::Fix> = Vec::new();
    let mut ordered = fixes.to_vec();
    ordered.sort_by_key(|fix| (fix.start, fix.end));
    for fix in ordered {
        if kept.last().is_none_or(|last| last.end < fix.start) {
            kept.push(fix);
        }
    }
    apply(text, &kept)
}

/// Runs a rule on the documented cases of its fixture at a path.
///
/// # Panics
///
/// Listing every case whose findings or fixed text differ from `ESLint`'s, but those named
/// with a reason, and every case so named that does not differ.
pub fn check(rule: &dyn Rule, fixture: &str, path: &str, known: &[(usize, &str)]) {
    let file = format!(
        "{}/tests/fixtures/jsdoc/{fixture}.txt",
        env!("CARGO_MANIFEST_DIR")
    );
    let text = std::fs::read_to_string(&file)
        .unwrap_or_default()
        .replace('\u{240d}', "\r");
    let all = cases(&text);
    assert!(!all.is_empty(), "no case in {file}");
    let mut report = String::new();
    let mut skipped = Vec::new();

    for case in &all {
        let findings = sorted(lint(rule, path, &case.code));
        if findings.iter().any(|each| each.rule == crate::PARSE_RULE) {
            skipped.push(case.number);
            continue;
        }
        let found: Vec<String> = findings
            .iter()
            .map(|each| format!("{}:{} {}", each.line, each.column, each.message))
            .collect();
        let output = fixed_in_passes(rule, path, &case.code);
        let expected_output = case.output.clone().unwrap_or_else(|| case.code.clone());
        let same = found == case.findings && output == expected_output;
        let excused = known.iter().any(|(number, _)| *number == case.number);
        if same == excused {
            let _ = writeln!(
                report,
                "case {}{}:\n{}\n  expected {:?}\n  found    {:?}\n  expected output {:?}\n  found output    {:?}",
                case.number,
                if excused {
                    " (named as known, yet the same)"
                } else {
                    ""
                },
                case.code,
                case.findings,
                found,
                expected_output,
                output
            );
        }
    }
    assert!(
        report.is_empty(),
        "{fixture}: {} case(s) differ from ESLint (skipped, unparsed: {skipped:?})\n{report}",
        report.matches("\ncase ").count() + usize::from(report.starts_with("case "))
    );
}
