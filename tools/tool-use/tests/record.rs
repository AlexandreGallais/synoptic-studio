//! One use, one line (EN-102): written in the main checkout's Git folder, a worktree's included;
//! dropped without a word when it cannot be written; never mixed with another process's line.
//! `[F21.AC7]` marks a test proving the feature's criterion. No test runs Git: the Git folders are
//! made by hand.

use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};
use std::thread;
use std::time::{Duration, Instant};

use tool_use::{
    Checkout, MAIN_CHECKOUT, ToolUse, append, checkout, civil_from_days, iso_instant, json_line,
    json_string, run, run_in,
};

/// Lines each of the two processes appends at once.
const LINES_PER_PROCESS: usize = 2000;

/// The variable that makes the ignored test below one of the two appending processes.
const CHILD_FILE: &str = "TOOL_USE_TEST_APPEND_TO";

/// A new temporary folder of its own, removed at the end of the test.
struct TempFolder(PathBuf);

impl TempFolder {
    fn new(name: &str) -> Self {
        let path = env::temp_dir().join(format!("tool-use-{name}-{}", std::process::id()));
        let _ = fs::remove_dir_all(&path);
        fs::create_dir_all(&path).expect("temporary folder");
        Self(path)
    }
}

impl Drop for TempFolder {
    fn drop(&mut self) {
        // Already gone is as good as removed; a read-only folder is made writable first.
        let _ = make_writable(&self.0);
        let _ = fs::remove_dir_all(&self.0);
    }
}

#[cfg(unix)]
fn make_writable(path: &Path) -> std::io::Result<()> {
    use std::os::unix::fs::PermissionsExt;

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        if entry.file_type()?.is_dir() {
            fs::set_permissions(entry.path(), fs::Permissions::from_mode(0o755))?;
            make_writable(&entry.path())?;
        }
    }
    Ok(())
}

#[cfg(not(unix))]
fn make_writable(_path: &Path) -> std::io::Result<()> {
    Ok(())
}

/// The lines of a file, none when it is missing.
fn lines(file: &Path) -> Vec<String> {
    fs::read_to_string(file)
        .map(|text| text.lines().map(str::to_owned).collect())
        .unwrap_or_default()
}

