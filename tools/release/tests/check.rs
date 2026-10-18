//! Tests of `release check` (EN-112, F21.AC6): a changelog that follows every rule it reads
//! passes, and each rule broken alone gives its finding at its line.

use release::changelog::HEADER;
use release::check::{check, is_date};

const REPO: &str = "https://github.com/o/r";

/// A valid changelog: the header, then 0.3.0 with a breaking change first, then release-please's
/// 0.2.0, then the link of 0.3.0. Line 5 is 0.3.0's heading.
fn valid() -> String {
    format!(
        "{HEADER}\n## [0.3.0] - 2026-10-18\n\n### Changed\n\n\
         - **Breaking:** remove `f` from the public API ([#9](https://x/9))\n\
         - Refactor the writer ([`abc1234`](https://x/c), [`def5678`](https://x/d))\n\n\
         ### Added\n\n- Add colors ([#9](https://x/9))\n\n### Fixed\n\n- Fix a hue ([#8](https://x/8))\n\n\
         ## [0.2.0](https://x/compare) (2026-10-14)\n\n### Features\n\n* **model:** x\n\n\
         [0.3.0]: https://github.com/o/r/releases/tag/v0.3.0\n"
    )
}

/// The findings of a changelog, each `line: message`.
fn findings(changelog: &str, package_version: &str) -> Vec<String> {
    check(changelog, REPO, package_version)
        .iter()
        .map(|finding| format!("{}: {}", finding.line, finding.message))
        .collect()
}

#[test]
fn f21_ac6_passes_an_entry_that_follows_every_rule_it_reads() {
    assert_eq!(findings(&valid(), "0.3.0"), Vec::<String>::new());

    let notice = valid().replace(
        "### Changed",
        "_If you are upgrading: please see the guide._\n\n### Changed",
    );
    assert_eq!(findings(&notice, "0.3.0"), Vec::<String>::new());

    let only_notice = format!(
        "{HEADER}\n## [0.3.1] - 2026-10-19\n\n_No change._\n\n[0.3.1]: {REPO}/releases/tag/v0.3.1\n"
    );
    assert_eq!(findings(&only_notice, "0.3.1"), Vec::<String>::new());
}

