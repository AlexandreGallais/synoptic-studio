//! Tests of the release's pure parts (EN-112, F21.AC6): the version and its bump, a commit read,
//! the draft's lines, groups, order and references, the changelog written and read back, the
//! manifests' versions. `f21_ac6_` marks a test proving the feature's criterion.

use std::collections::HashMap;

use release::backlog::{Backlog, Item, item};
use release::changelog::{HEADER, heading_version, notes, with_entry};
use release::commit::{Commit, header, merged_pull_request};
use release::draft::{
    Group, NO_CHANGE_NOTICE, bump, capitalised, entry, group_of, hints, lines, references,
    without_no_ops,
};
use release::manifest::{
    lock_versions, repository, version, with_lock_version, with_package_version,
};
use release::version::{Bump, Version};

const REPO: &str = "https://github.com/o/r";

/// A commit of a 40-character hash made of one repeated digit, its pull request given.
fn commit(digit: char, subject: &str, body: &str, pull_request: Option<u32>) -> Commit {
    Commit {
        hash: digit.to_string().repeat(40),
        subject: subject.to_owned(),
        body: body.to_owned(),
        pull_request,
    }
}

fn story(id: &str, title: &str, feature: &str) -> (String, Item) {
    (
        id.to_owned(),
        Item {
            id: id.to_owned(),
            title: title.to_owned(),
            feature: Some(feature.to_owned()),
        },
    )
}

fn backlog() -> Backlog {
    Backlog {
        stories: HashMap::from([
            story(
                "US-041",
                "the underline's and strike-through's colors",
                "F16",
            ),
            story("US-044", "the HTML layer withdrawn", "F17"),
            story("AUD-018", "Audit F15", "F15"),
            story("EN-039", "a font file read and kept", "F15"),
        ]),
        features: HashMap::from([(
            "F16".to_owned(),
            Item {
                id: "F16".to_owned(),
                title: "Colors".to_owned(),
                feature: None,
            },
        )]),
    }
}

#[test]
fn reads_a_version_with_or_without_its_v_and_nothing_else() {
    assert_eq!(
        Version::parse("v1.10.3"),
        Some(Version {
            major: 1,
            minor: 10,
            patch: 3
        })
    );
    assert_eq!(
        Version::parse("0.2.0").map(|v| v.to_string()),
        Some("0.2.0".to_owned())
    );
    for refused in [
        "", "1.2", "1.2.3.4", "01.2.3", "1.2.x", "1..3", "1.2.-3", "vv1.2.3",
    ] {
        assert_eq!(Version::parse(refused), None, "{refused}");
    }
    assert!(Version::parse("0.10.0") > Version::parse("0.9.9"));
}

#[test]
fn f21_ac6_bumps_the_minor_version_for_a_feature_or_a_break_before_1_0_0() {
    let bumped = |text: &str, bump: Bump| Version::parse(text).unwrap().bumped(bump).to_string();

    // REF-SEMVER, and release-please's `bump-minor-pre-major` kept: before 1.0.0 a breaking change
    // raises the minor version, from 1.0.0 the major one; a feature the minor, the rest the patch.
    assert_eq!(bumped("0.2.0", Bump::Breaking), "0.3.0");
    assert_eq!(bumped("0.2.5", Bump::Minor), "0.3.0");
    assert_eq!(bumped("0.2.0", Bump::Patch), "0.2.1");
    assert_eq!(bumped("1.4.2", Bump::Breaking), "2.0.0");
    assert_eq!(bumped("1.4.2", Bump::Minor), "1.5.0");
    assert_eq!(bumped("1.4.2", Bump::Patch), "1.4.3");
}

#[test]
fn reads_a_header_its_scope_and_its_bang() {
    let read = header("refactor(geometry)!: one path writer").unwrap();

    assert_eq!(
        (read.kind.as_str(), read.scope.as_deref(), read.bang),
        ("refactor", Some("geometry"), true)
    );
    assert_eq!(read.description, "one path writer");
    assert_eq!(header("refactor: four modules").unwrap().scope, None);
    assert_eq!(header("Revert \"feat: x\""), None);
    assert_eq!(header("feat(model):no space"), None);
}

