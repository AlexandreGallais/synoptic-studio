//! Tests of the ids a text cites (EN-152), which `check-ids` checks. No file is read: every
//! text is written here.

use knowledge::document::{citations, is_citation};

#[test]
fn f48_ac2_knows_the_ids_the_project_cites() {
    for id in [
        "REF-GIT-BRANCH",
        "REF-SQLITE-FTS5",
        "DERIV-bit",
        "DERIV-free-shape-stretch",
        "E49",
        "F48",
        "EN-152",
        "ADR-0081",
        "AUD-050",
        "JOURNAL-2026-10-25-3",
    ] {
        assert!(is_citation(id), "{id}");
    }
    for word in [
        "REF-",
        "REF-git",
        "DERIV-",
        "DERIV-Bit",
        "E",
        "F",
        "E2E",
        "T1",
        "AC4",
        "UTF-8",
        "EN-15",
        "X-123",
        "EN-152-",
        "SQLite",
        "en-152",
        "Ab-123",
        "EN-152-x",
        "",
    ] {
        assert!(!is_citation(word), "{word}");
    }
}

#[test]
fn f48_ac2_lists_the_ids_cited_once_each_in_the_order_first_met() {
    // `EN-151.T1` cites EN-151; `--EN-152--` its id without the dashes around it; `F48` twice.
    assert_eq!(
        citations("F48 and `REF-SQLITE-FTS5` (EN-151.T1), --EN-152--, F48's E2E.\n"),
        ["F48", "REF-SQLITE-FTS5", "EN-151", "EN-152"]
    );
    assert_eq!(citations("no id here"), Vec::<String>::new());
}
