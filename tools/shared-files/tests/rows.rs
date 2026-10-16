//! Tests of the rows moved from a story file into the logs at merge (EN-050, F18.AC6).

use std::fs;
use std::path::PathBuf;
use std::process::Command;

use shared_files::rows::{LOGS, Place, SECTION, insert_rows, take_rows};

/// A story file with rows for two logs and a note for the main thread.
const STORY: &str = "\
---
id: EN-999
---

# EN-999 — A story

## Tasks

- [x] T1 — A task

## For the main thread

A question for the Product Owner.

### Improvements

| 2026-10-16 | agent | An idea |

### Journal

- `#run` A finding.
- `#open` A question left.
";

#[test]
fn takes_the_rows_of_each_known_heading() {
    let taken = take_rows(STORY);
    let logs: Vec<(&str, Vec<String>)> = taken
        .rows
        .iter()
        .map(|(log, rows)| (log.path, rows.clone()))
        .collect();

    // In the order of the logs, the journal first, whatever their order in the story.
    assert_eq!(
        logs,
        [
            (
                "docs/process/journal.md",
                vec![
                    "- `#run` A finding.".to_owned(),
                    "- `#open` A question left.".to_owned()
                ]
            ),
            (
                "docs/process/improvements.md",
                vec!["| 2026-10-16 | agent | An idea |".to_owned()]
            ),
        ]
    );
    // The note stays for the main thread; the headings and their rows go.
    assert!(
        taken
            .story
            .contains(&format!("{SECTION}\n\nA question for the Product Owner.\n"))
    );
    assert!(!taken.story.contains("### "), "{}", taken.story);
    assert!(!taken.story.contains("An idea"), "{}", taken.story);
    assert!(taken.story.starts_with("---\nid: EN-999\n---\n\n# EN-999"));
    assert!(taken.story.ends_with('\n'));
}

#[test]
fn removes_a_section_left_empty() {
    let story = "# S\n\n## For the main thread\n\n### Feedback\n\n| 2026-10-16 | a | b | c |\n";
    let taken = take_rows(story);

    assert_eq!(taken.story, "# S\n");
    assert_eq!(taken.rows.len(), 1);
    assert_eq!(taken.rows[0].0, LOGS[2]);

    // A section followed by another keeps one blank line before it.
    let middle = "# S\n\n## For the main thread\n\n### Journal\n\n- a\n\n## Notes\n\nkept\n";

    assert_eq!(take_rows(middle).story, "# S\n\n## Notes\n\nkept\n");
}

#[test]
fn leaves_alone_a_story_without_rows() {
    for story in [
        "# S\n\n## Tasks\n\n- [ ] T1\n",
        "# S\n\n## For the main thread\n\n- A question, under no log heading.\n",
        "# S\n\n## For the main thread\n\n### Decisions\n\n- kept\n",
        "# S\n\n## For the main thread\n\n### Journal\n\n",
    ] {
        let taken = take_rows(story);

        assert!(taken.rows.is_empty(), "{story}");
        if !story.contains("### Journal") {
            assert_eq!(taken.story, story);
        }
    }
}

#[test]
fn stops_a_log_heading_at_the_next_heading() {
    let story = "## For the main thread\n\n### Feedback\n\n| row |\n\n### Decisions\n\n- kept\n\n## Next\n\n| not a row |\n";
    let taken = take_rows(story);

    assert_eq!(taken.rows[0].1, ["| row |"]);
    assert!(taken.story.contains("### Decisions\n\n- kept\n"));
    assert!(taken.story.contains("## Next\n\n| not a row |\n"));
}

