//! Tests of the store (EN-151): the tables of research note 0035, WAL, `user_version`, two
//! connections writing at once and a newer file refused for writing. Every file is made in a folder
//! of the system's temporary folder; the real records folder is never opened (`[F48.AC4]`).

use std::fs;
use std::path::PathBuf;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use knowledge::store::{MIGRATIONS, SCHEMA_VERSION, open_store, schema_version, write};
use rusqlite::{Connection, params};

/// A folder of its own in the temporary folder, emptied first, and its store's path.
fn place(name: &str) -> PathBuf {
    let folder =
        std::env::temp_dir().join(format!("knowledge-store-{name}-{}", std::process::id()));
    let _ = fs::remove_dir_all(&folder);

    folder.join("records").join("records.sqlite")
}

/// Removes a test's folder.
fn clean(file: &std::path::Path) {
    let _ = fs::remove_dir_all(file.parent().and_then(|records| records.parent()).unwrap());
}

/// The names of the schema's objects of a type, sorted.
fn names(database: &Connection, kind: &str) -> Vec<String> {
    let mut statement = database
        .prepare("SELECT name FROM sqlite_master WHERE type = ? AND name NOT LIKE 'sqlite_%' ORDER BY name")
        .unwrap();

    statement
        .query_map([kind], |row| row.get(0))
        .unwrap()
        .collect::<rusqlite::Result<_>>()
        .unwrap()
}

/// A document of the knowledge, its every column given.
fn add_document(database: &Connection, id: &str) -> rusqlite::Result<usize> {
    database.execute(
        "INSERT INTO documents (id, kind, title, epic, feature, status, date, branch, state, private, path)
        VALUES (?, 'story', 'The tables', 'E49', 'F48', 'ready', '2026-10-25', 'chore/en-151', 'draft', 0, '')",
        [id],
    )
}

/// The documents whose sections hold a word, by the full-text index.
fn found(database: &Connection, word: &str) -> Vec<String> {
    let mut statement = database
        .prepare(
            "SELECT sections.document FROM sections_index
            JOIN sections ON sections.id = sections_index.rowid
            WHERE sections_index MATCH ? ORDER BY bm25(sections_index)",
        )
        .unwrap();

    statement
        .query_map([word], |row| row.get(0))
        .unwrap()
        .collect::<rusqlite::Result<_>>()
        .unwrap()
}

#[test]
fn f48_ac1_makes_every_table_of_note_0035_with_its_full_text_index_kept_by_triggers() {
    let file = place("tables");
    let database = open_store(&file).unwrap();
    let tables = names(&database, "table");
    let (indexing, triggers): (Vec<String>, Vec<String>) = names(&database, "trigger")
        .into_iter()
        .partition(|name| name.starts_with("records_index_"));

    clean(&file);
    // The FTS5 table adds its shadow tables (`sections_index_data`, …): the note's tables are
    // checked one by one, the shadows left out.
    for table in [
        "calls",
        "documents",
        "fields",
        "links",
        "sections",
        "sections_index",
        "turns",
    ] {
        assert!(tables.contains(&table.to_owned()), "{table} in {tables:?}");
    }
    // Three triggers keep `records_index` in step with each of the 28 tables it reads.
    assert_eq!(indexing.len(), 84);
    assert_eq!(
        triggers,
        [
            "sections_delete",
            "sections_insert",
            "sections_update",
            "uses_insert"
        ]
    );
}

#[test]
fn f48_ac1_keeps_the_index_of_the_sections_on_insert_update_and_delete() {
    let file = place("index");
    let database = open_store(&file).unwrap();

    add_document(&database, "EN-151").unwrap();
    database
        .execute(
            "INSERT INTO sections (document, branch, state, position, heading, text)
            VALUES (?, 'chore/en-151', 'draft', 0, 'Tables', ?)",
            params!["EN-151", "Two worktrees write at once."],
        )
        .unwrap();

    let inserted = found(&database, "worktrees");

    database
        .execute("UPDATE sections SET text = 'One busy timeout.'", [])
        .unwrap();

    let (old, new) = (found(&database, "worktrees"), found(&database, "busy"));

    // Deleting the document deletes its sections, which takes them out of the index.
    database
        .execute("DELETE FROM documents WHERE id = 'EN-151'", [])
        .unwrap();

    let deleted = found(&database, "busy");

    clean(&file);
    assert_eq!(inserted, ["EN-151"]);
    assert_eq!(old, Vec::<String>::new());
    assert_eq!(new, ["EN-151"]);
    assert_eq!(deleted, Vec::<String>::new());
}

