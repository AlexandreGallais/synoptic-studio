//! Tests of what the agent reads of the records (AUD-057): `show` rendering a record and its items
//! from whichever table holds its id, `search` on the full-text index `records_index`, `list` a
//! union of the tables, their filters. Every store is made in a folder of the system's temporary
//! folder; the real records folder is never opened (`[F48.AC4]`).

use std::fs;
use std::path::{Path, PathBuf};

use knowledge::read::{
    Filters, Hit, Listed, hit_lines, list, listed_line, parse_filters, search, section_of, show,
};
use knowledge::store::{open_store, write};
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

/// A record of each table, and their items: an epic of two features, one of criteria in rows and
/// one of criteria in its column; a story of tasks in rows and items, one dropped, one of no
/// feature with its tasks in its column; a decision, two notes, a derivation, a source, two rules
/// of one part, a retrospective, a memory and a convention.
const RECORDS: &str = "
  INSERT INTO epics (id, title, status, branch, state, vision) VALUES
    ('E49', 'Keep the records', 'in-progress', 'develop', 'merged', 'One file for every record.');
  INSERT INTO epic_items (epic, section, n, kind, item) VALUES
    ('E49', 'Open points', 1, 'bullet', 'The snapshot''s age.');
  INSERT INTO features (id, epic, title, status, position, branch, state, hypothesis, criteria) VALUES
    ('F48', 'E49', 'The database', 'done', 1, 'develop', 'merged', 'One file serves every worktree.', ''),
    ('F47', 'E49', 'The old files', 'ready', 2, 'develop', 'merged', '', '1. Gone.');
  INSERT INTO criteria (id, feature, number, text, struck, retired) VALUES
    ('F48.AC1', 'F48', 1, 'One file.', 0, ''),
    ('F48.AC2', 'F48', 2, 'A snapshot.', 1, 'moved to F50');
  INSERT INTO feature_items (feature, section, n, kind, item) VALUES
    ('F48', 'Plan', 1, 'paragraph', 'The store first.');
  INSERT INTO stories (id, epic, feature, kind, title, status, points, position, summary, criteria,
      tasks, test_card, question, branch, state) VALUES
    ('EN-900', 'E49', 'F48', 'EN', 'Add, search, show and list', 'ready', 3, 1, 'As ADR-0900 says.',
      '- A busy timeout waits for the other writer.', '', 'Open the file.', '',
      'chore/en-900-read', 'draft'),
    ('EN-901', 'E49', 'F48', 'EN', 'Dropped work', 'ready', NULL, 2, 'Never merged.', '', '', '',
      '', 'chore/en-901-gone', 'dropped'),
    ('SP-030', 'E49', '', 'SP', 'A spike', 'parked', NULL, 1, '', '', '- [ ] T1 — Read', '',
      'Which file?', 'develop', 'merged');
  INSERT INTO tasks (story, number, title, done) VALUES
    ('EN-900', 1, 'The tests', 1), ('EN-900', 2, 'The code', 0);
  INSERT INTO story_items (story, section, n, kind, item) VALUES
    ('EN-900', '', 1, 'paragraph', 'Read first.'),
    ('EN-900', 'Findings', 2, 'paragraph', 'Two writers measured.'),
    ('EN-900', 'Findings', 3, 'bullet', 'One waits.'),
    ('EN-900', 'Findings', 4, 'row', 'Writer: second · Wait: 5 ms'),
    ('EN-900', 'Findings', 5, 'code', '```text' || char(10) || 'busy' || char(10) || '```');
  INSERT INTO decisions (id, title, status, date, principle, decision, reason, consequences,
      status_note, replaces, replaced_by, removed_reason, changed, refs) VALUES
    ('ADR-0900', 'One file', 'accepted', '2026-10-20', 'One file for all.',
      'The records are one SQLite file.', 'Two files diverge.', '', '', 'ADR-0899', '', '',
      '2026-10-25', 'REF-SQLITE-WAL');
  INSERT INTO decision_items (decision, section, n, kind, item) VALUES
    ('ADR-0900', 'Reasons', 1, 'bullet', 'No merge of two files.');
  INSERT INTO notes (id, kind, title, status, date) VALUES
    ('NOTE-0900', 'note', 'The wait of a writer', 'done', '2026-10-24'),
    ('PAGE-research', 'page', 'Research protocol', 'accepted', '2026-10-01');
  INSERT INTO note_findings (note, n, section, kind, finding, source) VALUES
    ('NOTE-0900', 1, 'Findings', 'paragraph', 'SQLite waits within its busy timeout.',
      'REF-SQLITE-BUSY-TIMEOUT'),
    ('NOTE-0900', 2, 'Findings', 'bullet', 'No source for this.', '');
  INSERT INTO derivations (id, title, used_by) VALUES
    ('DERIV-setback', 'The setback of a fillet', 'EN-020');
  INSERT INTO derivation_steps (derivation, n, section, kind, statement, source) VALUES
    ('DERIV-setback', 1, 'Steps', 'paragraph', 't = r / tan(θ / 2)', 'REF-MATHWORLD');
  INSERT INTO sources (id, title, url, used_for, status, verified, topic, notes) VALUES
    ('REF-SQLITE-WAL', 'Write-Ahead Logging', 'https://sqlite.org/wal.html', 'Two writers.',
      'verified', '2026-10-20', 'records', 'Readers do not block writers.');
  INSERT INTO rules (id, page, section, name, rule, example, position) VALUES
    ('RULE-shapes-box-two-kinds', 'shapes', 'RULE-shapes-box', 'Two kinds', 'A box is fixed or free.',
      'A fixed box of 100 × 40.', 1),
    ('RULE-shapes-box-wrap', 'shapes', 'RULE-shapes-box', 'Wrap', 'Text wraps at spaces.',
      'a b wraps after a.', 2);
  INSERT INTO rule_items (page, n, section, kind, item) VALUES
    ('RULE-shapes-box', 1, '', 'paragraph', 'The box of a text.');
  INSERT INTO retrospectives (id, epic, title) VALUES ('RETRO-E49', 'E49', 'Keep the records');
  INSERT INTO retrospective_items (retrospective, section, n, kind, item) VALUES
    ('RETRO-E49', 'Helped', 1, 'bullet', 'One file.');
  INSERT INTO memories (name, description, type, private, modified, text) VALUES
    ('feedback-x', 'Short replies', 'feedback', 1, '2026-10-16', 'Be terse.');
  INSERT INTO conventions (area, topic, rule) VALUES ('records', 'ids', 'Code cites short ids.');
