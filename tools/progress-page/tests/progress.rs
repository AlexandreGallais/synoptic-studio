//! Tests of a story's and a feature's progress (EN-087).

use progress_page::agents::Agent;
use progress_page::backlog::StoryRow;
use progress_page::commits::Commit;
use progress_page::progress::{FeatureInput, features_progress, story_progress};
use run_totals::tally::Tokens;

const MINUTE: i64 = 60_000;
const NOW: i64 = 1_000 * MINUTE;

fn row(id: &str, status: &str) -> StoryRow {
    StoryRow {
        id: id.to_owned(),
        kind: "Enabler".to_owned(),
        title: format!("{id}'s title"),
        status: status.to_owned(),
    }
}

fn commit(hash: &str, minute: i64, refs: &[&str], closes: &[&str]) -> Commit {
    Commit {
        hash: hash.to_owned(),
        time: minute * MINUTE,
        subject: format!("subject {hash}"),
        refs: refs.iter().map(|id| (*id).to_owned()).collect(),
        closes: closes.iter().map(|id| (*id).to_owned()).collect(),
    }
}

fn agent(story: &str, start: i64, last: i64, running: bool, output: i64) -> Agent {
    Agent {
        id: format!("a-{story}"),
        kind: "story-worker".to_owned(),
        description: format!("{story} work"),
        story: Some(story.to_owned()),
        start: start * MINUTE,
        last: last * MINUTE,
        running,
        tokens: Tokens {
            output,
            ..Tokens::default()
        },
    }
}

#[test]
fn f20_ac5_a_done_story_lasts_from_its_first_activity_to_its_done_commit() {
    // Agent from minute 900, commits at 910 and 930, done commit at 940, agent last seen at 935:
    // 940 − 900 = 40 minutes.
    let commits = [
        commit("c1", 910, &["EN-001"], &[]),
        commit("c3", 940, &[], &["EN-001"]),
        commit("c2", 930, &["EN-001"], &[]),
        commit("x", 920, &["EN-002"], &[]),
    ];
    let story = story_progress(
        &row("EN-001", "done"),
        &commits,
        &[agent("EN-001", 900, 935, false, 7)],
        &["en-001".to_owned()],
        NOW,
    );

    assert_eq!(story.duration_ms(), 40 * MINUTE);
    assert_eq!(story.tokens.output, 7);
    // Its own commits, oldest first; a done story is never in progress, even with a worktree.
    let hashes: Vec<&str> = story.commits.iter().map(|c| c.hash.as_str()).collect();

    assert_eq!(hashes, ["c1", "c2", "c3"]);
    assert!(!story.in_progress);
}

#[test]
fn f20_ac5_a_story_worked_on_lasts_until_now() {
    // Agent from minute 980, still running at 999: 1000 − 980 = 20 minutes.
    let story = story_progress(
        &row("EN-002", "ready"),
        &[commit("c1", 990, &["EN-002"], &[])],
        &[agent("EN-002", 980, 999, true, 0)],
        &[],
        NOW,
    );

    assert!(story.in_progress);
    assert_eq!(story.end, NOW);
    assert_eq!(story.duration_ms(), 20 * MINUTE);
}

#[test]
fn a_story_stopped_without_done_lasts_to_its_last_activity() {
    // Worktree kept, agent ended at 960 after a start at 950, last commit at 955: 10 minutes.
    let story = story_progress(
        &row("EN-003", "ready"),
        &[commit("c1", 955, &["EN-003"], &[])],
        &[agent("EN-003", 950, 960, false, 0)],
        &["EN-003".to_owned()],
        NOW,
    );

    assert!(story.in_progress);
    assert_eq!(story.duration_ms(), 10 * MINUTE);
}

#[test]
fn a_story_untouched_by_the_run_has_no_duration() {
    let story = story_progress(&row("EN-004", "ready"), &[], &[], &[], NOW);

    assert!(!story.in_progress);
    assert_eq!((story.start, story.end, story.duration_ms()), (0, 0, 0));
}

#[test]
fn f20_ac5_a_feature_sums_its_stories() {
    // EN-001 from 900 to 940, EN-002 from 980 to now (1000): 1000 − 900 = 100 minutes; tokens
    // 7 + 5 = 12; commits 2 + 1 = 3.
    let feature = FeatureInput {
        epic: "E17".to_owned(),
        epic_title: "Tooling".to_owned(),
        id: "F20".to_owned(),
        title: "A diet".to_owned(),
        rows: vec![
            row("EN-001", "done"),
            row("EN-002", "ready"),
            row("EN-003", "ready"),
        ],
    };
    let commits = [
        commit("c1", 910, &["EN-001"], &[]),
        commit("c2", 940, &[], &["EN-001"]),
        commit("c3", 990, &["EN-002"], &[]),
    ];
    let agents = [
        agent("EN-001", 900, 935, false, 7),
        agent("EN-002", 980, 999, true, 5),
    ];
    let progress = features_progress(&[feature], &commits, &agents, &[], NOW);

    assert_eq!(progress[0].duration_ms(), 100 * MINUTE);
    assert_eq!(progress[0].tokens().total(), 12);
    assert_eq!(progress[0].commit_count(), 3);
    // A feature no story of which was touched.
    let idle = features_progress(
        &[FeatureInput {
            rows: vec![row("EN-009", "ready")],
            ..progress[0].feature.clone()
        }],
        &[],
        &[],
        &[],
        NOW,
    );

    assert_eq!(idle[0].duration_ms(), 0);
}
