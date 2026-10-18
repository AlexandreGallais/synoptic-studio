//! Tests of the shared files and of the `refuse-shared-files` pre-push check (EN-050, F18.AC6).

use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::process::{Output, Stdio};

use test_support::command;

use shared_files::shared::{
    Commit, Kind, closes, refusal, refused_files, shared_kind, story_of, worker_story,
};

/// A commit changing files.
fn commit(message: &str, files: &[&str]) -> Commit {
    Commit {
        hash: "abcd1234".into(),
        message: message.into(),
        files: files.iter().map(|&path| path.to_owned()).collect(),
        padded: Vec::new(),
        moved: Vec::new(),
    }
}

#[test]
fn names_every_kind_of_shared_file() {
    assert_eq!(
        shared_kind("docs/backlog/stories/README.md"),
        Some(Kind::Index)
    );
    assert_eq!(
        shared_kind("docs/backlog/features/README.md"),
        Some(Kind::Index)
    );
    assert_eq!(
        shared_kind("docs/backlog/epics/README.md"),
        Some(Kind::Index)
    );
    assert_eq!(shared_kind("docs/process/journal.md"), Some(Kind::Log));
    assert_eq!(shared_kind("docs/process/improvements.md"), Some(Kind::Log));
    assert_eq!(shared_kind("docs/process/feedback.md"), Some(Kind::Log));
    // [F23.AC1] the tool cards are a log too (ADR-0085).
    assert_eq!(
        shared_kind("docs/process/tool-incidents.md"),
        Some(Kind::Log)
    );
    assert_eq!(
        shared_kind("docs/backlog/features/E17-F18-a-safe-and-cheap-parallel-run.md"),
        Some(Kind::Feature)
    );
    assert_eq!(
        shared_kind("playground/checklists/epic-15-checklist.ts"),
        Some(Kind::Checklist)
    );
}

#[test]
fn leaves_the_other_files_to_the_worker() {
    for path in [
        "docs/backlog/stories/E17-F18-EN-050-shared-files-written-at-merge.md",
        "docs/backlog/epics/E17-fast-tooling-with-oxc-and-rust.md",
        "docs/process/agile.md",
        "docs/process/retrospectives/E15-retrospective.md",
        "playground/checklists/checklist-line.ts",
        "playground/checklists/index.ts",
        "docs/backlog/features/notes.txt",
        "src/index.ts",
    ] {
        assert_eq!(shared_kind(path), None, "{path}");
    }
}

#[test]
fn reads_the_story_of_a_story_branch() {
    assert_eq!(
        story_of("chore/en-050-shared-files-written-at-merge").as_deref(),
        Some("EN-050")
    );
    assert_eq!(
        story_of("refs/heads/feat/us-061-font-list").as_deref(),
        Some("US-061")
    );
    assert_eq!(story_of("docs/val-018-remarks").as_deref(), Some("VAL-018"));
    assert_eq!(story_of("feat/us-7").as_deref(), Some("US-7"));
    for branch in [
        "develop",
        "main",
        "feature/f18-safe-and-cheap-parallel-run",
        "feat/us-",
        "feat/us-12x-topic",
        "feat/-12-topic",
        "feat/US-12-topic",
    ] {
        assert_eq!(story_of(branch), None, "{branch}");
    }
}

#[test]
fn checks_only_the_branches_of_a_worker() {
    assert_eq!(
        worker_story("feat/us-061-font-list").as_deref(),
        Some("US-061")
    );
    assert_eq!(
        worker_story("chore/en-050-topic").as_deref(),
        Some("EN-050")
    );
    for branch in [
        "docs/val-018-remarks",
        "docs/ret-003-retrospective",
        "chore/sp-020-research",
        "feature/f18-safe-and-cheap-parallel-run",
        "develop",
    ] {
        assert_eq!(worker_story(branch), None, "{branch}");
    }
}

#[test]
fn knows_the_done_commit_by_its_closes_footer() {
    assert!(closes(
        &commit("docs(backlog): set EN-050 done\n\nCloses: EN-050\n", &[]),
        "EN-050"
    ));
    assert!(closes(
        &commit("docs(backlog): set both done\n\nCloses: F18, EN-050\n", &[]),
        "EN-050"
    ));
    assert!(!closes(
        &commit("feat(tooling): a task\n\nRefs: EN-050.T1\n", &[]),
        "EN-050"
    ));
    assert!(!closes(
        &commit("docs(backlog): set EN-051 done\n\nCloses: EN-051\n", &[]),
        "EN-050"
    ));
}

