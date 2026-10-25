//! Tests of `check-rule-docs` run as the `commit-msg` hook of a real commit (EN-059, F18.AC16).

use std::fs;
use std::path::PathBuf;
use std::process::Output;

// Every command starts without the inherited `GIT_*` variables: the pre-push hook running
// `check:all` sets `GIT_DIR`, which would point every `git` call at this repository.
use test_support::command;

/// A Git repository in a fresh temporary folder whose `commit-msg` hook runs the check, on a
/// story's branch with a story file of F47, the last feature with a VAL, and one of F98, audited
/// alone (ADR-0100).
struct Repository {
    root: PathBuf,
}

impl Repository {
    fn new(name: &str) -> Self {
        let root = std::env::temp_dir().join(format!("rule-docs-{name}-{}", std::process::id()));
        let _ = fs::remove_dir_all(&root);
        for folder in ["hooks", "docs/domain", "docs/backlog/stories", "docs/guide"] {
            fs::create_dir_all(root.join(folder)).unwrap();
        }
        let hook = root.join("hooks/commit-msg");
        fs::write(
            &hook,
            format!(
                "#!/bin/sh\nexec '{}' \"$1\"\n",
                env!("CARGO_BIN_EXE_check-rule-docs")
            ),
        )
        .unwrap();
        // Unix-only: Git runs a hook only when it is executable there; Windows has no such bit.
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            fs::set_permissions(&hook, fs::Permissions::from_mode(0o755)).unwrap();
        }
        let repository = Self { root };

        repository.git(&["init", "--quiet", "--initial-branch=develop"]);
        repository.git(&["config", "core.hooksPath", "hooks"]);
        repository.write("docs/domain/README.md", "# Domain\n");
        repository.write("docs/domain/shapes.md", "# Shapes\n");
        repository.write("docs/guide/f47-topic.md", "# F47\n");
        repository.write(
            "docs/backlog/stories/E99-F47-VAL-999-validate-f47.md",
            "# VAL\n",
        );
        repository.write(
            "docs/backlog/stories/E99-F47-EN-997-topic.md",
            "status: ready\n",
        );
        repository.write(
            "docs/backlog/stories/E99-F98-AUD-998-audit-f98.md",
            "# AUD\n",
        );
        repository.write(
            "docs/backlog/stories/E99-F98-EN-995-topic.md",
            "status: ready\n",
        );
        assert!(repository.commit("chore: start").status.success());
        repository.git(&["switch", "--quiet", "-c", "chore/en-997-topic"]);
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

    fn write(&self, path: &str, text: &str) {
        fs::write(self.root.join(path), text).unwrap();
    }

    /// A commit of every change, its hook run.
    fn commit(&self, message: &str) -> Output {
        assert!(self.git(&["add", "--all"]).status.success());
        self.git(&["commit", "--quiet", "-m", message])
    }

    /// A task of the story changing a rule.
    fn change_rule(&self) {
        self.write("docs/domain/shapes.md", "# Shapes\n\nA new rule.\n");
        let output = self.commit("feat: change a rule\n\nRefs: EN-997.T1");

        assert!(output.status.success(), "{output:?}");
    }
}

impl Drop for Repository {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}

/// The done commit of EN-997.
const DONE: &str = "docs(backlog): set EN-997 done\n\nCloses: EN-997";

#[test]
fn refuses_a_done_commit_lacking_the_docs_of_a_rule() {
    let repository = Repository::new("refused");

    repository.change_rule();
    repository.write(
        "docs/backlog/stories/E99-F47-EN-997-topic.md",
        "status: done\n",
    );

    let output = repository.commit(DONE);
    let error = String::from_utf8_lossy(&output.stderr);

    // [F18.AC16] the done commit is refused and names the three docs.
    assert!(!output.status.success(), "{error}");
    assert!(
        error.contains("EN-997 changes a rule in docs/domain/"),
        "{error}"
    );
    assert!(
        error.contains("the Q row in docs/domain/README.md"),
        "{error}"
    );
    assert!(
        error.contains("the decisions to confirm of F47's VAL"),
        "{error}"
    );
    assert!(
        error.contains("the guide page docs/guide/f47-*.md"),
        "{error}"
    );
}

#[test]
fn takes_the_docs_from_the_done_commit_or_the_tasks() {
    let repository = Repository::new("docs");

    repository.change_rule();
    repository.write("docs/domain/README.md", "# Domain\n\n| Q99 | a rule |\n");
    assert!(
        repository
            .commit("docs(docs): write the Q row\n\nRefs: EN-997.T2")
            .status
            .success()
    );
    repository.write("docs/guide/f47-topic.md", "# F47\n\nA step.\n");
    repository.write(
        "docs/backlog/stories/E99-F47-VAL-999-validate-f47.md",
        "# VAL\n\n1. A rule\n",
    );
    repository.write(
        "docs/backlog/stories/E99-F47-EN-997-topic.md",
        "status: done\n",
    );

    let output = repository.commit(DONE);

    assert!(output.status.success(), "{output:?}");
}

#[test]
fn takes_the_line_saying_none_is_affected() {
    let repository = Repository::new("none");

    repository.change_rule();
    repository.write(
        "docs/backlog/stories/E99-F47-EN-997-topic.md",
        "status: done\n",
    );

    let output = repository.commit(&format!(
        "{DONE}\nRule-docs: none affected, the rule's wording only"
    ));

    assert!(output.status.success(), "{output:?}");
}

#[test]
fn passes_a_story_without_a_rule_and_the_other_commits() {
    let repository = Repository::new("clean");

    repository.write("docs/guide/f47-topic.md", "# F47\n\nChanged.\n");
    assert!(
        repository
            .commit("docs: a task\n\nRefs: EN-997.T1")
            .status
            .success()
    );
    repository.write(
        "docs/domain/shapes.md",
        "# Shapes\n\nAnother story's rule.\n",
    );
    assert!(
        repository
            .commit("docs: a rule of another story\n\nRefs: US-996.T1")
            .status
            .success()
    );
    repository.write(
        "docs/backlog/stories/E99-F47-EN-997-topic.md",
        "status: done\n",
    );

    let output = repository.commit(DONE);

    assert!(output.status.success(), "{output:?}");
}

#[test]
fn asks_an_audited_feature_for_its_audit_and_not_a_val() {
    let repository = Repository::new("audited");

    repository.write("docs/domain/shapes.md", "# Shapes\n\nA new rule.\n");
    assert!(
        repository
            .commit("feat: change a rule\n\nRefs: EN-995.T1")
            .status
            .success()
    );
    repository.write(
        "docs/backlog/stories/E99-F98-EN-995-topic.md",
        "status: done\n",
    );

    let done = "docs(backlog): set EN-995 done\n\nCloses: EN-995";
    let output = repository.commit(done);
    let error = String::from_utf8_lossy(&output.stderr);

    // From F48 (ADR-0100), the feature's audit is asked for, no VAL nor guide page.
    assert!(!output.status.success(), "{error}");
    assert!(
        error.contains("the rule to recheck in F98's audit"),
        "{error}"
    );
    assert!(!error.contains("VAL"), "{error}");
    assert!(!error.contains("docs/guide/"), "{error}");

    repository.write("docs/domain/README.md", "# Domain\n\n| Q99 | a rule |\n");
    repository.write(
        "docs/backlog/stories/E99-F98-AUD-998-audit-f98.md",
        "# AUD\n\n- Recheck the new rule of shapes.md.\n",
    );

    let output = repository.commit(done);

    assert!(output.status.success(), "{output:?}");
}
