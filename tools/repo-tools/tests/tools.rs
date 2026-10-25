//! `epic-check`, `branches-clean`, `records-save` and `backlog-status` run in repositories of
//! their own, `npm` and `gh` stubs first on `PATH`. Every command starts through
//! `test_support::command`, without the inherited `GIT_*` variables, its Git configuration a test
//! file of its own.

#![cfg(unix)]

use std::env;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use knowledge::snapshot::{NOT_SET, snapshot_name};
use knowledge::store::open_store;

/// A test's folder, its stubs and its Git configuration.
struct Lab {
    root: PathBuf,
}

impl Lab {
    fn new(name: &str, stubs: &[(&str, &str)]) -> Lab {
        let root = env::temp_dir().join(format!("repo-tools-{}-{name}", std::process::id()));

        let _ = fs::remove_dir_all(&root);
        fs::create_dir_all(root.join("bin")).expect("folder");

        let root = fs::canonicalize(&root).expect("canonical");

        for (name, text) in stubs {
            let path = root.join("bin").join(name);

            fs::write(&path, text).expect("stub");
            fs::set_permissions(&path, fs::Permissions::from_mode(0o755)).expect("mode");
        }
        fs::write(
            root.join("gitconfig"),
            "[user]\n\tname = Test\n\temail = test@example.com\n[commit]\n\tgpgsign = false\n[init]\n\tdefaultBranch = main\n",
        )
        .expect("config");
        Lab { root }
    }

    fn command(&self, program: &str, cwd: &Path) -> Command {
        let mut command = test_support::command(program);

        command
            .current_dir(cwd)
            .env(
                "PATH",
                format!(
                    "{}:{}",
                    self.root.join("bin").display(),
                    env::var("PATH").unwrap_or_default()
                ),
            )
            .env("HOME", self.root.join("home"))
            .env("GIT_CONFIG_GLOBAL", self.root.join("gitconfig"))
            .env("GIT_CONFIG_NOSYSTEM", "1")
            .env_remove("AGENT_RECORDS_ROOT")
            .env_remove("AGENT_KNOWLEDGE_SNAPSHOT")
            .env_remove("AGENT_LIBRARY_FILE");
        command
    }

    fn git(&self, cwd: &Path, words: &[&str]) -> String {
        let output = self.command("git", cwd).args(words).output().expect("git");

        assert!(
            output.status.success(),
            "git {}: {}",
            words.join(" "),
            String::from_utf8_lossy(&output.stderr)
        );
        String::from_utf8_lossy(&output.stdout).trim().to_owned()
    }

    fn commit(&self, cwd: &Path, file: &str) {
        fs::write(cwd.join(file), format!("{file}\n")).expect("file");
        self.git(cwd, &["add", file]);
        self.git(cwd, &["commit", "-q", "-m", file]);
    }
}

impl Drop for Lab {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}

fn text(output: &Output) -> String {
    String::from_utf8_lossy(&output.stdout).into_owned()
}

