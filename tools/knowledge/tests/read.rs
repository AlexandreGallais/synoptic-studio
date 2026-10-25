//! Tests of what a branch adds, reads and marks merged (EN-152): the rows of three branches and
//! their states, the filters, the search ranked by BM25 and the text read back. Every store is
//! made in a folder of the system's temporary folder; the real records folder is never opened
//! (`[F48.AC4]`).

use std::fs;
use std::path::{Path, PathBuf};

use knowledge::add::{Origin, add_document, is_kind, next_id};
use knowledge::read::{
    Filters, Hit, Listed, hit_lines, list, listed_line, parse_filters, search, section_of, show,
};
use knowledge::store::{open_store, write};
use knowledge::sync::{mark_merged, merged_branches};
use rusqlite::Connection;

/// A store of its own in the temporary folder, emptied first.
fn place(name: &str) -> PathBuf {
    let folder = std::env::temp_dir().join(format!("knowledge-read-{name}-{}", std::process::id()));
    let _ = fs::remove_dir_all(&folder);

    folder.join("records.sqlite")
}

/// Removes a test's folder.
fn clean(file: &Path) {
    let _ = fs::remove_dir_all(file.parent().unwrap());
}

/// A story of an id, a title and a line of text.
fn story(id: &str, title: &str, line: &str) -> String {
    format!(
        "---\nid: {id}\nepic: E49\nfeature: F48\nstatus: ready\ndate: 2026-10-20\n---\n\n# {title}\n\n{line}\n"
    )
}

/// Adds a text of a kind on a branch, the day 2026-10-24; its id.
fn add(database: &Connection, kind: &str, branch: &str, text: &str) -> String {
    write(database, |transaction| {
        add_document(
            transaction,
            text,
            Origin {
                kind,
                path: "",
                branch,
                today: "2026-10-24",
            },
        )
    })
    .unwrap()
}

/// Sets the state of a branch's rows.
fn set_state(database: &Connection, branch: &str, state: &str) {
    database
        .execute(
            "UPDATE documents SET state = ? WHERE branch = ?",
            [state, branch],
        )
        .unwrap();
}

/// The ids of the documents a branch lists, filtered.
fn ids(database: &Connection, reader: &str, filters: &Filters) -> Vec<String> {
    list(database, reader, filters)
        .unwrap()
        .into_iter()
        .map(|listed| listed.id)
        .collect()
}

/// The rows of three branches and a fourth dropped: `feature/a` merged, `chore/b` and `chore/c`
/// drafts, `chore/d` dropped.
fn three_branches(file: &Path) -> Connection {
    let database = open_store(file).unwrap();

    add(
        &database,
        "story",
        "feature/a",
        &story("EN-901", "Merged", "Busy."),
    );
    add(
        &database,
        "story",
        "chore/b",
        &story("EN-902", "Of b", "Busy."),
    );
    add(
        &database,
        "story",
        "chore/c",
        &story("EN-903", "Of c", "Busy."),
    );
    add(
        &database,
        "story",
        "chore/d",
        &story("EN-904", "Of d", "Busy."),
    );
    set_state(&database, "feature/a", "merged");
    set_state(&database, "chore/d", "dropped");
    database
}

/// The ids every document cites, sorted.
fn cited(database: &Connection) -> Vec<String> {
    database
        .prepare("SELECT \"to\" FROM links ORDER BY \"to\"")
        .unwrap()
        .query_map([], |row| row.get(0))
        .unwrap()
        .collect::<rusqlite::Result<_>>()
        .unwrap()
}

/// Filters of the words given.
fn filters(words: &[&str]) -> Filters {
    let words: Vec<String> = words.iter().map(|word| (*word).to_owned()).collect();

    parse_filters(&words).unwrap().1
}

