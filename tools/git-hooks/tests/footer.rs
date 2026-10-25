//! Tests of the story footer (EN-108, F21.AC4): a commit on a story branch names its task or its
//! story, with the branch's own id.

use git_hooks::footer::{carries_footer, footer_refusal, story_of_branch};

#[test]
fn reads_the_story_of_a_story_branch_only() {
    assert_eq!(
        story_of_branch("chore/en-108-git-hooks-in-rust").as_deref(),
        Some("EN-108")
    );
    assert_eq!(
        story_of_branch("feat/us-061-font").as_deref(),
        Some("US-061")
    );
    assert_eq!(
        story_of_branch("refs/heads/fix/en-050-x").as_deref(),
        Some("EN-050")
    );
    // Not asked: a feature branch, develop, main, a `docs/` branch, a branch without an id.
    for branch in [
        "feature/f21-agent-tools-in-rust",
        "develop",
        "main",
        "docs/sp-023-research-f21",
        "docs/val-018-remarks",
        "chore/topic-without-id",
        "",
    ] {
        assert_eq!(story_of_branch(branch), None, "{branch}");
    }
}

#[test]
fn f21_ac4_accepts_a_task_footer_of_the_branch_s_story() {
    assert!(carries_footer("feat: x\n\nRefs: EN-108.T3\n", "EN-108"));
    assert!(carries_footer(
        "feat: x\n\nRefs: ADR-0076, EN-108.T12\n",
        "EN-108"
    ));
    // A `Closes:` footer, of the done commits a story no longer has (AUD-053), another story, the
    // story without its task, a task in the body's text, a `Closes:` of a task.
    assert!(!carries_footer("docs: x\n\nCloses: EN-108\n", "EN-108"));
    assert!(!carries_footer("feat: x\n\nRefs: EN-109.T3\n", "EN-108"));
    assert!(!carries_footer("feat: x\n\nRefs: EN-108\n", "EN-108"));
    assert!(!carries_footer(
        "feat: x\n\nSee Refs: EN-108.T3\n",
        "EN-108"
    ));
    assert!(!carries_footer("feat: x\n\nCloses: EN-108.T3\n", "EN-108"));
    assert!(!carries_footer("feat: x\n\nRefs: EN-108.T\n", "EN-108"));
    assert!(!carries_footer("feat: x\n\nRefs: EN-1080.T1\n", "EN-108"));
}

#[test]
fn f21_ac4_refuses_a_story_commit_without_its_footer_naming_the_one_expected() {
    let refusal = footer_refusal(
        "feat(tooling): x\n\nRefs: EN-109.T1\n",
        "chore/en-108-git-hooks",
    );

    assert_eq!(
        refusal.as_deref(),
        Some(
            "✖   a commit on the story branch chore/en-108-git-hooks carries the footer \
             `Refs: EN-108.T<n>`, a task of its story [story-footer]"
        )
    );
    // Its footer, another branch, or a message commitlint ignores: nothing to refuse.
    assert_eq!(
        footer_refusal("feat: x\n\nRefs: EN-108.T1\n", "chore/en-108-x"),
        None
    );
    assert_eq!(
        footer_refusal("feat: x\n", "feature/f21-agent-tools-in-rust"),
        None
    );
    assert_eq!(footer_refusal("feat: x\n", "develop"), None);
    assert_eq!(footer_refusal("feat: x\n", "docs/val-018-remarks"), None);
    assert_eq!(footer_refusal("fixup! feat: x\n", "chore/en-108-x"), None);
}
