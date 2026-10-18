//! Tests of the commit message check (EN-108, F21.AC4): the cases of commitlint 21.2.3's own tests
//! (`@commitlint/rules/src/*.test.ts`, `@commitlint/is-ignored/src/is-ignored.test.ts`,
//! `REF-COMMITLINT-TESTS`) for each rule the project configures, at its configuration, then the
//! project's messages. Each verdict is commitlint's, as its test states it.

use git_hooks::case::Case;
use git_hooks::message::{
    Commit, SUBJECT_CASES, body_leading_blank, ends_with_full_stop, footer_leading_blank,
    format_report, header_trim, header_within, is_ignored, lint, max_line_length, parse, scope_in,
    subject_cases,
};

fn parsed(message: &str) -> Commit {
    parse(message, None).expect("a message")
}

/// The rules a message breaks, by name.
fn errors(message: &str) -> Vec<&'static str> {
    lint(message, None).map_or_else(Vec::new, |report| {
        report.errors.iter().map(|finding| finding.rule).collect()
    })
}

/// The rules a message warns about, by name.
fn warnings(message: &str) -> Vec<&'static str> {
    lint(message, None).map_or_else(Vec::new, |report| {
        report.warnings.iter().map(|finding| finding.rule).collect()
    })
}

/// The names of the cases a subject is in, among the project's four.
fn cases(message: &str) -> Vec<&'static str> {
    subject_cases(&parsed(message), &SUBJECT_CASES)
        .iter()
        .map(|case| case.name())
        .collect()
}

#[test]
fn parses_the_header_body_and_footer() {
    let commit = parsed("feat(model)!: add a shape\n\nWhy it is needed.\n\nRefs: US-004.T2\n");

    assert_eq!(commit.kind.as_deref(), Some("feat"));
    assert_eq!(commit.scope.as_deref(), Some("model"));
    assert_eq!(commit.subject.as_deref(), Some("add a shape"));
    assert_eq!(commit.body.as_deref(), Some("Why it is needed."));
    assert_eq!(commit.footer.as_deref(), Some("Refs: US-004.T2"));

    // A body line starting with `word: ` is a footer token: the body ends there (`AGENTS.md`).
    let token = parsed("docs: x\n\nNote: this line\nis read as a footer\n");
    assert_eq!(token.body, None);
    assert_eq!(
        token.footer.as_deref(),
        Some("Note: this line\nis read as a footer")
    );

    // An indented `key: value` is not a footer: the token starts the line.
    let indented = parsed("docs: x\n\n  key: value\n");
    assert_eq!(indented.body.as_deref(), Some("  key: value"));

    // A note keyword, case aside, starts the footer.
    let note = parsed("feat: x\n\nbreaking change: the API\nmore text\n");
    assert_eq!(
        note.footer.as_deref(),
        Some("breaking change: the API\nmore text")
    );

    // The comment character's lines and everything after the scissors are left out.
    let edited = parse(
        "docs: x\n# a comment\n\nbody\n# ------------------------ >8 ------------------------\ndiff\n",
        Some("#"),
    )
    .expect("a message");
    assert_eq!(edited.body.as_deref(), Some("body"));

    // A header that is not `type(scope): subject`: no type, scope nor subject.
    let free = parsed("WIP on develop: 46c66abe docs(backlog): set EN-104 done");
    assert_eq!((free.kind, free.scope, free.subject), (None, None, None));
    assert_eq!(parse(" \n\n", None), None);
}

