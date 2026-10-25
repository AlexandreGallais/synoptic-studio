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

    // The epic's backlog in the records of the test's folder, the project named after `repo`.
    open_store(&lab.root.join("records/repo.sqlite"))
        .expect("records")
        .execute_batch(
            "INSERT INTO stories (id, epic, feature, kind, title, status) VALUES
              ('EN-001', 'E99', 'F99', 'EN', 'a', 'done'),
              ('EN-002', 'E99', 'F99', 'EN', 'b', 'ready'),
              ('EN-003', 'E99', 'F99', 'EN', 'c', 'dropped'),
              ('EN-004', 'E98', 'F98', 'EN', 'd', 'ready');",
        )
        .expect("stories");
    fs::create_dir_all(&repo).expect("repo");
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
        .env("AGENT_RECORDS_ROOT", lab.root.join("records"))
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
            "| check:all (code, site, playground, dependencies) | **to look at** | test (lane test → playground:test): a.test.ts > b |",
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
    // EN-003, dropped, is closed (I110); EN-004 is another epic's.
    assert_eq!(lines[10], "| backlog of E99 | **to look at** | EN-002 |");
    assert!(
        lines[11].starts_with("| private records | **to look at** | never saved in "),
        "{}",
        lines[11]
    );
    assert_eq!(
        lines[lines.len() - 1],
        "| golden file of refactors | ok | none written yet |"
    );
}

/// A story pushed then deleted on GitHub without a merge, whose drafts `branches-clean` drops
/// (AUD-050), and the records of the lab: a draft of each branch, the merged one's, the abandoned
/// one's and one of a branch not deleted; the records' file.
fn abandoned_story(lab: &Lab, main: &Path) -> PathBuf {
    lab.git(
        main,
        &["switch", "-q", "-c", "chore/en-904-abandoned", "develop"],
    );
    lab.commit(main, "abandoned");
    lab.git(
        main,
        &["push", "-q", "-u", "origin", "chore/en-904-abandoned"],
    );
    lab.git(main, &["switch", "-q", "develop"]);
    lab.git(
        main,
        &["push", "-q", "origin", "--delete", "chore/en-904-abandoned"],
    );

    let records = lab.root.join("records/origin.sqlite");

    open_store(&records)
        .expect("records")
        .execute_batch(
            "INSERT INTO stories (id, epic, feature, kind, title, status, branch, state) VALUES
              ('EN-901', 'E99', 'F99', 'EN', 'a', 'ready', 'feat/us-901-merged', 'draft'),
              ('EN-904', 'E99', 'F99', 'EN', 'b', 'ready', 'chore/en-904-abandoned', 'draft'),
              ('EN-905', 'E99', 'F99', 'EN', 'c', 'ready', 'chore/en-905-elsewhere', 'draft');",
        )
        .expect("stories");
    records
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
    let records = abandoned_story(&lab, &main);
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
        .env("AGENT_RECORDS_ROOT", lab.root.join("records"))
        .output()
        .expect("branches-clean");
    let states = knowledge::sql::read(
        &open_store(&records).expect("records"),
        "SELECT id, state FROM stories ORDER BY id",
    );

    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    // US-901's draft merged, EN-904's dropped, EN-905's, of a branch not deleted, kept.
    assert_eq!(
        text(&output),
        "deleted: chore/en-904-abandoned, feat/us-901-merged, feature/f99-x\nworktrees kept: none\nlocal: chore/en-903-fresh, develop, main\non GitHub: origin/develop, origin/feature/f99-x, origin/main\nstashes: 0\nworktrees: 2\nrecords: 1 row(s) marked merged, 1 dropped\n"
    );
    assert_eq!(
        states.as_deref(),
        Ok("id\tstate\nEN-901\tmerged\nEN-904\tdropped\nEN-905\tdraft\n")
    );
    assert!(!merged.exists());
    assert!(fresh.exists());
}

