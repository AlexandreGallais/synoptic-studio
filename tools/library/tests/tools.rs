//! Tests of the three tools' answers and of the use each records, on the cases of
//! `library-pages.test.ts` (EN-106). Every library is in memory; a page is stored fresh, so that
//! no call goes online, and a failing fetch is a connection to a port nothing listens on.

use std::net::TcpListener;
use std::path::Path;

use library::chunks::page_chunks;
use library::server::{Library, tools};
use library::store::{StoredPage, open_library, save_page};
use library::tools::{call_tool, js_number, tool_list, tool_result};
use rusqlite::Connection;
use serde_json::{Value, json};

/// Markdown of the stored page.
const MARKDOWN: &str = "# Hooks\n\nIntro.\n\n## Input\n\nThe `duration_ms` field.";

/// Address of the stored page.
const URL: &str = "https://x.org/h.md";

/// A library holding the page, read just now.
fn stored() -> Connection {
    let database = open_library(Path::new(":memory:")).unwrap();
    let page = StoredPage {
        fetched_at: calendar::now_ms(),
        markdown: MARKDOWN.to_owned(),
        title: "Hooks".to_owned(),
        url: URL.to_owned(),
        ..StoredPage::default()
    };

    save_page(&database, &page, &page_chunks(MARKDOWN)).unwrap();

    database
}

/// The uses recorded: project, tool, origin, address.
fn uses(database: &Connection) -> Vec<(String, String, String, String)> {
    database
        .prepare("SELECT project, tool, origin, url FROM project_uses ORDER BY rowid")
        .unwrap()
        .query_map([], |row| {
            Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?))
        })
        .unwrap()
        .collect::<Result<_, _>>()
        .unwrap()
}

/// An address on the loopback where nothing listens.
fn closed_address() -> String {
    let port = TcpListener::bind("127.0.0.1:0")
        .unwrap()
        .local_addr()
        .unwrap()
        .port();

    format!("http://127.0.0.1:{port}/a")
}

#[test]
fn f21_ac3_the_three_tools_answer_as_the_typescript_server_did() {
    let database = stored();
    let call = |name: &str, input: Value| call_tool(&database, "studio", name, &input).unwrap();

    assert_eq!(
        call("library_search", json!({ "query": "nothing" })),
        "No section found: get the page with library_get."
    );
    assert_eq!(
        call("library_search", json!({ "query": "duration", "limit": 1 })),
        "https://x.org/h.md §1 — Hooks › Hooks › Input\n  ## Input\n\nThe `[duration]_ms` field."
    );
    assert!(
        call("library_get", json!({ "url": URL }))
            .ends_with("(library), changed not stated\n\n0. Hooks — 15 characters\n1. Hooks › Input — 34 characters")
    );
    assert_eq!(
        call("library_read", json!({ "url": URL, "sections": [1, 0] })),
        "# Hooks\n\nIntro.\n\n## Input\n\nThe `duration_ms` field."
    );
    // Any other name reads, as the TypeScript server's last branch did.
    assert_eq!(
        call("library_other", json!({ "url": URL, "sections": ["1"] })),
        "## Input\n\nThe `duration_ms` field."
    );
}

#[test]
fn each_search_and_each_page_read_records_one_use_with_its_project() {
    let database = stored();

    call_tool(&database, "studio", "library_get", &json!({ "url": URL })).unwrap();
    call_tool(
        &database,
        "atlas",
        "library_read",
        &json!({ "url": URL, "sections": [0] }),
    )
    .unwrap();
    call_tool(
        &database,
        "atlas",
        "library_search",
        &json!({ "query": "duration" }),
    )
    .unwrap();
    call_tool(
        &database,
        "atlas",
        "library_search",
        &json!({ "query": "nothing" }),
    )
    .unwrap();

    let owned = |a: &str, b: &str, c: &str, d: &str| {
        (a.to_owned(), b.to_owned(), c.to_owned(), d.to_owned())
    };

    assert_eq!(
        uses(&database),
        [
            owned("studio", "get", "library", URL),
            owned("atlas", "read", "library", URL),
            owned("atlas", "search", "found", ""),
            owned("atlas", "search", "none", ""),
        ]
    );
}

#[test]
fn a_get_whose_fetch_fails_is_recorded_as_failed_so_the_page_hook_lets_it_be_read_otherwise() {
    let database = stored();
    let url = closed_address();
    let (text, is_error) = tool_result(&database, "studio", "library_get", &json!({ "url": url }));

    assert!(is_error);
    assert!(text.starts_with("Error: curl: (7)"), "{text}");
    assert_eq!(
        uses(&database),
        [(
            "studio".to_owned(),
            "get".to_owned(),
            "failed".to_owned(),
            url
        )]
    );
}

#[test]
fn a_read_or_a_search_that_fails_records_nothing() {
    let database = stored();
    let (_, read_failed) = tool_result(
        &database,
        "studio",
        "library_read",
        &json!({ "url": closed_address(), "sections": [0] }),
    );
    let (text, search_failed) = tool_result(
        &database,
        "studio",
        "library_search",
        &json!({ "query": " " }),
    );

    assert!(read_failed && search_failed);
    assert!(text.starts_with("Error: fts5: syntax error"), "{text}");
    assert_eq!(uses(&database), []);
}

#[test]
fn a_call_without_arguments_reads_each_as_empty() {
    let (text, is_error) = tool_result(&stored(), "studio", "library_search", &Value::Null);

    assert!(is_error, "{text}");
}

#[test]
fn a_search_limit_that_is_not_a_whole_number_is_refused_as_sqlite_refused_it() {
    let (text, is_error) = tool_result(
        &stored(),
        "studio",
        "library_search",
        &json!({ "query": "hooks", "limit": 1.5 }),
    );

    assert_eq!(
        (text.as_str(), is_error),
        ("Error: datatype mismatch", true)
    );
}

#[test]
fn a_section_number_is_read_as_javascript_number_reads_it() {
    for (value, number) in [
        (json!(2), 2.0),
        (json!("3"), 3.0),
        (json!(" 4 "), 4.0),
        (json!(""), 0.0),
        (json!(true), 1.0),
        (json!(null), 0.0),
        (json!("0x10"), 16.0),
        (json!("1e1"), 10.0),
        (json!("-Infinity"), f64::NEG_INFINITY),
    ] {
        assert_eq!(js_number(&value).to_bits(), number.to_bits(), "{value}");
    }

    for value in [
        json!("a"),
        json!("--1"),
        json!([1]),
        json!({}),
        json!("0x"),
        json!("infinity"),
    ] {
        assert!(js_number(&value).is_nan(), "{value}");
    }
}

#[test]
fn the_tools_are_listed_with_the_schemas_they_had() {
    let listed = tools();

    assert_eq!(
        listed
            .iter()
            .map(|tool| tool.name.as_ref())
            .collect::<Vec<_>>(),
        ["library_search", "library_get", "library_read"]
    );
    assert_eq!(
        listed[2].input_schema["required"],
        json!(["url", "sections"])
    );
    assert_eq!(tool_list().len(), 3);
}

#[test]
fn the_server_answers_a_call_from_its_library_for_its_project() {
    let server = Library::new(stored(), "studio".to_owned(), None);
    let (text, is_error) = server.answer("library_read", &json!({ "url": URL, "sections": [0] }));

    assert_eq!((text.as_str(), is_error), ("# Hooks\n\nIntro.", false));
}
