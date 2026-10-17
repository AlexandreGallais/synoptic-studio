//! Tests of the library's store: the schema `library-store.ts` wrote, read and written again
//! without a migration (EN-106). Every file is made in a folder of the system's temporary folder;
//! the machine's own library is never opened.

use std::fs;
use std::path::{Path, PathBuf};

use library::chunks::Chunk;
use library::store::{
    StoredPage, find_page, fts_query, library_file, list_pages, open_library, page_sections,
    save_page, search_library, touch_page,
};
use rusqlite::Connection;

/// A small page about the fillet of a corner.
fn page() -> StoredPage {
    StoredPage {
        etag: "\"v1\"".to_owned(),
        fetched_at: 1000,
        last_modified: String::new(),
        markdown: "# Fillet\n\nA fillet replaces a corner by an arc.\n\n## Setback\n\nThe setback is r / tan(θ/2)."
            .to_owned(),
        modified_at: 0,
        title: "Fillets".to_owned(),
        url: "https://example.org/fillet".to_owned(),
    }
}

/// The page's two sections, as `pageChunks` cuts it at its headings.
fn chunks() -> Vec<Chunk> {
    vec![
        Chunk {
            heading: "Fillet".to_owned(),
            index: 0,
            text: "# Fillet\n\nA fillet replaces a corner by an arc.".to_owned(),
        },
        Chunk {
            heading: "Fillet › Setback".to_owned(),
            index: 1,
            text: "## Setback\n\nThe setback is r / tan(θ/2).".to_owned(),
        },
    ]
}

/// A library in memory holding the page.
fn stored() -> Connection {
    let database = open_library(Path::new(":memory:")).unwrap();

    save_page(&database, &page(), &chunks()).unwrap();

    database
}

/// An empty folder of its own in the temporary folder.
fn folder(name: &str) -> PathBuf {
    let folder = std::env::temp_dir().join(format!("library-store-{name}-{}", std::process::id()));

    let _ = fs::remove_dir_all(&folder);
    fs::create_dir_all(&folder).unwrap();

    folder
}

#[test]
fn a_stored_page_and_its_sections_are_found() {
    let database = stored();

    assert_eq!(
        find_page(&database, "https://example.org/fillet").unwrap(),
        Some(page())
    );
    assert_eq!(
        find_page(&database, "https://example.org/none").unwrap(),
        None
    );
    assert_eq!(
        page_sections(&database, "https://example.org/fillet")
            .unwrap()
            .iter()
            .map(|section| section.heading.as_str())
            .collect::<Vec<_>>(),
        ["Fillet", "Fillet › Setback"]
    );
}

#[test]
fn a_search_finds_the_section_holding_every_word_stemmed_else_any_word() {
    let database = stored();
    let indexes = |words: &str| -> Vec<i64> {
        search_library(&database, words, &5)
            .unwrap()
            .iter()
            .map(|hit| hit.index)
            .collect()
    };

    // `setbacks` is stemmed to `setback` by the porter tokenizer: only section 1 holds both words.
    assert_eq!(indexes("setbacks tan"), [1]);
    // No section holds both words: any word, and `corner` is in section 0.
    assert_eq!(indexes("corner nowhere"), [0]);
    assert_eq!(indexes("nothing"), Vec::<i64>::new());

    let hit = &search_library(&database, "arc", &5).unwrap()[0];

    assert_eq!(
        (
            hit.title.as_str(),
            hit.heading.as_str(),
            hit.snippet.as_str()
        ),
        (
            "Fillets",
            "Fillet",
            "# Fillet\n\nA fillet replaces a corner by an [arc]."
        )
    );
}

#[test]
fn a_search_without_words_is_an_error_as_in_sqlite() {
    // `MATCH ''` is an FTS5 syntax error, which the TypeScript server gave back as the tool's error.
    assert!(search_library(&stored(), "  ", &5).is_err());
}

#[test]
fn each_word_is_quoted_so_that_the_text_is_never_read_as_query_syntax() {
    assert_eq!(fts_query(" a\"b  OR ", "AND"), "\"a\"\"b\" AND \"OR\"");
    assert_eq!(fts_query("bm25 rank", "OR"), "\"bm25\" OR \"rank\"");
    assert_eq!(fts_query("", "AND"), "");
}

#[test]
fn a_page_read_again_is_marked_and_pages_are_listed_last_read_first() {
    let database = open_library(Path::new(":memory:")).unwrap();
    let old = StoredPage {
        fetched_at: 500,
        url: "https://example.org/old".to_owned(),
        ..page()
    };

    save_page(&database, &page(), &[]).unwrap();
    save_page(&database, &old, &[]).unwrap();
    touch_page(&database, "https://example.org/fillet", 2000).unwrap();

    assert_eq!(
        list_pages(&database)
            .unwrap()
            .iter()
            .map(|listed| (listed.url.as_str(), listed.fetched_at))
            .collect::<Vec<_>>(),
        [
            ("https://example.org/fillet", 2000),
            ("https://example.org/old", 500)
        ]
    );
}

#[test]
fn a_page_saved_again_replaces_its_sections() {
    let database = stored();

    save_page(&database, &page(), &chunks()[..1]).unwrap();

    assert_eq!(
        page_sections(&database, "https://example.org/fillet")
            .unwrap()
            .len(),
        1
    );
}

