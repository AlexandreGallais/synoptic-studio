//! Tests of a story branch's id, the one reader of the workspace (moved from `shared-files`,
//! `worktree` and `git-hooks` at AUD-053).

use git_repository::story_of_branch;

#[test]
fn reads_the_story_of_a_story_branch() {
    for (branch, story) in [
        ("chore/en-050-shared-files-written-at-merge", "EN-050"),
        ("refs/heads/feat/us-061-font-list", "US-061"),
        ("docs/rev-003-review-e15", "REV-003"),
        ("chore/en-100", "EN-100"),
    ] {
        assert_eq!(story_of_branch(branch).as_deref(), Some(story), "{branch}");
    }
}

#[test]
fn reads_no_story_from_any_other_branch() {
    for branch in [
        "develop",
        "main",
        "feature/f18-safe-and-cheap-parallel-run",
        "chore/tidy-up",
        "feat/us-",
        "feat/us-7",
        "feat/us-0612-topic",
        "feat/us-12x-topic",
        "feat/-123-topic",
        "feat/US-123-topic",
        "feat/u1-123-topic",
        "feat/us-1é-topic",
    ] {
        assert_eq!(story_of_branch(branch), None, "{branch}");
    }
}
