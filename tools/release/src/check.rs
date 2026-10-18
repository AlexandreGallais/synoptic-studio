//! `release check`: the latest entry of `CHANGELOG.md` against the rules it can check
//! (`docs/tooling/releases.md`, What a release entry holds): the header, the heading, its date
//! and its link, the version, the groups and their order, one line per change with its references
//! last, the breaking changes first, the notice, and no line left as drafted.

use std::sync::LazyLock;

use regex::Regex;

use crate::changelog::{entries, heading_version, link_definition};
use crate::draft::{BREAKING_MARK, DRAFT_MARK, GROUPS};
use crate::version::Version;

/// The heading of a release: `## [0.3.0] - 2026-10-18` (`REF-COMMON-CHANGELOG`, 2.2).
static HEADING: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^## \[([^\]]+)\] - ([0-9]{4})-([0-9]{2})-([0-9]{2})$")
        .unwrap_or_else(|_| unreachable!("a valid pattern"))
});

/// A change's references at its end: one or more Markdown links, comma-separated, in parentheses
/// (`REF-COMMON-CHANGELOG`, 2.4.2).
static REFERENCES: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r" \(\[[^\]]+\]\([^)\s]+\)(?:, \[[^\]]+\]\([^)\s]+\))*\)$")
        .unwrap_or_else(|_| unreachable!("a valid pattern"))
});

/// One rule broken, at a line of the changelog.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Finding {
    /// Its line, from 1.
    pub line: usize,
    /// What the rule asks.
    pub message: String,
}

/// Whether a date of the proleptic Gregorian calendar exists.
#[must_use]
pub fn is_date(year: u32, month: u32, day: u32) -> bool {
    let leap = year.is_multiple_of(4) && (!year.is_multiple_of(100) || year.is_multiple_of(400));
    let days = match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 if leap => 29,
        2 => 28,
        _ => 0,
    };

    (1..=days).contains(&day)
}

/// What is wrong with the heading of the latest entry, at its line; its version when it reads.
fn heading_findings(
    line: usize,
    heading: &str,
    findings: &mut Vec<Finding>,
) -> Option<(String, Version)> {
    let mut found = |message: String| findings.push(Finding { line, message });
    let Some(captures) = HEADING.captures(heading) else {
        found("the heading must be `## [x.y.z] - YYYY-MM-DD`".to_owned());
        return None;
    };
    let number = |index: usize| captures[index].parse::<u32>().unwrap_or(0);

    if !is_date(number(2), number(3), number(4)) {
        found(format!(
            "the date {}-{}-{} is not a day of the calendar",
            &captures[2], &captures[3], &captures[4]
        ));
    }
    let written = captures[1].to_owned();
    let Some(version) = Version::parse(&written).filter(|_| !written.starts_with('v')) else {
        found(format!(
            "the version `{written}` must be x.y.z, without a `v`"
        ));
        return None;
    };

    Some((written, version))
}

/// What is wrong with one change: its references, its breaking mark and its draft mark.
fn change_findings(line: usize, text: &str, findings: &mut Vec<Finding>) {
    let mut found = |message: &str| {
        findings.push(Finding {
            line,
            message: message.to_owned(),
        });
    };

    if !REFERENCES.is_match(text) {
        found("a change ends with its references, each a Markdown link, in parentheses");
    }
    if text.contains(DRAFT_MARK.trim_end()) {
        found("a drafted line is rewritten in the imperative mood, its `[draft]` mark removed");
    }
    if text
        .strip_prefix(BREAKING_MARK)
        .unwrap_or(text)
        .contains(BREAKING_MARK.trim_end())
    {
        found("`**Breaking:**` opens its line");
    }
}

/// The finding of a group closed without a change, at the line that closes it.
fn close(group: Option<usize>, changes: usize, line: usize, findings: &mut Vec<Finding>) {
    if group.is_some() && changes == 0 {
        findings.push(Finding {
            line,
            message: "a group lists at least one change".to_owned(),
        });
    }
}

