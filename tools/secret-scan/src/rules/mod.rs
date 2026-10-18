//! The 27 detecting rules of `@secretlint/secretlint-rule-preset-recommend` 13.0.7 and its comment
//! filter, ported from its bundle `module/index.js` (`REF-SECRETLINT-PRESET`), each module naming
//! its rule; their cases are the preset's own snapshots (`tests/preset.rs`). They run in the
//! preset's order over one file, each where its content types allow: every rule on a text file,
//! the GCP rule alone on a binary one.
//!
//! Each rule takes its options' defaults, which the repository never changed: every `allows` list
//! empty, so that no match is allowed, and the AWS rule's `enableIDScanRule` false, so that its
//! patterns of access key IDs and account IDs do not run. [`Options`] carries the second, for the
//! preset's cases that turn it on.

mod aws;
mod basic_auth;
mod database;
mod gcp;
mod npm;
mod one_password;
mod private_key;
mod tokens;

use crate::comments;
use crate::source::{Finding, Source};

/// The id of the preset, which secretlint's report prints before each rule's.
pub const PRESET: &str = "@secretlint/secretlint-rule-preset-recommend";

/// The options of the preset's rules that the scan can set.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Options {
    /// The AWS rule's `enableIDScanRule`: its access key ID and account ID patterns run too.
    pub aws_id_scan: bool,
}

/// What every rule reports in a file, in the preset's order, the comment filter applied, the
/// duplicates removed and the findings sorted by their range, as `@secretlint/core` 13.0.7's
/// `cleanupMessages` leaves them.
#[must_use]
pub fn scan(source: &Source<'_>, options: Options) -> Vec<Finding> {
    let mut findings = Vec::new();

    if source.is_text {
        aws::report(source, options, &mut findings);
    }
    gcp::report(source, &mut findings);
    if source.is_text {
        private_key::report(source, &mut findings);
        npm::report(source, &mut findings);
        basic_auth::report(source, &mut findings);
        tokens::report(source, &mut findings);
        one_password::report(source, &mut findings);
        database::report(source, &mut findings);
        tokens::report_after_database(source, &mut findings);
    }
    let ignored = if source.is_text {
        comments::ignored_ranges(source)
    } else {
        Vec::new()
    };
    let kept: Vec<Finding> = findings
        .into_iter()
        .filter(|finding| !comments::is_ignored(finding, &ignored))
        .collect();
    let mut unique: Vec<Finding> = kept
        .iter()
        .enumerate()
        .filter(|(index, finding)| {
            !kept[index + 1..]
                .iter()
                .any(|later| later.range == finding.range && later.message == finding.message)
        })
        .map(|(_, finding)| finding.clone())
        .collect();

    unique.sort_by_key(|finding| finding.range);
    unique
}

/// A finding of a rule.
fn finding(
    rule_id: &'static str,
    message_id: &'static str,
    message: String,
    data: Vec<String>,
    range: (usize, usize),
) -> Finding {
    Finding {
        rule_id,
        message_id,
        message,
        data,
        range,
    }
}
