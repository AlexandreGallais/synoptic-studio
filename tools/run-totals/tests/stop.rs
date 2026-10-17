//! Tests of when a run must stop on its own and which stories wait for an answer (EN-086).

use run_totals::stop::{WEEK_LIMIT_PERCENT, parked_story, stop_text};

#[test]
fn f20_ac6_the_week_limit_is_ninety_percent() {
    assert_eq!(WEEK_LIMIT_PERCENT, 90);
}

#[test]
fn f20_ac6_stops_at_ninety_percent_of_the_week_and_not_below() {
    // 89 < 90: the run goes on; 90 and 100 ≥ 90: it stops.
    assert_eq!(
        stop_text(Some(89), &[]),
        (false, "go: week 89 % used, the limit is 90 %\n".to_owned())
    );
    assert_eq!(
        stop_text(Some(90), &[]),
        (true, "stop: week 90 % used, the limit is 90 %\n".to_owned())
    );
    assert!(stop_text(Some(100), &[]).0);
    assert!(!stop_text(Some(0), &[]).0);
}

#[test]
fn f20_ac6_goes_on_when_the_usage_is_unknown() {
    // No `usage.json` yet (before the session's first response): nothing says to stop.
    assert_eq!(
        stop_text(None, &[]),
        (false, "go: week usage unknown\n".to_owned())
    );
}

#[test]
fn f20_ac6_names_the_parked_stories_without_stopping_for_them() {
    let parked = ["EN-101".to_owned(), "US-020".to_owned()];

    assert_eq!(
        stop_text(Some(55), &parked),
        (
            false,
            "go: week 55 % used, the limit is 90 %\nparked: EN-101, US-020\n".to_owned()
        )
    );
}

#[test]
fn f20_ac6_reads_a_parked_story_from_its_front_matter() {
    let parked = "---\nid: EN-101\nepic: E17\nstatus: parked\n---\n\n# EN-101\n";

    assert_eq!(parked_story(parked), Some("EN-101".to_owned()));
}

#[test]
fn f20_ac6_ignores_a_story_not_parked_or_without_front_matter() {
    assert_eq!(parked_story("---\nid: EN-102\nstatus: ready\n---\n"), None);
    // `status: parked` in the body, not in the front matter.
    assert_eq!(
        parked_story("---\nid: EN-103\nstatus: done\n---\n\nstatus: parked\n"),
        None
    );
    assert_eq!(parked_story("# A page\n\nstatus: parked\n"), None);
    assert_eq!(parked_story(""), None);
    // Parked, but no id: nothing to name.
    assert_eq!(parked_story("---\nstatus: parked\n---\n"), None);
}