#[test]
fn en114_records_save_writes_what_serves_into_the_projects_file_of_the_chosen_root() {
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
    let claude = lab.root.join("home/.claude/projects").join(project);
    let library = lab.root.join("library.sqlite");

    fs::create_dir_all(claude.join("memory")).expect("transcripts");
    fs::write(
        claude.join("s1.jsonl"),
        "{\"type\":\"user\",\"sessionId\":\"s1\",\"uuid\":\"u1\",\"timestamp\":\"2026-10-22T14:00:00.000Z\",\"message\":{\"content\":\"Bonjour\"}}\n",
    )
    .expect("transcript");
    fs::write(
        claude.join("memory/feedback-x.md"),
        "---\nname: feedback-x\n---\nA memory.\n",
    )
    .expect("memory");
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
    fs::write(
        repo.join(".git/metrics/turns.jsonl"),
        "{\"session\":\"s1\",\"start\":1}\n",
    )
    .expect("turns");
    library::store::open_library(&library).expect("library");

    let save = |root: &Path| {
        lab.command(env!("CARGO_BIN_EXE_records-save"), &repo)
            .env("AGENT_RECORDS_ROOT", root)
            .env("AGENT_LIBRARY_FILE", &library)
            .output()
            .expect("records-save")
    };
    let saved = save(&root);
    let file = root.join("studio.sqlite");
    let read = |query: &str| knowledge::sql::read(&open_store(&file).expect("records"), query);

    assert_eq!(
        text(&saved),
        format!(
            "metrics: 1 rows\ntranscripts: 1 rows\nmemories: 1 rows\nlibrary: 0 rows\n{NOT_SET}\nSaved in {}.\n",
            file.display()
        )
    );
    assert_eq!(
        read("SELECT session, text FROM messages").as_deref(),
        Ok("session\ttext\ns1\tBonjour\n")
    );
    assert_eq!(
        read("SELECT count(*) AS n FROM saves").as_deref(),
        Ok("n\n1\n")
    );
    // No transcript copied, no folder of the project's: one file.
    assert_eq!(
        fs::read_dir(&root).expect("root").count(),
        fs::read_dir(&root)
            .expect("root")
            .filter(|entry| entry.as_ref().is_ok_and(|e| e.path().is_file()))
            .count()
    );

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

    let saved = lab
        .command(env!("CARGO_BIN_EXE_records-save"), &repo)
        .env("AGENT_RECORDS_ROOT", &root)
        .env("AGENT_KNOWLEDGE_SNAPSHOT", &synced)
        .output()
        .expect("records-save");
    let day = snapshot_name("studio", &calendar::today());
    let copied = synced.join(&day).is_file();

    assert!(
        saved.status.success(),
        "{}",
        String::from_utf8_lossy(&saved.stderr)
    );
    assert_eq!(
        text(&saved),
        format!(
            "metrics: missing\ntranscripts: missing\nmemories: missing\nlibrary: missing\n{day} written into the folder AGENT_KNOWLEDGE_SNAPSHOT names: 1 kept, 0 removed\nSaved in {}.\n",
            root.join("studio.sqlite").display()
        )
    );
    assert!(copied);
}

#[test]
fn f21_ac2_backlog_status_writes_the_records_and_names_an_item_it_does_not_find() {
    let lab = Lab::new("status", &[]);
    let repo = lab.root.join("repo");
    let records = lab.root.join("records/repo.sqlite");

    fs::create_dir_all(&repo).expect("repo");
    lab.git(&repo, &["init", "-q"]);
    open_store(&records)
        .expect("records")
        .execute_batch(
            "INSERT INTO stories (id, epic, feature, kind, title, status) VALUES ('EN-902', 'E99', 'F99', 'EN', 'b', 'ready');",
        )
        .expect("story");

    let status = |words: &[&str]| {
        lab.command(env!("CARGO_BIN_EXE_backlog-status"), &repo)
            .env("AGENT_RECORDS_ROOT", lab.root.join("records"))
            .args(words)
            .output()
            .expect("backlog-status")
    };
    let set = status(&["EN-902", "in-progress"]);
    let unknown = status(&["EN-901", "done"]);
    let wrong = status(&["EN-902", "finished"]);
    let read = knowledge::sql::read(
        &open_store(&records).expect("records"),
        "SELECT status FROM stories WHERE id = 'EN-902'",
    );

    assert!(set.status.success(), "{set:?}");
    assert_eq!(text(&set), "EN-902 in-progress in the records\n");
    assert_eq!(read.as_deref(), Ok("status\nin-progress\n"));
    assert_eq!(unknown.status.code(), Some(1));
    assert_eq!(
        String::from_utf8_lossy(&unknown.stderr),
        "backlog-status: usage: target/release/backlog-status <ID> <status>; no item EN-901 in the records\n"
    );
    assert_eq!(wrong.status.code(), Some(1));
    assert!(String::from_utf8_lossy(&wrong.stderr).contains("`finished` is not a status"));
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
