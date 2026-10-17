//! Tests of the record of the library's uses and of its report, on the cases of
//! `library-usage.test.ts` (EN-106). Every library is in memory.

use std::path::Path;

use library::store::{StoredPage, open_library, save_page};
use library::usage::{Use, project_of, record_use, usage_report};
use rusqlite::Connection;

/// A library holding one page and its uses: studio got it online, read it from the library and
/// searched twice; atlas read it from the library.
fn library() -> Connection {
    let database = open_library(Path::new(":memory:")).unwrap();
    let page = StoredPage {
        fetched_at: 1,
        markdown: "# Hooks".to_owned(),
        title: "Hooks".to_owned(),
        url: "https://x.org/hooks".to_owned(),
        ..StoredPage::default()
    };
    let read = Use {
        origin: "library",
        project: "studio",
        query: "",
        tool: "read",
        url: "https://x.org/hooks",
    };

    save_page(&database, &page, &[]).unwrap();
    record_use(
        &database,
        &Use {
            origin: "online",
            tool: "get",
            ..read.clone()
        },
        1000,
    )
    .unwrap();
    record_use(&database, &read, 2000).unwrap();
    record_use(
        &database,
        &Use {
            project: "atlas",
            ..read.clone()
        },
        3000,
    )
    .unwrap();
    record_use(
        &database,
        &Use {
            origin: "found",
            query: "duration",
            tool: "search",
            url: "",
            ..read.clone()
        },
        3000,
    )
    .unwrap();
    record_use(
        &database,
        &Use {
            origin: "none",
            query: "duration",
            tool: "search",
            url: "",
            ..read
        },
        4000,
    )
    .unwrap();

    database
}

#[test]
fn the_pages_consulted_and_the_share_served_without_going_online_are_counted() {
    // Three consultations, two from the library: 66.7 %, rounded to 67.
    assert!(
        usage_report(&library(), 0)
            .unwrap()
            .starts_with("3 pages consulted, 67 % served by the library without going online.")
    );
}

#[test]
fn the_sources_and_the_searches_are_ranked() {
    let report = usage_report(&library(), 0).unwrap();

    assert!(report.contains("| Hooks | <https://x.org/hooks> | 3 | 1 |"));
    assert!(report.contains("| `duration` | 2 | 1 |"));
}

#[test]
fn the_uses_are_grouped_by_the_project_that_asked() {
    let report = usage_report(&library(), 0).unwrap();

    // studio: get online, read from the library, two searches; atlas: one read from the library.
    assert!(report.contains("| studio | 2 | 1 | 2 |"));
    assert!(report.contains("| atlas | 1 | 0 | 0 |"));
}

#[test]
fn only_what_happened_since_a_time_is_counted() {
    assert!(
        usage_report(&library(), 2500)
            .unwrap()
            .contains("1 pages consulted, 100 % served")
    );
}

#[test]
fn an_empty_library_reports_zero_and_empty_tables() {
    let report = usage_report(&open_library(Path::new(":memory:")).unwrap(), 0).unwrap();

    assert_eq!(
        report,
        [
            "0 pages consulted, 0 % served by the library without going online.",
            "### Projects",
            "| Project | Pages consulted | Of which online | Searches |\n| --- | --- | --- | --- |",
            "### Sources consulted most",
            "| Source | Address | Consulted | Of which online |\n| --- | --- | --- | --- |",
            "### Searches",
            "| Words searched | Times | Found |\n| --- | --- | --- |",
        ]
        .join("\n\n")
    );
}

#[test]
fn a_use_without_project_and_a_pipe_in_a_cell_are_written_for_markdown() {
    let database = open_library(Path::new(":memory:")).unwrap();

    record_use(
        &database,
        &Use {
            origin: "none",
            project: "",
            query: "a|b",
            tool: "search",
            url: "",
        },
        1,
    )
    .unwrap();

    let report = usage_report(&database, 0).unwrap();

    assert!(report.contains("| (not recorded) | 0 | 0 | 1 |"));
    assert!(report.contains("| `a\\|b` | 1 | 0 |"));
}

#[test]
fn the_project_is_the_name_of_its_folder() {
    assert_eq!(
        project_of("/home/a/github/synoptic-studio"),
        "synoptic-studio"
    );
    assert_eq!(project_of("/home/a/github/atlas/"), "atlas");
}

#[test]
fn the_project_of_a_worktree_is_the_project_not_the_story() {
    assert_eq!(
        project_of("/home/a/github/SVG editor/.claude/worktrees/en-101"),
        "SVG editor"
    );
    assert_eq!(
        project_of("/home/a/github/SVG editor/.claude/worktrees/en-101/scripts"),
        "SVG editor"
    );
    assert_eq!(
        project_of("/home/a/github/SVG editor/.claude/worktrees"),
        "SVG editor"
    );
    // A folder only named like it is no worktree folder.
    assert_eq!(project_of("/home/a/x/.claude/worktreesy/b"), "b");
}

#[test]
fn no_folder_has_no_project() {
    assert_eq!(project_of(""), "");
    assert_eq!(project_of("/"), "");
}