#[test]
fn f18_ac11_epic_check_prints_its_table_and_fails_on_a_finding() {
    let lab = Lab::new(
        "epic",
        &[
            (
                "npm",
                "#!/bin/sh\nprintf '%s\\n' '✓ lint 1.0 s' '✗ test 2.0 s' ' FAIL  a.test.ts > b' 'check:all failed in 3.0 s: test'; exit 1\n",
            ),
            (
                "gh",
                "#!/bin/sh\ncase \"$1 $3\" in \
                 'pr --base') echo '[{\"number\":9,\"title\":\"Release\",\"statusCheckRollup\":[{\"name\":\"Rust\",\"workflowName\":\"CI\",\"status\":\"COMPLETED\",\"conclusion\":\"FAILURE\"}]}]' ;; \
                 pr*) echo '#7 a pull request' ;; issue*) echo 0 ;; api*) exit 1 ;; esac\n",
            ),
        ],
    );
    let repo = lab.root.join("repo");

    fs::create_dir_all(repo.join("docs/backlog/features")).expect("folder");
    fs::create_dir_all(repo.join("docs/backlog/stories")).expect("folder");
    fs::write(
        repo.join("docs/backlog/stories/E99-F99-EN-1-a.md"),
        "status: done\n",
    )
    .expect("story");
    fs::write(
        repo.join("docs/backlog/stories/E99-F99-EN-2-b.md"),
        "status: ready\n",
    )
    .expect("story");
    fs::write(
        repo.join("docs/backlog/stories/E99-F99-EN-3-c.md"),
        "status: dropped\n",
    )
    .expect("story");
    lab.git(&repo, &["init", "-q"]);
    lab.commit(&repo, "a");
    lab.git(&repo, &["branch", "feat/old"]);
    fs::create_dir_all(repo.join(".git/metrics/running")).expect("metrics");
    fs::write(
        repo.join(".git/metrics/tool-calls.jsonl"),
        "{\"error\":\"\",\"time\":\"2026-10-13T10:00:00Z\",\"tool\":\"Bash\"}\nnot a record\n",
    )
    .expect("log");
    fs::write(
        repo.join(".git/metrics/running/1.json"),
        "{\"start\":0,\"summary\":\"ls\",\"tool\":\"Bash\"}",
    )
    .expect("running");

    let output = lab
        .command(env!("CARGO_BIN_EXE_epic-check"), &repo)
        .args(["E99", "2026-10-13"])
        .output()
        .expect("epic-check");
    let table = text(&output);
    let lines: Vec<&str> = table.lines().collect();

    assert_eq!(output.status.code(), Some(1), "{table}");
    assert_eq!(
        lines[..6],
        [
            "| Check | Result | Detail |",
            "| --- | --- | --- |",
            "| check:all (code, site, playground, dependencies) | **to look at** | test (lane test): a.test.ts > b |",
            "| branches left | **to look at** | feat/old |",
            "| pull requests open | ok | #7 a pull request |",
            "| issues open | ok | 0 |",
        ]
    );
    // GitHub refused the alerts: its error is the detail.
    assert!(lines[6].starts_with(
        "| code scanning alerts open (GitHub, Security) | **to look at** | Error: Command failed: gh api repos/"
    ));
    // The last pull request into main, its failed job named (I110).
    assert_eq!(
        lines[7],
        "| CI of the last pull request into main | **to look at** | #9 Release: failed, 1 of 1 jobs: CI / Rust |"
    );
    assert_eq!(
        lines[8],
        "| calls failed during the epic (2 % at most) | ok | 0 of 1; apart: 0 refused by the worktree isolation |"
    );
    // The running call's file was just written: not lost.
    assert!(lines[9].ends_with("| ok | none |"), "{}", lines[9]);
    // EN-3, dropped, is closed (I110).
    assert_eq!(
        lines[10],
        "| backlog of E99 | **to look at** | E99-F99-EN-2-b |"
    );
    assert!(
        lines[11]
            .starts_with("| private records | **to look at** | AGENT_RECORDS_ROOT is not set: ")
    );
    assert_eq!(
        lines[lines.len() - 1],
        "| golden file of refactors | ok | none written yet |"
    );
}