#[test]
fn f48_ac1_refuses_a_state_other_than_draft_merged_or_dropped() {
    let file = place("state");
    let database = open_store(&file).unwrap();

    add_document(&database, "EN-151").unwrap();

    let states: Vec<bool> = ["merged", "dropped", "draft", "parked"]
        .iter()
        .map(|state| {
            database
                .execute("UPDATE documents SET state = ?", [state])
                .is_ok()
        })
        .collect();

    clean(&file);
    assert_eq!(states, [true, true, true, false]);
}

#[test]
fn f48_ac1_opens_the_file_in_wal_for_every_connection_after() {
    let file = place("wal");
    let mode = |database: &Connection| -> String {
        database
            .query_row("PRAGMA journal_mode", [], |row| row.get(0))
            .unwrap()
    };
    let first = mode(&open_store(&file).unwrap());
    // WAL is persistent (`REF-SQLITE-WAL`, §3.3): a plain connection opened after sees it.
    let plain = mode(&Connection::open(&file).unwrap());

    clean(&file);
    assert_eq!(first, "wal");
    assert_eq!(plain, "wal");
}

#[test]
fn f48_ac1_sets_user_version_to_the_schemas_number_and_keeps_the_rows_when_opened_again() {
    let file = place("version");
    let first = open_store(&file).unwrap();

    add_document(&first, "EN-151").unwrap();
    drop(first);

    let again = open_store(&file).unwrap();
    let version = schema_version(&again).unwrap();
    let pragma: i64 = again
        .query_row("PRAGMA user_version", [], |row| row.get(0))
        .unwrap();
    let rows: i64 = again
        .query_row("SELECT count(*) FROM documents", [], |row| row.get(0))
        .unwrap();

    clean(&file);
    // Schema 6 since the records restructured, its items and full-text index added.
    assert_eq!(SCHEMA_VERSION, 6);
    assert_eq!(version, 6);
    assert_eq!(pragma, 6);
    assert_eq!(rows, 1);
}

#[test]
fn f48_ac1_makes_one_schema_when_four_connections_open_a_new_file_at_once() {
    let file = place("opened-at-once");
    let opened: Vec<Result<i64, String>> = (0..4)
        .map(|_| {
            let file = file.clone();
            thread::spawn(move || {
                open_store(&file).and_then(|database| {
                    schema_version(&database).map_err(|error| error.to_string())
                })
            })
        })
        .collect::<Vec<_>>()
        .into_iter()
        .map(|opening| opening.join().unwrap())
        .collect();

    clean(&file);
    assert_eq!(opened, vec![Ok(SCHEMA_VERSION); 4]);
}

#[test]
fn f48_ac1_takes_the_write_lock_at_the_start_of_each_write() {
    let file = place("immediate");
    let database = open_store(&file).unwrap();
    let other = Connection::open(&file).unwrap();

    other.busy_timeout(Duration::ZERO).unwrap();

    // Before the write runs a statement, another connection can no longer start one: the lock was
    // taken by `BEGIN IMMEDIATE`; a `BEGIN DEFERRED` would have taken none yet
    // (`REF-SQLITE-TRANSACTION`, §2.2).
    let refused = write(&database, |_| {
        Ok(other.execute_batch("BEGIN IMMEDIATE; ROLLBACK;").is_err())
    });
    let free_after = other.execute_batch("BEGIN IMMEDIATE; ROLLBACK;").is_ok();

    clean(&file);
    assert_eq!(refused, Ok(true));
    assert!(free_after);
}

#[test]
fn f48_ac1_lets_two_connections_write_at_once_the_second_waiting_for_the_first() {
    let file = place("two-writers");
    let (inside, entered) = mpsc::channel();
    let first_file = file.clone();
    let first = thread::spawn(move || {
        let database = open_store(&first_file).unwrap();

        write(&database, |transaction| {
            add_document(transaction, "EN-151")?;
            inside.send(()).unwrap();
            // The second writer starts while this one holds the lock, and waits for it.
            thread::sleep(Duration::from_millis(300));
            add_document(transaction, "EN-152")
        })
    });

    entered.recv().unwrap();

    let database = open_store(&file).unwrap();
    let second = write(&database, |transaction| add_document(transaction, "EN-153"));
    let first = first.join().unwrap();
    let rows: i64 = database
        .query_row("SELECT count(*) FROM documents", [], |row| row.get(0))
        .unwrap();

    clean(&file);
    assert_eq!(first, Ok(1));
    assert_eq!(second, Ok(1));
    assert_eq!(rows, 3);
}