";

/// A store holding [`RECORDS`].
fn records(file: &Path) -> Connection {
    let database = open_store(file).unwrap();

    write(&database, |transaction| transaction.execute_batch(RECORDS)).unwrap();
    database
}

/// Filters of the words given.
fn filters(words: &[&str]) -> Filters {
    let words: Vec<String> = words.iter().map(|word| (*word).to_owned()).collect();

    parse_filters(&words).unwrap().1
}

/// What `show` renders of an id, filtered.
fn shown(database: &Connection, id: &str, words: &[&str]) -> Option<String> {
    show(database, id, &filters(words)).unwrap()
}

#[test]
fn f48_ac2_shows_a_story_its_facts_columns_tasks_and_items_as_markdown() {
    let file = place("story");
    let database = records(&file);
    let story = shown(&database, "EN-900", &[]);
    let spike = shown(&database, "SP-030", &[]);
    let dropped = shown(&database, "EN-901", &[]);
    let with_all = shown(&database, "EN-901", &["--all"]);

    clean(&file);
    // The facts but the empty ones, the columns as parts, the tasks from their rows, the item of
    // no section under the facts, the others under their section.
    assert_eq!(
        story.as_deref(),
        Some(
            "# EN-900 — Add, search, show and list\n\nstory EN · epic E49 · feature F48 · status ready · points 3 · draft on chore/en-900-read\n\nRead first.\n\n## Summary\n\nAs ADR-0900 says.\n\n## Acceptance criteria\n\n- A busy timeout waits for the other writer.\n\n## Tasks\n\n- [x] T1 — The tests\n- [ ] T2 — The code\n\n## Product Owner test\n\nOpen the file.\n\n## Findings\n\nTwo writers measured.\n\n- One waits.\n- Writer: second · Wait: 5 ms\n\n```text\nbusy\n```\n"
        )
    );
    // No task row: the tasks' column.
    assert_eq!(
        spike.as_deref(),
        Some(
            "# SP-030 — A spike\n\nstory SP · epic E49 · status parked · merged on develop\n\n## Tasks\n\n- [ ] T1 — Read\n\n## Question\n\nWhich file?\n"
        )
    );
    assert_eq!(dropped, None);
    assert!(
        with_all
            .as_deref()
            .is_some_and(|text| text.starts_with("# EN-901 — Dropped work\n")),
        "{with_all:?}"
    );
}

