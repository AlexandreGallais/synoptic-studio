//! Tests of `git-hooks commit-msg` and `git-hooks commits` in temporary repositories (EN-108,
//! F21.AC4): a commit refused by its message or by its missing story footer (a hook prints on
//! Git's standard error), comments of the
//! commit's editor left out, and the messages of a range checked as the CI does.

#![cfg(unix)]

use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;
use std::process::Output;

// Every command starts without the inherited `GIT_*` variables: the pre-push hook running
// `check:all` sets `GIT_DIR`, which would point every `git` call at this repository.
use test_support::command;

/// A temporary repository whose `commit-msg` hook runs the binary, with a first commit on
/// `develop`.
struct Repository {
    root: PathBuf,
}

impl Repository {
    fn new(name: &str) -> Self {
        let root = std::env::temp_dir().canonicalize().unwrap().join(format!(
            "git-hooks-commit-msg-{name}-{}",
            std::process::id()
        ));
        let _ = fs::remove_dir_all(&root);
        fs::create_dir_all(root.join("hooks")).unwrap();
        let hook = root.join("hooks/commit-msg");

        fs::write(
            &hook,
            format!(
                "#!/bin/sh\nexec '{}' commit-msg \"$1\"\n",
                env!("CARGO_BIN_EXE_git-hooks")
            ),
        )
        .unwrap();
        fs::set_permissions(&hook, fs::Permissions::from_mode(0o755)).unwrap();
        let repository = Self { root };

        repository.git(&["init", "--quiet", "--initial-branch=develop"]);
        repository.git(&["config", "core.hooksPath", "hooks"]);
        assert!(repository.commit("chore: start").status.success());
        repository
    }

    fn git(&self, arguments: &[&str]) -> Output {
        command("git")
            .args(["-c", "commit.gpgsign=false"])
            .args(arguments)
            .current_dir(&self.root)
            .env("GIT_AUTHOR_NAME", "test")
            .env("GIT_AUTHOR_EMAIL", "test@example.com")
            .env("GIT_COMMITTER_NAME", "test")
            .env("GIT_COMMITTER_EMAIL", "test@example.com")
            .output()
            .unwrap()
    }

    /// An empty commit with this message, its hook run.
    fn commit(&self, message: &str) -> Output {
        let file = self.root.join("message.txt");

        fs::write(&file, message).unwrap();
        self.git(&[
            "commit",
            "--quiet",
            "--allow-empty",
            "--cleanup=verbatim",
            "-F",
            file.to_str().unwrap(),
        ])
    }

    fn remove(self) {
        fs::remove_dir_all(&self.root).unwrap();
    }
}

fn text(bytes: &[u8]) -> String {
    String::from_utf8_lossy(bytes).into_owned()
}

#[test]
fn f21_ac4_refuses_a_message_commitlint_refuses() {
    let repository = Repository::new("refused");

    let refused = repository.commit("Docs: Add a page.\n");
    assert!(!refused.status.success());
    let printed = text(&refused.stderr);
    assert!(
        printed.contains("⧗   input: Docs: Add a page.\n"),
        "{printed}"
    );
    assert!(printed.contains("[subject-case]"), "{printed}");
    assert!(
        printed.contains("✖   found 4 problems, 0 warnings"),
        "{printed}"
    );

    // A warning is printed and the commit passes.
    let warned = repository.commit("docs(docs): add a page\nwithout a blank line\n");
    assert!(warned.status.success(), "{warned:?}");
    assert!(text(&warned.stderr).contains("[body-leading-blank]"));
    repository.remove();
}

#[test]
fn f21_ac4_leaves_out_the_editor_s_comment_lines() {
    let repository = Repository::new("comments");

    // A comment line longer than 100 characters, kept by `--cleanup=verbatim`: not the message's.
    let long = "#".repeat(120);
    let passed = repository.commit(&format!("docs: add a page\n\n{long}\n"));
    assert!(passed.status.success(), "{passed:?}");

    // `core.commentChar` read from Git's configuration.
    repository.git(&["config", "core.commentChar", ";"]);
    let custom = repository.commit(&format!("docs: add a page\n\n;{long}\n"));
    assert!(custom.status.success(), "{custom:?}");
    repository.remove();
}

#[test]
fn f21_ac4_a_story_branch_asks_for_its_footer() {
    let repository = Repository::new("story");

    repository.git(&["switch", "--quiet", "-c", "chore/en-998-topic"]);
    let refused = repository.commit("feat(tooling): add a check\n\nRefs: EN-997.T1\n");
    assert!(!refused.status.success());
    assert!(text(&refused.stderr).contains("`Refs: EN-998.T<n>`, a task of its story"));

    assert!(
        repository
            .commit("feat(tooling): add a check\n\nRefs: EN-998.T1\n")
            .status
            .success()
    );
    assert!(
        !repository
            .commit("docs(backlog): set EN-998 done\n\nCloses: EN-998\n")
            .status
            .success()
    );

    // A feature branch asks for none.
    repository.git(&["switch", "--quiet", "-c", "feature/f98-topic"]);
    assert!(
        repository
            .commit("docs(backlog): plan the feature\n")
            .status
            .success()
    );
    repository.remove();
}

#[test]
fn f21_ac4_checks_each_commit_of_a_range() {
    let repository = Repository::new("range");
    let start = text(&repository.git(&["rev-parse", "HEAD"]).stdout)
        .trim()
        .to_owned();

    // Commits made without the hook, as a pull request may hold them.
    repository.git(&["config", "core.hooksPath", "no-hooks"]);
    assert!(repository.commit("feat: add a thing\n").status.success());
    assert!(
        repository
            .commit("Merge branch 'develop'\n")
            .status
            .success()
    );
    let good = text(&repository.git(&["rev-parse", "HEAD"]).stdout)
        .trim()
        .to_owned();
    assert!(
        repository
            .commit("chore(foo): add a rule\n")
            .status
            .success()
    );
    let bad = text(&repository.git(&["rev-parse", "HEAD"]).stdout)
        .trim()
        .to_owned();

    let run = |from: &str, to: &str| {
        command(env!("CARGO_BIN_EXE_git-hooks"))
            .args(["commits", "--from", from, "--to", to])
            .current_dir(&repository.root)
            .output()
            .unwrap()
    };

    let passed = run(&start, &good);
    assert!(passed.status.success(), "{passed:?}");
    assert_eq!(text(&passed.stdout), "");

    let refused = run(&start, &bad);
    assert_eq!(refused.status.code(), Some(1));
    assert!(
        text(&refused.stdout).starts_with(&format!("{bad}\n⧗   input: chore(foo): add a rule\n"))
    );

    // Without a common ancestor the range is refused, never passed.
    let orphan = repository.git(&["commit-tree", "-m", "orphan", &format!("{start}^{{tree}}")]);
    let orphan = text(&orphan.stdout).trim().to_owned();
    assert_eq!(run(&orphan, &bad).status.code(), Some(1));
    repository.remove();
}
