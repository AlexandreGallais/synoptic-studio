//! Tests of the release's pure parts (EN-112, F21.AC6; EN-117, F22.AC2): the version and its
//! bump, a commit read, the draft's lines, groups, order and references, the release text written,
//! read and joined into a release's notes, the repository's address. `f21_ac6_` and `f22_ac2_`
//! mark a test proving a feature's criterion.

use std::collections::HashMap;

use release::backlog::{Backlog, Item, backlog_of_rows};
use release::body::{Text, joined, release_text, text, written};
use release::commit::{Commit, header, merged_branch, merged_pull_request};
use release::draft::{
    Group, NO_CHANGE_NOTICE, bump, capitalised, group_of, hints, lines, references,
    text as draft_text, without_no_ops,
};
use release::manifest::repository;
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
    // A footer over two lines, as dc83b1b wrote it, read whole down to the next footer.
    let long = "B.\n\nBREAKING CHANGE: `f` is removed;\n`g(a, b)` takes angles.\nRefs: EN-021.T3\n";
    assert_eq!(
        commit('a', "refactor(geometry)!: x", long, None)
            .breaking()
            .as_deref(),
        Some("`f` is removed; `g(a, b)` takes angles.")
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
fn reads_the_head_branch_of_a_merge_its_owner_left_out() {
    assert_eq!(
        merged_branch("Merge pull request #22 from o/dependabot/cargo/regex-1.11.1"),
        Some("dependabot/cargo/regex-1.11.1".to_owned())
    );
    assert_eq!(
        merged_branch("Merge pull request #195 from AlexandreGallais/develop"),
        Some("develop".to_owned())
    );
    // No owner, a merge of a branch, a subject that only names one.
    for subject in [
        "Merge pull request #22 from develop",
        "Merge pull request #22 from o/",
        "Merge branch 'main' into develop",
        "fix(io): Merge pull request #22 from o/develop",
    ] {
        assert_eq!(merged_branch(subject), None, "{subject}");
    }
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
    // A `feat` among fixes raises the minor version, whatever comes first (AUD-023).
    let mixed = [
        commit('a', "fix(model): x", "", None),
        commit('b', "feat(render): y", "", None),
    ];

    assert_eq!(bump(&lines(&mixed, &Backlog::default())), Bump::Minor);
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
fn f22_ac2_drafts_the_text_groups_in_order_each_line_marked_without_a_heading() {
    let drafted = lines(&release_commits()[..6], &backlog());
    let pr = |number: u32| format!("([#{number}](https://github.com/o/r/pull/{number}))");

    // No `## [x.y.z] - date`: the version is computed at the button, from the tags.
    assert_eq!(
        draft_text(&drafted, REPO),
        format!(
            "### Changed\n\n\
             - **Breaking:** [draft] The HTML layer withdrawn {}\n\n### Added\n\n\
             - [draft] The underline's and strike-through's colors {}\n\n### Removed\n\n\
             - [draft] Remove the HTML layer option {}\n",
            pr(8),
            pr(9),
            pr(8)
        )
    );
    assert_eq!(draft_text(&[], REPO), format!("{NO_CHANGE_NOTICE}\n"));
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
fn reads_the_stories_and_features_from_the_records_rows() {
    let row = |cells: [&str; 3]| cells.map(str::to_owned).to_vec();
    let read = backlog_of_rows(&[
        row(["US-001", "A: \"b\"", "F02"]),
        row(["REV-009", "A review", ""]),
        row(["F02", "plain", ""]),
        vec!["short".to_owned()],
    ]);

    assert_eq!(
        read.stories.get("US-001"),
        Some(&Item {
            id: "US-001".to_owned(),
            title: "A: \"b\"".to_owned(),
            feature: Some("F02".to_owned()),
        })
    );
    // A story of no feature, and a feature, have none.
    assert_eq!(
        read.stories.get("REV-009").map(|item| &item.feature),
        Some(&None)
    );
    assert_eq!(
        read.features.get("F02").map(|item| item.title.as_str()),
        Some("plain")
    );
    assert_eq!((read.stories.len(), read.features.len()), (2, 1));
}

#[test]
fn reads_the_repository_s_address_from_package_json() {
    let package = "{\n  \"name\": \"editor\",\n  \"version\": \"0.0.0-development\",\n  \"repository\": {\n    \"type\": \"git\",\n    \"url\": \"git+https://github.com/o/r.git\"\n  }\n}\n";

    assert_eq!(repository(package).as_deref(), Some(REPO));
    assert_eq!(
        repository("{\"repository\": {\"url\": \"https://github.com/o/r\"}}").as_deref(),
        Some(REPO)
    );
    assert_eq!(repository("{}"), None);
}

#[test]
fn f22_ac2_reads_a_body_up_to_its_first_break_its_line_endings_made_unix() {
    assert_eq!(
        release_text("### Fixed\r\n\r\n- Fix x (l)\r\n\r\n---\r\n\r\n🤖 Generated\r\n"),
        "### Fixed\n\n- Fix x (l)\n\n"
    );
    assert_eq!(
        release_text("### Fixed\n\n- Fix x (l)\n"),
        "### Fixed\n\n- Fix x (l)\n"
    );
    // A break is a line `---` alone, spaces after it allowed: `----` and `--- x` are text.
    assert_eq!(
        release_text("_a_\n----\n--- x\n---  \ny\n"),
        "_a_\n----\n--- x\n"
    );
    assert_eq!(release_text("x"), "x\n");
    assert_eq!(release_text("---\nx\n"), "");
    assert_eq!(release_text(""), "");
}

/// A change of a group, as a text holds it.
fn change(group: Group, line: &str) -> (Group, String) {
    (group, line.to_owned())
}

#[test]
fn writes_and_reads_a_text_its_notice_then_its_groups_in_order() {
    let changes = [
        change(Group::Changed, "**Breaking:** drop `f` (l)"),
        change(Group::Added, "Add x (l)"),
        change(Group::Fixed, "Fix z (l)"),
        change(Group::Added, "Add y (l)"),
    ];
    let notice = "_Read the guide._";
    // The groups in the order Changed, Added, Removed, Fixed, whatever the order given; a
    // group's changes in theirs.
    let expected = "_Read the guide._\n\n### Changed\n\n- **Breaking:** drop `f` (l)\n\n\
                    ### Added\n\n- Add x (l)\n- Add y (l)\n\n### Fixed\n\n- Fix z (l)\n";

    assert_eq!(written(Some(notice), &changes), expected);
    assert_eq!(
        text(expected),
        Text {
            notice: Some(notice.to_owned()),
            changes: vec![
                change(Group::Changed, "**Breaking:** drop `f` (l)"),
                change(Group::Added, "Add x (l)"),
                change(Group::Added, "Add y (l)"),
                change(Group::Fixed, "Fix z (l)"),
            ],
        }
    );
    assert_eq!(written(Some(notice), &[]), "_Read the guide._\n");
    assert_eq!(written(None, &changes[1..2]), "### Added\n\n- Add x (l)\n");
    assert_eq!(written(None, &[]), "");
    assert_eq!(text(""), Text::default());
    assert_eq!(text("\n\n"), Text::default());
}

/// The texts of two pull requests: #21, the latest, then #20.
fn two_texts() -> (Text, Text) {
    (
        text(
            "### Changed\n\n- Refactor h (l21)\n\n### Added\n\n- Add b (l21)\n\n### Fixed\n\n- Fix c (l21)\n",
        ),
        text(
            "### Changed\n\n- **Breaking:** rename g (l20)\n- Refactor d (l20)\n\n### Added\n\n- Add f (l20)\n",
        ),
    )
}

#[test]
fn f22_ac2_joins_the_bodies_by_group_the_breaking_first_then_the_latest() {
    let (latest, older) = two_texts();

    // Changed: #20's breaking change first, then #21's line before #20's; Added likewise; Fixed
    // from #21 alone.
    assert_eq!(
        joined(&[(21, latest), (20, older)]),
        Ok("### Changed\n\n- **Breaking:** rename g (l20)\n- Refactor h (l21)\n- Refactor d (l20)\n\n\
            ### Added\n\n- Add b (l21)\n- Add f (l20)\n\n### Fixed\n\n- Fix c (l21)\n"
            .to_owned())
    );
}

#[test]
fn f22_ac3_joins_the_notices_the_one_of_no_change_only_without_a_change() {
    let (latest, older) = two_texts();
    let no_change = text(&format!("{NO_CHANGE_NOTICE}\n"));
    let guide = |text: &Text| Text {
        notice: Some("_Read the guide._".to_owned()),
        ..text.clone()
    };

    // A pull request of tooling only beside one of changes: no notice.
    assert_eq!(
        joined(&[(21, no_change.clone()), (20, older.clone())]),
        Ok(written(None, &older.changes))
    );
    // No change in any of them, or no pull request at all: the notice of no change.
    let alone = format!("{NO_CHANGE_NOTICE}\n");
    assert_eq!(
        joined(&[(21, no_change.clone()), (20, no_change.clone())]),
        Ok(alone.clone())
    );
    assert_eq!(joined(&[]), Ok(alone));
    // A notice of its own is kept first, written once when two bodies hold the same.
    let kept = joined(&[(21, guide(&latest)), (20, older.clone())]).unwrap();
    assert!(
        kept.starts_with("_Read the guide._\n\n### Changed\n\n- **Breaking:**"),
        "{kept}"
    );
    assert_eq!(
        joined(&[(21, guide(&latest)), (20, guide(&older))]),
        Ok(kept)
    );
    assert_eq!(
        joined(&[(21, guide(&Text::default()))]),
        Ok("_Read the guide._\n".to_owned())
    );
    // Two notices of their own: the release would hold two.
    let other = Text {
        notice: Some("_Another._".to_owned()),
        ..older
    };
    assert_eq!(
        joined(&[(21, guide(&latest)), (20, other)]),
        Err(
            "pull requests #21 and #20 each hold a notice: a release holds one at most, so \
             write it in one body"
                .to_owned()
        )
    );
}