#[test]
fn f48_ac2_shows_a_feature_and_an_epic_with_their_criteria_stories_and_features() {
    let file = place("feature");
    let database = records(&file);
    let feature = shown(&database, "F48", &[]);
    let in_column = shown(&database, "F47", &[]);
    let epic = shown(&database, "E49", &[]);
    let criterion = shown(&database, "F48.AC2", &[]);

    database
        .execute_batch("UPDATE features SET state = 'dropped' WHERE id = 'F47'; UPDATE epics SET state = 'dropped'")
        .unwrap();

    let dropped = (shown(&database, "F47", &[]), shown(&database, "E49", &[]));

    clean(&file);
    // The criteria from their rows, a retired one with its reason; the stories but the dropped.
    assert_eq!(
        feature.as_deref(),
        Some(
            "# F48 — The database\n\nfeature · epic E49 · status done · merged on develop\n\n## Hypothesis\n\nOne file serves every worktree.\n\n## Acceptance criteria\n\n1. One file.\n2. A snapshot. (retired: moved to F50)\n\n## Stories\n\n- EN-900 [ready] Add, search, show and list\n\n## Plan\n\nThe store first.\n"
        )
    );
    assert_eq!(
        in_column.as_deref(),
        Some(
            "# F47 — The old files\n\nfeature · epic E49 · status ready · merged on develop\n\n## Acceptance criteria\n\n1. Gone.\n"
        )
    );
    assert_eq!(
        epic.as_deref(),
        Some(
            "# E49 — Keep the records\n\nepic · status in-progress · merged on develop\n\n## Vision\n\nOne file for every record.\n\n## Features\n\n- F48 [done] The database\n- F47 [ready] The old files\n\n## Stories of no feature\n\n- SP-030 [parked] A spike\n\n## Open points\n\n- The snapshot's age.\n"
        )
    );
    assert_eq!(
        criterion.as_deref(),
        Some(
            "# F48.AC2\n\ncriterion · feature F48 · struck · retired: moved to F50\n\n## Criterion\n\nA snapshot.\n"
        )
    );
    assert_eq!(dropped, (None, None));
}

#[test]
fn f48_ac2_shows_a_decision_a_note_a_derivation_and_a_source() {
    let file = place("decision");
    let database = records(&file);
    let decision = shown(&database, "ADR-0900", &[]);
    let note = shown(&database, "NOTE-0900", &[]);
    let derivation = shown(&database, "DERIV-setback", &[]);
    let source = shown(&database, "REF-SQLITE-WAL", &[]);

    clean(&file);
    assert_eq!(
        decision.as_deref(),
        Some(
            "# ADR-0900 — One file\n\ndecision · status accepted · date 2026-10-20 · changed 2026-10-25\n\n## Principle\n\nOne file for all.\n\n## Decision\n\nThe records are one SQLite file.\n\n## Reason\n\nTwo files diverge.\n\n## Replaces\n\nADR-0899\n\n## References\n\nREF-SQLITE-WAL\n\n## Reasons\n\n- No merge of two files.\n"
        )
    );
    // Each finding with the source it cites, none for one that cites none.
    assert_eq!(
        note.as_deref(),
        Some(
            "# NOTE-0900 — The wait of a writer\n\nnote · status done · date 2026-10-24\n\n## Findings\n\nSQLite waits within its busy timeout. (REF-SQLITE-BUSY-TIMEOUT)\n\n- No source for this.\n"
        )
    );
    assert_eq!(
        derivation.as_deref(),
        Some(
            "# DERIV-setback — The setback of a fillet\n\nderivation\n\n## Used by\n\nEN-020\n\n## Steps\n\nt = r / tan(θ / 2) (REF-MATHWORLD)\n"
        )
    );
    assert_eq!(
        source.as_deref(),
        Some(
            "# REF-SQLITE-WAL — Write-Ahead Logging\n\nsource · verified 2026-10-20 · topic records\n\n## Address\n\nhttps://sqlite.org/wal.html\n\n## Used for\n\nTwo writers.\n\n## Notes\n\nReaders do not block writers.\n"
        )
    );
}