#[test]
fn f48_ac1_rolls_a_failed_write_back_whole() {
    let file = place("rollback");
    let database = open_store(&file).unwrap();
    // The second insert repeats the id: the first is undone with it.
    let failed = write(&database, |transaction| {
        add_document(transaction, "EN-151")?;
        add_document(transaction, "EN-151")
    });
    let rows: i64 = database
        .query_row("SELECT count(*) FROM documents", [], |row| row.get(0))
        .unwrap();
    let after = write(&database, |transaction| add_document(transaction, "EN-151"));

    clean(&file);
    assert!(failed.is_err());
    assert_eq!(rows, 0);
    assert_eq!(after, Ok(1));
}

#[test]
fn f48_ac1_refuses_to_write_to_a_newer_file_and_still_reads_it() {
    let file = place("newer");
    let database = open_store(&file).unwrap();

    add_document(&database, "EN-151").unwrap();
    database
        .pragma_update(None, "user_version", SCHEMA_VERSION + 1)
        .unwrap();
    drop(database);

    let newer = open_store(&file).unwrap();
    let version = schema_version(&newer).unwrap();
    let refused = write(&newer, |transaction| add_document(transaction, "EN-152"));
    let rows: i64 = newer
        .query_row("SELECT count(*) FROM documents", [], |row| row.get(0))
        .unwrap();

    clean(&file);
    assert_eq!(version, 7);
    assert_eq!(
        refused,
        Err(
            "the file's schema is 7, newer than this binary's 6: rebuild the tools from your base, target/release/git-sequences sync"
                .to_owned()
        )
    );
    assert_eq!(rows, 1);
}

#[test]
fn f48_ac1_fails_aloud_on_a_file_that_is_no_database() {
    let file = place("not-a-database");

    fs::create_dir_all(file.parent().unwrap()).unwrap();
    fs::write(
        &file,
        "not a SQLite file, long enough to hold a header of a hundred bytes.....................",
    )
    .unwrap();

    let opened = open_store(&file);

    clean(&file);
    assert!(opened.is_err_and(|error| error.starts_with(&file.display().to_string())));
}