#[test]
fn f18_ac8_branches_clean_deletes_the_merged_and_gone_branches_and_keeps_a_fresh_story() {
    let lab = Lab::new("branches", &[]);
    let origin = lab.root.join("origin.git");
    let main = lab.root.join("main");
    let origin_path = origin.to_str().expect("path");

    lab.git(&lab.root, &["init", "-q", "--bare", origin_path]);
    lab.git(&lab.root, &["clone", "-q", origin_path, "main"]);
    lab.commit(&main, "start");
    lab.git(&main, &["push", "-q", "origin", "HEAD:main"]);
    lab.git(&main, &["switch", "-q", "-c", "develop"]);
    lab.git(&main, &["push", "-q", "-u", "origin", "develop"]);
    lab.git(&main, &["switch", "-q", "-c", "feature/f99-x"]);
    lab.git(&main, &["push", "-q", "-u", "origin", "feature/f99-x"]);
    // A story merged into develop, its branch then deleted on GitHub, its worktree left.
    lab.git(
        &main,
        &["switch", "-q", "-c", "feat/us-901-merged", "develop"],
    );
    lab.commit(&main, "merged");
    lab.git(&main, &["push", "-q", "-u", "origin", "feat/us-901-merged"]);
    lab.git(&main, &["switch", "-q", "develop"]);
    lab.git(&main, &["merge", "-q", "--ff-only", "feat/us-901-merged"]);
    lab.git(&main, &["push", "-q"]);
    lab.git(
        &main,
        &["push", "-q", "origin", "--delete", "feat/us-901-merged"],
    );
    let merged = lab.root.join("us-901");
    lab.git(
        &main,
        &[
            "worktree",
            "add",
            "-q",
            merged.to_str().expect("path"),
            "feat/us-901-merged",
        ],
    );
    // A story just made from its feature, without a commit.
    let fresh = lab.root.join("en-903");
    lab.git(
        &main,
        &[
            "worktree",
            "add",
            "-q",
            "-b",
            "chore/en-903-fresh",
            fresh.to_str().expect("path"),
            "origin/feature/f99-x",
        ],
    );
    lab.git(
        &main,
        &[
            "branch",
            "-q",
            "--set-upstream-to=origin/feature/f99-x",
            "chore/en-903-fresh",
        ],
    );

    let output = lab
        .command(env!("CARGO_BIN_EXE_branches-clean"), &main)
        .output()
        .expect("branches-clean");

    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(
        text(&output),
        "deleted: feat/us-901-merged, feature/f99-x\nworktrees kept: none\nlocal: chore/en-903-fresh, develop, main\non GitHub: origin/develop, origin/feature/f99-x, origin/main\nstashes: 0\nworktrees: 2\n"
    );
    assert!(!merged.exists());
    assert!(fresh.exists());
}

#[test]
fn en114_records_save_copies_into_the_projects_folder_of_the_chosen_root() {
    let lab = Lab::new("records", &[]);
    let repo = lab.root.join("repo");
    let root = lab.root.join("records");
    let project: String = repo
        .to_string_lossy()
        .chars()
        .map(|char| {
            if char.is_ascii_alphanumeric() {
                char
            } else {
                '-'
            }
        })
        .collect();
    let transcripts = lab.root.join("home/.claude/projects").join(project);
    let library = lab.root.join("library.sqlite");

    fs::create_dir_all(transcripts.join("sub")).expect("transcripts");
    fs::write(transcripts.join("sub/s.jsonl"), "{}\n").expect("transcript");
    fs::create_dir_all(&root).expect("root");
    lab.git(&lab.root, &["init", "-q", "repo"]);
    lab.git(
        &repo,
        &[
            "remote",
            "add",
            "origin",
            "https://example.com/owner/studio.git",
        ],
    );
    fs::create_dir_all(repo.join(".git/metrics")).expect("metrics");
    fs::write(repo.join(".git/metrics/turns.jsonl"), "{}\n").expect("turns");
    let made = Command::new("sqlite3")
        .arg(&library)
        .arg("create table t(x); insert into t values (1), (2);")
        .status()
        .expect("sqlite3");
    assert!(made.success());

    let save = |root: &Path| {
        lab.command(env!("CARGO_BIN_EXE_records-save"), &repo)
            .env("AGENT_RECORDS_ROOT", root)
            .env("AGENT_LIBRARY_FILE", &library)
            .output()
            .expect("records-save")
    };
    let saved = save(&root);
    let folder = root.join("studio");
    let rows = Command::new("sqlite3")
        .arg(folder.join("library/library.sqlite"))
        .arg("select count(*) from t")
        .output()
        .expect("sqlite3");

    assert_eq!(
        text(&saved),
        format!(
            "missing metrics/tool-calls.jsonl\nsaved metrics/turns.jsonl\nsaved transcripts\nsaved library/library.sqlite\n{NOT_SET}\nSaved in {}.\n",
            folder.display()
        )
    );
    assert_eq!(String::from_utf8_lossy(&rows.stdout), "2\n");
    assert!(folder.join("transcripts/sub/s.jsonl").exists());
    assert!(
        fs::read_to_string(folder.join("last-save.txt"))
            .expect("last save")
            .starts_with("20")
    );
    // Nothing Git: the records are never committed nor sent.
    assert!(!folder.join(".git").exists());

    // A root that does not exist: it says so, fails and writes nothing.
    let nowhere = lab.root.join("nowhere");
    let missing = save(&nowhere);

    assert_eq!(missing.status.code(), Some(1));
    assert!(String::from_utf8_lossy(&missing.stderr).contains("nowhere does not exist"));
    assert!(!nowhere.exists());

    // No root chosen: it says so and fails.
    let unset = lab
        .command(env!("CARGO_BIN_EXE_records-save"), &repo)
        .output()
        .expect("records-save");

    assert_eq!(unset.status.code(), Some(1));
    assert!(String::from_utf8_lossy(&unset.stderr).contains("AGENT_RECORDS_ROOT is not set"));
}