#[test]
fn f48_ac2_shows_a_rule_a_part_of_the_rules_a_retrospective_and_a_memory() {
    let file = place("rule");
    let database = records(&file);
    let rule = shown(&database, "RULE-shapes-box-two-kinds", &[]);
    let part = shown(&database, "RULE-shapes-box", &[]);
    let retrospective = shown(&database, "RETRO-E49", &[]);
    let memory = shown(&database, "MEMORY-feedback-x", &[]);
    let nowhere = [
        shown(&database, "EN-999", &[]),
        shown(&database, "MEMORY-nope", &[]),
        shown(&database, "RULE-nope", &[]),
        shown(&database, "feedback-x", &[]),
    ];

    clean(&file);
    assert_eq!(
        rule.as_deref(),
        Some(
            "# RULE-shapes-box-two-kinds — Two kinds\n\nrule · page shapes · part RULE-shapes-box\n\n## Rule\n\nA box is fixed or free.\n\n## Example\n\nA fixed box of 100 × 40.\n"
        )
    );
    // Its item of no section, then its rules by position.
    assert_eq!(
        part.as_deref(),
        Some(
            "# RULE-shapes-box\n\na part of the business rules\n\nThe box of a text.\n\n## Rules\n\n- **Two kinds** (RULE-shapes-box-two-kinds): A box is fixed or free. Example: A fixed box of 100 × 40.\n- **Wrap** (RULE-shapes-box-wrap): Text wraps at spaces. Example: a b wraps after a.\n"
        )
    );
    assert_eq!(
        retrospective.as_deref(),
        Some(
            "# RETRO-E49 — Keep the records\n\nretrospective · epic E49\n\n## Helped\n\n- One file.\n"
        )
    );
    assert_eq!(
        memory.as_deref(),
        Some(
            "# MEMORY-feedback-x — Short replies\n\nmemory · type feedback · modified 2026-10-16\n\n## Text\n\nBe terse.\n"
        )
    );
    assert_eq!(nowhere, [None, None, None, None]);
}

#[test]
fn f48_ac2_shows_one_part_of_a_record_as_page_does() {
    let file = place("part");
    let database = records(&file);
    let text = shown(&database, "EN-900", &[]).unwrap_or_default();

    clean(&file);
    // `## Acceptance criteria` is the text's line 11: the title, a blank line, the facts, a blank
    // line, the item of no section, a blank line, `## Summary`, a blank line, its text, a blank
    // line.
    assert_eq!(
        section_of("EN-900", &text, "Acceptance criteria").as_deref(),
        Some("EN-900:11\n## Acceptance criteria\n\n- A busy timeout waits for the other writer.\n")
    );
    assert_eq!(section_of("EN-900", &text, "Missing"), None);
}

/// The ids and headings of the hits of some words, filtered.
fn found(database: &Connection, words: &str, filter: &[&str]) -> Vec<String> {
    search(database, words, &filters(filter))
        .unwrap()
        .into_iter()
        .map(|hit| format!("{} › {}", hit.id, hit.heading))
        .collect()
}

#[test]
fn f48_ac2_searches_the_index_of_every_table_filtered_by_the_record_each_row_belongs_to() {
    let file = place("search");
    let database = records(&file);
    let story = search(&database, "busy timeout", &filters(&["--kind", "story"])).unwrap();
    let of_a_note = found(&database, "within", &["--kind", "note"]);
    let none_of_a_decision = found(&database, "busy", &["--kind", "decision"]);
    // A task's row belongs to its story, a criterion's to its feature.
    let task = found(&database, "code", &["--epic", "E49", "--kind", "story"]);
    let criterion = found(
        &database,
        "snapshot",
        &["--kind", "feature", "--status", "done"],
    );
    // A convention is no record: its table is its kind, and a filter of records leaves it out.
    let convention = found(&database, "short ids", &["--kind", "conventions"]);
    let filtered_out = found(&database, "short ids", &["--epic", "E49"]);
    let dropped = found(&database, "never", &[]);
    let with_all = found(&database, "never", &["--all"]);
    let either = found(&database, "timeout zzz", &["--kind", "story"]);

    clean(&file);
    // The story's own row: its columns joined, every word between brackets.
    assert_eq!(
        story,
        [Hit {
            id: "EN-900".to_owned(),
            heading: "story".to_owned(),
            snippet: "Add, search, show and list\nAs ADR-0900 says.\n- A [busy] [timeout] waits for the other writer.\nOpen the file.\n".to_owned(),
        }]
    );
    assert_eq!(of_a_note, ["NOTE-0900 › Findings"]);
    assert_eq!(none_of_a_decision, Vec::<String>::new());
    assert_eq!(task, ["EN-900.T2 › task"]);
    assert_eq!(criterion, ["F48.AC2 › criterion"]);
    assert_eq!(convention, ["conventions 1 › records › ids"]);
    assert_eq!(filtered_out, Vec::<String>::new());
    assert_eq!(dropped, Vec::<String>::new());
    assert_eq!(with_all, ["EN-901 › story"]);
    // No row holds both words: any of them.
    assert_eq!(either, ["EN-900 › story"]);
}