#[test]
fn f48_ac2_stores_an_added_story_as_a_draft_of_its_branch_with_its_tables() {
    let file = place("added");
    let database = open_store(&file).unwrap();
    let text = story("EN-900", "E49 · F48 · EN-900 — Add", "As ADR-0081 says.");
    let id = add(&database, "story", "chore/en-900", &text);
    let row: (String, String, String, String, String, String, String, String, i64) = database
        .query_row(
            "SELECT kind, title, epic, feature, status, date, branch, state, private FROM documents WHERE id = ?",
            [&id],
            |row| {
                Ok((
                    row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?, row.get(4)?,
                    row.get(5)?, row.get(6)?, row.get(7)?, row.get(8)?,
                ))
            },
        )
        .unwrap();
    let count = |table: &str| -> i64 {
        database
            .query_row(&format!("SELECT count(*) FROM {table}"), [], |row| {
                row.get(0)
            })
            .unwrap()
    };
    let links = cited(&database);
    let shown = show(&database, "chore/en-900", "EN-900", &Filters::default()).unwrap();
    let (fields, sections) = (count("fields"), count("sections"));

    clean(&file);
    assert_eq!(id, "EN-900");
    // No title in the front matter: the first heading's.
    assert_eq!(
        row,
        (
            "story".into(),
            "E49 · F48 · EN-900 — Add".into(),
            "E49".into(),
            "F48".into(),
            "ready".into(),
            "2026-10-20".into(),
            "chore/en-900".into(),
            "draft".into(),
            0
        )
    );
    // id, epic, feature, status, date; the front matter, then the title's section.
    assert_eq!(fields, 5);
    assert_eq!(sections, 2);
    assert_eq!(links, ["ADR-0081", "E49", "F48"]);
    assert_eq!(shown.as_deref(), Some(text.as_str()));
}

#[test]
fn f48_ac2_makes_the_id_of_a_log_row_unique_across_branches() {
    let file = place("log-rows");
    let database = open_store(&file).unwrap();
    let first = add(&database, "journal", "chore/b", "- `#run` EN-050: one.\n");
    // Another branch's row of the same day takes the next number, though chore/b's is a draft.
    let second = add(&database, "journal", "chore/c", "- `#run` EN-050: two.\n");
    let other_day = next_id(&database, "journal", "2026-10-25").unwrap();
    let date: String = database
        .query_row(
            "SELECT date FROM documents WHERE id = ?",
            [&second],
            |row| row.get(0),
        )
        .unwrap();

    clean(&file);
    assert_eq!(first, "JOURNAL-2026-10-24-1");
    assert_eq!(second, "JOURNAL-2026-10-24-2");
    assert_eq!(other_day, "JOURNAL-2026-10-25-1");
    assert_eq!(date, "2026-10-24");
}

#[test]
fn f48_ac2_numbers_after_the_highest_not_the_count() {
    let file = place("highest");
    let database = open_store(&file).unwrap();

    add(
        &database,
        "journal",
        "chore/b",
        "---\nid: JOURNAL-2026-10-24-9\n---\nrow\n",
    );

    let next = next_id(&database, "journal", "2026-10-24").unwrap();

    clean(&file);
    // One row, numbered 9: the next is 10, and "-10" sorts before "-9" as text, so the number is
    // read as a number.
    assert_eq!(next, "JOURNAL-2026-10-24-10");
}

#[test]
fn f48_ac2_replaces_its_own_branchs_row_when_added_again() {
    let file = place("again");
    let database = open_store(&file).unwrap();

    add(
        &database,
        "story",
        "chore/b",
        &story("EN-902", "First", "ADR-0001."),
    );

    let again = story("EN-902", "Second", "ADR-0002.");

    add(&database, "story", "chore/b", &again);

    let rows: i64 = database
        .query_row("SELECT count(*) FROM documents", [], |row| row.get(0))
        .unwrap();
    let links = cited(&database);
    let shown = show(&database, "chore/b", "EN-902", &Filters::default()).unwrap();

    clean(&file);
    assert_eq!(rows, 1);
    // The first text's citation of ADR-0001 went with its row.
    assert_eq!(links, ["ADR-0002", "E49", "F48"]);
    assert_eq!(shown, Some(again));
}

#[test]
fn f48_ac2_knows_a_kind_as_one_lower_case_word() {
    for kind in ["story", "journal", "log-row"] {
        assert!(is_kind(kind), "{kind}");
    }
    for kind in [
        "",
        "Story",
        "two words",
        "-story",
        "story-",
        "st0ry",
        "log_row",
    ] {
        assert!(!is_kind(kind), "{kind}");
    }
}

