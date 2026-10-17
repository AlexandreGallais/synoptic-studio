//! Tests of the page's address kept for its run (EN-087).

use progress_page::address::{address_for, is_artifact_address, render_address};

const URL: &str = "https://claude.ai/code/artifact/5fbea6f3-1";

#[test]
fn accepts_only_a_claude_ai_address() {
    assert!(is_artifact_address(URL));
    assert!(!is_artifact_address("https://claude.ai/"));
    assert!(!is_artifact_address("http://claude.ai/code/artifact/1"));
    assert!(!is_artifact_address("https://example.com/claude.ai/1"));
    assert!(!is_artifact_address("https://claude.ai/code/artifact/1 2"));
}

#[test]
fn f20_ac5_keeps_the_address_for_its_run_only() {
    let saved = render_address(1_792_266_000_000, URL);

    assert_eq!(
        saved,
        "{\"run\":1792266000000,\"url\":\"https://claude.ai/code/artifact/5fbea6f3-1\"}\n"
    );
    assert_eq!(address_for(&saved, 1_792_266_000_000).as_deref(), Some(URL));
    // Another run, a missing file, an address that is not one.
    assert_eq!(address_for(&saved, 1_792_266_000_001), None);
    assert_eq!(address_for("", 1_792_266_000_000), None);
    assert_eq!(
        address_for(&render_address(5, "https://example.com/"), 5),
        None
    );
}