#[test]
fn f21_ac6_keeps_the_library_and_the_playground_and_leaves_the_rest_out() {
    for kept in [
        "feat(model): x",
        "fix(playground): x",
        "perf(render): x",
        "refactor: x",
        "revert(io): x",
        "Revert \"feat(model): x\"",
    ] {
        assert!(commit('a', kept, "", None).is_user_visible(), "{kept}");
    }
    for left in [
        "feat(tooling): x",
        "fix(ci): x",
        "docs(playground): x",
        "test(model): x",
        "chore: x",
        "build(deps): x",
        "style: x",
        "a subject outside the convention",
    ] {
        assert!(!commit('a', left, "", None).is_user_visible(), "{left}");
    }
}

#[test]
fn reads_the_story_the_break_the_revert_and_the_pull_requests() {
    let footer = commit(
        'a',
        "feat(model): x",
        "Body.\n\nBREAKING CHANGE: `f` is removed\nRefs: EN-039.T3\nRefs: US-001\n",
        None,
    );

    assert_eq!(footer.breaking().as_deref(), Some("`f` is removed"));
    assert_eq!(footer.story().as_deref(), Some("EN-039"));
    assert_eq!(
        commit('a', "fix(io)!: drop x", "", None)
            .breaking()
            .as_deref(),
        Some("drop x")
    );
    assert_eq!(
        commit('a', "fix(io): x", "BREAKING-CHANGE: y", None)
            .breaking()
            .as_deref(),
        Some("y")
    );
    assert_eq!(
        commit('a', "fix(io): x", "no footer", None).breaking(),
        None
    );
    assert_eq!(commit('a', "fix(io): x", "Refs #12", None).story(), None);
    let reverted = format!("This reverts commit {}.", "b".repeat(40));
    assert_eq!(
        commit('a', "Revert \"x\"", &reverted, None).reverted(),
        Some("b".repeat(40))
    );
    assert_eq!(
        commit('a', "fix(io): x (#57)", "", None).squashed_pull_request(),
        Some(57)
    );
    assert_eq!(
        commit('a', "fix(io): x (#57) y", "", None).squashed_pull_request(),
        None
    );
    assert_eq!(
        merged_pull_request("Merge pull request #340 from o/feat/x"),
        Some(340)
    );
    assert_eq!(
        merged_pull_request("Merge branch 'main' into develop"),
        None
    );
    assert_eq!(commit('a', "x", "", None).short(), "aaaaaaa");
}

#[test]
fn groups_each_commit_by_its_impact() {
    let group = |subject: &str| group_of(&commit('a', subject, "", None));

    assert_eq!(group("feat(model): add x"), Group::Added);
    assert_eq!(group("feat(model): Remove x"), Group::Removed);
    assert_eq!(group("feat(model): drop x"), Group::Removed);
    assert_eq!(group("feat(model): removed x stays added"), Group::Added);
    assert_eq!(group("fix(model): x"), Group::Fixed);
    assert_eq!(group("refactor(model): x"), Group::Changed);
    assert_eq!(group("perf(model): x"), Group::Changed);
    assert_eq!(group("Revert \"x\""), Group::Changed);
}

#[test]
fn f21_ac6_leaves_out_a_revert_and_what_it_reverts_both_in_the_release() {
    let reverts_b = format!("This reverts commit {}.", "b".repeat(40));
    let reverts_older = format!("This reverts commit {}.", "e".repeat(40));
    let commits = [
        commit('a', "Revert \"feat(model): x\"", &reverts_b, None),
        commit('b', "feat(model): x", "", None),
        commit('c', "Revert \"feat(model): y\"", &reverts_older, None),
        commit('d', "fix(model): z", "", None),
    ];
    let kept: Vec<char> = without_no_ops(&commits)
        .iter()
        .map(|commit| commit.hash.chars().next().unwrap())
        .collect();

    // `a` reverts `b`, both here: gone; `c` reverts a commit of an earlier release: kept.
    assert_eq!(kept, ['c', 'd']);
}

#[test]
fn capitalises_the_first_letter_only() {
    assert_eq!(capitalised("éclat of x"), "Éclat of x");
    assert_eq!(capitalised("`f` stays"), "`f` stays");
    assert_eq!(capitalised(""), "");
}

