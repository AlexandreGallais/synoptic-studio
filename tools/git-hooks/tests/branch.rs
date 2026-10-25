//! Tests of the branch name (EN-108, F21.AC4): the one expression gives validate-branch-name's
//! verdict on every branch name of the repository's history, and refuses what it refused.

use std::fs;
use std::path::Path;

use git_hooks::branch::{branch_refusal, is_valid_branch};

#[test]
fn f21_ac4_passes_every_branch_name_of_the_history() {
    // The names of the pull requests (head and base), the merges and the branches, on 2026-10-18,
    // each with validate-branch-name 1.3.2's verdict on `.validate-branch-namerc.json`: all pass but
    // release-please's branch, refused since EN-112 removed release-please (ADR-0101), and the six
    // branches without the id of a story the auto-merge takes, refused since EN-118 (I50).
    let fixture = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/branch-names.txt");
    let text = fs::read_to_string(fixture).unwrap();
    let names: Vec<(&str, &str)> = text
        .lines()
        .filter_map(|line| line.split_once(' '))
        .collect();

    assert_eq!(names.len(), 295);
    for (verdict, name) in names {
        assert_eq!(is_valid_branch(name), verdict == "pass", "{name}");
    }
}

#[test]
fn f21_ac4_refuses_what_validate_branch_name_refused() {
    // Each verdict is the JavaScript pattern's, tried on 2026-10-18; those of `release-please--…`
    // are the pattern's since EN-112, those of `release/…` since REV-009 (ADR-0101).
    for name in [
        "worktree-agent-a13cbef",
        "feat/US-061-topic",
        "feature/f1-topic",
        "feature/f123-topic",
        "chore/x_y",
        "chore/",
        "Main",
        "feat/a--b",
        "feat/a-",
        "release/1.0",
        "dependabot/",
        "release-please--",
        "release-please--branches--develop",
        "release/0.3",
        "release/v0.3.0",
        "release/0.03.0",
        "release/0.3.0-rc",
        "release/0.3.0",
        "release/10.0.12",
        "develop2",
        "origin/develop",
        "main\nx",
    ] {
        assert!(!is_valid_branch(name), "{name:?}");
    }
    for name in [
        "feat/us-061-topic",
        "docs/val-018-remarks",
        "feature/f21-x",
        "dependabot/npm_and_yarn/vite-8.3.4",
    ] {
        assert!(is_valid_branch(name), "{name}");
    }
}

#[test]
fn f23_ac2_refuses_a_fix_without_a_story_id_as_the_auto_merge_does() {
    // [F23.AC2] One form for a work branch, `<type>/<id>-<topic>`, the id of a story the
    // auto-merge takes (`US`, `EN`, `SP`, `CHK`, `AUD`, `VAL`, `REV`): a fix without a story id
    // takes the id of the story it serves (I50), refused at the push rather than at the merge.
    for name in [
        "fix/shared-files-table-padding",
        "chore/tidy-up",
        "docs/f21-refinement",
        "docs/val-remarks-e15",
        "fix/en-12-short",
        "fix/en-0730-long",
        "fix/en-073",
        "fix/xy-073-unknown-kind",
        "refactor/nam-004-naming-pass",
        "docs/ret-006-retrospective",
    ] {
        assert!(!is_valid_branch(name), "{name}");
    }
    for name in [
        "fix/en-073-shared-files-table-padding",
        "feat/us-061-topic",
        "docs/sp-030-refinement-of-e18",
        "docs/chk-012-checkpoint",
        "docs/aud-025-audit-f23",
        "feat/val-024-validate-f23",
        "docs/rev-006-review-2026-10-18",
    ] {
        assert!(is_valid_branch(name), "{name}");
    }
}

#[test]
fn names_the_branch_and_the_rule_it_breaks() {
    let refusal = branch_refusal("worktree-agent-a13").unwrap();

    assert!(refusal.starts_with("✖   branch name \"worktree-agent-a13\" refused: it must be"));
    assert!(refusal.contains("docs/tooling/git-workflow.md"));
    assert!(refusal.contains("`<type>/<story-id>-<kebab-case-topic>`"));
    assert_eq!(branch_refusal("develop"), None);
}

/// Runs `git-hooks branch` with these arguments in a folder.
fn branch(folder: &Path, arguments: &[&str]) -> std::process::Output {
    // Without the inherited `GIT_*` variables: the pre-push hook sets `GIT_DIR`.
    test_support::command(env!("CARGO_BIN_EXE_git-hooks"))
        .arg("branch")
        .args(arguments)
        .current_dir(folder)
        .output()
        .unwrap()
}

#[test]
fn f21_ac4_checks_the_current_branch_or_the_one_named() {
    let root = std::env::temp_dir().join(format!("git-hooks-branch-{}", std::process::id()));
    let _ = fs::remove_dir_all(&root);
    fs::create_dir_all(&root).unwrap();
    // A home of its own and no system file: the user's Git configuration (a global
    // `core.hooksPath`, a template folder) never reaches the test's repository (AUD-023).
    let git = |arguments: &[&str]| {
        let output = test_support::command("git")
            .env("HOME", &root)
            .env("GIT_CONFIG_NOSYSTEM", "1")
            .args([
                "-c",
                "commit.gpgsign=false",
                "-c",
                "user.name=test",
                "-c",
                "user.email=t@e",
            ])
            .args(arguments)
            .current_dir(&root)
            .output()
            .unwrap();

        assert!(output.status.success(), "{output:?}");
    };

    git(&["init", "--quiet", "--initial-branch=develop"]);
    git(&["commit", "--quiet", "--allow-empty", "-m", "chore: start"]);
    assert!(branch(&root, &[]).status.success());
    assert!(
        branch(&root, &["--name", "feat/us-061-topic"])
            .status
            .success()
    );

    let refused = branch(&root, &["--name", "worktree-agent-a13"]);
    assert_eq!(refused.status.code(), Some(1));
    assert!(String::from_utf8_lossy(&refused.stderr).contains("\"worktree-agent-a13\" refused"));

    git(&["switch", "--quiet", "-c", "Topic"]);
    assert_eq!(branch(&root, &[]).status.code(), Some(1));

    // A detached HEAD has no branch to check: refused, as validate-branch-name did.
    git(&["switch", "--quiet", "--detach"]);
    let detached = branch(&root, &[]);
    assert_eq!(detached.status.code(), Some(1));
    assert!(String::from_utf8_lossy(&detached.stderr).contains("no branch"));
    fs::remove_dir_all(&root).unwrap();
}
