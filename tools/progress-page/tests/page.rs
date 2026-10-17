//! Tests of the page's HTML (EN-087).

use progress_page::agents::Agent;
use progress_page::backlog::StoryRow;
use progress_page::commits::Commit;
use progress_page::page::{Page, RunView, Usage, civil_from_days, escape, render, utc_text};
use progress_page::progress::{FeatureInput, features_progress};
use run_totals::tally::Tokens;

// 2026-10-17 19:50:00 UTC (see tests/agents.rs).
const NOW: i64 = 1_792_266_600_000;
const MINUTE: i64 = 60_000;

#[test]
fn writes_dates_in_utc() {
    assert_eq!(civil_from_days(0), (1970, 1, 1));
    // 20,743 days: 2026-10-17 (tests/agents.rs); 59 days: 1970-03-01 (31 + 28 = 59).
    assert_eq!(civil_from_days(20_743), (2026, 10, 17));
    assert_eq!(civil_from_days(59), (1970, 3, 1));
    // 2024-02-29: 19,782 days (54 years of which 13 leap = 19,723, then 31 + 28 = 59).
    assert_eq!(civil_from_days(19_782), (2024, 2, 29));
    assert_eq!(civil_from_days(-1), (1969, 12, 31));
    assert_eq!(utc_text(NOW), "2026-10-17 19:50 UTC");
    assert_eq!(utc_text(NOW + 59_999), "2026-10-17 19:50 UTC");
}

#[test]
fn escapes_text_for_html() {
    assert_eq!(
        escape("<b class=\"x\">a & b</b>"),
        "&lt;b class=&quot;x&quot;&gt;a &amp; b&lt;/b&gt;"
    );
}

fn page() -> Page {
    let worker = Agent {
        id: "a1".to_owned(),
        kind: "story-worker".to_owned(),
        description: "EN-087 <progress> page".to_owned(),
        story: Some("EN-087".to_owned()),
        start: NOW - 30 * MINUTE,
        last: NOW - MINUTE,
        running: true,
        tokens: Tokens {
            input: 0,
            cache_write: 1_000,
            cache_read: 200_000,
            output: 3_000,
        },
    };
    let feature = FeatureInput {
        epic: "E17".to_owned(),
        epic_title: "Fast tooling".to_owned(),
        id: "F20".to_owned(),
        title: "A context diet".to_owned(),
        rows: vec![
            StoryRow {
                id: "EN-086".to_owned(),
                kind: "Enabler".to_owned(),
                title: "Parked questions".to_owned(),
                status: "done".to_owned(),
            },
            StoryRow {
                id: "EN-087".to_owned(),
                kind: "Enabler".to_owned(),
                title: "A progress page".to_owned(),
                status: "ready".to_owned(),
            },
        ],
    };
    let commits = [Commit {
        hash: "0123456789abcdef".to_owned(),
        time: NOW - 10 * MINUTE,
        subject: "feat(tooling): write the page".to_owned(),
        refs: vec!["EN-087".to_owned()],
        closes: Vec::new(),
    }];

    Page {
        now: NOW,
        run: Some(RunView {
            start: NOW - 125 * MINUTE,
            end: 0,
            tokens: Some(Tokens {
                input: 4,
                cache_write: 10_000,
                cache_read: 1_000_000,
                output: 20_000,
            }),
        }),
        usage: Some(Usage {
            five_hour: 15,
            seven_day: 55,
        }),
        features: features_progress(
            &[feature],
            &commits,
            std::slice::from_ref(&worker),
            &[],
            NOW,
        ),
        agents: vec![worker],
    }
}

#[test]
fn f20_ac5_shows_the_run_its_agents_and_the_tree() {
    let html = render(&page());

    // The run: started 125 min before, so 2 h 05, open; tokens 4 + 10,000 + 1,000,000 + 20,000.
    assert!(html.contains(
        "<td>2026-10-17 17:45 UTC</td><td>2 h 05, open</td><td class=\"n\">1,030,004</td>"
    ));
    assert!(html.contains("15 % of the five hours, 55 % of the week"));
    // The worker: 30 min, 1,000 + 200,000 + 3,000 = 204,000 tokens, its description escaped.
    assert!(html.contains("<td>EN-087</td><td>story-worker</td><td>EN-087 &lt;progress&gt; page</td><td class=\"n\">30 min</td><td class=\"n\">204,000</td>"));
    // The tree: epic, feature with 1 of 2 closed, the story in progress and its commit.
    assert!(html.contains("<h2>E17 · Fast tooling</h2>"));
    assert!(html.contains("<h3>F20 · A context diet</h3><p class=\"muted\">1 of 2 stories closed · 30 min · 204,000 tokens · 1 commits</p>"));
    assert!(html.contains("<tr class=\"now\"><td>EN-087</td>"));
    assert!(html.contains("<td class=\"ready\">in progress</td>"));
    assert!(html.contains("<li><code>01234567</code> feat(tooling): write the page</li>"));
    assert!(html.contains("<td class=\"done\">done</td><td class=\"n\">—</td>"));
    // The rest of the run: 1,030,004 − 204,000 = 826,004.
    assert!(html.contains("agents outside the stories: 826,004."));
    // Static: no script, nothing loaded from elsewhere.
    assert!(!html.contains("<script"));
    assert!(!html.contains("http"));
}

#[test]
fn says_when_there_is_no_run_no_agent_and_no_feature() {
    let html = render(&Page {
        run: None,
        usage: None,
        features: Vec::new(),
        agents: Vec::new(),
        ..page()
    });

    assert!(html.contains("No run recorded"));
    assert!(html.contains("<h2>Agents running</h2><p class=\"muted\">None.</p>"));
    assert!(html.contains("No feature branch"));
    assert!(!html.contains("Plan used"));
    assert!(!html.contains("outside the stories"));
}

#[test]
fn shows_an_ended_run_without_its_agents_done() {
    let mut ended = page();

    ended.run = Some(RunView {
        start: NOW - 3 * MINUTE,
        end: NOW - MINUTE,
        tokens: None,
    });
    ended.agents[0].running = false;

    let html = render(&ended);

    // 3 − 1 = 2 minutes; no count yet: every kind 0.
    assert!(html.contains("<td>2 min, ended</td><td class=\"n\">—</td>"));
    assert!(html.contains("<p class=\"muted\">None.</p>"));
}
