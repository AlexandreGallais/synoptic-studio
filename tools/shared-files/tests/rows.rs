//! Tests of the rows moved from a story file into the logs at merge (EN-050, F18.AC6).

use std::fs;
use std::path::PathBuf;
use std::process::Command;

use shared_files::date::{civil_from_days, today};
use shared_files::rows::{LOGS, Place, SECTION, insert_rows, take_rows};

/// The day of the tests that insert rows: the journal's newest section below is that day's.
const TODAY: &str = "2026-10-16";

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
    let middle = "# S\n\n## For the main thread\n\n### Journal\n\n- `#run` a\n\n## Notes\n\nkept\n";

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
    let story = "## For the main thread\n\n### Feedback\n\n| 2026-10-17 | row |\n\n### Decisions\n\n- kept\n\n## Next\n\n| not a row |\n";
    let taken = take_rows(story);

    assert_eq!(taken.rows[0].1, ["| 2026-10-17 | row |"]);
    assert_eq!(taken.left, Vec::new());
    assert!(taken.story.contains("### Decisions\n\n- kept\n"));
    assert!(taken.story.contains("## Next\n\n| not a row |\n"));
}

#[test]
fn moves_only_dated_table_rows_into_a_table_log() {
    // EN-058 wrote a header and its separator, EN-055 and EN-061 bullets, US-061 `- None.`.
    let story = "# S\n\n## For the main thread\n\n### Improvements\n\n| Date | From | Item | Kind |\n| ---------- | :--: | --- | --- |\n| 2026-10-17 | EN-058 | An idea | tool |\n- A bullet note.\n| soon | EN-058 | Undated | tool |\n- None.\n\n### Feedback\n\n- None new.\n";
    let taken = take_rows(story);

    assert_eq!(
        taken.rows,
        [(
            LOGS[1],
            vec!["| 2026-10-17 | EN-058 | An idea | tool |".to_owned()]
        )]
    );
    assert_eq!(
        taken.left,
        [(
            LOGS[1],
            vec![
                "- A bullet note.".to_owned(),
                "| soon | EN-058 | Undated | tool |".to_owned()
            ]
        )]
    );
    // The lines left stay under their heading; the header, the placeholders and the empty
    // `### Feedback` go.
    assert_eq!(
        taken.story,
        "# S\n\n## For the main thread\n\n### Improvements\n\n- A bullet note.\n| soon | EN-058 | Undated | tool |\n"
    );
}

#[test]
fn moves_only_tagged_bullets_into_the_journal() {
    // The journal's bullets open with a tag, searched with `grep`: `- \`#run\` …`.
    let story = "# S\n\n## For the main thread\n\n### Journal\n\n- `#run` A finding.\n| 2026-10-17 | a row |\nA paragraph.\n- EN-055: untagged.\n- `#Run` Capital.\n- `#` Empty tag.\n- `#run`\n- None.\n- Nothing new\n- `#open-question` Another.\n\n## Next\n";
    let taken = take_rows(story);

    assert_eq!(
        taken.rows,
        [(
            LOGS[0],
            vec![
                "- `#run` A finding.".to_owned(),
                "- `#open-question` Another.".to_owned()
            ]
        )]
    );
    assert_eq!(
        taken.left,
        [(
            LOGS[0],
            vec![
                "| 2026-10-17 | a row |".to_owned(),
                "A paragraph.".to_owned(),
                "- EN-055: untagged.".to_owned(),
                "- `#Run` Capital.".to_owned(),
                "- `#` Empty tag.".to_owned(),
                "- `#run`".to_owned()
            ]
        )]
    );
    assert_eq!(
        taken.story,
        "# S\n\n## For the main thread\n\n### Journal\n\n| 2026-10-17 | a row |\nA paragraph.\n- EN-055: untagged.\n- `#Run` Capital.\n- `#` Empty tag.\n- `#run`\n\n## Next\n"
    );
}