#[test]
fn f21_ac4_subject_case_refuses_commitlint_s_refused_cases() {
    // `subject-case.test.ts`, `never` and the project's four cases. Refused:
    assert_eq!(
        cases("test: SUBJECT"),
        ["sentence-case", "start-case", "upper-case"]
    );
    assert_eq!(cases("test: Sub ject"), ["sentence-case"]);
    assert_eq!(cases("test: SubJect"), ["sentence-case", "pascal-case"]);
    assert_eq!(cases("test: ТақыРып"), ["sentence-case", "pascal-case"]);
    assert_eq!(cases("test: Sub Ject"), ["sentence-case", "start-case"]);
    assert_eq!(cases("test: Мав зуъ"), ["sentence-case"]);
    assert_eq!(
        cases("chore(deps): ОБНОВЛЕНЫ ВСЕ ЗАВИСИМОСТИ"),
        ["sentence-case", "start-case", "upper-case"]
    );
    // commitlint lists sentence-case alone for these two: `deburr` turns Û and Ä into U and A,
    // which this port leaves (module `case`); the verdict is the same.
    assert_eq!(
        cases("test: ÛNDERWERP"),
        ["sentence-case", "start-case", "upper-case"]
    );
    assert_eq!(cases("test: Äm Ne"), ["sentence-case", "start-case"]);
    // Passed: a lower case first letter, a caseless script, no letter first, no subject.
    for message in [
        "test: subject",
        "test: тема",
        "test: sUbJeCt",
        "test: 这是一次提交",
        "test: subJect",
        "test: θέΜα",
        "test: sub-ject",
        "test: áb-har",
        "test: sub_ject",
        "test: сэ_дэв",
        "chore(deps): обновлены все зависимости",
        "fix(面试): 修复评价功能",
        "feat(مميزات): إضافة وظيفة جديدة",
        "fix(תיקון): תיקון בעיה",
        "chore(deps): update зависимости",
        "test: 1.0.0",
        "test:\n",
    ] {
        assert_eq!(cases(message), Vec::<&str>::new(), "{message}");
    }

    // The test "with never should report only the case that matched".
    let pascal = parsed("test: SubJect");
    assert_eq!(
        subject_cases(&pascal, &[Case::Sentence, Case::Start]),
        [Case::Sentence]
    );
}

#[test]
fn f21_ac4_subject_full_stop_refuses_a_final_period() {
    // `subject-full-stop.test.ts`, `never .`.
    assert!(ends_with_full_stop(&parsed("test: subject.\n")));
    assert!(ends_with_full_stop(&parsed("type(scope): subject.\n")));
    assert!(ends_with_full_stop(&parsed("type.scope: subject.\n")));
    assert!(!ends_with_full_stop(&parsed("test: subject\n")));
    assert!(!ends_with_full_stop(&parsed(
        "test: subject ends with ellipsis..."
    )));
    // The header ends at its colon.
    assert!(!ends_with_full_stop(&parsed("test:\n")));
}

#[test]
fn f21_ac4_header_trim_refuses_white_space_around_the_header() {
    // `header-trim.test.ts`.
    assert_eq!(header_trim(&parsed("test: subject")), None);
    for message in [" test: subject", "\t\ttest: subject"] {
        assert_eq!(
            header_trim(&parsed(message)),
            Some("header must not start with whitespace")
        );
    }
    for message in ["test: subject  ", "test: subject\t\t"] {
        assert_eq!(
            header_trim(&parsed(message)),
            Some("header must not end with whitespace")
        );
    }
    for message in [
        " test: subject ",
        "\t\ttest: subject\t",
        "\t \ttest: subject \t  \t",
    ] {
        assert_eq!(
            header_trim(&parsed(message)),
            Some("header must not be surrounded by whitespace")
        );
    }
}