/// The release of the tests: commits the latest first, as `git log` gives them.
fn release_commits() -> Vec<Commit> {
    vec![
        commit(
            '1',
            "docs(backlog): close US-041",
            "Refs: US-041\n",
            Some(9),
        ),
        commit(
            '2',
            "fix(playground): keep the hue",
            "Refs: US-041.T3\n",
            Some(9),
        ),
        commit(
            '3',
            "feat(playground): choose the colors",
            "Refs: US-041.T2\n",
            Some(9),
        ),
        commit(
            '4',
            "feat(render): draw each stroke",
            "Refs: US-041.T1\n",
            Some(9),
        ),
        commit(
            '5',
            "feat(playground): remove the HTML layer option",
            "Refs: US-044.T2\n",
            Some(8),
        ),
        commit(
            '6',
            "refactor(render)!: write every text as its group",
            "Refs: US-044.T1\n",
            Some(8),
        ),
        commit(
            '7',
            "fix(io): refuse a crafted table",
            "Refs: AUD-018.T1\n",
            None,
        ),
        commit(
            '8',
            "fix(io): read the tables once",
            "Refs: EN-039.T4\n",
            None,
        ),
        commit('9', "feat(tooling): a status line", "", None),
        commit('0', "perf(playground): keep a text's metrics", "", None),
    ]
}

#[test]
fn f21_ac6_drafts_one_line_per_story_in_its_group_a_feature_s_lines_together() {
    let drafted = lines(&release_commits(), &backlog());
    let summary: Vec<(Group, bool, &str, Option<&str>, usize)> = drafted
        .iter()
        .map(|line| {
            (
                line.group,
                line.breaking,
                line.text.as_str(),
                line.feature.as_deref(),
                line.commits.len(),
            )
        })
        .collect();

    // US-044 has a removal and a breaking refactoring: two lines, the removal on its own titled by
    // its commit, the other by the story. US-041's feat and fix are one Added line.
    // The audit's fix keeps its commit's words; EN-039's its title. The tooling and docs leave.
    // Order: Changed, Added, Removed, Fixed; the breaking one first; F16 before F15, a feature's lines
    // in the order of the history; no feature last.
    assert_eq!(
        summary,
        [
            (
                Group::Changed,
                true,
                "The HTML layer withdrawn",
                Some("F17"),
                1
            ),
            (Group::Changed, false, "Keep a text's metrics", None, 1),
            (
                Group::Added,
                false,
                "The underline's and strike-through's colors",
                Some("F16"),
                3
            ),
            (
                Group::Removed,
                false,
                "Remove the HTML layer option",
                Some("F17"),
                1
            ),
            (
                Group::Fixed,
                false,
                "Refuse a crafted table",
                Some("F15"),
                1
            ),
            (
                Group::Fixed,
                false,
                "A font file read and kept",
                Some("F15"),
                1
            ),
        ]
    );
    assert_eq!(bump(&drafted), Bump::Breaking);
}

#[test]
fn titles_a_story_s_only_line_by_its_story_a_removal_too() {
    let commits = [commit(
        '5',
        "feat(playground): remove the option",
        "Refs: US-044.T2\n",
        None,
    )];

    assert_eq!(
        lines(&commits, &backlog())[0].text,
        "The HTML layer withdrawn"
    );
}

#[test]
fn bumps_by_the_lines_a_feature_then_a_patch() {
    let feat = [commit('a', "feat(model): x", "", None)];
    let fix = [commit('a', "fix(model): x", "", None)];
    let tooling = [commit('a', "feat(tooling): x", "", None)];

    assert_eq!(bump(&lines(&feat, &Backlog::default())), Bump::Minor);
    assert_eq!(bump(&lines(&fix, &Backlog::default())), Bump::Patch);
    assert_eq!(bump(&lines(&tooling, &Backlog::default())), Bump::Patch);
}

