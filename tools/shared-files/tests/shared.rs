//! Tests of a story branch's id (EN-050).

use shared_files::shared::story_of;

#[test]
fn reads_the_story_of_a_story_branch() {
    assert_eq!(
        story_of("chore/en-050-shared-files-written-at-merge").as_deref(),
        Some("EN-050")
    );
    assert_eq!(
        story_of("refs/heads/feat/us-061-font-list").as_deref(),
        Some("US-061")
    );
    assert_eq!(story_of("docs/val-018-remarks").as_deref(), Some("VAL-018"));
    assert_eq!(story_of("feat/us-7").as_deref(), Some("US-7"));
    for branch in [
        "develop",
        "main",
        "feature/f18-safe-and-cheap-parallel-run",
        "feat/us-",
        "feat/us-12x-topic",
        "feat/-12-topic",
        "feat/US-12-topic",
    ] {
        assert_eq!(story_of(branch), None, "{branch}");
    }
}