/// A file of schema 1, as EN-151's binary made it, holding a document, a field, a section and a
/// link.
fn schema_1_file(file: &std::path::Path) {
    fs::create_dir_all(file.parent().unwrap()).unwrap();

    let database = Connection::open(file).unwrap();

    database.execute_batch(MIGRATIONS[0]).unwrap();
    database.pragma_update(None, "user_version", 1).unwrap();
    add_document(&database, "EN-151").unwrap();
    database
        .execute_batch(
            "INSERT INTO fields (document, key, value) VALUES ('EN-151', 'epic', 'E49');
            INSERT INTO sections (document, position, heading, text)
              VALUES ('EN-151', 0, 'Tables', 'Two worktrees write at once.');
            INSERT INTO links (\"from\", \"to\") VALUES ('EN-151', 'ADR-0081');",
        )
        .unwrap();
}

/// A count of a query's rows.
fn count(database: &Connection, query: &str) -> i64 {
    database.query_row(query, [], |row| row.get(0)).unwrap()
}

#[test]
fn f48_ac1_brings_a_schema_1_file_up_to_the_last_its_rows_kept_with_their_branch_and_state() {
    let file = place("from-1");

    schema_1_file(&file);

    let database = open_store(&file).unwrap();
    let version = schema_version(&database).unwrap();
    let kept = [
        "SELECT count(*) FROM documents WHERE id = 'EN-151' AND branch = 'chore/en-151' AND state = 'draft'",
        "SELECT count(*) FROM fields WHERE document = 'EN-151' AND branch = 'chore/en-151' AND state = 'draft'",
        "SELECT count(*) FROM sections WHERE document = 'EN-151' AND branch = 'chore/en-151' AND state = 'draft'",
        "SELECT count(*) FROM links WHERE \"from\" = 'EN-151' AND branch = 'chore/en-151' AND state = 'draft'",
    ]
    .map(|query| count(&database, query));
    // The index still finds the section, whose rowid the rebuild kept.
    let indexed = found(&database, "worktrees");
    let (indexing, triggers): (Vec<String>, Vec<String>) = names(&database, "trigger")
        .into_iter()
        .partition(|name| name.starts_with("records_index_"));

    clean(&file);
    assert_eq!(version, SCHEMA_VERSION);
    assert_eq!(kept, [1, 1, 1, 1]);
    assert_eq!(indexed, ["EN-151"]);
    assert_eq!(indexing.len(), 84);
    assert_eq!(
        triggers,
        [
            "sections_delete",
            "sections_insert",
            "sections_update",
            "uses_insert"
        ]
    );
}

#[test]
fn f48_ac1_keys_a_document_by_its_id_its_branch_and_its_state_from_schema_3() {
    let file = place("key");

    schema_1_file(&file);

    let database = open_store(&file).unwrap();
    // The same id on another branch is a row of its own, and so is its merged row on the same
    // branch; the same id, branch and state, refused.
    let row = |branch: &str, state: &str| {
        database.execute(
            "INSERT INTO documents (id, kind, title, epic, feature, status, date, branch, state, private, path)
            VALUES ('EN-151', 'story', 'Changed', '', '', '', '', ?, ?, 0, '')",
            [branch, state],
        )
    };
    let other_branch = row("chore/en-152", "draft");
    let merged = row("chore/en-151", "merged");
    let same = add_document(&database, "EN-151");
    let section_of_another = database.execute(
        "INSERT INTO sections (document, branch, state, position, heading, text)
        VALUES ('EN-151', 'chore/en-152', 'merged', 0, 'A', 'b')",
        [],
    );

    // A document whose state changes takes its rows with it, the index still finding them.
    database
        .execute(
            "UPDATE documents SET state = 'dropped'
            WHERE id = 'EN-151' AND branch = 'chore/en-151' AND state = 'draft'",
            [],
        )
        .unwrap();

    let moved = [
        "SELECT count(*) FROM fields WHERE state = 'dropped'",
        "SELECT count(*) FROM sections WHERE state = 'dropped'",
        "SELECT count(*) FROM links WHERE state = 'dropped'",
    ]
    .map(|query| count(&database, query));
    let indexed = found(&database, "worktrees");

    // A document deleted takes its own rows with it, never another branch's nor state's.
    database
        .execute(
            "INSERT INTO sections (document, branch, state, position, heading, text)
            VALUES ('EN-151', 'chore/en-152', 'draft', 0, 'Tables', 'Changed.')",
            [],
        )
        .unwrap();
    database
        .execute(
            "DELETE FROM documents
            WHERE id = 'EN-151' AND branch = 'chore/en-151' AND state = 'dropped'",
            [],
        )
        .unwrap();

    let left = [
        "SELECT count(*) FROM documents",
        "SELECT count(*) FROM fields",
        "SELECT count(*) FROM sections",
        "SELECT count(*) FROM links",
    ]
    .map(|query| count(&database, query));

    clean(&file);
    assert_eq!((other_branch, merged), (Ok(1), Ok(1)));
    assert!(same.is_err());
    // A section of a document no row holds breaks the foreign key.
    assert!(section_of_another.is_err());
    assert_eq!(moved, [1, 1, 1]);
    assert_eq!(indexed, ["EN-151"]);
    assert_eq!(left, [2, 0, 1, 0]);
}

#[test]
fn f48_ac1_fails_the_opening_when_the_file_cannot_be_put_in_wal() {
    // An in-memory database has no WAL: each switch returns its mode, `memory`, unchanged
    // (`REF-SQLITE-PRAGMA`, journal_mode), the second one's failure the opening's.
    let opened = open_store(std::path::Path::new(":memory:")).map(|_| ());

    assert_eq!(
        opened,
        Err(":memory:: the file stays in the memory journal mode, WAL refused".to_owned())
    );
}

#[test]
fn f48_ac1_puts_the_file_in_wal_while_another_connection_writes_it_in_its_old_mode() {
    let file = place("wal-while-writing");

    fs::create_dir_all(file.parent().unwrap()).unwrap();

    let (holding, held) = mpsc::channel();
    let writer_file = file.clone();
    // A connection of the rollback journal holds the write lock; switching to WAL needs the file
    // alone, which SQLite refused at once rather than wait, as EN-152 measured.
    let writer = thread::spawn(move || {
        let database = Connection::open(&writer_file).unwrap();

        database
            .execute_batch("BEGIN IMMEDIATE; CREATE TABLE other (a);")
            .unwrap();
        holding.send(()).unwrap();
        thread::sleep(Duration::from_millis(300));
        database.execute_batch("COMMIT;").unwrap();
    });

    held.recv().unwrap();

    let opened = open_store(&file).map(|database| {
        database
            .query_row("PRAGMA journal_mode", [], |row| row.get::<_, String>(0))
            .unwrap()
    });

    writer.join().unwrap();
    clean(&file);
    assert_eq!(opened, Ok("wal".to_owned()));
}

/// The columns of a table, in their order.
fn columns(database: &Connection, table: &str) -> Vec<String> {
    database
        .prepare("SELECT name FROM pragma_table_info(?)")
        .unwrap()
        .query_map([table], |row| row.get(0))
        .unwrap()
        .collect::<rusqlite::Result<_>>()
        .unwrap()
}

/// The rows of `records_index` matching a query, `<id> <kind> <section>`, by their key.
fn indexed(database: &Connection, query: &str) -> Vec<String> {
    database
        .prepare(
            "SELECT id || ' ' || kind || ' ' || section FROM records_index
            WHERE records_index MATCH ? ORDER BY ref",
        )
        .unwrap()
        .query_map([query], |row| row.get(0))
        .unwrap()
        .collect::<rusqlite::Result<_>>()
        .unwrap()
}

#[test]
fn makes_the_records_items_and_their_full_text_index_kept_by_triggers_in_a_new_file() {
    let file = place("schema-6");
    let database = open_store(&file).unwrap();
    let tables = names(&database, "table");
    let shapes = [
        "stories",
        "features",
        "epics",
        "decisions",
        "notes",
        "derivations",
        "retrospectives",
    ]
    .map(|table| columns(&database, table));

    database
        .execute_batch(
            "INSERT INTO stories (id, epic, kind, title, status, summary) VALUES
              ('EN-1', 'E49', 'EN', 'Busy writer', 'ready', 'Two worktrees.');
            INSERT INTO story_items (story, section, n, kind, item) VALUES
              ('EN-1', 'Findings', 1, 'bullet', 'A timeout of five seconds.');",
        )
        .unwrap();

    let inserted = indexed(&database, "busy OR timeout");

    database
        .execute_batch(
            "UPDATE stories SET title = 'Calm writer' WHERE id = 'EN-1';
            DELETE FROM story_items WHERE story = 'EN-1';",
        )
        .unwrap();

    let changed = (
        indexed(&database, "busy OR timeout"),
        indexed(&database, "calm"),
    );

    clean(&file);
    for table in [
        "story_items",
        "feature_items",
        "epic_items",
        "decision_items",
        "retrospective_items",
        "rule_items",
        "derivation_steps",
        "note_findings",
        "conventions",
        "method_rules",
        "preferences",
        "tools",
        "records_index",
    ] {
        assert!(tables.contains(&table.to_owned()), "{table} in {tables:?}");
    }
    // No Markdown copy: no `text` column; the columns schema 6 adds.
    for shape in &shapes {
        assert!(!shape.contains(&"text".to_owned()), "{shape:?}");
    }
    assert!(shapes[0].contains(&"summary".to_owned()));
    assert!(shapes[1].contains(&"hypothesis".to_owned()));
    assert!(shapes[2].contains(&"vision".to_owned()));
    for column in ["consequences", "refs", "status_note", "decision", "reason"] {
        assert!(shapes[3].contains(&column.to_owned()), "{column}");
    }
    // One index row per row, the story's under its kind, the item's under its section.
    assert_eq!(
        inserted,
        ["EN-1 stories story", "EN-1 story_items Findings"]
    );
    assert_eq!(
        changed,
        (Vec::<String>::new(), vec!["EN-1 stories story".to_owned()])
    );
}

/// A file of schema 5 as the records were before their restructuring: the backlog's and the
/// decisions' tables with their `text` column, a story, a decision and a journal entry.
fn schema_5_file(file: &std::path::Path) {
    fs::create_dir_all(file.parent().unwrap()).unwrap();

    let database = Connection::open(file).unwrap();

    // As the store migrates, the foreign keys off while a table is rebuilt.
    database.pragma_update(None, "foreign_keys", false).unwrap();

    for migration in &MIGRATIONS[..3] {
        database.execute_batch(migration).unwrap();
    }
    database
        .execute_batch(
            "CREATE TABLE stories (
              id TEXT PRIMARY KEY, epic TEXT NOT NULL, feature TEXT NOT NULL DEFAULT '', kind TEXT NOT NULL,
              title TEXT NOT NULL, status TEXT NOT NULL, points INTEGER, position INTEGER NOT NULL DEFAULT 0,
              topic TEXT NOT NULL DEFAULT '', criteria TEXT NOT NULL DEFAULT '',
              tasks TEXT NOT NULL DEFAULT '', test_card TEXT NOT NULL DEFAULT '',
              question TEXT NOT NULL DEFAULT '', text TEXT NOT NULL DEFAULT '',
              branch TEXT NOT NULL DEFAULT '', state TEXT NOT NULL DEFAULT 'draft'
            );
            CREATE TABLE decisions (
              id TEXT PRIMARY KEY, title TEXT NOT NULL, status TEXT NOT NULL, date TEXT NOT NULL,
              principle TEXT NOT NULL, text TEXT NOT NULL
            );",
        )
        .unwrap();
    for migration in &MIGRATIONS[3..] {
        database.execute_batch(migration).unwrap();
    }
    database
        .execute_batch(
            "INSERT INTO stories (id, epic, kind, title, status, text) VALUES
              ('EN-151', 'E49', 'EN', 'The tables', 'done', '# EN-151');
            INSERT INTO decisions (id, title, status, date, principle, text) VALUES
              ('ADR-0081', 'Private records', 'accepted', '2026-10-15', 'Outside the repository.', '# ADR');
            INSERT INTO journal (date, session, tag, text) VALUES
              ('2026-10-25', 'Morning', 'run', 'A busy timeout met.');",
        )
        .unwrap();
    database.pragma_update(None, "user_version", 5).unwrap();
}