#[test]
fn f48_ac2_reads_the_merged_rows_and_its_own_drafts_only() {
    let file = place("three-branches");
    let database = three_branches(&file);
    let none = Filters::default();
    let (of_b, of_c, of_other) = (
        ids(&database, "chore/b", &none),
        ids(&database, "chore/c", &none),
        ids(&database, "feature/z", &none),
    );
    let all = ids(&database, "chore/b", &filters(&["--all"]));
    let as_c = ids(&database, "chore/b", &filters(&["--branch", "chore/c"]));
    let merged = ids(&database, "chore/b", &filters(&["--state", "merged"]));
    let dropped = ids(&database, "chore/b", &filters(&["--state", "dropped"]));
    let drafts_of_c = show(&database, "chore/b", "EN-903", &none).unwrap();
    let dropped_shown = show(&database, "chore/b", "EN-904", &filters(&["--all"])).unwrap();

    clean(&file);
    assert_eq!(of_b, ["EN-901", "EN-902"]);
    assert_eq!(of_c, ["EN-901", "EN-903"]);
    assert_eq!(of_other, ["EN-901"]);
    // `--all` adds the dropped row, never another branch's draft.
    assert_eq!(all, ["EN-901", "EN-902", "EN-904"]);
    assert_eq!(as_c, ["EN-903"]);
    assert_eq!(merged, ["EN-901"]);
    assert_eq!(dropped, ["EN-904"]);
    assert_eq!(drafts_of_c, None);
    assert!(dropped_shown.is_some());
}

#[test]
fn f48_ac2_reads_its_own_draft_of_an_id_over_the_merged_row() {
    let file = place("own-draft");
    let database = three_branches(&file);
    let draft = story("EN-901", "Changed on b", "Busy.");

    add(&database, "story", "chore/b", &draft);

    let none = Filters::default();
    let (of_b, of_c) = (
        show(&database, "chore/b", "EN-901", &none).unwrap(),
        show(&database, "chore/c", "EN-901", &none).unwrap(),
    );
    let listed = list(&database, "chore/b", &none).unwrap();

    clean(&file);
    assert_eq!(of_b, Some(draft));
    assert_eq!(of_c, Some(story("EN-901", "Merged", "Busy.")));
    // Listed once, its draft.
    assert_eq!(
        listed
            .iter()
            .map(|row| (row.id.as_str(), row.state.as_str()))
            .collect::<Vec<_>>(),
        [("EN-901", "draft"), ("EN-902", "draft")]
    );
}

#[test]
fn f48_ac2_marks_merged_the_drafts_of_the_merged_branches_each_replacing_its_merged_row() {
    let file = place("sync");
    let database = three_branches(&file);

    add(
        &database,
        "story",
        "chore/b",
        &story("EN-901", "Changed on b", "Busy."),
    );

    let marked = write(&database, |transaction| {
        mark_merged(transaction, &["chore/b".to_owned(), "develop".to_owned()])
    })
    .unwrap();
    let again = write(&database, |transaction| {
        mark_merged(transaction, &["chore/b".to_owned()])
    })
    .unwrap();
    let rows: Vec<(String, String, String)> = database
        .prepare("SELECT id, branch, state FROM documents ORDER BY id, branch")
        .unwrap()
        .query_map([], |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)))
        .unwrap()
        .collect::<rusqlite::Result<_>>()
        .unwrap();
    let of_c = show(&database, "chore/c", "EN-901", &Filters::default()).unwrap();

    clean(&file);
    // chore/b's two drafts; feature/a's merged EN-901 replaced by chore/b's; the dropped row kept.
    assert_eq!(marked, 2);
    assert_eq!(again, 0);
    assert_eq!(
        rows,
        [
            ("EN-901", "chore/b", "merged"),
            ("EN-902", "chore/b", "merged"),
            ("EN-903", "chore/c", "draft"),
            ("EN-904", "chore/d", "dropped"),
        ]
        .map(|(id, branch, state)| (id.to_owned(), branch.to_owned(), state.to_owned()))
    );
    assert_eq!(of_c, Some(story("EN-901", "Changed on b", "Busy.")));
}

