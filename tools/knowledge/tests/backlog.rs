//! Tests of the backlog in the records (EN-154): the done features' criteria exported for the CI,
//! the ids a text cites checked against every table that holds ids, and a branch's rows merged or
//! dropped. Every file is made in a folder of the system's temporary folder; the real records
//! folder is never opened.

use std::fs;
use std::path::PathBuf;

use knowledge::backlog::{
    Family, Traced, criteria_json, done_criteria, drop_branch, family, merge_branch, missing_ids,
    set_status,
};
use knowledge::store::{open_store, write};
use rusqlite::Connection;

/// An epic, a done feature of three criteria, the second struck through, and two stories, as the
/// trunk holds them.
const BACKLOG: &str = "
  INSERT INTO epics (id, title, status, position, branch, state) VALUES
    ('E49', 'Keep the knowledge', 'in-progress', 2, 'develop', 'merged');
  INSERT INTO features (id, epic, title, status, position, branch, state) VALUES
    ('F48', 'E49', 'The database', 'done', 1, 'develop', 'merged');
  INSERT INTO criteria (id, feature, number, text, struck) VALUES
    ('F48.AC1', 'F48', 1, 'One file.', 0),
    ('F48.AC2', 'F48', 2, '~~Two files.~~ Dropped on 2026-10-25.', 1),
    ('F48.AC3', 'F48', 3, 'A snapshot.', 0);
  INSERT INTO stories (id, epic, feature, kind, title, status, points, position, branch, state) VALUES
    ('EN-151', 'E49', 'F48', 'EN', 'The tables', 'done', 3, 2, 'develop', 'merged'),
    ('EN-152', 'E49', 'F48', 'EN', 'Add', 'parked', NULL, 1, 'develop', 'merged');
";

/// A store holding [`BACKLOG`], in a folder of its own in the temporary folder.
fn place(name: &str) -> (PathBuf, Connection) {
    let folder =
        std::env::temp_dir().join(format!("knowledge-backlog-{name}-{}", std::process::id()));
    let _ = fs::remove_dir_all(&folder);
    let database = open_store(&folder.join("records/records.sqlite")).unwrap();

    write(&database, |transaction| transaction.execute_batch(BACKLOG)).unwrap();
    (folder, database)
}

/// The rows of a query, each row's columns as text joined by `|`.
fn rows(database: &Connection, query: &str) -> Vec<String> {
    let mut statement = database.prepare(query).unwrap();
    let count = statement.column_count();

    statement
        .query_map([], |row| {
            Ok((0..count)
                .map(|at| {
                    row.get::<_, rusqlite::types::Value>(at)
                        .map(|value| match value {
                            rusqlite::types::Value::Null => String::new(),
                            rusqlite::types::Value::Integer(number) => number.to_string(),
                            rusqlite::types::Value::Text(text) => text,
                            other => format!("{other:?}"),
                        })
                        .unwrap_or_default()
                })
                .collect::<Vec<_>>()
                .join("|"))
        })
        .unwrap()
        .collect::<rusqlite::Result<_>>()
        .unwrap()
}

#[test]
fn tells_each_family_of_backlog_ids_by_its_form() {
    assert_eq!(family("E49"), Some(Family::Epic));
    assert_eq!(family("F48"), Some(Family::Feature));
    for story in [
        "US-001", "EN-154", "SP-034", "AUD-050", "REV-009", "VAL-033", "RET-002",
    ] {
        assert_eq!(family(story), Some(Family::Story), "{story}");
    }
    // One digit or three, a kind the backlog never held, four digits: none.
    for word in [
        "E4", "F100", "ADR-0081", "NOTE-035", "EN-15", "EN-1540", "en-154", "E49-x",
    ] {
        assert_eq!(family(word), None, "{word}");
    }
}

#[test]
fn exports_the_criteria_of_the_done_features_but_the_struck_ones_their_retirement_kept() {
    let (folder, database) = place("export");

    database
        .execute(
            "UPDATE criteria SET retired = 'the files moved \"into\" the records' WHERE id = 'F48.AC3'",
            [],
        )
        .unwrap();

    let criteria = done_criteria(&database).unwrap();

    let _ = fs::remove_dir_all(&folder);
    assert_eq!(
        criteria,
        [
            Traced {
                id: "F48.AC1".to_owned(),
                retired: String::new()
            },
            Traced {
                id: "F48.AC3".to_owned(),
                retired: "the files moved \"into\" the records".to_owned()
            },
        ]
    );
    assert_eq!(
        criteria_json(&criteria),
        "[\n  { \"id\": \"F48.AC1\" },\n  { \"id\": \"F48.AC3\", \"retired\": \"the files moved \\\"into\\\" the records\" }\n]\n"
    );
}

