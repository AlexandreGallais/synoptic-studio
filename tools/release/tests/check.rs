//! Tests of `release check` (EN-117, F22.AC2): a release text, the body of a pull request into
//! `main` up to its first line `---`, passes when it follows every rule the tool reads, and each
//! rule broken alone gives its finding at its line.

use release::body::release_text;
use release::check::check;
use release::draft::NO_CHANGE_NOTICE;

/// A valid text: a breaking change first in its group, then the groups in their order. Line 1 is
/// `### Changed`, line 6 `### Added`, line 10 `### Fixed`.
const VALID: &str = "### Changed\n\n\
     - **Breaking:** remove `f` from the public API ([#9](https://x/9))\n\
     - Refactor the writer ([`abc1234`](https://x/c), [`def5678`](https://x/d))\n\n\
     ### Added\n\n- Add colors ([#9](https://x/9))\n\n### Fixed\n\n- Fix a hue ([#8](https://x/8))\n";

/// The findings of a body, each `line: message`.
fn findings(body: &str) -> Vec<String> {
    check(&release_text(body))
        .iter()
        .map(|finding| format!("{}: {}", finding.line, finding.message))
        .collect()
}

#[test]
fn f22_ac2_passes_a_text_that_follows_every_rule_it_reads() {
    assert_eq!(findings(VALID), Vec::<String>::new());

    let notice = format!("_If you are upgrading: please see the guide._\n\n{VALID}");
    assert_eq!(findings(&notice), Vec::<String>::new());
    assert_eq!(
        findings(&format!("{NO_CHANGE_NOTICE}\n")),
        Vec::<String>::new()
    );
}

#[test]
fn f22_ac2_reads_the_body_up_to_its_first_break_whatever_its_line_endings() {
    // GitHub may give a body with `\r\n`; what follows `---` is for the reviewer, not the notes.
    let body = format!(
        "{VALID}\n---\n\nThe evidence: `check:all` green.\n\n🤖 Generated with [Claude Code](https://claude.com/claude-code)\n"
    );

    assert_eq!(findings(&body), Vec::<String>::new());
    assert_eq!(findings(&body.replace('\n', "\r\n")), Vec::<String>::new());
    // Without the break, the evidence (line 15) and the attribution (line 17) are paragraphs
    // after the lists: line 12 is the last change, lines 13, 14 and 16 are blank.
    assert_eq!(
        findings(&body.replace("---\n", "")),
        [
            "15: a release text holds one notice at most, then only groups and their lists",
            "17: a release text holds one notice at most, then only groups and their lists",
        ]
    );
}

/// Each case breaks one rule of the valid text: what replaces what, and the finding expected.
const CASES: [(&str, &str, &str); 14] = [
    (
        "### Changed\n",
        "## [0.3.0] - 2026-10-18\n\n### Changed\n",
        "1: a release text holds no heading but its groups', `### …`: the version is the tag's",
    ),
    (
        "### Fixed\n",
        "#### Fixed\n",
        "10: a release text holds no heading but its groups', `### …`: the version is the tag's",
    ),
    (
        "### Added",
        "### Security",
        "6: a group is one of Changed, Added, Removed, Fixed, not `Security`",
    ),
    (
        "### Fixed\n\n- Fix a hue ([#8](https://x/8))\n",
        "### Fixed\n\n- Fix a hue ([#8](https://x/8))\n\n### Added\n\n- Add y ([#7](https://x/7))\n",
        "14: the groups come once each, in the order Changed, Added, Removed, Fixed",
    ),
    // A group repeated is out of order too, its index no greater than the one before (AUD-023).
    (
        "### Fixed\n\n- Fix a hue ([#8](https://x/8))\n",
        "### Fixed\n\n- Fix a hue ([#8](https://x/8))\n\n### Fixed\n\n- Fix b ([#7](https://x/7))\n",
        "14: the groups come once each, in the order Changed, Added, Removed, Fixed",
    ),
    (
        "- Add colors ([#9](https://x/9))\n",
        "",
        "9: a group lists at least one change",
    ),
    (
        "- Fix a hue ([#8](https://x/8))",
        "- Fix a hue (#8)",
        "12: a change ends with its references, each a Markdown link, in parentheses",
    ),
    (
        "- Fix a hue",
        "- [draft] Fix a hue",
        "12: a drafted line is rewritten in the imperative mood, its `[draft]` mark removed",
    ),
    (
        "- Refactor the writer",
        "- Refactor the writer, **Breaking:** too",
        "4: `**Breaking:**` opens its line",
    ),
    (
        "- Refactor the writer",
        "- **Breaking:** refactor the writer",
        "",
    ),
    (
        "### Changed\n\n- **Breaking:**",
        "### Changed\n\n- Change x ([#1](https://x/1))\n- **Breaking:**",
        "4: a breaking change comes first in its group",
    ),
    (
        "### Changed\n\n",
        "_One._\n\n_Two._\n\n### Changed\n\n",
        "3: a release text holds one notice at most, then only groups and their lists",
    ),
    (
        "- Add colors ([#9](https://x/9))\n",
        "- Add colors ([#9](https://x/9))\n\nA paragraph.\n",
        "10: a release text holds one notice at most, then only groups and their lists",
    ),
    (
        "### Changed\n\n",
        "A notice.\n\n### Changed\n\n",
        "1: a notice is one sentence in emphasis, `_…_`",
    ),
];

#[test]
fn f22_ac2_refuses_each_rule_broken_at_its_line() {
    for (from, to, expected) in CASES {
        let body = VALID.replacen(from, to, 1);
        let expected: Vec<String> = if expected.is_empty() {
            Vec::new()
        } else {
            vec![expected.to_owned()]
        };

        assert_eq!(findings(&body), expected, "{from:?} → {to:?}");
    }
}

#[test]
fn refuses_a_change_outside_a_group_an_empty_group_and_an_empty_text() {
    assert_eq!(
        findings("- Add x ([#1](https://x/1))\n"),
        ["1: a change is listed under a group"]
    );
    assert_eq!(
        findings("### Fixed\n"),
        ["1: a group lists at least one change"]
    );
    for empty in [
        "",
        "\n\n",
        "---\n\n### Fixed\n\n- Fix a hue ([#8](https://x/8))\n",
    ] {
        assert_eq!(
            findings(empty),
            [
                "1: a release text lists its changes by group, or holds a notice saying why it has none"
            ],
            "{empty:?}"
        );
    }
}