#[test]
fn appends_rows_after_the_first_table() {
    let log =
        "# Log\n\nIntro.\n\n| Date | Item |\n| --- | --- |\n| 1 | a |\n\nAfter.\n\n| other |\n";
    let rows = ["| 2 | b |".to_owned()];

    assert_eq!(
        insert_rows(log, Place::Table, &rows).unwrap(),
        "# Log\n\nIntro.\n\n| Date | Item |\n| --- | --- |\n| 1 | a |\n| 2 | b |\n\nAfter.\n\n| other |\n"
    );
    // A table ending the file.
    assert_eq!(
        insert_rows("| a |\n| - |\n", Place::Table, &rows).unwrap(),
        "| a |\n| - |\n| 2 | b |\n"
    );
    assert!(insert_rows("# Log\n", Place::Table, &rows).is_err());
}

#[test]
fn appends_journal_rows_to_the_newest_section() {
    let log = "# Journal\n\nNewest first.\n\n## 2026-10-16 — Run\n\n- `#run` a\n\n## 2026-10-15 — Old\n\n- `#run` b\n";
    let rows = ["- `#open` c".to_owned()];

    assert_eq!(
        insert_rows(log, Place::NewestSection, &rows).unwrap(),
        "# Journal\n\nNewest first.\n\n## 2026-10-16 — Run\n\n- `#run` a\n- `#open` c\n\n## 2026-10-15 — Old\n\n- `#run` b\n"
    );
    // The newest section ending the file.
    assert_eq!(
        insert_rows("# J\n\n## Day\n\n- a\n", Place::NewestSection, &rows).unwrap(),
        "# J\n\n## Day\n\n- a\n- `#open` c\n"
    );
    assert!(insert_rows("# J\n", Place::NewestSection, &rows).is_err());
}

#[test]
fn moves_the_rows_of_a_merged_story_into_the_logs() {
    let root: PathBuf = std::env::temp_dir().join(format!("move-rows-{}", std::process::id()));
    let _ = fs::remove_dir_all(&root);
    fs::create_dir_all(root.join("docs/backlog/stories")).unwrap();
    fs::create_dir_all(root.join("docs/process")).unwrap();

    let story = "docs/backlog/stories/E17-F18-EN-999-a-story.md";
    let journal = "# Journal\n\n## 2026-10-16 — Run\n\n- `#run` before\n";
    let improvements = "# Improvement log\n\n| Date | From | Item |\n| --- | --- | --- |\n| 2026-10-15 | agent | Old |\n";

    fs::write(root.join(story), STORY).unwrap();
    fs::write(root.join("docs/process/journal.md"), journal).unwrap();
    fs::write(root.join("docs/process/improvements.md"), improvements).unwrap();
    fs::write(root.join("docs/process/feedback.md"), "# Feedback\n").unwrap();

    let run = |branch: &str| {
        Command::new(env!("CARGO_BIN_EXE_move-rows"))
            .arg(branch)
            .current_dir(&root)
            .output()
            .unwrap()
    };
    let output = run("chore/en-999-a-story");
    let read = |path: &str| fs::read_to_string(root.join(path)).unwrap();

    // [F18.AC6] the rows under the known headings are in their logs, gone from the story.
    assert!(output.status.success(), "{output:?}");
    assert_eq!(
        String::from_utf8_lossy(&output.stdout),
        format!("docs/process/journal.md\ndocs/process/improvements.md\n{story}\n")
    );
    assert_eq!(
        read("docs/process/journal.md"),
        "# Journal\n\n## 2026-10-16 — Run\n\n- `#run` before\n- `#run` A finding.\n- `#open` A question left.\n"
    );
    assert!(
        read("docs/process/improvements.md")
            .ends_with("| 2026-10-15 | agent | Old |\n| 2026-10-16 | agent | An idea |\n")
    );
    assert_eq!(read("docs/process/feedback.md"), "# Feedback\n");
    assert!(!read(story).contains("An idea"));

    // Run again: nothing left to move; another branch moves nothing.
    assert_eq!(
        String::from_utf8_lossy(&run("chore/en-999-a-story").stdout),
        ""
    );
    assert_eq!(String::from_utf8_lossy(&run("develop").stdout), "");
    assert_eq!(String::from_utf8_lossy(&run("feat/us-1-none").stdout), "");

    let _ = fs::remove_dir_all(&root);
}