#[test]
fn f48_ac3_records_save_takes_the_snapshot_too() {
    let lab = Lab::new("records-snapshot", &[]);
    let repo = lab.root.join("repo");
    let root = lab.root.join("records");
    let synced = lab.root.join("synced");

    fs::create_dir_all(&synced).expect("synced");
    lab.git(&lab.root, &["init", "-q", "repo"]);
    lab.git(
        &repo,
        &[
            "remote",
            "add",
            "origin",
            "https://example.com/owner/studio.git",
        ],
    );
    open_store(&root.join("studio/records.sqlite")).expect("records");

    let saved = lab
        .command(env!("CARGO_BIN_EXE_records-save"), &repo)
        .env("AGENT_RECORDS_ROOT", &root)
        .env("AGENT_KNOWLEDGE_SNAPSHOT", &synced)
        .output()
        .expect("records-save");
    let day = snapshot_name(&calendar::today());
    let copied = synced.join(&day).is_file();

    assert!(
        saved.status.success(),
        "{}",
        String::from_utf8_lossy(&saved.stderr)
    );
    assert_eq!(
        text(&saved),
        format!(
            "missing metrics/tool-calls.jsonl\nmissing metrics/turns.jsonl\nmissing transcripts\nmissing library/library.sqlite\n{day} written into the folder AGENT_KNOWLEDGE_SNAPSHOT names: 1 kept, 0 removed\nSaved in {}.\n",
            root.join("studio").display()
        )
    );
    assert!(copied);
}

#[test]
fn f21_ac2_backlog_status_names_an_item_it_does_not_find() {
    let lab = Lab::new("status", &[]);
    let output = lab
        .command(env!("CARGO_BIN_EXE_backlog-status"), &lab.root)
        .args(["EN-1", "done"])
        .output()
        .expect("backlog-status");

    assert_eq!(output.status.code(), Some(1));
    assert_eq!(
        String::from_utf8_lossy(&output.stderr),
        "usage: target/release/backlog-status <ID> <status>; no item EN-1\n"
    );
}

#[test]
fn rev009_epic_check_prints_its_usage_on_help_and_checks_nothing() {
    let lab = Lab::new("epic-help", &[]);
    let output = lab
        .command(env!("CARGO_BIN_EXE_epic-check"), &lab.root)
        .arg("--help")
        .output()
        .expect("epic-check");

    assert_eq!(output.status.code(), Some(0));
    assert!(
        String::from_utf8_lossy(&output.stdout)
            .starts_with("usage: target/release/epic-check <epic> <first day>")
    );
    assert!(output.stderr.is_empty(), "{output:?}");
}