#[test]
fn f48_ac2_keeps_the_draft_written_last_of_two_merged_at_once() {
    let file = place("two-drafts");
    let database = open_store(&file).unwrap();

    add(
        &database,
        "story",
        "chore/c",
        &story("EN-905", "Written first", "Busy."),
    );
    add(
        &database,
        "story",
        "chore/e",
        &story("EN-905", "Written last", "Busy."),
    );

    // Given in either order, the row written last holds.
    let marked = write(&database, |transaction| {
        mark_merged(transaction, &["chore/e".to_owned(), "chore/c".to_owned()])
    })
    .unwrap();
    let rows: i64 = database
        .query_row("SELECT count(*) FROM documents", [], |row| row.get(0))
        .unwrap();
    let shown = show(&database, "develop", "EN-905", &Filters::default()).unwrap();

    clean(&file);
    assert_eq!(marked, 2);
    assert_eq!(rows, 1);
    assert_eq!(shown, Some(story("EN-905", "Written last", "Busy.")));
}

#[test]
fn f48_ac2_reads_the_branches_git_lists_as_merged() {
    assert_eq!(
        merged_branches("develop\n(HEAD detached at 1a2b3c4)\n\n  feature/f48-x  \nchore/en-1\n"),
        ["develop", "feature/f48-x", "chore/en-1"]
    );
    assert_eq!(merged_branches(""), Vec::<String>::new());
}

#[test]
fn f48_ac2_lists_by_kind_epic_feature_status_and_date() {
    let file = place("filters");
    let database = open_store(&file).unwrap();

    add(
        &database,
        "story",
        "develop",
        &story("EN-901", "A story", "Text."),
    );
    add(
        &database,
        "note",
        "develop",
        "---\nid: NOTE-0035\nepic: E49\ndate: 2026-10-25\n---\n\n# 0035 — Knowledge\n",
    );
    add(&database, "journal", "develop", "- `#run` EN-050: one.\n");

    let found = |words: &[&str]| ids(&database, "develop", &filters(words));
    let results = [
        found(&[]),
        found(&["--kind", "note"]),
        found(&["--epic", "E49"]),
        found(&["--feature", "F48"]),
        found(&["--status", "ready"]),
        // The story dated 2026-10-20, the journal's row the day it was added, 2026-10-24.
        found(&["--since", "2026-10-24"]),
        found(&["--kind", "story", "--since", "2026-10-21"]),
    ];

    clean(&file);
    assert_eq!(
        results,
        [
            vec!["EN-901", "JOURNAL-2026-10-24-1", "NOTE-0035"],
            vec!["NOTE-0035"],
            vec!["EN-901", "NOTE-0035"],
            vec!["EN-901"],
            vec!["EN-901"],
            vec!["JOURNAL-2026-10-24-1", "NOTE-0035"],
            vec![],
        ]
    );
}

#[test]
fn f48_ac2_prints_one_line_per_document_listed() {
    let listed = Listed {
        id: "EN-152".into(),
        kind: "story".into(),
        status: "ready".into(),
        date: "2026-10-25".into(),
        state: "draft".into(),
        branch: "chore/en-152".into(),
        title: "Add, search".into(),
    };
    let no_status = Listed {
        status: String::new(),
        ..listed.clone()
    };

    assert_eq!(
        listed_line(&listed),
        "EN-152 [story, ready, 2026-10-25] Add, search — draft on chore/en-152"
    );
    assert_eq!(
        listed_line(&no_status),
        "EN-152 [story, 2026-10-25] Add, search — draft on chore/en-152"
    );
}

#[test]
fn f48_ac2_parses_the_filters_and_keeps_the_other_words() {
    let words: Vec<String> = [
        "busy",
        "--kind",
        "note",
        "timeout",
        "--epic",
        "E49",
        "--feature",
        "F48",
        "--status",
        "ready",
        "--since",
        "2026-10-01",
        "--branch",
        "chore/b",
        "--state",
        "draft",
        "--all",
    ]
    .map(str::to_owned)
    .to_vec();
    let (rest, parsed) = parse_filters(&words).unwrap();
    let wrong = |words: &[&str]| {
        parse_filters(
            &words
                .iter()
                .map(|word| (*word).to_owned())
                .collect::<Vec<_>>(),
        )
        .unwrap_err()
    };

    assert_eq!(rest, ["busy", "timeout"]);
    assert_eq!(
        parsed,
        Filters {
            kind: Some("note".into()),
            epic: Some("E49".into()),
            feature: Some("F48".into()),
            status: Some("ready".into()),
            since: Some("2026-10-01".into()),
            branch: Some("chore/b".into()),
            state: Some("draft".into()),
            all: true,
        }
    );
    assert_eq!(wrong(&["--kind"]), "--kind needs a value");
    assert_eq!(wrong(&["--kind", "--all"]), "--kind needs a value");
    assert_eq!(wrong(&["--colour", "red"]), "--colour: no such filter");
    assert_eq!(
        wrong(&["--state", "parked"]),
        "--state parked: a state is draft, merged or dropped"
    );
}