/// The findings of the entry's body: a notice, then the groups in order, each a list of changes.
fn body_findings(body: &[(usize, &str)], heading_line: usize, findings: &mut Vec<Finding>) {
    let mut notices = 0;
    let mut group: Option<usize> = None;
    let mut changes = 0;
    let mut breaking_open = true;
    let mut seen_group = false;

    for &(line, text) in body.iter().filter(|(_, text)| !text.trim().is_empty()) {
        if let Some(name) = text.strip_prefix("### ") {
            close(group, changes, line, findings);
            let index = GROUPS.iter().position(|(_, heading)| *heading == name);

            match (index, group) {
                (None, _) => findings.push(Finding {
                    line,
                    message: format!(
                        "a group is one of Changed, Added, Removed, Fixed, not `{name}`"
                    ),
                }),
                (Some(index), Some(previous)) if index <= previous => findings.push(Finding {
                    line,
                    message: "the groups come once each, in the order Changed, Added, Removed, \
                              Fixed"
                        .to_owned(),
                }),
                _ => {}
            }
            group = index.or(group);
            changes = 0;
            breaking_open = true;
            seen_group = true;
        } else if let Some(change) = text.strip_prefix("- ") {
            if !seen_group {
                findings.push(Finding {
                    line,
                    message: "a change is listed under a group".to_owned(),
                });
            }
            changes += 1;
            if change.starts_with(BREAKING_MARK) && !breaking_open {
                findings.push(Finding {
                    line,
                    message: "a breaking change comes first in its group".to_owned(),
                });
            }
            breaking_open = breaking_open && change.starts_with(BREAKING_MARK);
            change_findings(line, change, findings);
        } else if !seen_group && notices == 0 {
            notices += 1;
            if !(text.starts_with('_') && text.ends_with('_')) {
                findings.push(Finding {
                    line,
                    message: "a notice is one sentence in emphasis, `_…_`".to_owned(),
                });
            }
        } else {
            findings.push(Finding {
                line,
                message: "an entry holds one notice at most, then only groups and their lists"
                    .to_owned(),
            });
        }
    }
    let last = body
        .iter()
        .rev()
        .find(|(_, text)| !text.trim().is_empty())
        .map_or(heading_line, |(line, _)| *line);

    close(group, changes, last, findings);
    if !seen_group && notices == 0 && changes == 0 {
        findings.push(Finding {
            line: heading_line,
            message: "an entry holds a group of changes, or a notice saying why it has none"
                .to_owned(),
        });
    }
}

/// The findings on a changelog's latest entry, given the repository's address and the version of
/// `package.json`; none when it follows every rule the tool can check.
#[must_use]
pub fn check(changelog: &str, repository: &str, package_version: &str) -> Vec<Finding> {
    let mut findings = Vec::new();

    if !changelog.starts_with("# Changelog\n") {
        findings.push(Finding {
            line: 1,
            message: "the changelog opens with `# Changelog`".to_owned(),
        });
    }
    if !changelog.contains("[Semantic Versioning](https://semver.org/)") {
        findings.push(Finding {
            line: 1,
            message: "the header says the versions follow Semantic Versioning".to_owned(),
        });
    }

    let all = entries(changelog);
    let Some((heading_line, heading, body)) = all.first() else {
        findings.push(Finding {
            line: 1,
            message: "the changelog holds no entry".to_owned(),
        });
        return findings;
    };

    if let Some((written, version)) = heading_findings(*heading_line, heading, &mut findings) {
        if written != package_version {
            findings.push(Finding {
                line: *heading_line,
                message: format!("the version is {written}, `package.json` says {package_version}"),
            });
        }
        let link = link_definition(repository, &written);

        if !changelog.lines().any(|line| line == link) {
            findings.push(Finding {
                line: *heading_line,
                message: format!("the version links to its GitHub release: `{link}`"),
            });
        }
        if let Some(previous) = all
            .get(1)
            .and_then(|(_, heading, _)| heading_version(heading))
            .and_then(Version::parse)
            .filter(|previous| *previous >= version)
        {
            findings.push(Finding {
                line: *heading_line,
                message: format!("the latest entry comes first: {written} is not above {previous}"),
            });
        }
    }
    body_findings(body, *heading_line, &mut findings);
    findings
}
