//! Tests of the units of work: the cases of `scripts/metrics-work.test.ts`.

use metrics::work::{
    IDLE_MS, Moment, TaskCommit, active_ms, commits_by_unit, task_commits, task_ids, task_of,
    unit_of, work_stats,
};

/// A tool call of 100 ms on `branch` at `time`, with 10 tokens of which 1 written.
fn moment(time: i64, branch: &str) -> Moment {
    Moment {
        branch: branch.to_owned(),
        duration_ms: 100.0,
        is_failure: false,
        is_tool_call: true,
        output_tokens: 1,
        session: "s".to_owned(),
        time,
        tokens: 10,
    }
}

#[test]
fn reads_the_story_else_the_feature_else_keeps_the_branch() {
    assert_eq!(unit_of("feat/us-018-turn-a-shape"), "US-018");
    assert_eq!(unit_of("docs/ret-001-retrospective-e01"), "RET-001");
    assert_eq!(unit_of("refactor/nam-001-naming-pass-e15"), "NAM-001");
    assert_eq!(unit_of("feature/f04-free-rotation"), "F04");
    assert_eq!(unit_of("main"), "main");
    assert_eq!(unit_of(""), "(no branch)");
    // A story id inside a word, or in capitals, is not one; nor a feature outside `feature/`.
    assert_eq!(unit_of("chore/xen-107-x"), "chore/xen-107-x");
    assert_eq!(unit_of("chore/EN-107-x"), "chore/EN-107-x");
    assert_eq!(unit_of("docs/f04-x"), "docs/f04-x");
    assert_eq!(unit_of("chore/en-1070"), "chore/en-1070");
}

#[test]
fn adds_the_gaps_between_moments_each_capped_at_the_idle_limit() {
    // Gaps: 1 000 (kept), then IDLE_MS + 5 000 (capped at IDLE_MS).
    assert_eq!(
        active_ms(&[1000 + IDLE_MS + 5000 + 1000, 0, 1000]),
        1000 + IDLE_MS
    );
    assert_eq!(active_ms(&[42]), 0);
    assert_eq!(active_ms(&[]), 0);
}

#[test]
fn reads_the_task_ids_of_the_refs_lines_nothing_else() {
    assert_eq!(
        task_ids("Body.\n\nRefs: US-011.T2, ADR-0007\nRefs: EN-001.T1\nCloses: US-011"),
        ["US-011.T2", "EN-001.T1"]
    );
    assert_eq!(
        task_ids("Refs: us-011.T2, US-11.T2, US-011.T"),
        [] as [std::string::String; 0]
    );
}

#[test]
fn keeps_task_commits_once_oldest_first() {
    let log = [
        [
            "bbb",
            "2026-10-12T10:00:00+02:00",
            "second",
            "Refs: US-001.T2",
        ],
        [
            "aaa",
            "2026-10-12T09:00:00+02:00",
            "first",
            "Refs: US-001.T1",
        ],
        [
            "ccc",
            "2026-10-12T09:00:00+02:00",
            "first",
            "Refs: US-001.T1",
        ],
        ["ddd", "2026-10-12T11:00:00+02:00", "docs", "No reference"],
    ]
    .map(|fields| fields.join("\u{1f}"))
    .join("\u{1e}\n");
    let commits = task_commits(&log);
    // 2026-10-12T07:00:00Z: 20 738 days and 7 h (09:00 at UTC+2).
    let seven = 20_738 * 86_400_000 + 7 * 3_600_000;

    assert_eq!(
        commits
            .iter()
            .map(|commit| (commit.hash.as_str(), commit.subject.as_str(), commit.time))
            .collect::<Vec<_>>(),
        [
            ("ccc", "first", seven),
            ("bbb", "second", seven + 3_600_000)
        ]
    );
}

#[test]
fn gives_a_moment_to_the_next_task_commit_of_its_story() {
    let commits = [
        TaskCommit {
            hash: "aaaaaaa1".to_owned(),
            subject: "first".to_owned(),
            tasks: vec!["US-001.T1".to_owned()],
            time: 2000,
        },
        TaskCommit {
            hash: "bbbbbbb2".to_owned(),
            subject: "second".to_owned(),
            tasks: vec!["US-001.T2".to_owned(), "US-001.T3".to_owned()],
            time: 5000,
        },
    ];
    let units = commits_by_unit(&commits);

    assert_eq!(
        task_of(&moment(1000, "feat/us-001-x"), &units),
        "US-001.T1 aaaaaaa first"
    );
    // A commit at the moment's time counts.
    assert_eq!(
        task_of(&moment(2000, "feat/us-001-x"), &units),
        "US-001.T1 aaaaaaa first"
    );
    assert_eq!(
        task_of(&moment(3000, "feat/us-001-x"), &units),
        "US-001.T2, US-001.T3 bbbbbbb second"
    );
    assert_eq!(
        task_of(&moment(6000, "feat/us-001-x"), &units),
        "US-001 (no task commit)"
    );
    assert_eq!(
        task_of(&moment(1000, "main"), &units),
        "main (no task commit)"
    );
    // A commit naming two tasks of its story is listed once under it.
    assert_eq!(units["US-001"].len(), 2);
}

#[test]
fn sums_the_moments_of_each_unit() {
    let moments = [
        moment(0, "feat/us-001-x"),
        moment(1000, "feat/us-001-x"),
        moment(0, "main"),
    ];
    let stats = work_stats(&moments, |entry| unit_of(&entry.branch));

    assert_eq!(stats.len(), 2);
    assert_eq!(
        (
            stats[0].key.as_str(),
            stats[0].active_ms,
            stats[0].calls,
            stats[0].output_tokens,
            stats[0].tokens
        ),
        ("US-001", 1000, 2, 2, 20)
    );
    assert!((stats[0].tool_ms - 200.0).abs() < f64::EPSILON);
    assert_eq!(
        (stats[1].key.as_str(), stats[1].active_ms, stats[1].calls),
        ("main", 0, 1)
    );
}