#[test]
fn drops_a_subsection_holding_only_placeholders() {
    for placeholder in ["- None.", "- none new", "- Nothing.", "- N/A", "| None |"] {
        let story = format!("# S\n\n## For the main thread\n\n### Feedback\n\n{placeholder}\n");
        let taken = take_rows(&story);

        assert!(taken.rows.is_empty(), "{placeholder}");
        assert!(taken.left.is_empty(), "{placeholder}");
        assert_eq!(taken.story, "# S\n", "{placeholder}");
    }
    // A bullet that only starts like a placeholder is a journal row.
    let taken =
        take_rows("## For the main thread\n\n### Journal\n\n- `#tests` None of them failed.\n");

    assert_eq!(taken.rows[0].1, ["- `#tests` None of them failed."]);
}

#[test]
fn appends_rows_after_the_first_table() {
    let log =
        "# Log\n\nIntro.\n\n| Date | Item |\n| --- | --- |\n| 1 | a |\n\nAfter.\n\n| other |\n";
    let rows = ["| 2 | b |".to_owned()];

    assert_eq!(
        insert_rows(log, Place::Table, &rows, TODAY).unwrap(),
        "# Log\n\nIntro.\n\n| Date | Item |\n| --- | --- |\n| 1 | a |\n| 2 | b |\n\nAfter.\n\n| other |\n"
    );
    // A table ending the file.
    assert_eq!(
        insert_rows("| a |\n| - |\n", Place::Table, &rows, TODAY).unwrap(),
        "| a |\n| - |\n| 2 | b |\n"
    );
    assert!(insert_rows("# Log\n", Place::Table, &rows, TODAY).is_err());
}

#[test]
fn appends_journal_rows_to_the_newest_section() {
    let log = "# Journal\n\nNewest first.\n\n## 2026-10-16 — Run\n\n- `#run` a\n\n## 2026-10-15 — Old\n\n- `#run` b\n";
    let rows = ["- `#open` c".to_owned()];

    assert_eq!(
        insert_rows(log, Place::NewestSection, &rows, TODAY).unwrap(),
        "# Journal\n\nNewest first.\n\n## 2026-10-16 — Run\n\n- `#run` a\n- `#open` c\n\n## 2026-10-15 — Old\n\n- `#run` b\n"
    );
    // The newest section ending the file.
    assert_eq!(
        insert_rows("# J\n\n## Day\n\n- a\n", Place::NewestSection, &rows, TODAY).unwrap(),
        "# J\n\n## Day\n\n- a\n- `#open` c\n"
    );
    assert!(insert_rows("# J\n", Place::NewestSection, &rows, TODAY).is_err());
}

#[test]
fn opens_a_section_dated_today_when_the_newest_one_is_older() {
    let log = "# Journal\n\nNewest first.\n\n## 2026-10-16 — Run\n\n- `#run` a\n";
    let rows = ["- `#run` b".to_owned(), "- `#open` c".to_owned()];

    // The newest section is the day before: today's rows go into a new section above it.
    assert_eq!(
        insert_rows(log, Place::NewestSection, &rows, "2026-10-17").unwrap(),
        "# Journal\n\nNewest first.\n\n## 2026-10-17 — Run continued\n\n- `#run` b\n- `#open` c\n\n## 2026-10-16 — Run\n\n- `#run` a\n"
    );
    // A newest section dated after today (a clock behind) still takes them.
    assert_eq!(
        insert_rows(log, Place::NewestSection, &rows, "2026-10-15").unwrap(),
        "# Journal\n\nNewest first.\n\n## 2026-10-16 — Run\n\n- `#run` a\n- `#run` b\n- `#open` c\n"
    );
    // A table log never opens a section, whatever its dates.
    assert_eq!(
        insert_rows(
            "## 2026-10-01\n\n| a |\n",
            Place::Table,
            &rows[..1],
            "2026-10-17"
        )
        .unwrap(),
        "## 2026-10-01\n\n| a |\n- `#run` b\n"
    );
}

