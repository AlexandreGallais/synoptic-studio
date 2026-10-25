//! `release check`: a release text, the body of a pull request into `main` up to its first
//! thematic break, against the rules it can check (`the records' conventions, area release`, What a release
//! text holds): a notice at most, then the groups in their order, one line per change with its
//! references last, the breaking changes first, and no line left as drafted.

use std::sync::LazyLock;

use regex::Regex;

use crate::draft::{BREAKING_MARK, DRAFT_MARK, GROUPS};

/// A change's references at its end: one or more Markdown links, comma-separated, in parentheses
/// (`REF-COMMON-CHANGELOG`, 2.4.2).
static REFERENCES: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r" \(\[[^\]]+\]\([^)\s]+\)(?:, \[[^\]]+\]\([^)\s]+\))*\)$")
        .unwrap_or_else(|_| unreachable!("a valid pattern"))
});

/// One rule broken, at a line of the text.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Finding {
    /// Its line, from 1.
    pub line: usize,
    /// What the rule asks.
    pub message: String,
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

/// Where the walk through a text is: the group open, its changes, whether only breaking changes
/// came in it yet, whether a group came at all, and the notices met.
struct Walk {
    group: Option<usize>,
    changes: usize,
    breaking_open: bool,
    seen_group: bool,
    notices: usize,
}

/// A group's heading, `### name`, at its line.
fn group_findings(walk: &mut Walk, line: usize, name: &str, findings: &mut Vec<Finding>) {
    close(walk.group, walk.changes, line, findings);
    let index = GROUPS.iter().position(|(_, heading)| *heading == name);

    match (index, walk.group) {
        (None, _) => findings.push(Finding {
            line,
            message: format!("a group is one of Changed, Added, Removed, Fixed, not `{name}`"),
        }),
        (Some(index), Some(previous)) if index <= previous => findings.push(Finding {
            line,
            message: "the groups come once each, in the order Changed, Added, Removed, Fixed"
                .to_owned(),
        }),
        _ => {}
    }
    walk.group = index.or(walk.group);
    walk.changes = 0;
    walk.breaking_open = true;
    walk.seen_group = true;
}

/// A change, `- …`, at its line.
fn listed_findings(walk: &mut Walk, line: usize, change: &str, findings: &mut Vec<Finding>) {
    if !walk.seen_group {
        findings.push(Finding {
            line,
            message: "a change is listed under a group".to_owned(),
        });
    }
    walk.changes += 1;
    if change.starts_with(BREAKING_MARK) && !walk.breaking_open {
        findings.push(Finding {
            line,
            message: "a breaking change comes first in its group".to_owned(),
        });
    }
    walk.breaking_open = walk.breaking_open && change.starts_with(BREAKING_MARK);
    change_findings(line, change, findings);
}

/// Any other line: the notice before the groups, or a line out of place.
fn other_findings(walk: &mut Walk, line: usize, text: &str, findings: &mut Vec<Finding>) {
    let message = if text.starts_with('#') {
        "a release text holds no heading but its groups', `### …`: the version is the tag's"
    } else if walk.seen_group || walk.notices > 0 {
        "a release text holds one notice at most, then only groups and their lists"
    } else {
        walk.notices += 1;
        if text.starts_with('_') && text.ends_with('_') {
            return;
        }
        "a notice is one sentence in emphasis, `_…_`"
    };

    findings.push(Finding {
        line,
        message: message.to_owned(),
    });
}

/// The findings on a release text; none when it follows every rule the tool can check.
#[must_use]
pub fn check(text: &str) -> Vec<Finding> {
    let mut findings = Vec::new();
    let mut walk = Walk {
        group: None,
        changes: 0,
        breaking_open: true,
        seen_group: false,
        notices: 0,
    };
    let lines: Vec<(usize, &str)> = text
        .lines()
        .enumerate()
        .map(|(index, line)| (index + 1, line))
        .filter(|(_, line)| !line.trim().is_empty())
        .collect();

    for &(line, text) in &lines {
        if let Some(name) = text.strip_prefix("### ") {
            group_findings(&mut walk, line, name, &mut findings);
        } else if let Some(change) = text.strip_prefix("- ") {
            listed_findings(&mut walk, line, change, &mut findings);
        } else {
            other_findings(&mut walk, line, text, &mut findings);
        }
    }

    let last = lines.last().map_or(1, |(line, _)| *line);

    close(walk.group, walk.changes, last, &mut findings);
    if !walk.seen_group && walk.notices == 0 && walk.changes == 0 {
        findings.push(Finding {
            line: 1,
            message: "a release text lists its changes by group, or holds a notice saying why it \
                      has none"
                .to_owned(),
        });
    }
    findings
}