/// Each case breaks one rule of the valid changelog: what replaces what, the version of
/// `package.json`, and the finding expected.
const CASES: [(&str, &str, &str, &str); 18] = [
    (
        "# Changelog\n",
        "# Changes\n",
        "0.3.0",
        "1: the changelog opens with `# Changelog`",
    ),
    (
        "[Semantic Versioning](https://semver.org/)",
        "SemVer",
        "0.3.0",
        "1: the header says the versions follow Semantic Versioning",
    ),
    (
        "## [0.3.0] - 2026-10-18",
        "## [0.3.0] (2026-10-18)",
        "0.3.0",
        "5: the heading must be `## [x.y.z] - YYYY-MM-DD`",
    ),
    (
        "2026-10-18",
        "2026-02-29",
        "0.3.0",
        "5: the date 2026-02-29 is not a day of the calendar",
    ),
    (
        "## [0.3.0]",
        "## [v0.3.0]",
        "0.3.0",
        "5: the version `v0.3.0` must be x.y.z, without a `v`",
    ),
    (
        "",
        "",
        "0.3.1",
        "5: the version is 0.3.0, `package.json` says 0.3.1",
    ),
    (
        "releases/tag/v0.3.0",
        "releases/v0.3.0",
        "0.3.0",
        "5: the version links to its GitHub release: `[0.3.0]: https://github.com/o/r/releases/tag/v0.3.0`",
    ),
    (
        "## [0.2.0](https://x/compare)",
        "## [0.3.0](https://x/compare)",
        "0.3.0",
        "5: the latest entry comes first: 0.3.0 is not above 0.3.0",
    ),
    (
        "### Added",
        "### Security",
        "0.3.0",
        "12: a group is one of Changed, Added, Removed, Fixed, not `Security`",
    ),
    (
        "### Fixed\n\n- Fix a hue ([#8](https://x/8))\n",
        "### Fixed\n\n- Fix a hue ([#8](https://x/8))\n\n### Added\n\n- Add y ([#7](https://x/7))\n",
        "0.3.0",
        "20: the groups come once each, in the order Changed, Added, Removed, Fixed",
    ),
    (
        "- Add colors ([#9](https://x/9))\n",
        "",
        "0.3.0",
        "15: a group lists at least one change",
    ),
    (
        "- Fix a hue ([#8](https://x/8))",
        "- Fix a hue (#8)",
        "0.3.0",
        "18: a change ends with its references, each a Markdown link, in parentheses",
    ),
    (
        "- Fix a hue",
        "- [draft] Fix a hue",
        "0.3.0",
        "18: a drafted line is rewritten in the imperative mood, its `[draft]` mark removed",
    ),
    (
        "- Refactor the writer",
        "- Refactor the writer, **Breaking:** too",
        "0.3.0",
        "10: `**Breaking:**` opens its line",
    ),
    (
        "- Refactor the writer",
        "- **Breaking:** refactor the writer",
        "0.3.0",
        "",
    ),
    (
        "### Changed\n\n- **Breaking:**",
        "### Changed\n\n- Change x ([#1](https://x/1))\n- **Breaking:**",
        "0.3.0",
        "10: a breaking change comes first in its group",
    ),
    (
        "### Changed\n\n",
        "_One._\n\n_Two._\n\n### Changed\n\n",
        "0.3.0",
        "9: an entry holds one notice at most, then only groups and their lists",
    ),
    (
        "- Add colors ([#9](https://x/9))\n",
        "- Add colors ([#9](https://x/9))\n\nA paragraph.\n",
        "0.3.0",
        "16: an entry holds one notice at most, then only groups and their lists",
    ),
];

#[test]
fn f21_ac6_refuses_each_rule_broken_at_its_line() {
    let valid = valid();

    for (from, to, package_version, expected) in CASES {
        let changelog = valid.replacen(from, to, 1);
        let expected: Vec<String> = if expected.is_empty() {
            Vec::new()
        } else {
            vec![expected.to_owned()]
        };

        assert_eq!(
            findings(&changelog, package_version),
            expected,
            "{from:?} → {to:?}"
        );
    }
}

#[test]
fn refuses_a_notice_without_emphasis_a_change_outside_a_group_and_an_empty_entry() {
    let entry = |body: &str| {
        format!(
            "{HEADER}\n## [0.3.0] - 2026-10-18\n\n{body}\n[0.3.0]: {REPO}/releases/tag/v0.3.0\n"
        )
    };

    assert_eq!(
        findings(&entry("A notice.\n"), "0.3.0"),
        ["7: a notice is one sentence in emphasis, `_…_`"]
    );
    assert_eq!(
        findings(&entry("- Add x ([#1](https://x/1))\n"), "0.3.0"),
        ["7: a change is listed under a group"]
    );
    assert_eq!(
        findings(&entry(""), "0.3.0"),
        ["5: an entry holds a group of changes, or a notice saying why it has none"]
    );
    assert_eq!(
        findings(&entry("### Fixed\n"), "0.3.0"),
        ["7: a group lists at least one change"]
    );
    assert_eq!(
        findings(&format!("{HEADER}\n"), "0.3.0"),
        ["1: the changelog holds no entry"]
    );
}

#[test]
fn knows_the_days_of_the_calendar() {
    // 2028 is a leap year, 2100 is not, 2000 is.
    assert!(is_date(2028, 2, 29));
    assert!(!is_date(2100, 2, 29));
    assert!(is_date(2000, 2, 29));
    assert!(is_date(2026, 12, 31));
    assert!(!is_date(2026, 4, 31));
    assert!(!is_date(2026, 13, 1));
    assert!(!is_date(2026, 1, 0));
}
