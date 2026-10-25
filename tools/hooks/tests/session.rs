//! Tests of the session's state and of the `session-state` hook (EN-104), the cases of
//! `.claude/hooks/session-state.test.ts` carried over.

use std::fs::{self, File, FileTimes};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{Duration, SystemTime};

use hooks::session::{StoryRow, feature_line, feature_stories, is_stale, state_text};

#[test]
fn a_call_is_stale_only_past_one_day() {
    // 86,400,000 ms is one day: a file written exactly a day ago stays, 1 ms more is removed.
    assert!(!is_stale(86_400_000));
    assert!(is_stale(86_400_001));
    assert!(!is_stale(0));
}

/// A row of a feature's stories.
fn row(id: &str, status: &str, title: &str) -> StoryRow {
    StoryRow {
        id: id.into(),
        status: status.into(),
        title: title.into(),
    }
}

/// The rows `knowledge sql` gives for two features, in the order of `FEATURE_STORIES`, then a row
/// of another shape.
fn records_rows() -> Vec<Vec<String>> {
    let mut rows: Vec<Vec<String>> = [
        ["E01-F01-x", "EN-001", "done", "First"],
        ["E01-F01-x", "SP-001", "draft", "Research"],
        ["E01-F01-x", "VAL-001", "draft", "Validate"],
        ["E02-F02-y", "US-002", "draft", "Later"],
    ]
    .iter()
    .map(|cells| cells.iter().map(|&cell| cell.to_owned()).collect())
    .collect();

    rows.push(vec!["short".to_owned()]);
    rows
}

#[test]
fn groups_the_stories_of_each_feature_in_their_order() {
    let features = feature_stories(&records_rows());

    // A row of another shape is left out.
    assert_eq!(
        features,
        [
            (
                "E01-F01-x".to_owned(),
                vec![
                    row("EN-001", "done", "First"),
                    row("SP-001", "draft", "Research"),
                    row("VAL-001", "draft", "Validate"),
                ]
            ),
            (
                "E02-F02-y".to_owned(),
                vec![row("US-002", "draft", "Later")]
            ),
        ]
    );
}

#[test]
fn names_the_next_unfinished_story_of_a_started_feature() {
    let features = feature_stories(&records_rows());

    assert_eq!(
        feature_line(&features[0].0, &features[0].1).as_deref(),
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
fn prints_the_state_with_or_without_a_feature_started() {
    assert_eq!(
        state_text("develop", 2, "abc1234 docs: x", &[], &[]),
        "Project state (SessionStart hook, ADR-0021):\n- branch develop, 2 uncommitted file(s), last commit: abc1234 docs: x\n- no feature started\n- a story is implemented only when ready; the Product Owner's /run sets its scope ready (ADR-0072).\n"
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

/// A repository with one commit, and a copy of the hook beside a `knowledge` script standing for
/// the records: two features, one started, so that no test reads the real records (EN-154).
fn project(name: &str) -> PathBuf {
    let root = std::env::temp_dir().join(format!("session-state-{name}-{}", std::process::id()));
    let bin = root.join("bin");

    let _ = fs::remove_dir_all(&root);
    fs::create_dir_all(&bin).expect("folders");
    fs::copy(
        env!("CARGO_BIN_EXE_session-state"),
        bin.join("session-state"),
    )
    .expect("hook copied");
    fs::write(
        bin.join("knowledge"),
        "#!/bin/sh\nprintf 'feature\\tid\\tstatus\\ttitle\\nE01-F01-first\\tEN-002\\tin-progress\\tOnly\\nE02-F02-later\\tEN-001\\tdone\\tFirst\\nE02-F02-later\\tSP-001\\tdraft\\tResearch\\nE02-F02-later\\tVAL-001\\tdraft\\tValidate\\n'\n",
    )
    .expect("script");
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;

        fs::set_permissions(bin.join("knowledge"), fs::Permissions::from_mode(0o755))
            .expect("script runs");
    }
    fs::write(root.join(".gitignore"), "bin/\n").expect("ignored");
    git(&root, &["init", "-q", "-b", "develop"]);
    git(&root, &["add", "."]);
    git(&root, &["commit", "-q", "-m", "docs: start"]);
    fs::write(root.join("new.txt"), "x").expect("uncommitted");
    root
}

#[test]
fn prints_the_project_state_and_removes_the_calls_left_running_for_a_day() {
    let root = project("state");
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

    let output = clean(&root.join("bin/session-state").display().to_string())
        .env("CLAUDE_PROJECT_DIR", &root)
        .output()
        .expect("hook runs");
    let text = String::from_utf8_lossy(&output.stdout);

    assert!(output.status.success());
    // In the records' order: E01 before E02.
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

#[test]
fn leaves_the_features_out_when_the_records_cannot_be_read() {
    let root = project("unread");

    // No `knowledge` beside the hook: the session still starts, its state without the features.
    fs::remove_file(root.join("bin/knowledge")).expect("script removed");

    let output = clean(&root.join("bin/session-state").display().to_string())
        .env("CLAUDE_PROJECT_DIR", &root)
        .output()
        .expect("hook runs");
    let text = String::from_utf8_lossy(&output.stdout);

    assert!(output.status.success());
    assert!(text.contains("\n- no feature started\n"), "{text}");
    assert!(
        String::from_utf8_lossy(&output.stderr).contains("the records cannot be read"),
        "{output:?}"
    );
    let _ = fs::remove_dir_all(&root);
}