#[test]
fn f21_ac7_writes_one_line_per_use_in_the_main_checkout() {
    let root = TempFolder::new("main");
    fs::create_dir_all(root.0.join(".git")).expect("Git folder");
    let inside = root.0.join("tools").join("format");
    fs::create_dir_all(&inside).expect("a folder inside");

    let code = run_in(Some(&inside), "format", "--check", || 3);

    assert_eq!(code, 3);
    let written = lines(&root.0.join(".git/metrics/tool-uses.jsonl"));
    assert_eq!(written.len(), 1);
    let line = &written[0];
    assert!(
        line.starts_with(
            r#"{"tool":"format","subcommand":"--check","worktree":"main","start":"20"#
        ),
        "{line}"
    );
    assert!(line.ends_with(r#","exitCode":3}"#), "{line}");
    assert!(line.contains(r#"Z","durationMs":"#), "{line}");

    run_in(Some(&inside), "lint", "", || 0);
    assert_eq!(
        lines(&root.0.join(".git/metrics/tool-uses.jsonl")).len(),
        2,
        "a second use is appended"
    );
}

#[test]
fn f21_ac7_writes_a_worktrees_use_in_the_main_checkout_naming_the_worktree() {
    let root = TempFolder::new("worktree");
    let git_dir = root.0.join("main/.git/worktrees/en-102");
    fs::create_dir_all(&git_dir).expect("worktree's Git folder");
    fs::write(git_dir.join("commondir"), "../..\n").expect("commondir");
    let worktree = root.0.join("main/.claude/worktrees/en-102");
    fs::create_dir_all(&worktree).expect("worktree");
    fs::write(
        worktree.join(".git"),
        format!("gitdir: {}\n", git_dir.display()),
    )
    .expect(".git file");

    assert_eq!(
        checkout(&worktree),
        Some(Checkout {
            uses_file: git_dir.join("../..").join("metrics/tool-uses.jsonl"),
            worktree: "en-102".to_owned(),
        })
    );
    assert_eq!(run_in(Some(&worktree), "git-sequences", "sync", || 0), 0);
    let written = lines(&root.0.join("main/.git/metrics/tool-uses.jsonl"));
    assert_eq!(written.len(), 1);
    assert!(
        written[0].contains(r#""worktree":"en-102""#),
        "{}",
        written[0]
    );
    assert!(!git_dir.join("metrics").exists());
}

#[test]
fn a_relative_gitdir_and_a_submodule_without_commondir_are_read_too() {
    let root = TempFolder::new("relative");
    fs::create_dir_all(root.0.join(".git/modules/child")).expect("submodule's Git folder");
    fs::create_dir_all(root.0.join("child")).expect("submodule");
    fs::write(root.0.join("child/.git"), "gitdir: ../.git/modules/child\n").expect(".git file");

    assert_eq!(
        checkout(&root.0.join("child")),
        Some(Checkout {
            uses_file: root
                .0
                .join("child/../.git/modules/child/metrics/tool-uses.jsonl"),
            worktree: MAIN_CHECKOUT.to_owned(),
        })
    );
}

#[test]
fn f21_ac7_drops_the_use_without_a_git_folder_and_keeps_the_code() {
    let root = TempFolder::new("none");
    fs::write(root.0.join(".git"), "not a gitdir line\n").expect("a .git file of no worktree");

    assert_eq!(checkout(&root.0), None);
    assert_eq!(run_in(Some(&root.0), "lint", "", || 1), 1);
    assert_eq!(run_in(None, "lint", "", || 2), 2);
    assert_eq!(
        fs::read_dir(&root.0).expect("folder").count(),
        1,
        "nothing written beside the .git file"
    );
}

#[test]
fn f21_ac7_drops_the_use_when_metrics_cannot_be_made_and_keeps_the_code() {
    let root = TempFolder::new("blocked");
    fs::create_dir_all(root.0.join(".git")).expect("Git folder");
    fs::write(root.0.join(".git/metrics"), "a file where the folder goes").expect("blocking file");

    assert_eq!(run_in(Some(&root.0), "format", "--write", || 0), 0);
    assert!(append(&root.0.join(".git/metrics/tool-uses.jsonl"), "{}\n").is_err());
}

#[test]
fn append_never_creates_the_git_folder() {
    let root = TempFolder::new("no-git");

    assert!(append(&root.0.join(".git/metrics/tool-uses.jsonl"), "{}\n").is_err());
    assert!(!root.0.join(".git").exists());
}

#[cfg(unix)]
#[test]
fn f21_ac7_drops_the_use_in_a_read_only_folder_and_keeps_the_code() {
    use std::os::unix::fs::PermissionsExt;

    let root = TempFolder::new("read-only");
    let metrics = root.0.join(".git/metrics");
    fs::create_dir_all(&metrics).expect("metrics folder");
    fs::set_permissions(&metrics, fs::Permissions::from_mode(0o555)).expect("read-only");

    assert_eq!(run_in(Some(&root.0), "mutate", "", || 1), 1);
    assert!(!metrics.join("tool-uses.jsonl").exists());
}

#[test]
fn a_debug_build_records_nothing_and_keeps_the_code() {
    // The tests are a debug build: `run` returns the code without looking for a Git folder.
    assert_eq!(run("lint", "", || 4), ExitCode::from(4));
}

/// One of the two appending processes when its variable names a file; nothing otherwise.
#[test]
#[ignore = "run by the test below as a second process"]
fn append_as_a_child_process() {
    let Ok(file) = env::var(CHILD_FILE) else {
        return;
    };
    let line = json_line(&ToolUse {
        tool: "a-tool-with-a-long-name-so-that-a-line-is-long".repeat(4),
        subcommand: format!("child-{}", std::process::id()),
        worktree: MAIN_CHECKOUT.to_owned(),
        start_ms: 0,
        duration_ms: 1,
        exit_code: 0,
    });
    // Both processes start appending together, once the parent has started them both.
    let go = PathBuf::from(format!("{file}.go"));
    let deadline = Instant::now() + Duration::from_secs(20);
    while !go.exists() {
        assert!(Instant::now() < deadline, "the parent never said go");
        thread::sleep(Duration::from_millis(1));
    }
    for _ in 0..LINES_PER_PROCESS {
        append(Path::new(&file), &line).expect("appended");
    }
}

#[test]
fn f21_ac7_two_processes_appending_at_once_never_mix_their_lines() {
    let root = TempFolder::new("concurrent");
    fs::create_dir_all(root.0.join(".git")).expect("Git folder");
    let file = root.0.join(".git/metrics/tool-uses.jsonl");
    let start = || {
        Command::new(env::current_exe().expect("this test binary"))
            .args([
                "--exact",
                "append_as_a_child_process",
                "--ignored",
                "--quiet",
            ])
            .env(CHILD_FILE, &file)
            .spawn()
            .expect("child starts")
    };
    let mut children = [start(), start()];
    fs::create_dir_all(root.0.join(".git/metrics")).expect("metrics folder");
    fs::write(root.0.join(".git/metrics/tool-uses.jsonl.go"), "").expect("go");

    for child in &mut children {
        assert!(child.wait().expect("child ends").success());
    }
    let written = lines(&file);
    assert_eq!(written.len(), 2 * LINES_PER_PROCESS);
    let tool = "a-tool-with-a-long-name-so-that-a-line-is-long".repeat(4);
    for line in &written {
        assert!(
            line.starts_with(&format!(r#"{{"tool":"{tool}","subcommand":"child-"#))
                && line.ends_with(
                    r#""start":"1970-01-01T00:00:00.000Z","durationMs":1,"exitCode":0}"#
                ),
            "a mixed line: {line}"
        );
    }
}

#[test]
fn writes_the_line_of_a_use() {
    // 2026-10-16 is 20 742 days after 1970-01-01: 20 742 × 86 400 000 = 1 792 108 800 000 ms, then
    // 1 h 02 min 03.045 s = 3 723 045 ms.
    let tool_use = ToolUse {
        tool: "git-sequences".to_owned(),
        subcommand: "sync".to_owned(),
        worktree: "en-102".to_owned(),
        start_ms: 1_792_108_800_000 + 3_723_045,
        duration_ms: 812,
        exit_code: 1,
    };

    assert_eq!(
        json_line(&tool_use),
        "{\"tool\":\"git-sequences\",\"subcommand\":\"sync\",\"worktree\":\"en-102\",\
         \"start\":\"2026-10-16T01:02:03.045Z\",\"durationMs\":812,\"exitCode\":1}\n"
    );
}

#[test]
fn escapes_quotes_backslashes_and_control_characters() {
    assert_eq!(json_string(""), r#""""#);
    assert_eq!(json_string(r#"a"b\c"#), r#""a\"b\\c""#);
    assert_eq!(json_string("a\nb\rc\td\u{1}é"), r#""a\nb\rc\td\u0001é""#);
}

#[test]
fn writes_instants_at_the_epoch_on_a_leap_day_and_before_1970() {
    assert_eq!(iso_instant(0), "1970-01-01T00:00:00.000Z");
    // 2024-02-29: 54 years of 365 days and the 13 leap days 1972 to 2020 give 2024-01-01 at day
    // 19 723; then 31 + 28 days: 19 782 × 86 400 000 = 1 709 164 800 000 ms.
    assert_eq!(iso_instant(1_709_164_800_000), "2024-02-29T00:00:00.000Z");
    assert_eq!(iso_instant(-1), "1969-12-31T23:59:59.999Z");
    // 2000-03-01, after the leap day of a year divisible by 400: day 11 017.
    assert_eq!(civil_from_days(11_017), (2000, 3, 1));
    assert_eq!(civil_from_days(11_016), (2000, 2, 29));
}

/// Days from 1970-01-01 to a date (H. Hinnant, `days_from_civil`, as `hooks` computes it).
fn days_from_civil(year: i64, month: i64, day: i64) -> i64 {
    let year = if month <= 2 { year - 1 } else { year };
    let era = year.div_euclid(400);
    let year_of_era = year - era * 400;
    let day_of_year = (153 * (month + if month > 2 { -3 } else { 9 }) + 2) / 5 + day - 1;
    let day_of_era = year_of_era * 365 + year_of_era / 4 - year_of_era / 100 + day_of_year;

    era * 146_097 + day_of_era - 719_468
}

#[test]
fn every_day_from_1900_to_2200_is_the_next_date_and_counts_back_to_its_number() {
    // 1900-01-01 is day −25 567; each next day is the next date of the calendar.
    let mut previous = civil_from_days(-25_567);
    assert_eq!(previous, (1900, 1, 1));
    for days in -25_566..84_000 {
        let date = civil_from_days(days);
        let next_in_month = (previous.0, previous.1, previous.2 + 1);
        let next_month = if previous.1 == 12 {
            (previous.0 + 1, 1, 1)
        } else {
            (previous.0, previous.1 + 1, 1)
        };
        assert!(
            date == next_in_month || date == next_month,
            "{previous:?} then {date:?}"
        );
        assert_eq!(days_from_civil(date.0, date.1, date.2), days);
        previous = date;
    }
}
