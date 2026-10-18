//! Tests of the session's state and of the `session-state` hook (EN-104), the cases of
//! `.claude/hooks/session-state.test.ts` carried over.

use std::fs::{self, File, FileTimes};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{Duration, SystemTime};

use hooks::session::{StoryRow, feature_line, is_feature_file, is_stale, state_text, story_rows};

#[test]
fn a_call_is_stale_only_past_one_day() {
    // 86,400,000 ms is one day: a file written exactly a day ago stays, 1 ms more is removed.
    assert!(!is_stale(86_400_000));
    assert!(is_stale(86_400_001));
    assert!(!is_stale(0));
}

/// Story table of a feature with one finished story out of three.
const TABLE: &str = "| ID | Type | Title | Status |\n| --- | --- | --- | --- |\n| [EN-001](../stories/a.md) | Enabler | First | done |\n| [SP-001](../stories/b.md) | Spike | Research | draft |\n| [VAL-001](../stories/c.md) | Validation | Validate | draft |";

/// A row of a story table.
fn row(id: &str, status: &str, title: &str) -> StoryRow {
    StoryRow {
        id: id.into(),
        status: status.into(),
        title: title.into(),
    }
}

#[test]
fn reads_the_story_rows_of_a_feature_in_delivery_order() {
    let rows: Vec<String> = story_rows(TABLE)
        .iter()
        .map(|row| format!("{} {}", row.id, row.status))
        .collect();

    assert_eq!(rows, ["EN-001 done", "SP-001 draft", "VAL-001 draft"]);
}

#[test]
fn reads_a_row_as_the_regular_expression_did() {
    // Spaces kept inside the title, trimmed around it; a carriage return ends the line.
    assert_eq!(
        story_rows("|  [US-018](x.md)  | Story |  Turn a  shape  | in-progress |\r\n"),
        [row("US-018", "in-progress", "Turn a  shape")]
    );
    for line in [
        "| [US-01](x) | t | a | done |",
        "| [us-001](x) | t | a | done |",
        "| [US-0012](x) | t | a | done |",
        "| [-001](x) | t | a | done |",
        "| [US-001](x) | t | a | Done |",
        "| [US-001](x) | t | a | done | more |",
        "| [US-001](x) | t | a | done | x",
        "| [US-001](x) | t | a |  |",
        "| [US-001] | t | a | done |",
        " | [US-001](x) | t | a | done |",
    ] {
        assert_eq!(story_rows(line), [], "{line}");
    }
}

#[test]
fn names_the_next_unfinished_story_of_a_started_feature() {
    assert_eq!(
        feature_line("E01-F01-x.md", &story_rows(TABLE)).as_deref(),
        Some("E01-F01-x: 1/3 stories finished; next SP-001 (draft) — Research")
    );
}

#[test]
fn sees_a_feature_started_by_its_first_story_in_progress() {
    assert_eq!(
        feature_line(
            "E14-F11-x.md",
            &[
                row("SP-010", "in-progress", "Research"),
                row("VAL-010", "draft", "Validate")
            ]
        )
        .as_deref(),
        Some("E14-F11-x: 0/2 stories finished; next SP-010 (in-progress) — Research")
    );
}

#[test]
fn is_silent_for_a_feature_not_started_or_finished() {
    assert_eq!(feature_line("f.md", &[row("US-001", "draft", "t")]), None);
    assert_eq!(feature_line("f.md", &[row("US-001", "done", "t")]), None);
    assert_eq!(feature_line("f.md", &[row("US-001", "dropped", "t")]), None);
    // A story dropped is finished: it starts the feature and is not the next one.
    assert_eq!(
        feature_line(
            "f.md",
            &[row("US-001", "dropped", "t"), row("US-002", "draft", "u")]
        )
        .as_deref(),
        Some("f: 1/2 stories finished; next US-002 (draft) — u")
    );
    assert_eq!(feature_line("f.md", &[]), None);
}

#[test]
fn tells_a_feature_file_by_its_name() {
    assert!(is_feature_file("E17-F21-agent-tools-in-rust.md"));
    assert!(is_feature_file("E01-F01-.md"));
    for name in [
        "E17-F21-x.md.bak",
        "E17-EN-104-x.md",
        "E1-F21-x.md",
        "E17-F2-x.md",
        "README.md",
        "E17-F21x.md",
    ] {
        assert!(!is_feature_file(name), "{name}");
    }
}