#[test]
fn refuses_every_shared_file_in_a_task_commit() {
    let task = commit(
        "feat(tooling): a task\n\nRefs: EN-050.T1\n",
        &[
            "tools/shared-files/src/lib.rs",
            "docs/backlog/stories/README.md",
            "docs/process/journal.md",
            "docs/process/tool-incidents.md",
            "docs/backlog/features/E17-F18-a-safe-and-cheap-parallel-run.md",
            "playground/checklists/epic-15-text-lines.ts",
        ],
    );

    assert_eq!(
        refused_files(&task, "EN-050"),
        [
            "docs/backlog/stories/README.md",
            "docs/process/journal.md",
            "docs/process/tool-incidents.md",
            "docs/backlog/features/E17-F18-a-safe-and-cheap-parallel-run.md",
            "playground/checklists/epic-15-text-lines.ts",
        ]
    );
}

#[test]
fn f23_ac1_lets_the_done_commit_write_the_logs_but_not_a_checklist() {
    let mut done = commit(
        "docs(backlog): set EN-050 done\n\nCloses: EN-050\n",
        &[
            "docs/backlog/stories/E17-F18-EN-050-shared-files-written-at-merge.md",
            "docs/backlog/stories/README.md",
            "docs/backlog/features/E17-F18-a-safe-and-cheap-parallel-run.md",
            "playground/checklists/epic-15-text-lines.ts",
            "docs/process/improvements.md",
            "docs/process/journal.md",
            "docs/process/tool-incidents.md",
        ],
    );

    // The done commit sets the statuses and moves the story's rows and cards into the logs; a
    // checklist stays the main thread's, written at the feature's validation.
    done.moved = [
        "docs/process/improvements.md",
        "docs/process/journal.md",
        "docs/process/tool-incidents.md",
    ]
    .map(str::to_owned)
    .to_vec();
    assert_eq!(
        refused_files(&done, "EN-050"),
        ["playground/checklists/epic-15-text-lines.ts"]
    );

    // [F23.AC1] a log the done commit changes otherwise than by the story's rows is refused
    // (AUD-025): `improvements.md` is no longer among the moved ones.
    done.moved.remove(0);
    assert_eq!(
        refused_files(&done, "EN-050"),
        [
            "playground/checklists/epic-15-text-lines.ts",
            "docs/process/improvements.md"
        ]
    );
}

#[test]
fn names_each_refused_file_with_its_commit() {
    let task = commit("a task", &["src/index.ts", "docs/process/feedback.md"]);
    let text = refusal("chore/en-050-topic", "EN-050", &[task]);

    assert!(
        text.contains("chore/en-050-topic is EN-050's branch"),
        "{text}"
    );
    assert!(
        text.contains("  docs/process/feedback.md (commit abcd1234)"),
        "{text}"
    );
    assert!(!text.contains("src/index.ts"), "{text}");
    assert!(text.contains("## For the main thread"), "{text}");
    assert_eq!(
        refusal(
            "chore/en-050-topic",
            "EN-050",
            &[commit("a task", &["src/index.ts"])]
        ),
        ""
    );
}

/// A Git repository in a fresh temporary folder, with a bare remote holding its first commit.
struct Repository {
    root: PathBuf,
}

impl Repository {
    fn new(name: &str) -> Self {
        let root = std::env::temp_dir().join(format!("shared-files-{name}-{}", std::process::id()));
        let _ = fs::remove_dir_all(&root);
        fs::create_dir_all(root.join("work/docs/process")).unwrap();
        let repository = Self { root };

        repository.git(&["init", "--quiet", "--bare", "../remote.git"]);
        repository.git(&["init", "--quiet", "--initial-branch=develop"]);
        repository.git(&["remote", "add", "origin", "../remote.git"]);
        repository.write("docs/process/journal.md", "# Journal\n");
        repository.write("README.md", "# Work\n");
        repository.commit("chore: start");
        repository.git(&["push", "--quiet", "origin", "develop"]);
        repository
    }

    fn work(&self) -> PathBuf {
        self.root.join("work")
    }

    fn git(&self, arguments: &[&str]) -> String {
        let output = command("git")
            .args(["-c", "commit.gpgsign=false"])
            .args(arguments)
            .current_dir(self.work())
            .env("GIT_AUTHOR_NAME", "test")
            .env("GIT_AUTHOR_EMAIL", "test@example.com")
            .env("GIT_COMMITTER_NAME", "test")
            .env("GIT_COMMITTER_EMAIL", "test@example.com")
            .output()
            .unwrap();

        assert!(output.status.success(), "{output:?}");
        String::from_utf8_lossy(&output.stdout).trim().to_owned()
    }