#[test]
fn f21_ac4_scope_enum_refuses_a_scope_out_of_the_list() {
    // `scope-enum.test.ts`, `always`.
    let scope = |message: &str| parsed(message).scope.unwrap_or_default();

    assert!(scope_in(&scope("foo(bar): baz"), &["bar"]));
    assert!(scope_in(&scope("foo(bar): baz"), &["bar", "baz"]));
    assert!(!scope_in(&scope("foo(bar): baz"), &["foo"]));
    for message in [
        "foo(bar,baz): qux",
        "foo(bar, baz): qux",
        "foo(bar/baz): qux",
    ] {
        assert!(scope_in(&scope(message), &["bar", "baz"]), "{message}");
        assert!(!scope_in(&scope(message), &["foo"]), "{message}");
        assert!(!scope_in(&scope(message), &["bar"]), "{message}");
    }
    assert!(scope_in(&scope("foo(bar/baz): qux"), &["bar/baz"]));
    // No scope, or an empty one: nothing to check.
    assert_eq!(parsed("foo: baz").scope, None);
    assert_eq!(parsed("foo(): baz").scope, None);
    // The project's list: `eslint` went with EN-085.
    assert_eq!(errors("chore(eslint): run oxlint first"), ["scope-enum"]);
    assert_eq!(errors("docs(backlog): set EN-103 done"), Vec::<&str>::new());
}

#[test]
fn f21_ac4_type_rules_refuse_an_empty_unknown_or_cased_type() {
    // `type-empty.test.ts`, `never`.
    assert!(errors("(scope):").contains(&"type-empty"));
    assert!(!errors("type: subject").contains(&"type-empty"));
    // `type-case.test.ts`, `always lower-case`.
    for message in [
        "tYpE: subject",
        "TYPE: subject",
        "tyPe: subject",
        "TyPe: subject",
    ] {
        assert!(errors(message).contains(&"type-case"), "{message}");
    }
    for message in [
        "(scope): subject",
        "type: subject",
        "ty_pe: subject",
        "ty-pe: subject",
    ] {
        assert!(!errors(message).contains(&"type-case"), "{message}");
    }
    // `type-enum`, `always` with config-conventional's types.
    assert_eq!(errors("a: subject"), ["type-enum"]);
    assert_eq!(errors("feat: subject"), Vec::<&str>::new());
}

#[test]
fn f21_ac4_subject_empty_refuses_a_header_without_subject() {
    // `subject-empty.test.ts`, `never`.
    assert!(errors("test: \nbody").contains(&"subject-empty"));
    assert!(!errors("test: subject\nbody").contains(&"subject-empty"));
}

#[test]
fn f21_ac4_lengths_refuse_long_lines_but_lines_with_urls() {
    // `header-max-length.test.ts`, a limit of 7: "test: a" fits, "test: ab" does not.
    assert!(header_within(&parsed("test: a"), 7));
    assert!(!header_within(&parsed("test: ab"), 7));
    // `body-max-line-length.test.ts`, a limit of 1.
    let body = |message: &str| parsed(message).body.unwrap_or_default();
    assert!(max_line_length(&body("test: subject\na"), 1));
    assert!(!max_line_length(&body("test: subject\nab"), 1));
    assert!(max_line_length(&body("test:subject\na\na\na"), 1));
    assert!(!max_line_length(&body("test:subject\na\nab\na"), 1));
    let url = "https://example.com/URL/with/a/very/long/path";
    assert!(max_line_length(
        &body(&format!("test:subject\na\n{url}\na")),
        1
    ));
    assert!(max_line_length(
        &body(&format!("test:subject\n\nThis is a [link]({url}).")),
        30
    ));
    // `footer-max-line-length.test.ts`, a limit of 18.
    let footer = |message: &str| parsed(message).footer.unwrap_or_default();
    assert!(max_line_length(
        &footer("test: subject\nBREAKING CHANGE: a"),
        18
    ));
    assert!(!max_line_length(
        &footer("test: subject\nBREAKING CHANGE: ab"),
        18
    ));
    // Counted in UTF-16 units: "docs: " and 94 arrows are 100, one more is 101.
    let arrows = "→".repeat(94);
    assert_eq!(errors(&format!("docs: {arrows}")), Vec::<&str>::new());
    assert_eq!(errors(&format!("docs: {arrows}→")), ["header-max-length"]);
}