#[test]
fn prints_the_state_with_or_without_a_feature_started() {
    assert_eq!(
        state_text("develop", 2, "abc1234 docs: x", &[], &[]),
        "Project state (SessionStart hook, ADR-0021):\n- branch develop, 2 uncommitted file(s), last commit: abc1234 docs: x\n- no feature started\n- a story is implemented only when ready; the Product Owner's /run sets its scope ready (ADR-0045).\n"
    );
    assert!(
        state_text("d", 0, "c", &["E01-F01-x: 1/3".into()], &[])
            .contains("\n- E01-F01-x: 1/3\n- a story")
    );
    // The stale release binaries named after the features, before the rule.
    assert!(
        state_text("d", 0, "c", &[], &["target/release/a is missing".into(), "target/release/b is older than s.rs".into()])
            .contains("\n- no feature started\n- release binaries older than their sources, which the hooks run: target/release/a is missing; target/release/b is older than s.rs; `npm run rust:build` rebuilds them\n- a story")
    );
}

/// A command without the `GIT_*` variables of the process running the tests: under the pre-push
/// hook, `GIT_DIR` names the repository itself (`.claude/rules/tooling.md`); started through the
/// shared `test_support::command` (EN-103), with a home of its own.
fn clean(program: &str) -> Command {
    let mut command = test_support::command(program);
    let home = std::env::temp_dir().join(format!("session-home-{}", std::process::id()));

    fs::create_dir_all(&home).expect("home");
    command
        .env("HOME", home)
        .env("GIT_CONFIG_NOSYSTEM", "1")
        .env("GIT_AUTHOR_NAME", "t")
        .env("GIT_AUTHOR_EMAIL", "t@example.org")
        .env("GIT_COMMITTER_NAME", "t")
        .env("GIT_COMMITTER_EMAIL", "t@example.org");
    command
}

/// Runs Git in a folder, panicking on a failure.
fn git(folder: &Path, arguments: &[&str]) {
    let status = clean("git")
        .args(arguments)
        .current_dir(folder)
        .status()
        .expect("git runs");

    assert!(status.success(), "git {arguments:?}");
}

/// A repository with two feature files, one started, and one commit.
fn project() -> PathBuf {
    let root = std::env::temp_dir().join(format!("session-state-{}", std::process::id()));
    let features = root.join("docs/backlog/features");

    let _ = fs::remove_dir_all(&root);
    fs::create_dir_all(&features).expect("folders");
    fs::write(features.join("E02-F02-later.md"), TABLE).expect("feature");
    fs::write(
        features.join("E01-F01-first.md"),
        "| [EN-002](x) | t | Only | in-progress |\n",
    )
    .expect("feature");
    fs::write(features.join("README.md"), TABLE).expect("readme");
    git(&root, &["init", "-q", "-b", "develop"]);
    git(&root, &["add", "."]);
    git(&root, &["commit", "-q", "-m", "docs: start"]);
    fs::write(root.join("new.txt"), "x").expect("uncommitted");
    root
}

#[test]
fn prints_the_project_state_and_removes_the_calls_left_running_for_a_day() {
    let root = project();
    let running = root.join(".git/metrics/running");
    let old = SystemTime::now() - Duration::from_secs(86_401);

    fs::create_dir_all(&running).expect("running");
    fs::write(running.join("old.json"), "{}").expect("old");
    fs::write(running.join("new.json"), "{}").expect("new");
    File::options()
        .write(true)
        .open(running.join("old.json"))
        .and_then(|file| file.set_times(FileTimes::new().set_modified(old)))
        .expect("old time");

    let output = clean(env!("CARGO_BIN_EXE_session-state"))
        .env("CLAUDE_PROJECT_DIR", &root)
        .output()
        .expect("hook runs");
    let text = String::from_utf8_lossy(&output.stdout);

    assert!(output.status.success());
    // Sorted by name: E01 before E02; the README is no feature.
    assert!(
        text.starts_with("Project state (SessionStart hook, ADR-0021):\n- branch develop, 1 uncommitted file(s), last commit: "),
        "{text}"
    );
    assert!(text.contains(" docs: start\n- E01-F01-first: 0/1 stories finished; next EN-002 (in-progress) — Only\n- E02-F02-later: 1/3 stories finished; next SP-001 (draft) — Research\n- a story"), "{text}");
    assert!(!running.join("old.json").exists());
    assert!(running.join("new.json").exists());
    let _ = fs::remove_dir_all(&root);
}

#[test]
fn fails_without_a_repository() {
    let folder = std::env::temp_dir().join(format!("session-none-{}", std::process::id()));

    fs::create_dir_all(&folder).expect("folder");
    let output = clean(env!("CARGO_BIN_EXE_session-state"))
        .env("CLAUDE_PROJECT_DIR", &folder)
        .env("GIT_CEILING_DIRECTORIES", std::env::temp_dir())
        .output()
        .expect("hook runs");

    assert_eq!(output.status.code(), Some(1));
    assert_eq!(output.stdout, [] as [u8; 0]);
    let _ = fs::remove_dir_all(&folder);
}