#[test]
fn f48_ac2_searches_at_most_eight_rows() {
    let file = place("limit");
    let database = open_store(&file).unwrap();

    // Nine journal rows holding the word: eight printed, as the library's search.
    for n in 1..=9 {
        database
            .execute(
                "INSERT INTO journal (date, session, tag, text) VALUES ('2026-10-25', 'a', 'run', ?)",
                [format!("Busy {n}.")],
            )
            .unwrap();
    }

    let hits = search(&database, "busy", &Filters::default()).unwrap();

    clean(&file);
    assert_eq!(hits.len(), 8);
}

/// The ids `list` reads, filtered.
fn ids(database: &Connection, words: &[&str]) -> Vec<String> {
    list(database, &filters(words))
        .unwrap()
        .into_iter()
        .map(|listed| listed.id)
        .collect()
}

#[test]
fn f48_ac2_lists_the_records_of_every_table_by_kind_then_id_filtered() {
    let file = place("list");
    let database = records(&file);
    let every = ids(&database, &[]);
    let of_epic = ids(&database, &["--epic", "E49"]);
    let of_feature = ids(&database, &["--feature", "F48", "--kind", "story"]);
    let rules = ids(&database, &["--kind", "rule"]);
    let since = ids(&database, &["--since", "2026-10-20"]);
    let of_branch = ids(&database, &["--branch", "chore/en-900-read"]);
    let drafts = ids(&database, &["--state", "draft"]);
    let dropped = ids(&database, &["--state", "dropped"]);
    let parked = ids(&database, &["--status", "parked", "--all"]);
    let lines: Vec<String> = list(&database, &filters(&["--kind", "decision"]))
        .unwrap()
        .iter()
        .map(listed_line)
        .collect();

    clean(&file);
    assert_eq!(
        every,
        [
            "ADR-0900",
            "DERIV-setback",
            "E49",
            "F47",
            "F48",
            "MEMORY-feedback-x",
            "NOTE-0900",
            "PAGE-research",
            "RETRO-E49",
            "RULE-shapes-box",
            "RULE-shapes-box-two-kinds",
            "RULE-shapes-box-wrap",
            "REF-SQLITE-WAL",
            "EN-900",
            "SP-030",
        ]
    );
    assert_eq!(
        of_epic,
        ["E49", "F47", "F48", "RETRO-E49", "EN-900", "SP-030"]
    );
    assert_eq!(of_feature, ["EN-900"]);
    assert_eq!(
        rules,
        [
            "RULE-shapes-box",
            "RULE-shapes-box-two-kinds",
            "RULE-shapes-box-wrap"
        ]
    );
    // Dated on 2026-10-20 or later: the decision, the note, the source.
    assert_eq!(since, ["ADR-0900", "NOTE-0900", "REF-SQLITE-WAL"]);
    assert_eq!(of_branch, ["EN-900"]);
    assert_eq!(drafts, ["EN-900"]);
    assert_eq!(dropped, ["EN-901"]);
    assert_eq!(parked, ["SP-030"]);
    assert_eq!(
        lines,
        ["ADR-0900 [decision, accepted, 2026-10-20] One file — merged"]
    );
}

#[test]
fn f48_ac2_prints_one_line_per_record_listed() {
    let story = Listed {
        id: "EN-900".into(),
        kind: "story".into(),
        status: "ready".into(),
        date: String::new(),
        state: "draft".into(),
        branch: "chore/en-900-read".into(),
        title: "Add".into(),
    };
    let memory = Listed {
        id: "MEMORY-x".into(),
        kind: "memory".into(),
        status: String::new(),
        date: "2026-10-16".into(),
        state: "merged".into(),
        branch: String::new(),
        title: "Short".into(),
    };

    assert_eq!(
        listed_line(&story),
        "EN-900 [story, ready] Add — draft on chore/en-900-read"
    );
    assert_eq!(
        listed_line(&memory),
        "MEMORY-x [memory, 2026-10-16] Short — merged"
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
fn f48_ac2_prints_a_hit_on_two_lines() {
    let hit = Hit {
        id: "NOTE-0035".into(),
        heading: "Busy timeout".into(),
        snippet: "[Busy] timeout\n\nThe   [busy] one.\n".into(),
    };
    let top = Hit {
        heading: String::new(),
        ..hit.clone()
    };

    assert_eq!(
        hit_lines(&hit),
        "NOTE-0035 › Busy timeout\n    [Busy] timeout The [busy] one."
    );
    assert_eq!(
        hit_lines(&top),
        "NOTE-0035\n    [Busy] timeout The [busy] one."
    );
}
