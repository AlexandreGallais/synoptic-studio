//! Tests of what `should-stop` says (EN-086, EN-154): the run goes on whatever the week's usage,
//! and the stories waiting for an answer are named, read from the records' rows.

use run_totals::stop::{GO, PARKED, parked_of, stop_text};

#[test]
fn f20_ac6_never_stops_for_the_weeks_usage_any_more() {
    // The stop at 90 % of the week was dropped on 2026-10-25: the first word is always `go`.
    assert_eq!(stop_text(&[]), format!("{GO}\n"));
    assert!(GO.starts_with("go: "));
}

#[test]
fn f20_ac6_names_the_parked_stories_without_stopping_for_them() {
    let parked = ["EN-101".to_owned(), "US-020".to_owned()];

    assert_eq!(
        stop_text(&parked),
        format!("{GO}\nparked: EN-101, US-020\n")
    );
}

#[test]
fn f20_ac6_reads_the_parked_stories_from_the_records_rows() {
    let rows = vec![
        vec!["EN-101".to_owned()],
        vec![String::new()],
        Vec::new(),
        vec!["US-020".to_owned()],
    ];

    assert_eq!(parked_of(&rows), ["EN-101", "US-020"]);
    assert_eq!(
        PARKED,
        "SELECT id FROM stories WHERE status = 'parked' ORDER BY id"
    );
}
