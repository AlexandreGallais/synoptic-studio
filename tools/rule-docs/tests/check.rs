//! Tests of the docs a rule touches (EN-059, F18.AC16), without Git.

use rule_docs::check::{
    Doc, closed_stories, docs_of, is_story, missing, none_affected, of_story, refusal, story_files,
};

/// No doc missing.
const NO_DOC: [Doc; 0] = [];

/// No id nor path.
const NOTHING: [&str; 0] = [];

/// Paths as the changed files of a story.
fn files(paths: &[&str]) -> Vec<String> {
    paths.iter().map(|&path| path.to_owned()).collect()
}

/// The docs of a feature's story, F18's.
fn f18() -> Vec<Doc> {
    vec![
        Doc::QRow,
        Doc::Decisions("F18".into()),
        Doc::Guide("f18-".into()),
    ]
}

#[test]
fn reads_the_stories_a_done_commit_closes() {
    assert_eq!(
        closed_stories("docs(backlog): set EN-059 done\n\nCloses: EN-059\n"),
        ["EN-059"]
    );
    assert_eq!(
        closed_stories("docs(backlog): set VAL-019 and F18 done\n\nCloses: VAL-019, F18\n"),
        ["VAL-019"]
    );
    assert_eq!(closed_stories("feat: a task\n\nRefs: EN-059.T1\n"), NOTHING);
    assert_eq!(
        closed_stories("feat: a task\n\n# Closes: EN-059\n"),
        NOTHING
    );
    for id in ["F18", "E17", "EN-059.T1", "en-059", "EN-", "-059", "EN-05x"] {
        assert!(!is_story(id), "{id}");
    }
}

#[test]
fn knows_a_story_commit_by_its_footers() {
    assert!(of_story("feat: a task\n\nRefs: EN-059.T1\n", "EN-059"));
    assert!(of_story("feat: a task\n\nRefs: EN-059\n", "EN-059"));
    assert!(of_story("docs: done\n\nCloses: EN-059\n", "EN-059"));
    assert!(of_story(
        "feat: two\n\nRefs: US-061.T2, EN-059.T12\n",
        "EN-059"
    ));
    for message in [
        "feat: a task\n\nRefs: EN-0590\n",
        "feat: a task\n\nRefs: EN-059.T\n",
        "feat: a task\n\nRefs: EN-059.Tx\n",
        "feat: a task naming EN-059 in its body\n\nRefs: US-061\n",
        "feat: a task\n\n# Refs: EN-059\n",
    ] {
        assert!(!of_story(message, "EN-059"), "{message}");
    }
}

#[test]
fn reads_the_line_saying_none_is_affected() {
    assert!(none_affected(
        "docs: done\n\nRule-docs: none affected, the wording only\nCloses: EN-059\n"
    ));
    for message in [
        "docs: done\n\nRule-docs: none affected, \nCloses: EN-059\n",
        "docs: done\n\nRule-docs: none affected\nCloses: EN-059\n",
        "docs: done\n\nrule-docs: none affected, a reason\n",
        "docs: done\n\n# Rule-docs: none affected, a reason\n",
    ] {
        assert!(!none_affected(message), "{message}");
    }
}

#[test]
fn reads_the_files_of_the_story_commits_from_the_log() {
    let log = "\u{1e}feat: a task\n\nRefs: EN-059.T1\n\u{1f}\n\ndocs/domain/shapes.md\nsrc/a.ts\n\
               \u{1e}feat: another story\n\nRefs: EN-0590\n\u{1f}\n\ndocs/guide/f18-run.md\n\
               \u{1e}docs: done\n\nCloses: EN-059\n\u{1f}\n\ndocs/domain/README.md\n";

    assert_eq!(
        story_files(log, "EN-059"),
        ["docs/domain/shapes.md", "src/a.ts", "docs/domain/README.md"]
    );
    assert_eq!(story_files("", "EN-059"), NOTHING);
}