#[test]
fn f21_ac4_leading_blanks_are_warnings() {
    // `body-leading-blank.test.ts`.
    assert!(body_leading_blank(&parsed("test: subject")));
    assert!(!body_leading_blank(&parsed("test: subject\nbody")));
    assert!(body_leading_blank(&parsed("test: subject\n\nbody")));
    // `footer-leading-blank.test.ts`, `always`.
    for message in [
        "test: subject",
        "test: subject\nbody",
        "test: subject\nbody\n\n",
        "feat(new-parser): introduces a new parsing library\n\nBREAKING CHANGE: new library does not support foo-construct",
        "test: subject\nbody\n\nBREAKING CHANGE: something important",
        "test: subject\nmulti\nline\nbody\n\nBREAKING CHANGE: something important",
        "fix: some issue\n\ndetailed explanation\n\ncloses #123",
    ] {
        assert!(footer_leading_blank(&parsed(message)), "{message:?}");
    }
    assert!(!footer_leading_blank(&parsed(
        "test: subject\nbody\nBREAKING CHANGE: something important"
    )));
    let commented = parse(
        "feat(new-parser): introduces a new parsing library\n\nBody Line 1\n# comment\nBody Line 2\n\nBREAKING CHANGE: new library does not support foo-construct",
        Some("#"),
    )
    .expect("a message");
    assert!(footer_leading_blank(&commented));
    // A warning lets the commit pass.
    assert_eq!(
        warnings("docs: x\nbody\nRefs: EN-108.T3"),
        ["body-leading-blank", "footer-leading-blank"]
    );
    assert_eq!(errors("docs: x\nbody\nRefs: EN-108.T3"), Vec::<&str>::new());
}

#[test]
fn f21_ac4_ignores_the_messages_commitlint_ignores() {
    // `is-ignored.test.ts`.
    let versions = [
        "0.0.1",
        "0.1.0",
        "1.0.0",
        "0.0.1-alpha",
        "0.0.1-some-crazy-tag",
        "0.0.1-0",
        "0.0.1-999",
        "0.0.1-alpha.0",
        "0.0.1-alpha.999",
        "0.0.1-some-crazy-tag.0",
        "0.0.1-some-crazy-tag.999",
        "0.0.1-1e69d54",
        "v0.0.1",
        " v3.0.0",
    ];
    let amendments = [
        "Signed-off-by: Developer <example@example.com>",
        "Change-Id: I895114872a515a269487a683124b63303818e19c",
        "Signed-off-by: Developer <example@example.com>\nChange-Id: I895114872a515a269487a683124b63303818e19c",
    ];

    for version in versions {
        assert!(is_ignored(version), "{version}");
        assert!(is_ignored(&format!("chore: {version}")), "{version}");
        assert!(
            is_ignored(&format!("chore(release): {version}")),
            "{version}"
        );
        for amendment in amendments {
            assert!(
                is_ignored(&format!("{version}\n\n{amendment}")),
                "{version}"
            );
        }
    }

    for message in [
        "Merge branch 'iss53'",
        "Merge branch 'ctrom-YarnBuild'\n",
        "Merge branch 'ctrom-YarnBuild'\r\n",
        "Merge branch 'ctrom-YarnBuild'\n\n\n",
        "Merge branch 'ctrom-YarnBuild'\r\n\r\n\r\n",
        "Merge pull request #369",
        "Merge branch 'ctrom-YarnBuild'\n ",
        "Merge branch 'ctrom-YarnBuild'\r\n # some comment",
        "Merge tag '1.1.1'",
        "Merge tag 'a tag'",
        "Merge tag '1.1.1'\n",
        "Merge tag '1.1.1'\r\n",
        "Merge tag '1.1.1'\n\n\n",
        "Merge tag '1.1.1'\r\n\r\n\r\n",
        "Merge tag '1.1.1'\n ",
        "Merge tag '1.1.1'\r\n # some comment",
        "Revert \"docs: add recipe for linting of all commits in a PR (#36)\"\n\nThis reverts commit 1e69d542c16c2a32acfd139e32efa07a45f19111.",
        "revert \"docs: add recipe for linting of all commits in a PR (#36)\"\n\nThis reverts commit 1e69d542c16c2a32acfd139e32efa07a45f19111.",
        "2.3.3-beta.1 [skip ci]",
        "2.3.3-beta.1 [ci skip]",
        "2.3.3-beta.1 [skip-ci]",
        "2.3.3-beta.1 [ci-skip]",
        "2.3.3-beta.1 (skip ci)",
        "2.3.3-beta.1 (ci skip)",
        "2.3.3-beta.1 (skip-ci)",
        "2.3.3-beta.1 (ci-skip)",
        "chore(release): 2.3.3-beta.1 [skip ci]",
        "chore(release): 2.3.3-beta.1 [ci skip]",
        "chore: 2.3.3-beta.1 [skip ci]",
        "chore: 2.3.3-beta.1 [ci skip]",
        "amend! initial commit",
        "fixup! initial commit",
        "squash! initial commit",
        "Merged in feature/facebook-friends-sync (pull request #8)",
        "Merged develop into feature/component-form-select-card",
        "Automatic merge",
        "Auto-merged develop into master",
        "Merge remote-tracking branch",
        "Merged PR 123: Description here",
        // The project's own: release-please's release commits of either branch (ADR-0067, the
        // case `tests/release-flow.test.ts` held) and Dependabot's.
        "chore(develop): release 0.3.0",
        "chore(main): release 0.2.0",
        "build(deps): bump x\n\nSigned-off-by: dependabot[bot] <support@github.com>",
    ] {
        assert!(is_ignored(message), "{message:?}");
    }

    for message in [
        "",
        "initial commit",
        "foo bar Merge branch xxx",
        "foo bar Merge tag '1.1.1'",
        "chore(feature): release 0.3.0",
        "chore(develop): released by hand",
        "feat(develop): release 0.3.0",
        "chore: 01.0.0",
    ] {
        assert!(!is_ignored(message), "{message:?}");
    }
}