#[test]
fn f48_ac2_ranks_the_sections_holding_every_word_by_bm25() {
    let file = place("search");
    let database = open_store(&file).unwrap();

    // The note's section holds "busy" four times and "timeout" thrice in 14 words, its heading's
    // two counted; the story's holds each once among 40: BM25 grows with a word's count and falls
    // with the section's length, so the note's comes first. The journal's row holds "busy" alone: every word being found in two
    // sections, it is left out.
    add(
        &database,
        "note",
        "develop",
        "---\nid: NOTE-0035\n---\n## Busy timeout\n\nThe busy timeout waits. A busy writer waits for the busy timeout.\n",
    );
    add(
        &database,
        "story",
        "develop",
        &story(
            "EN-151",
            "The tables",
            "Two worktrees open one file in WAL, each write in one immediate transaction, the second writer waiting for the first within the busy timeout rusqlite sets, five seconds, on each new connection it opens, as the pages say.",
        ),
    );
    add(&database, "journal", "develop", "- `#run` EN-050: busy.\n");
    add(
        &database,
        "story",
        "chore/c",
        &story("EN-903", "Of c", "Busy timeout."),
    );

    let found = |words: &str, filters: &Filters| -> Vec<(String, String)> {
        search(&database, "develop", words, filters)
            .unwrap()
            .into_iter()
            .map(|hit| (hit.id, hit.heading))
            .collect()
    };
    let every = found("busy timeout", &Filters::default());
    let any = found("busy nothing", &Filters::default());
    let of_kind = found("busy timeout", &filters(&["--kind", "story"]));
    let first = search(&database, "develop", "busy timeout", &Filters::default()).unwrap();
    let none = search(&database, "develop", "", &Filters::default());

    clean(&file);
    assert_eq!(
        every,
        [
            ("NOTE-0035".to_owned(), "Busy timeout".to_owned()),
            ("EN-151".to_owned(), "The tables".to_owned()),
        ]
    );
    // No section holds "nothing": any word then, the note's still first, the journal's row in.
    assert_eq!(any.len(), 3);
    assert_eq!(any[0].0, "NOTE-0035");
    assert_eq!(of_kind, [("EN-151".to_owned(), "The tables".to_owned())]);
    // 14 words, under the snippet's 24: the whole section, each word found between brackets.
    assert_eq!(
        first[0].snippet,
        "## [Busy] [timeout]\n\nThe [busy] [timeout] waits. A [busy] writer waits for the [busy] [timeout].\n"
    );
    assert!(none.is_err());
}

#[test]
fn f48_ac2_prints_a_hit_on_two_lines() {
    let hit = Hit {
        id: "NOTE-0035".into(),
        heading: "Busy timeout".into(),
        snippet: "## [Busy] timeout\n\nThe   [busy] one.\n".into(),
    };
    let top = Hit {
        heading: String::new(),
        ..hit.clone()
    };

    assert_eq!(
        hit_lines(&hit),
        "NOTE-0035 › Busy timeout\n    ## [Busy] timeout The [busy] one."
    );
    assert_eq!(
        hit_lines(&top),
        "NOTE-0035\n    ## [Busy] timeout The [busy] one."
    );
}

#[test]
fn f48_ac2_shows_one_section_of_a_document_as_page_does() {
    let text = "# Title\n\nIntro.\n\n## Decision\n\nWAL.\n\n### Detail\n\nMore.\n\n## Next\n";

    // "Decision" from its line, 5, down to "Next", its sub-section in, its trailing blank line out.
    assert_eq!(
        section_of("ADR-0081", text, "Decision").as_deref(),
        Some("ADR-0081:5\n## Decision\n\nWAL.\n\n### Detail\n\nMore.\n")
    );
    assert_eq!(section_of("ADR-0081", text, "Missing"), None);
}