#[test]
fn reads_the_docs_from_the_story_file_name() {
    assert_eq!(
        docs_of("E17-F18-EN-059-done-commit-checks-the-docs.md", "EN-059"),
        Some(f18())
    );
    assert_eq!(
        docs_of("E15-NAM-001-naming-pass-e15.md", "NAM-001"),
        Some(vec![Doc::QRow, Doc::Guide("e15-".into())])
    );
    for name in [
        "E17-F18-EN-058-topic.md",
        "E17-F18-EN-0590-topic.md",
        "README.md",
        "X17-F18-EN-059-topic.md",
        "E17-G18-EN-059-topic.md",
        "E17-F18-X-EN-059-topic.md",
    ] {
        assert_eq!(docs_of(name, "EN-059"), None, "{name}");
    }
}

#[test]
fn names_each_doc_by_its_path() {
    assert!(Doc::QRow.is("docs/domain/README.md"));
    assert!(!Doc::QRow.is("docs/domain/shapes.md"));
    assert!(
        Doc::Decisions("F18".into()).is("docs/backlog/stories/E17-F18-VAL-019-validate-f18.md")
    );
    assert!(!Doc::Decisions("F18".into()).is("docs/backlog/stories/E17-F18-EN-059-topic.md"));
    assert!(
        !Doc::Decisions("F18".into()).is("docs/backlog/stories/E17-F19-VAL-020-validate-f19.md")
    );
    assert!(
        !Doc::Decisions("F1".into()).is("docs/backlog/stories/E17-F18-VAL-019-validate-f18.md")
    );
    assert!(Doc::Guide("f18-".into()).is("docs/guide/f18-parallel-run.md"));
    assert!(!Doc::Guide("f18-".into()).is("docs/guide/f18-images/shot.png"));
    assert!(!Doc::Guide("f18-".into()).is("docs/guide/f18-images/notes.md"));
    assert!(Doc::Guide("f18-".into()).is("docs/guide/f18-run.MD"));
    assert!(!Doc::Guide("f18-".into()).is("docs/guide/f19-rust.md"));
    assert!(!Doc::Guide("f1-".into()).is("docs/guide/f18-parallel-run.md"));
}

#[test]
fn asks_for_every_doc_a_rule_change_lacks() {
    // [F18.AC16] a rule changed alone: the three docs are asked for.
    assert_eq!(missing(&f18(), &files(&["docs/domain/shapes.md"])), f18());
    assert_eq!(
        missing(
            &f18(),
            &files(&[
                "docs/domain/shapes.md",
                "docs/domain/README.md",
                "docs/guide/f18-run.md"
            ])
        ),
        [Doc::Decisions("F18".into())]
    );
    assert_eq!(
        missing(
            &f18(),
            &files(&[
                "docs/domain/interaction.md",
                "docs/domain/README.md",
                "docs/backlog/stories/E17-F18-VAL-019-validate-f18.md",
                "docs/guide/f18-run.md",
            ])
        ),
        NO_DOC
    );
}

#[test]
fn asks_nothing_of_a_story_that_changes_no_rule() {
    assert_eq!(
        missing(&f18(), &files(&["src/index.ts", "docs/guide/f18-run.md"])),
        NO_DOC
    );
    assert_eq!(missing(&f18(), &[]), NO_DOC);
}

#[test]
fn names_what_is_missing_and_the_way_out() {
    let text = refusal("EN-059", &[Doc::QRow, Doc::Guide("f18-".into())]);

    assert!(
        text.contains("EN-059 changes a rule in docs/domain/"),
        "{text}"
    );
    assert!(
        text.contains("  the Q row in docs/domain/README.md\n"),
        "{text}"
    );
    assert!(
        text.contains("  the guide page docs/guide/f18-*.md\n"),
        "{text}"
    );
    assert!(text.contains("`Rule-docs: none affected, <why>`"), "{text}");
    assert_eq!(refusal("EN-059", &[]), "");
    assert!(
        refusal("EN-059", &[Doc::Decisions("F18".into())])
            .contains("the decisions to confirm of F18's VAL")
    );
}