#[test]
fn brings_a_schema_5_file_up_to_6_additively_its_rows_indexed_and_their_text_kept() {
    let file = place("from-5");

    schema_5_file(&file);

    let database = open_store(&file).unwrap();
    let version = schema_version(&database).unwrap();
    let stories = columns(&database, "stories");
    let decisions = columns(&database, "decisions");
    let kept = count(
        &database,
        "SELECT count(*) FROM stories WHERE id = 'EN-151' AND text = '# EN-151'",
    );
    let rows = [
        indexed(&database, "tables"),
        indexed(&database, "repository"),
        indexed(&database, "busy"),
    ];

    database
        .execute(
            "UPDATE stories SET summary = 'Two writers.' WHERE id = 'EN-151'",
            [],
        )
        .unwrap();

    let updated = indexed(&database, "writers");

    clean(&file);
    assert_eq!(version, SCHEMA_VERSION);
    // The new columns after the old, the `text` column and its row kept.
    assert_eq!(stories.last().map(String::as_str), Some("summary"));
    assert!(stories.contains(&"text".to_owned()));
    assert!(decisions.contains(&"consequences".to_owned()));
    assert_eq!(kept, 1);
    // The rows already held, indexed once by the migration.
    assert_eq!(
        rows,
        [
            vec!["EN-151 stories story".to_owned()],
            vec!["ADR-0081 decisions decision".to_owned()],
            vec!["journal 1 journal Morning".to_owned()],
        ]
    );
    assert_eq!(updated, ["EN-151 stories story"]);
}

#[test]
fn redoes_nothing_on_a_file_of_schema_5_that_already_has_the_shape_of_6() {
    let file = place("shaped");
    let database = open_store(&file).unwrap();

    // The records restructured by hand: the shape of schema 6, the number 5.
    database
        .execute_batch(
            "INSERT INTO stories (id, epic, kind, title, status) VALUES ('EN-1', 'E49', 'EN', 'Busy', 'ready');
            PRAGMA user_version = 5;",
        )
        .unwrap();
    drop(database);

    let again = open_store(&file).unwrap();
    let version = schema_version(&again).unwrap();
    let rows = indexed(&again, "busy");
    let triggers = count(
        &again,
        "SELECT count(*) FROM sqlite_master WHERE type = 'trigger' AND name LIKE 'records_index_%'",
    );

    clean(&file);
    assert_eq!(version, SCHEMA_VERSION);
    // Not filled again: the story indexed once, the triggers once each.
    assert_eq!(rows, ["EN-1 stories story"]);
    assert_eq!(triggers, 84);
}
