//! Tests of the backlog in the records (EN-154): its files read, imported into the tables of
//! schema 4 and their documents, again without a change, the done features' criteria exported for
//! the CI, the ids a text cites checked, and a branch's rows merged or dropped. Every file is made
//! in a folder of the system's temporary folder; the real records folder is never opened.

use std::fs;
use std::path::{Path, PathBuf};

use knowledge::backlog::{
    Criterion, Family, Task, Traced, criteria_json, criteria_of, done_criteria, drop_branch,
    epic_order, family, import_backlog, item_of, merge_branch, missing_ids, order_in, section_body,
    set_status, tasks_of, topic_of,
};
use knowledge::store::{open_store, write};
use rusqlite::Connection;

/// An epic, its index, a feature of two criteria, the second struck through, and two stories.
const FILES: [(&str, &str); 5] = [
    (
        "epics/README.md",
        "# Epics\n\n| Order | ID | Epic | Status |\n| --- | --- | --- | --- |\n| 1 | [E01](./E01-draw.md) | Draw | done |\n| 2 | [E49](./E49-keep.md) | Keep | in-progress |\n",
    ),
    (
        "epics/E49-keep.md",
        "---\nid: E49\ntitle: Keep the knowledge\nstatus: in-progress\n---\n\n# E49\n\n## Features\n\n| Feature | Status |\n| --- | --- |\n| [F48](../features/E49-F48-the-database.md) — The database | done |\n",
    ),
    (
        "features/E49-F48-the-database.md",
        "---\nid: F48\nepic: E49\ntitle: The database\nstatus: done\n---\n\n# F48\n\n## Acceptance criteria\n\n1. One file.\n2. ~~Two files.~~ Dropped on 2026-10-25.\n3. A snapshot.\n\n## Stories\n\n| ID | Type | Title | Status |\n| --- | --- | --- | --- |\n| [EN-152](../stories/E49-F48-EN-152-add.md) | EN | Add | done |\n| [EN-151](../stories/E49-F48-EN-151-the-tables.md) | EN | The tables | done |\n",
    ),
    (
        "stories/E49-F48-EN-151-the-tables.md",
        "---\nid: EN-151\nepic: E49\nfeature: F48\ntitle: The tables\nstatus: done\npoints: 3\n---\n\n# EN-151\n\n## Acceptance criteria\n\n- Given a file, then it opens.\n\n## Tasks\n\n- [x] T1 — The tests (1 h)\n- [ ] T2 — The code\n\n## Product Owner test\n\nOpen it.\n",
    ),
    (
        "stories/E49-F48-EN-152-add.md",
        "---\nid: EN-152\nepic: E49\nfeature: F48\ntitle: Add\nstatus: parked\n---\n\n# EN-152\n\n## Question for the Product Owner\n\nWhich kind?\n",
    ),
];

/// A backlog of [`FILES`] and a store, each in a folder of their own in the temporary folder.
fn place(name: &str) -> (PathBuf, Connection) {
    let folder =
        std::env::temp_dir().join(format!("knowledge-backlog-{name}-{}", std::process::id()));
    let _ = fs::remove_dir_all(&folder);

    for (path, text) in FILES {
        let file = folder.join("backlog").join(path);

        fs::create_dir_all(file.parent().unwrap()).unwrap();
        fs::write(file, text).unwrap();
    }

    let database = open_store(&folder.join("records/records.sqlite")).unwrap();

    (folder, database)
}