#[test]
fn f21_ac6_references_the_pull_request_else_the_commits_the_best_alone_beyond_two() {
    let line = |commits: Vec<Commit>| lines(&commits, &Backlog::default()).remove(0);
    let story = |digit: char, pull_request: Option<u32>| {
        commit(digit, "fix(model): x", "Refs: US-001\n", pull_request)
    };

    assert_eq!(
        references(&line(vec![story('a', Some(3)), story('b', Some(2))]), REPO),
        "([#3](https://github.com/o/r/pull/3), [#2](https://github.com/o/r/pull/2))"
    );
    assert_eq!(
        references(
            &line(vec![
                story('a', Some(4)),
                story('b', Some(3)),
                story('c', Some(2))
            ]),
            REPO
        ),
        "([#4](https://github.com/o/r/pull/4))"
    );
    let a = "a".repeat(40);
    let b = "b".repeat(40);
    let c = "c".repeat(40);
    // The oldest first: `b` before `a`; beyond two, the oldest alone, `c`.
    assert_eq!(
        references(&line(vec![story('a', None), story('b', None)]), REPO),
        format!(
            "([`bbbbbbb`](https://github.com/o/r/commit/{b}), [`aaaaaaa`](https://github.com/o/r/commit/{a}))"
        )
    );
    assert_eq!(
        references(
            &line(vec![story('a', None), story('b', None), story('c', None)]),
            REPO
        ),
        format!("([`ccccccc`](https://github.com/o/r/commit/{c}))")
    );
    // A pull request wins over the commits of the same line.
    assert_eq!(
        references(&line(vec![story('a', None), story('b', Some(7))]), REPO),
        "([#7](https://github.com/o/r/pull/7))"
    );
}

#[test]
fn f21_ac6_writes_the_entry_groups_in_order_each_line_marked_as_drafted() {
    let drafted = lines(&release_commits()[..6], &backlog());
    let pr = |number: u32| format!("([#{number}](https://github.com/o/r/pull/{number}))");

    assert_eq!(
        entry("0.3.0", "2026-10-18", &drafted, REPO),
        format!(
            "## [0.3.0] - 2026-10-18\n\n### Changed\n\n\
             - **Breaking:** [draft] The HTML layer withdrawn {}\n\n### Added\n\n\
             - [draft] The underline's and strike-through's colors {}\n\n### Removed\n\n\
             - [draft] Remove the HTML layer option {}\n",
            pr(8),
            pr(9),
            pr(8)
        )
    );
    assert_eq!(
        entry("0.2.1", "2026-10-18", &[], REPO),
        format!("## [0.2.1] - 2026-10-18\n\n{NO_CHANGE_NOTICE}\n")
    );
}

#[test]
fn hints_list_each_line_s_feature_story_commits_and_break() {
    let text = hints(&lines(&release_commits()[..6], &backlog()), &backlog());

    assert!(
        text.contains("Added · F16 Colors · US-041: The underline's"),
        "{text}"
    );
    assert!(
        text.contains("    3333333 feat(playground): choose the colors\n"),
        "{text}"
    );
    assert!(
        text.contains("Changed · F17 · US-044: The HTML layer withdrawn"),
        "{text}"
    );
    assert!(
        text.contains("      breaking: write every text as its group\n"),
        "{text}"
    );
    let alone = hints(&lines(&release_commits()[9..], &backlog()), &backlog());
    assert!(
        alone.starts_with("Changed · no feature · no story: Keep"),
        "{alone}"
    );
}

#[test]
fn reads_an_item_from_its_front_matter() {
    let read = item("---\nid: US-1\nepic: E1\nfeature: F2\ntitle: \"A: \\\"b\\\"\"\n---\n\n# x\n");

    assert_eq!(
        read,
        Some(Item {
            id: "US-1".to_owned(),
            title: "A: \"b\"".to_owned(),
            feature: Some("F2".to_owned()),
        })
    );
    assert_eq!(
        item("---\nid: F2\ntitle: plain\n---\n").unwrap().feature,
        None
    );
    assert_eq!(item("# no front matter\n"), None);
    assert_eq!(item("---\ntitle: no id\n---\n"), None);
}

const OLD: &str = "# Changelog\n\n## [0.2.0](https://x/compare) (2026-10-14)\n\n### Features\n\n\
* **model:** x ([abc](https://x))\n\n## 0.1.0 (2026-10-13)\n\nA base.\n";