    fn write(&self, path: &str, text: &str) {
        fs::write(self.work().join(path), text).unwrap();
    }

    fn commit(&self, message: &str) -> String {
        self.git(&["add", "--all"]);
        self.git(&["commit", "--quiet", "-m", message]);
        self.git(&["rev-parse", "HEAD"])
    }

    /// The check run as `pre-push` runs it, for a new branch.
    fn check(&self, branch: &str, hash: &str) -> Output {
        let zero = "0".repeat(40);

        self.check_input(&format!(
            "refs/heads/{branch} {hash} refs/heads/{branch} {zero}"
        ))
    }

    /// The check run as `pre-push` runs it, given one line of its standard input.
    fn check_input(&self, line: &str) -> Output {
        let mut child = command(env!("CARGO_BIN_EXE_refuse-shared-files"))
            .current_dir(self.work())
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .unwrap();

        writeln!(child.stdin.take().unwrap(), "{line}").unwrap();
        child.wait_with_output().unwrap()
    }
}

impl Drop for Repository {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}

#[test]
fn refuses_at_push_a_worker_branch_that_edits_a_log() {
    let repository = Repository::new("refused");

    repository.git(&["switch", "--quiet", "-c", "feat/us-999-topic"]);
    repository.write("README.md", "# Work, changed\n");
    repository.commit("feat: a task");
    repository.write("docs/process/journal.md", "# Journal\n\n## A row\n");

    let hash = repository.commit("docs: a row in the journal");
    let output = repository.check("feat/us-999-topic", &hash);
    let error = String::from_utf8_lossy(&output.stderr);

    // [F18.AC6] the push is refused, and the message names the shared file.
    assert_eq!(output.status.code(), Some(1), "{error}");
    assert!(
        error.contains("feat/us-999-topic is US-999's branch"),
        "{error}"
    );
    assert!(
        error.contains("  docs/process/journal.md (commit "),
        "{error}"
    );
    assert!(!error.contains("README.md"), "{error}");
}

#[test]
fn lets_through_a_clean_branch_and_the_main_thread_branches() {
    let repository = Repository::new("clean");

    repository.git(&["switch", "--quiet", "-c", "feat/us-998-topic"]);
    repository.write("README.md", "# Work, changed\n");

    let clean = repository.commit("feat: a task");

    assert_eq!(
        repository.check("feat/us-998-topic", &clean).status.code(),
        Some(0)
    );

    repository.write("docs/process/journal.md", "# Journal\n\n## A row\n");

    let logged = repository.commit("docs: a row in the journal");

    assert_eq!(
        repository
            .check("docs/val-998-topic", &logged)
            .status
            .code(),
        Some(0)
    );
    assert_eq!(
        repository.check("feature/f99-topic", &logged).status.code(),
        Some(0)
    );
}

#[test]
fn reads_the_branch_from_the_remote_ref_and_lets_a_deletion_through() {
    let repository = Repository::new("refs");

    repository.git(&["switch", "--quiet", "-c", "feat/us-999-topic"]);
    repository.write("docs/process/journal.md", "# Journal\n\n## A row\n");

    let hash = repository.commit("docs: a row in the journal");
    let zero = "0".repeat(40);

    // `git push origin HEAD:feat/us-999-topic`: the local ref is `HEAD`, the branch is the remote's.
    let pushed =
        repository.check_input(&format!("HEAD {hash} refs/heads/feat/us-999-topic {zero}"));

    assert_eq!(pushed.status.code(), Some(1), "{pushed:?}");
    assert!(
        String::from_utf8_lossy(&pushed.stderr).contains("feat/us-999-topic is US-999's branch"),
        "{pushed:?}"
    );

    // `git push --delete origin feat/us-1-x`: nothing pushed, nothing to check.
    let deleted = repository.check_input(&format!("(delete) {zero} refs/heads/feat/us-1-x {hash}"));

    assert_eq!(deleted.status.code(), Some(0), "{deleted:?}");
    assert_eq!(String::from_utf8_lossy(&deleted.stderr), "");
}