#[test]
fn a_library_made_before_the_project_moves_its_uses_into_the_table_that_names_it() {
    let folder = folder("migration");
    let path = folder.join("library.sqlite");
    let old = Connection::open(&path).unwrap();

    old.execute_batch(
        "CREATE TABLE uses (time INTEGER NOT NULL, tool TEXT NOT NULL, url TEXT NOT NULL, query TEXT NOT NULL, origin TEXT NOT NULL);
        INSERT INTO uses VALUES (1, 'get', 'https://x.org', '', 'online');",
    )
    .unwrap();
    drop(old);

    let database = open_library(&path).unwrap();

    // An older copy of the scripts still writes and reads `uses`, five columns, no project.
    database
        .execute_batch("INSERT INTO uses VALUES (2, 'read', 'https://x.org', '', 'library')")
        .unwrap();
    // Opened again: nothing left to move, nothing created twice.
    drop(open_library(&path).unwrap());

    let rows: Vec<(i64, String)> = database
        .prepare("SELECT time, project FROM project_uses ORDER BY time")
        .unwrap()
        .query_map([], |row| Ok((row.get(0)?, row.get(1)?)))
        .unwrap()
        .collect::<Result<_, _>>()
        .unwrap();
    let view: i64 = database
        .query_row("SELECT COUNT(*) FROM uses", [], |row| row.get(0))
        .unwrap();

    drop(database);
    fs::remove_dir_all(&folder).unwrap();

    assert_eq!(rows, [(1, String::new()), (2, String::new())]);
    assert_eq!(view, 2);
}

#[test]
fn a_store_as_the_typescript_server_left_it_is_read_and_written_without_a_migration() {
    let folder = folder("typescript");
    let path = folder.join("library.sqlite");
    let written = Connection::open(&path).unwrap();

    // The schema `library-store.ts` created, and rows as `node:sqlite` bound them: a position is
    // a real there, since a JavaScript number is bound as one into a column without affinity.
    written
        .execute_batch(
            "CREATE TABLE pages (url TEXT PRIMARY KEY, title TEXT NOT NULL, fetched_at INTEGER NOT NULL,
              modified_at INTEGER NOT NULL, etag TEXT NOT NULL, last_modified TEXT NOT NULL,
              markdown TEXT NOT NULL);
            CREATE VIRTUAL TABLE chunks USING fts5(url UNINDEXED, position UNINDEXED, heading, text,
              tokenize = 'porter unicode61');
            CREATE TABLE project_uses (time INTEGER NOT NULL, tool TEXT NOT NULL, url TEXT NOT NULL,
              query TEXT NOT NULL, origin TEXT NOT NULL, project TEXT NOT NULL);
            CREATE VIEW uses AS SELECT time, tool, url, query, origin FROM project_uses;
            CREATE TRIGGER uses_insert INSTEAD OF INSERT ON uses BEGIN
              INSERT INTO project_uses VALUES (NEW.time, NEW.tool, NEW.url, NEW.query, NEW.origin, '');
            END;
            INSERT INTO pages VALUES ('https://x.org/h', 'Hooks', 1760000000000, 0, '', '', '# Hooks');
            INSERT INTO chunks VALUES ('https://x.org/h', 0.0, 'Hooks', '# Hooks');
            INSERT INTO chunks VALUES ('https://x.org/h', 1.0, 'Hooks › Input', '## Input duration_ms');
            INSERT INTO project_uses VALUES (1, 'get', 'https://x.org/h', '', 'online', 'studio');",
        )
        .unwrap();
    drop(written);

    let database = open_library(&path).unwrap();
    let hits = search_library(&database, "duration_ms", &5).unwrap();

    assert_eq!(hits.iter().map(|hit| hit.index).collect::<Vec<_>>(), [1]);
    assert_eq!(
        find_page(&database, "https://x.org/h")
            .unwrap()
            .unwrap()
            .fetched_at,
        1_760_000_000_000
    );
    assert_eq!(
        page_sections(&database, "https://x.org/h")
            .unwrap()
            .iter()
            .map(|section| section.index)
            .collect::<Vec<_>>(),
        [0, 1]
    );

    // The older checkout's scripts insert through the view: the row lands in `project_uses`.
    database
        .execute_batch("INSERT INTO uses VALUES (2, 'read', 'https://x.org/h', '', 'library')")
        .unwrap();

    let objects: i64 = database
        .query_row(
            "SELECT COUNT(*) FROM sqlite_master WHERE name IN ('uses', 'uses_insert')",
            [],
            |row| row.get(0),
        )
        .unwrap();
    let uses: i64 = database
        .query_row("SELECT COUNT(*) FROM project_uses", [], |row| row.get(0))
        .unwrap();

    drop(database);
    fs::remove_dir_all(&folder).unwrap();

    assert_eq!((objects, uses), (2, 2));
}

#[test]
fn a_missing_folder_is_made_for_a_new_library() {
    let folder = folder("fresh");
    let path = folder.join("deep").join("library.sqlite");

    drop(open_library(&path).unwrap());

    let exists = path.exists();

    fs::remove_dir_all(&folder).unwrap();

    assert!(exists);
}

#[test]
fn the_library_file_is_named_by_its_own_variable_else_the_user_data_folder() {
    assert_eq!(
        library_file(Some("/tmp/l.sqlite".to_owned()), Some("/home/a".to_owned())),
        Some(PathBuf::from("/tmp/l.sqlite"))
    );
    assert_eq!(
        library_file(None, Some("/home/a".to_owned())),
        Some(PathBuf::from(
            "/home/a/.local/share/agent-library/library.sqlite"
        ))
    );
    assert_eq!(
        library_file(Some(String::new()), Some("/home/a".to_owned())),
        Some(PathBuf::from(
            "/home/a/.local/share/agent-library/library.sqlite"
        ))
    );
    assert_eq!(library_file(None, None), None);
}
