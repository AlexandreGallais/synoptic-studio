//! Tests of the commits read from `git log` (EN-087).

use progress_page::commits::{footer_stories, parse_log, story_of_reference};

#[test]
fn reads_the_story_of_a_reference() {
    assert_eq!(story_of_reference("EN-087.T1").as_deref(), Some("EN-087"));
    assert_eq!(story_of_reference("VAL-021").as_deref(), Some("VAL-021"));
    // A feature, an issue, lower case, no number, letters after the dash.
    assert_eq!(story_of_reference("F20"), None);
    assert_eq!(story_of_reference("#12"), None);
    assert_eq!(story_of_reference("en-087"), None);
    assert_eq!(story_of_reference("EN-"), None);
    assert_eq!(story_of_reference("EN-08a"), None);
    assert_eq!(story_of_reference("E-1"), None);
}

#[test]
fn reads_every_story_of_a_footer_once() {
    let body = "subject\n\nRefs: EN-087.T1, EN-087.T2\nCloses: EN-087 F20\nRefs: AUD-022.T3\nNot Refs: EN-001\n";

    assert_eq!(footer_stories(body, "Refs"), vec!["EN-087", "AUD-022"]);
    assert_eq!(footer_stories(body, "Closes"), vec!["EN-087"]);
    assert_eq!(footer_stories("no footer", "Refs"), Vec::<String>::new());
}

#[test]
fn f20_ac5_parses_the_log_and_drops_a_rebased_copy() {
    // Three records as `git log --format=%H%x1f%at%x1f%s%x1f%B%x1e` writes them; the third is the
    // first one rebased: another hash, the same author time and subject.
    let log = "aaaa1111\u{1f}1792266000\u{1f}feat(tooling): write the page\u{1f}feat(tooling): write the page\n\nRefs: EN-087.T1\n\u{1e}\n\
bbbb2222\u{1f}1792266600\u{1f}docs(backlog): set EN-087 done\u{1f}docs(backlog): set EN-087 done\n\nCloses: EN-087\n\u{1e}\n\
cccc3333\u{1f}1792266000\u{1f}feat(tooling): write the page\u{1f}feat(tooling): write the page\n\nRefs: EN-087.T1\n\u{1e}\n";
    let commits = parse_log(log);

    assert_eq!(commits.len(), 2);
    assert_eq!(commits[0].hash, "aaaa1111");
    // Seconds to milliseconds: 1792266000 s = 1792266000000 ms.
    assert_eq!(commits[0].time, 1_792_266_000_000);
    assert_eq!(commits[0].refs, vec!["EN-087"]);
    assert_eq!(commits[0].closes, Vec::<String>::new());
    assert_eq!(commits[1].closes, vec!["EN-087"]);
    assert!(commits[1].is_of("EN-087"));
    assert!(!commits[1].is_of("EN-086"));
    // Nothing logged, a record cut short.
    assert_eq!(parse_log(""), Vec::new());
    assert_eq!(parse_log("dddd\u{1f}12\u{1e}"), Vec::new());
}
