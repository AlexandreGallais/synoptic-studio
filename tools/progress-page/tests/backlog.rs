//! Tests of what the page reads from the backlog (EN-087).

use progress_page::backlog::{StoryRow, feature_of_branch, file_of, front_matter, story_rows};

const FEATURE: &str = "---
id: F20
epic: E17
title: \"A context diet for every agent\"
status: in-progress
---

# E17 · F20 — A context diet

| Subject | Decision |
| --- | --- |
| Run | a scope |

## Stories

| ID | Type | Title | Status |
| --- | --- | --- | --- |
| [SP-022](../stories/E17-F20-SP-022-research-f20.md) | Spike | Research for F20 | done |
| [EN-087](../stories/E17-F20-EN-087-progress-page.md) | Enabler | A progress page | ready |
| VAL-021 | Validation | Validate F20 | ready |

After the table.
";

#[test]
fn reads_a_field_of_the_front_matter_without_its_quotes() {
    assert_eq!(front_matter(FEATURE, "epic").as_deref(), Some("E17"));
    assert_eq!(
        front_matter(FEATURE, "title").as_deref(),
        Some("A context diet for every agent")
    );
    // A field after the front matter, a file without one, an empty file.
    assert_eq!(front_matter(FEATURE, "Run"), None);
    assert_eq!(front_matter("# Title\nepic: E17\n", "epic"), None);
    assert_eq!(front_matter("", "epic"), None);
}

#[test]
fn f20_ac5_reads_the_story_rows_of_the_feature_table_only() {
    let rows = story_rows(FEATURE);

    // The decisions table is skipped; a linked id is read as its label, a bare one as it is.
    assert_eq!(
        rows,
        vec![
            StoryRow {
                id: "SP-022".to_owned(),
                kind: "Spike".to_owned(),
                title: "Research for F20".to_owned(),
                status: "done".to_owned(),
            },
            StoryRow {
                id: "EN-087".to_owned(),
                kind: "Enabler".to_owned(),
                title: "A progress page".to_owned(),
                status: "ready".to_owned(),
            },
            StoryRow {
                id: "VAL-021".to_owned(),
                kind: "Validation".to_owned(),
                title: "Validate F20".to_owned(),
                status: "ready".to_owned(),
            },
        ]
    );
    assert_eq!(story_rows("| a |\n|"), Vec::new());
}

#[test]
fn reads_the_feature_of_a_feature_branch() {
    assert_eq!(
        feature_of_branch("origin/feature/f20-a-context-diet").as_deref(),
        Some("F20")
    );
    assert_eq!(feature_of_branch("feature/f7-x").as_deref(), Some("F7"));
    // Not a feature branch, no number, no topic after it.
    assert_eq!(feature_of_branch("chore/en-087-progress-page"), None);
    assert_eq!(feature_of_branch("feature/fx-topic"), None);
    assert_eq!(feature_of_branch("feature/f20"), None);
}

#[test]
fn finds_the_file_of_an_epic_or_a_feature_by_its_exact_id() {
    // Features listed first, so that the epic's own file must be told from its features'.
    let names: Vec<String> = [
        "docs/backlog/features/E17-F2-other.md",
        "docs/backlog/features/E17-F20-a-context-diet.md",
        "docs/backlog/epics/E1-draw.md",
        "docs/backlog/epics/E17-fast-tooling.md",
    ]
    .map(str::to_owned)
    .to_vec();

    assert_eq!(
        file_of(&names, "F20").map(String::as_str),
        Some("docs/backlog/features/E17-F20-a-context-diet.md")
    );
    assert_eq!(
        file_of(&names, "F2").map(String::as_str),
        Some("docs/backlog/features/E17-F2-other.md")
    );
    // An epic is the file starting with its id, never a feature file of that epic.
    assert_eq!(
        file_of(&names, "E17").map(String::as_str),
        Some("docs/backlog/epics/E17-fast-tooling.md")
    );
    assert_eq!(file_of(&names, "F21"), None);
}