#[test]
fn f21_ac4_passes_the_project_s_messages_and_names_what_it_refuses() {
    let task = "feat(tooling): check the commit message in Rust\n\nThe rules of commitlint, ported.\n\n\
                Refs: EN-108.T3\nCo-Authored-By: Claude Opus 5.5 <noreply@anthropic.com>\n";

    assert_eq!(
        lint(task, Some("#")).map(|report| format_report(&report)),
        Some(String::new())
    );
    // An id first is sentence case.
    assert_eq!(
        errors("docs(backlog): SP-023 adds a note"),
        ["subject-case"]
    );

    let report = lint("Docs(eslint): Add a rule.", None).expect("a report");
    assert_eq!(
        format_report(&report),
        "⧗   input: Docs(eslint): Add a rule.\n\
         ✖   subject must not be sentence-case [subject-case]\n\
         ✖   subject may not end with full stop [subject-full-stop]\n\
         ✖   type must be lower-case [type-case]\n\
         ✖   type must be one of [build, chore, ci, docs, feat, fix, perf, refactor, revert, style, test] [type-enum]\n\
         ✖   scope must be one of [adr, backlog, ci, deps, docs, geometry, interaction, io, math, model, playground, render, routing, tooling] [scope-enum]\n\
         \n✖   found 5 problems, 0 warnings\n\
         ⓘ   The rules: docs/tooling/git-workflow.md, Commit messages\n"
    );
    // A missing header is too long for commitlint, its length undefined.
    let headless = lint("#c\n\nbody\n", Some("#")).expect("a report");
    assert!(headless.errors.iter().any(|finding| finding.message
        == "header must not be longer than 100 characters, current length is undefined"));
}