#[test]
fn writes_the_date_a_number_of_days_after_1970_falls_on() {
    // 0 days: 1970-01-01; -1: the day before.
    assert_eq!(civil_from_days(0), "1970-01-01");
    assert_eq!(civil_from_days(-1), "1969-12-31");
    // 31 days of January + 28 of February 1970: 1970-03-01.
    assert_eq!(civil_from_days(59), "1970-03-01");
    // 30 years of 365 days + 7 leap days (1972 … 1996) = 10 957: 2000-01-01; + 31 + 28 = 11 016:
    // 2000-02-29, a leap day of a year divisible by 400.
    assert_eq!(civil_from_days(10_957), "2000-01-01");
    assert_eq!(civil_from_days(11_016), "2000-02-29");
    // 2026-10-17: 56 years × 365 = 20 440 days + 14 leap days (1972 … 2024) = 20 454, then
    // 273 + 16 = 289 days of 2026 before it (Jan 31, Feb 28, Mar 31, Apr 30, May 31, Jun 30,
    // Jul 31, Aug 31, Sep 30 = 273) → 20 743.
    assert_eq!(civil_from_days(20_743), "2026-10-17");
    assert_eq!(today().len(), 10);
    assert!(today().as_str() > "2026-10-16");
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
    // The journal's newest section is older than today: the rows open today's (AUD-020).
    assert_eq!(
        read("docs/process/journal.md"),
        format!(
            "# Journal\n\n## {} — Run continued\n\n- `#run` A finding.\n- `#open` A question left.\n\n## 2026-10-16 — Run\n\n- `#run` before\n",
            today()
        )
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

#[test]
fn leaves_a_line_of_another_shape_in_the_story_and_names_it() {
    let root: PathBuf = std::env::temp_dir().join(format!("move-rows-left-{}", std::process::id()));
    let _ = fs::remove_dir_all(&root);
    fs::create_dir_all(root.join("docs/backlog/stories")).unwrap();
    fs::create_dir_all(root.join("docs/process")).unwrap();

    let story = "docs/backlog/stories/E17-F18-US-998-a-story.md";
    let feedback = "# Feedback\n\n| Date | What |\n| --- | --- |\n| 2026-10-16 | Old |\n";
    let read = |path: &str| fs::read_to_string(root.join(path)).unwrap();

    fs::write(
        root.join(story),
        "# US-998\n\n## For the main thread\n\n### Feedback\n\n| Date | What |\n| --- | --- |\n| 2026-10-17 | New |\n- A note.\n\n### Journal\n\n- None.\n",
    )
    .unwrap();
    fs::write(root.join("docs/process/feedback.md"), feedback).unwrap();

    let output = Command::new(env!("CARGO_BIN_EXE_move-rows"))
        .arg("feat/us-998-a-story")
        .current_dir(&root)
        .output()
        .unwrap();

    assert!(output.status.success(), "{output:?}");
    assert_eq!(
        String::from_utf8_lossy(&output.stdout),
        format!("docs/process/feedback.md\n{story}\n")
    );
    assert_eq!(
        String::from_utf8_lossy(&output.stderr),
        format!(
            "move-rows: left in {story} under ### Feedback, not a row of docs/process/feedback.md: - A note.\n"
        )
    );
    // Only the dated row reached the log; the note stays in the story, the placeholder goes.
    assert_eq!(
        read("docs/process/feedback.md"),
        format!("{feedback}| 2026-10-17 | New |\n")
    );
    assert_eq!(
        read(story),
        "# US-998\n\n## For the main thread\n\n### Feedback\n\n- A note.\n"
    );

    // A story holding only a placeholder: no log written, the story file cleaned all the same.
    let only = "docs/backlog/stories/E17-F18-US-997-another.md";

    fs::write(
        root.join(only),
        "# US-997\n\n## For the main thread\n\n### Improvements\n\n- None.\n",
    )
    .unwrap();

    let output = Command::new(env!("CARGO_BIN_EXE_move-rows"))
        .arg("feat/us-997-another")
        .current_dir(&root)
        .output()
        .unwrap();

    assert_eq!(String::from_utf8_lossy(&output.stdout), format!("{only}\n"));
    assert_eq!(read(only), "# US-997\n");

    let _ = fs::remove_dir_all(&root);
}