#[test]
fn lets_through_a_log_unpadded_but_refuses_a_changed_or_added_row() {
    let repository = Repository::new("padding");
    let padded =
        "# Journal\n\n| Date       | Item  |\n| ---------- | ----- |\n| 2026-10-17 | a row |\n";

    repository.write("docs/process/journal.md", padded);
    repository.commit("docs: a padded table");
    repository.git(&["push", "--quiet", "origin", "develop"]);
    repository.git(&["switch", "--quiet", "-c", "chore/en-997-topic"]);
    repository.write(
        "docs/process/journal.md",
        "# Journal\n\n| Date | Item |\n| --- | --- |\n| 2026-10-17 | a row |\n",
    );

    let unpadded = repository.commit("style: tables without padding");
    let output = repository.check("chore/en-997-topic", &unpadded);

    // [F19.AC12] the formatter's padding change passes the shared files' check.
    assert_eq!(
        output.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );

    for (name, text) in [
        (
            "a changed cell",
            "# Journal\n\n| Date | Item |\n| --- | --- |\n| 2026-10-17 | a new row |\n",
        ),
        (
            "an added row",
            "# Journal\n\n| Date | Item |\n| --- | --- |\n| 2026-10-17 | a row |\n| 2026-10-18 | b |\n",
        ),
    ] {
        repository.git(&["reset", "--quiet", "--hard", "develop"]);
        repository.write("docs/process/journal.md", text);

        let hash = repository.commit("docs: a log changed");
        let output = repository.check("chore/en-997-topic", &hash);
        let error = String::from_utf8_lossy(&output.stderr);

        assert_eq!(output.status.code(), Some(1), "{name}: {error}");
        assert!(error.contains("docs/process/journal.md"), "{name}: {error}");
    }
}

#[test]
fn f23_ac1_refuses_a_log_in_a_task_commit_and_lets_through_the_done_commit_holding_rows() {
    let repository = Repository::new("done");

    repository.git(&["switch", "--quiet", "-c", "chore/en-996-topic"]);
    repository.write("README.md", "# Work, changed\n");
    repository.commit("feat: a task\n\nRefs: EN-996.T1");
    repository.write("docs/process/journal.md", "# Journal\n\n- `#run` a row\n");

    let task = repository.commit("feat: a row in a task\n\nRefs: EN-996.T2");
    let output = repository.check("chore/en-996-topic", &task);
    let error = String::from_utf8_lossy(&output.stderr);

    // [F23.AC1] a commit other than the done commit is refused, naming the log, and the message
    // says where the rows go.
    assert_eq!(output.status.code(), Some(1), "{error}");
    assert!(
        error.contains("  docs/process/journal.md (commit "),
        "{error}"
    );
    assert!(error.contains("done commit"), "{error}");

    // The same row, written by the worker in its story file, moved by the main thread in the
    // story's done commit, passes.
    let story = "docs/backlog/stories/E17-F23-EN-996-topic.md";
    let header = "---\nid: EN-996\nstatus: ready\n---\n\n# EN-996\n";

    repository.git(&["reset", "--quiet", "--hard", "HEAD~1"]);
    fs::create_dir_all(repository.work().join("docs/backlog/stories")).unwrap();
    repository.write(
        story,
        &format!("{header}\n## For the main thread\n\n### Journal\n\n- `#run` a row\n"),
    );
    repository.commit("feat: a row for the journal\n\nRefs: EN-996.T2");

    let task = repository.git(&["rev-parse", "HEAD"]);
    let done = |journal: &str| {
        repository.git(&["reset", "--quiet", "--hard", &task]);
        repository.write(story, &header.replace("ready", "done"));
        repository.write("docs/process/journal.md", journal);
        repository.commit("docs(backlog): set EN-996 done\n\nCloses: EN-996")
    };
    let moved = done("# Journal\n\n- `#run` a row\n");
    let output = repository.check("chore/en-996-topic", &moved);

    assert_eq!(
        output.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );

    // [F23.AC1] the done commit adding a row its story file did not hold, or deleting or
    // rewriting a line of a log, is refused, naming the log (AUD-025).
    for (name, journal) in [
        (
            "another row",
            "# Journal\n\n- `#run` a row\n- `#run` not ours\n",
        ),
        ("a line deleted", "- `#run` a row\n"),
        ("a line rewritten", "# The journal\n\n- `#run` a row\n"),
    ] {
        let hash = done(journal);
        let output = repository.check("chore/en-996-topic", &hash);
        let error = String::from_utf8_lossy(&output.stderr);

        assert_eq!(output.status.code(), Some(1), "{name}: {error}");
        assert!(
            error.contains("  docs/process/journal.md (commit "),
            "{name}: {error}"
        );
    }
}