#[test]
fn names_each_cited_id_the_records_lack_with_its_line() {
    let (folder, database) = place("ids");
    let text = "EN-151 and F48, E49.\nEN-999 then F47\n[F48.AC1] REF-NOPE ADR-9999\nJOURNAL-2026-10-25-1 JOURNAL-2026-10-25-2\n";
    // Before any decision or source is in, `ADR-` and `REF-` are kinds the records cannot check.
    let unchecked = missing_ids(&database, text).unwrap();

    database
        .execute_batch(
            "INSERT INTO decisions (id, title, status, date, principle) VALUES
              ('ADR-0081', 'Private records', 'accepted', '2026-10-15', 'Outside the repository.');
            INSERT INTO sources (id, title, url, used_for, status, verified, topic, notes) VALUES
              ('REF-SQLITE-WAL', 'WAL', 'https://sqlite.org/wal.html', 'x', 'verified', '2026-10-20', 'records', '');
            INSERT INTO derivations (id, title, used_by) VALUES ('DERIV-setback', 'Setback', 'EN-020');
            INSERT INTO notes (id, kind, title, status, date) VALUES
              ('NOTE-0035', 'note', 'The records', 'done', '2026-10-20');
            INSERT INTO rules (id, page, section, name, rule, example, position) VALUES
              ('RULE-shapes-box-wrap', 'shapes', 'RULE-shapes-box', 'Wrap', 'At spaces.', 'a b', 1);
            INSERT INTO rule_items (page, n, section, kind, item) VALUES
              ('RULE-shapes-text', 1, '', 'paragraph', 'Text.');",
        )
        .unwrap();

    let records = "ADR-0081 ADR-9999\nREF-SQLITE-WAL `REF-SQLITE-NOPE`\nDERIV-setback DERIV-nope\nNOTE-0035 NOTE-0999\nRULE-shapes-box RULE-shapes-box-wrap RULE-shapes-text RULE-shapes-nope\n";
    let missing = missing_ids(&database, records).unwrap();
    let journal = missing_ids(&database, text).unwrap();

    let _ = fs::remove_dir_all(&folder);
    // `JOURNAL-` ids name no row of the records: never checked.
    assert_eq!(unchecked, [(2, "EN-999".to_owned()), (2, "F47".to_owned())]);
    // A rule's id, its part's (`rules.section`) and a page of items (`rule_items.page`) are held.
    assert_eq!(
        missing,
        [
            (1, "ADR-9999".to_owned()),
            (2, "REF-SQLITE-NOPE".to_owned()),
            (3, "DERIV-nope".to_owned()),
            (4, "NOTE-0999".to_owned()),
            (5, "RULE-shapes-nope".to_owned()),
        ]
    );
    assert_eq!(
        journal,
        [
            (2, "EN-999".to_owned()),
            (2, "F47".to_owned()),
            (3, "REF-NOPE".to_owned()),
            (3, "ADR-9999".to_owned()),
        ]
    );
}

#[test]
fn merges_or_drops_the_drafts_of_one_branch_never_its_merged_rows_nor_another_branchs() {
    let (folder, database) = place("branches");

    database
        .execute_batch(
            "INSERT INTO stories (id, epic, feature, kind, title, status, branch, state)
              VALUES ('EN-160', 'E49', 'F48', 'EN', 'New', 'ready', 'chore/en-160-new', 'draft'),
                     ('EN-161', 'E49', 'F48', 'EN', 'Other', 'ready', 'chore/en-161-other', 'draft');
            UPDATE stories SET branch = 'chore/en-160-new' WHERE id = 'EN-151';
            INSERT INTO documents (id, kind, title, epic, feature, status, date, branch, state, private, path)
              VALUES ('NOTE-1', 'note', 'a', '', '', '', '2026-10-25', 'chore/en-160-new', 'draft', 0, ''),
                     ('NOTE-1', 'note', 'a', '', '', '', '2026-10-25', 'chore/en-160-new', 'dropped', 0, '');",
        )
        .unwrap();

    let dropped = write(&database, |transaction| {
        drop_branch(transaction, "chore/en-160-new")
    })
    .unwrap();
    let after_drop = rows(
        &database,
        "SELECT id, state FROM stories WHERE id IN ('EN-151', 'EN-160', 'EN-161') ORDER BY id",
    );
    let notes = rows(
        &database,
        "SELECT id, state FROM documents WHERE id = 'NOTE-1'",
    );
    let merged = write(&database, |transaction| {
        merge_branch(transaction, "chore/en-161-other")
    })
    .unwrap();
    let after_merge = rows(&database, "SELECT state FROM stories WHERE id = 'EN-161'");

    let _ = fs::remove_dir_all(&folder);
    // EN-160's story row and NOTE-1's draft: 2; EN-151 was merged before the branch worked on it.
    assert_eq!(dropped, 2);
    assert_eq!(
        after_drop,
        ["EN-151|merged", "EN-160|dropped", "EN-161|draft"]
    );
    // The older dropped row of NOTE-1 replaced by the new one.
    assert_eq!(notes, ["NOTE-1|dropped"]);
    assert_eq!((merged, after_merge), (1, vec!["merged".to_owned()]));
}

#[test]
fn sets_an_items_status_in_its_table_by_its_ids_form() {
    let (folder, database) = place("status");

    let results = [
        set_status(&database, "EN-152", "ready"),
        set_status(&database, "F48", "in-progress"),
        set_status(&database, "E49", "done"),
        set_status(&database, "EN-999", "ready"),
    ];
    let wrong = set_status(&database, "ADR-0001", "done");
    let statuses = rows(
        &database,
        "SELECT (SELECT status FROM stories WHERE id = 'EN-152'), (SELECT status FROM features), (SELECT status FROM epics)",
    );

    let _ = fs::remove_dir_all(&folder);
    assert_eq!(results, [Ok(true), Ok(true), Ok(true), Ok(false)]);
    assert!(wrong.is_err_and(|error| error.contains("not an id of the backlog")));
    assert_eq!(statuses, ["ready|in-progress|done"]);
}

#[test]
fn reads_a_placeholder_id_of_zeros_as_no_item() {
    let (folder, database) = place("placeholders");

    let missing = missing_ids(&database, "US-000, F00, E00 and REV-000; US-001\n").unwrap();

    let _ = fs::remove_dir_all(&folder);
    assert_eq!(missing, [(1, "US-001".to_owned())]);
}
