//! Tests of the shared files and of the `refuse-shared-files` pre-push check (EN-050, F18.AC6).

use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::process::{Command, Output, Stdio};

use shared_files::shared::{
    Commit, Kind, closes, refusal, refused_files, shared_kind, story_of, worker_story,
};

/// A commit changing files.
fn commit(message: &str, files: &[&str]) -> Commit {
    Commit {
        hash: "abcd1234".into(),
        message: message.into(),
        files: files.iter().map(|&path| path.to_owned()).collect(),
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
            "docs/backlog/features/E17-F18-a-safe-and-cheap-parallel-run.md",
            "playground/checklists/epic-15-text-lines.ts",
        ],
    );

    assert_eq!(
        refused_files(&task, "EN-050"),
        [
            "docs/backlog/stories/README.md",
            "docs/process/journal.md",
            "docs/backlog/features/E17-F18-a-safe-and-cheap-parallel-run.md",
            "playground/checklists/epic-15-text-lines.ts",
        ]
    );
}

#[test]
fn lets_the_done_commit_set_the_status_but_not_write_a_log() {
    let done = commit(
        "docs(backlog): set EN-050 done\n\nCloses: EN-050\n",
        &[
            "docs/backlog/stories/E17-F18-EN-050-shared-files-written-at-merge.md",
            "docs/backlog/stories/README.md",
            "docs/backlog/features/E17-F18-a-safe-and-cheap-parallel-run.md",
            "docs/process/improvements.md",
        ],
    );

    assert_eq!(
        refused_files(&done, "EN-050"),
        ["docs/process/improvements.md"]
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

/// A command cut off from the Git repository running the tests: a hook (`pre-push` running
/// `check:all`) sets `GIT_DIR` and its kin, which would point every `git` call at that repository.
fn isolated(mut command: Command) -> Command {
    for (name, _) in std::env::vars_os() {
        if name.to_string_lossy().starts_with("GIT_") {
            command.env_remove(name);
        }
    }
    command
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
        let output = isolated(Command::new("git"))
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
        let mut child = isolated(Command::new(env!("CARGO_BIN_EXE_refuse-shared-files")))
            .current_dir(self.work())
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .unwrap();
        let zero = "0".repeat(40);

        writeln!(
            child.stdin.take().unwrap(),
            "refs/heads/{branch} {hash} refs/heads/{branch} {zero}"
        )
        .unwrap();
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
