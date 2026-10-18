//! The report of a scan, as `secretlint` 13.0.7 prints it by default (its `stylish` format, colours
//! and table alignment left out): each file with findings, then one line per finding — its line
//! and column, `error`, the message id, the message with every value masked, the preset and the
//! rule — then the count. A value is masked as `@secretlint/core` 13.0.7 masks it: each of the
//! finding's values, in turn, replaced everywhere in the message by as many `*` as it has UTF-16
//! code units, so that the report never shows a secret.

use std::fmt::Write as _;

use crate::js::utf16_len;
use crate::rules::PRESET;
use crate::source::{Finding, position};

/// A finding of one file, its position read.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Located {
    /// The finding.
    pub finding: Finding,
    /// Its line, from 1.
    pub line: usize,
    /// Its column, from 0, in UTF-16 code units.
    pub column: usize,
}

/// A finding's message, each of its values masked.
#[must_use]
pub fn masked(finding: &Finding) -> String {
    finding
        .data
        .iter()
        .filter(|value| !value.is_empty())
        .fold(finding.message.clone(), |message, value| {
            message.replace(value.as_str(), &"*".repeat(utf16_len(value)))
        })
}

/// The findings of a file with their positions, given the starts of its lines.
#[must_use]
pub fn locate(findings: Vec<Finding>, line_starts: &[usize]) -> Vec<Located> {
    findings
        .into_iter()
        .map(|finding| {
            let (line, column) = position(line_starts, finding.range.0);

            Located {
                finding,
                line,
                column,
            }
        })
        .collect()
}

/// The report of every file's findings, empty when there is none.
#[must_use]
pub fn format(files: &[(String, Vec<Located>)]) -> String {
    let mut report = String::new();
    let mut count = 0;

    for (path, findings) in files.iter().filter(|(_, findings)| !findings.is_empty()) {
        let _ = writeln!(report, "{path}");
        for located in findings {
            let finding = &located.finding;

            count += 1;
            let _ = writeln!(
                report,
                "  {}:{}  error  [{}] {}  {PRESET} > {}",
                located.line,
                located.column,
                finding.message_id,
                masked(finding),
                finding.rule_id
            );
        }
        report.push('\n');
    }
    if count > 0 {
        let plural = if count == 1 { "" } else { "s" };
        let _ = writeln!(
            report,
            "✖ {count} problem{plural} ({count} error{plural}, 0 warnings, 0 infos)"
        );
    }
    report
}

#[cfg(test)]
mod tests {
    use super::*;

    fn finding(message: &str, data: &[&str]) -> Finding {
        Finding {
            rule_id: "@secretlint/secretlint-rule-github",
            message_id: "GITHUB_TOKEN",
            message: message.to_owned(),
            data: data.iter().map(|value| (*value).to_owned()).collect(),
            range: (4, 8),
        }
    }

    #[test]
    fn every_value_is_masked_by_its_utf16_length() {
        // "tok" 3 units, "é😀" 3 units (1 and 2); the empty value masks nothing.
        let finding = finding("found tok in é😀, tok again", &["tok", "é😀", ""]);

        assert_eq!(masked(&finding), "found *** in ***, *** again");
    }

    #[test]
    fn the_report_names_the_file_the_line_and_the_rule_never_the_secret() {
        // The finding starts at index 4, on line 2 (starting at 3), column 1.
        let located = locate(
            vec![finding("found GitHub Token: abcd", &["abcd"])],
            &[0, 3],
        );
        let report = format(&[
            ("a.txt".to_owned(), located),
            ("b.txt".to_owned(), Vec::new()),
        ]);

        assert_eq!(
            report,
            "a.txt\n  2:1  error  [GITHUB_TOKEN] found GitHub Token: ****  @secretlint/secretlint-rule-preset-recommend > @secretlint/secretlint-rule-github\n\n✖ 1 problem (1 error, 0 warnings, 0 infos)\n"
        );
        assert!(!report.contains("abcd"));
        assert_eq!(format(&[("b.txt".to_owned(), Vec::new())]), "");
    }

    #[test]
    fn several_findings_are_counted_in_the_plural() {
        let located = locate(vec![finding("x", &[]), finding("y", &[])], &[0]);

        assert!(
            format(&[("a".to_owned(), located)])
                .ends_with("✖ 2 problems (2 errors, 0 warnings, 0 infos)\n")
        );
    }
}