#[test]
fn f21_ac6_writes_the_entry_first_the_header_again_its_link_at_the_end() {
    let new = "## [0.3.0] - 2026-10-18\n\n### Added\n\n- Add x ([#1](https://x/1))\n";
    let once = with_entry(OLD, new, "0.3.0", REPO);

    assert_eq!(
        once,
        format!(
            "{HEADER}\n{new}\n## [0.2.0](https://x/compare) (2026-10-14)\n\n### Features\n\n\
             * **model:** x ([abc](https://x))\n\n## 0.1.0 (2026-10-13)\n\nA base.\n\n\
             [0.3.0]: https://github.com/o/r/releases/tag/v0.3.0\n"
        )
    );
    let next = "## [0.3.1] - 2026-10-19\n\n### Fixed\n\n- Fix y ([#2](https://x/2))\n";
    let twice = with_entry(&once, next, "0.3.1", REPO);

    assert!(twice.ends_with(
        "A base.\n\n[0.3.1]: https://github.com/o/r/releases/tag/v0.3.1\n\
         [0.3.0]: https://github.com/o/r/releases/tag/v0.3.0\n"
    ));
    assert!(twice.contains(&format!("{next}\n## [0.3.0] - 2026-10-18\n")));
    assert_eq!(
        with_entry("", new, "0.3.0", REPO),
        format!("{HEADER}\n{new}\n[0.3.0]: https://github.com/o/r/releases/tag/v0.3.0\n")
    );
    assert!(with_entry(new, new, "0.3.0", REPO).contains(&format!("{new}\n{}", new.trim_end())));
}

#[test]
fn reads_an_entry_s_version_and_notes_back() {
    assert_eq!(heading_version("## [0.3.0] - 2026-10-18"), Some("0.3.0"));
    assert_eq!(
        heading_version("## [0.2.0](https://x) (2026-10-14)"),
        Some("0.2.0")
    );
    assert_eq!(heading_version("## 0.1.0 (2026-10-13)"), Some("0.1.0"));
    assert_eq!(heading_version("## Unreleased"), None);
    assert_eq!(heading_version("# 0.1.0"), None);

    let changelog = with_entry(
        OLD,
        "## [0.3.0] - 2026-10-18\n\n### Added\n\n- Add x (l)\n",
        "0.3.0",
        REPO,
    );

    assert_eq!(
        notes(&changelog, "0.3.0").as_deref(),
        Some("### Added\n\n- Add x (l)\n")
    );
    assert_eq!(notes(&changelog, "0.1.0").as_deref(), Some("A base.\n"));
    assert_eq!(notes(&changelog, "0.4.0"), None);
}

const PACKAGE: &str = "{\n  \"name\": \"editor\",\n  \"version\": \"0.2.0\",\n  \"repository\": {\n    \"type\": \"git\",\n    \"url\": \"git+https://github.com/o/r.git\"\n  },\n  \"devDependencies\": {\n    \"x\": \"1.0.0\"\n  }\n}\n";

const LOCK: &str = "{\n  \"name\": \"editor\",\n  \"version\": \"0.2.0\",\n  \"lockfileVersion\": 3,\n  \"packages\": {\n    \"\": {\n      \"name\": \"editor\",\n      \"version\": \"0.2.0\"\n    },\n    \"node_modules/x\": {\n      \"version\": \"0.2.0\"\n    }\n  }\n}\n";

#[test]
fn f21_ac6_sets_the_version_of_both_manifests_every_other_byte_kept() {
    assert_eq!(version(PACKAGE), Some("0.2.0"));
    assert_eq!(
        with_package_version(PACKAGE, "0.3.0").unwrap(),
        PACKAGE.replacen("0.2.0", "0.3.0", 1)
    );
    let lock = with_lock_version(LOCK, "0.3.0").unwrap();

    // The top version and the root package's change; `node_modules/x`'s, equal by chance, stays.
    assert_eq!(lock, LOCK.replacen("0.2.0", "0.3.0", 2));
    assert_eq!(lock_versions(&lock), (Some("0.3.0"), Some("0.3.0")));
    assert_eq!(lock_versions("{}"), (None, None));
    assert_eq!(with_lock_version("{\"version\": \"1.0.0\"}", "1.1.0"), None);
    assert_eq!(with_package_version("{}", "1.1.0"), None);
    assert_eq!(repository(PACKAGE).as_deref(), Some(REPO));
    assert_eq!(
        repository("{\"repository\": {\"url\": \"https://github.com/o/r\"}}").as_deref(),
        Some(REPO)
    );
    assert_eq!(repository("{}"), None);
}