/// Imports the place's backlog on a branch.
fn import(folder: &Path, database: &Connection) -> knowledge::backlog::Imported {
    write(database, |transaction| {
        import_backlog(
            transaction,
            &folder.join("backlog"),
            "develop",
            "2026-10-25",
        )
    })
    .unwrap()
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
fn reads_an_items_topic_fields_sections_tasks_and_criteria_from_its_file() {
    let (name, text) = FILES[3];
    let story = item_of(name.trim_start_matches("stories/"), text).unwrap();

    assert_eq!(
        (
            story.id.as_str(),
            story.epic.as_str(),
            story.feature.as_str()
        ),
        ("EN-151", "E49", "F48")
    );
    assert_eq!(
        (story.topic.as_str(), story.points),
        ("the-tables", Some(3))
    );
    assert_eq!(topic_of("E49-F48-the-database", "F48"), "the-database");
    assert_eq!(topic_of("E49-keep", "E49"), "keep");
    assert_eq!(topic_of("E02-REV-009-review", "REV-009"), "review");
    assert_eq!(section_body(text, "Product Owner test"), "Open it.");
    assert_eq!(section_body(text, "Missing"), "");
    assert_eq!(
        tasks_of(&section_body(text, "Tasks")),
        [
            Task {
                number: 1,
                title: "The tests (1 h)".to_owned(),
                done: true
            },
            Task {
                number: 2,
                title: "The code".to_owned(),
                done: false
            },
        ]
    );
    assert_eq!(
        criteria_of("1. One.\n2. ~~Two.~~ Dropped.\n- not one\n"),
        [
            Criterion {
                number: 1,
                text: "One.".to_owned(),
                struck: false
            },
            Criterion {
                number: 2,
                text: "~~Two.~~ Dropped.".to_owned(),
                struck: true
            },
        ]
    );
    // A file without an id in its front matter is no item.
    assert_eq!(item_of("README.md", "# Stories\n"), None);
}

#[test]
fn orders_the_children_as_their_parents_table_lists_them() {
    let order = order_in(FILES[2].1);

    // EN-152 is listed first in F48's table, EN-151 second.
    assert_eq!(order.get("EN-152"), Some(&1));
    assert_eq!(order.get("EN-151"), Some(&2));
    assert_eq!(epic_order(FILES[0].1).get("E49"), Some(&2));
}

#[test]
fn imports_every_item_into_its_table_and_its_document_then_again_without_a_change() {
    let (folder, database) = place("import");
    let first = import(&folder, &database);
    let tables = || {
        [
            rows(
                &database,
                "SELECT id, position, status, state FROM epics ORDER BY id",
            ),
            rows(&database, "SELECT id, epic, position, status FROM features"),
            rows(
                &database,
                "SELECT id, kind, feature, topic, points, position, status, test_card, question, branch, state FROM stories ORDER BY id",
            ),
            rows(
                &database,
                "SELECT story, number, done FROM tasks ORDER BY story, number",
            ),
            rows(&database, "SELECT id, struck FROM criteria ORDER BY id"),
            rows(
                &database,
                "SELECT id, kind, state FROM documents ORDER BY id",
            ),
        ]
    };
    let before = tables();
    let second = import(&folder, &database);
    let after = tables();

    let _ = fs::remove_dir_all(&folder);
    // One epic file (the index is no item), one feature, two stories; F48's 3 criteria; EN-151's
    // 2 tasks.
    assert_eq!(
        (
            first.epics,
            first.features,
            first.stories,
            first.criteria,
            first.tasks
        ),
        (1, 1, 2, 3, 2)
    );
    assert_eq!(first, second);
    assert_eq!(before, after);
    assert_eq!(before[0], ["E49|2|in-progress|merged"]);
    assert_eq!(before[1], ["F48|E49|1|done"]);
    assert_eq!(
        before[2],
        [
            "EN-151|EN|F48|the-tables|3|2|done|Open it.||develop|merged",
            "EN-152|EN|F48|add||1|parked||Which kind?|develop|merged",
        ]
    );
    assert_eq!(before[3], ["EN-151|1|1", "EN-151|2|0"]);
    assert_eq!(before[4], ["F48.AC1|0", "F48.AC2|1", "F48.AC3|0"]);
    assert_eq!(
        before[5],
        [
            "E49|epic|merged",
            "EN-151|story|merged",
            "EN-152|story|merged",
            "F48|feature|merged",
        ]
    );
}

#[test]
fn exports_the_criteria_of_the_done_features_but_the_struck_ones_their_retirement_kept() {
    let (folder, database) = place("export");

    import(&folder, &database);
    database
        .execute(
            "UPDATE criteria SET retired = 'the files moved \"into\" the records' WHERE id = 'F48.AC3'",
            [],
        )
        .unwrap();
    // A second import keeps the reason a criterion was retired.
    import(&folder, &database);

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

    import(&folder, &database);
    database
        .execute_batch(
            "INSERT INTO documents (id, kind, title, epic, feature, status, date, branch, state, private, path)
            VALUES ('JOURNAL-2026-10-25-1', 'journal', 'a', '', '', '', '2026-10-25', 'develop', 'merged', 0, '')",
        )
        .unwrap();

    let text = "EN-151 and F48, E49.\nEN-999 then F47\n[F48.AC1] REF-NOPE ADR-9999\nJOURNAL-2026-10-25-1 JOURNAL-2026-10-25-2\n";
    let missing = missing_ids(&database, text).unwrap();

    let _ = fs::remove_dir_all(&folder);
    // `REF-` and `ADR-` are no kind the records hold yet: not checked. `JOURNAL-` is.
    assert_eq!(
        missing,
        [
            (2, "EN-999".to_owned()),
            (2, "F47".to_owned()),
            (4, "JOURNAL-2026-10-25-2".to_owned()),
        ]
    );
}

#[test]
fn merges_or_drops_the_drafts_of_one_branch_never_its_merged_rows_nor_another_branchs() {
    let (folder, database) = place("branches");

    import(&folder, &database);
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

    import(&folder, &database);

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

    import(&folder, &database);

    let missing = missing_ids(&database, "US-000, F00, E00 and REV-000; US-001\n").unwrap();

    let _ = fs::remove_dir_all(&folder);
    assert_eq!(missing, [(1, "US-001".to_owned())]);
}
