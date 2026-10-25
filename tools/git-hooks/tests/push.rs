//! Tests of a push that only deletes branches (REV-008): `deletes_only` on the refs Git writes,
//! then `.githooks/pre-push` itself in a temporary folder, its tools stubbed but `git-hooks
//! push-deletes-only`, so that a deletion runs no `check:all` and any other push runs it, the
//! shared files' check reading the same refs either way.

#![cfg(unix)]

use std::fs;
use std::io::Write;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::process::Stdio;

use git_hooks::push::deletes_only;
use test_support::command;

/// The zero sha of SHA-1, 40 digits.
const ZERO: &str = "0000000000000000000000000000000000000000";

/// A commit's sha.
const SHA: &str = "67890abcdef0123456789abcdef0123456789abc";

#[test]
fn reads_a_push_deleting_branches_as_a_deletion_only() {
    // `git push origin --delete a b`: two lines, each `(delete)` and the zero sha as its local
    // side (`REF-GITHOOKS`, `pre-push`).
    let two = format!("(delete) {ZERO} refs/heads/a {SHA}\n(delete) {ZERO} refs/heads/b {SHA}\n");

    assert!(deletes_only(&two));
    // SHA-256's zero sha, 64 digits; a line without its final line feed; blank lines around.
    assert!(deletes_only(&format!(
        "\n(delete) {} refs/heads/a {SHA}",
        "0".repeat(64)
    )));
}

#[test]
fn reads_any_other_push_as_one_to_check() {
    let deletion = format!("(delete) {ZERO} refs/heads/a {SHA}\n");
    let update = format!("refs/heads/b {SHA} refs/heads/b {ZERO}\n");

    // A branch pushed, alone or beside a deletion.
    assert!(!deletes_only(&update));
    assert!(!deletes_only(&format!("{deletion}{update}")));
    assert!(!deletes_only(&format!("{update}{deletion}")));
    // No ref at all, or only blank lines: nothing proves a deletion.
    assert!(!deletes_only(""));
    assert!(!deletes_only("\n\n"));
    // A line Git would not write: three fields, or five.
    assert!(!deletes_only(&format!("(delete) {ZERO} refs/heads/a\n")));
    assert!(!deletes_only(&format!(
        "(delete) {ZERO} refs/heads/a {SHA} x\n"
    )));
    // A local sha with one digit other than zero.
    assert!(!deletes_only(&format!(
        "(delete) {}1 refs/heads/a {SHA}\n",
        &ZERO[1..]
    )));
}

/// A temporary copy of the hooks folder, its tools stubbed: `npm` writes what it got into
/// `calls`, `git-hooks branch` passes, `git-hooks push-deletes-only` is the binary under test.
struct Hook {
    root: PathBuf,
}

impl Hook {
    fn new(name: &str) -> Self {
        let root = std::env::temp_dir()
            .canonicalize()
            .unwrap()
            .join(format!("git-hooks-push-{name}-{}", std::process::id()));
        let hooks = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../.githooks");
        let _ = fs::remove_dir_all(&root);

        fs::create_dir_all(root.join(".githooks")).unwrap();
        fs::create_dir_all(root.join("target/release")).unwrap();
        fs::create_dir_all(root.join("bin")).unwrap();
        for file in ["pre-push", "tool.sh"] {
            fs::copy(hooks.join(file), root.join(".githooks").join(file)).unwrap();
        }
        Self::script(
            &root.join("target/release/git-hooks"),
            &format!(
                "[ \"$1\" = branch ] && exit 0\nexec '{}' \"$@\"\n",
                env!("CARGO_BIN_EXE_git-hooks")
            ),
        );
        Self::script(&root.join("bin/npm"), "echo \"npm $*\" >> calls\n");
        command("git")
            .args(["init", "--quiet"])
            .current_dir(&root)
            .output()
            .unwrap();
        Self { root }
    }

    fn script(path: &Path, body: &str) {
        fs::write(path, format!("#!/bin/sh\n{body}")).unwrap();
        fs::set_permissions(path, fs::Permissions::from_mode(0o755)).unwrap();
    }

    /// Runs the hook with the refs on its standard input; its success and the calls made.
    fn push(&self, refs: &str) -> (bool, String) {
        let path = format!(
            "{}:{}",
            self.root.join("bin").display(),
            std::env::var("PATH").unwrap_or_default()
        );
        let mut child = command("sh")
            .arg(".githooks/pre-push")
            .args(["origin", "https://example.com/r.git"])
            .env("PATH", path)
            .current_dir(&self.root)
            .stdin(Stdio::piped())
            .stdout(Stdio::null())
            .spawn()
            .unwrap();

        child
            .stdin
            .take()
            .unwrap()
            .write_all(refs.as_bytes())
            .unwrap();
        let ok = child.wait().unwrap().success();

        (
            ok,
            fs::read_to_string(self.root.join("calls")).unwrap_or_default(),
        )
    }
}

impl Drop for Hook {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}

#[test]
fn the_pre_push_hook_runs_no_check_for_a_push_that_only_deletes() {
    let hook = Hook::new("delete");
    let refs = format!("(delete) {ZERO} refs/heads/a {SHA}\n");
    let (ok, calls) = hook.push(&refs);

    assert!(ok);
    // `check:all` never ran.
    assert_eq!(calls, "");
}

#[test]
fn the_pre_push_hook_runs_check_all_for_a_push_of_a_branch() {
    let hook = Hook::new("update");
    let refs = format!("refs/heads/b {SHA} refs/heads/b {ZERO}\n");
    let (ok, calls) = hook.push(&refs);

    assert!(ok);
    assert_eq!(calls, "npm run check:all\n");
}
