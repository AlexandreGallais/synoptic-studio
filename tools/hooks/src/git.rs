//! Git and the hook's input, as the ported hooks read them (EN-104).

use std::io::{self, Read};
use std::path::Path;
use std::process::Command;

use crate::json::{self, Value};

/// Git's output in a folder, trimmed; `None` when it cannot run or fails, its error printed to
/// the hook's own error output.
#[must_use]
pub fn git(folder: &Path, arguments: &[&str]) -> Option<String> {
    let output = Command::new("git")
        .args(arguments)
        .current_dir(folder)
        .output()
        .ok()?;

    if output.status.success() {
        Some(String::from_utf8_lossy(&output.stdout).trim().to_owned())
    } else {
        eprint!("{}", String::from_utf8_lossy(&output.stderr));
        None
    }
}

/// The Git common folder of a folder, absolute: the main checkout's `.git`, a worktree's included.
#[must_use]
pub fn common_folder(folder: &Path) -> Option<String> {
    git(
        folder,
        &["rev-parse", "--path-format=absolute", "--git-common-dir"],
    )
}

/// The hook's input, the JSON on its standard input; `None` when it is not JSON, said on the
/// hook's error output: the hook then exits 1, an error that lets the call through, as the
/// TypeScript hooks did when `JSON.parse` threw.
#[must_use]
pub fn hook_input(hook: &str) -> Option<Value> {
    let mut text = String::new();
    let input = io::stdin()
        .read_to_string(&mut text)
        .ok()
        .and_then(|_| json::parse(&text));

    if input.is_none() {
        eprintln!("{hook}: the hook's input is not JSON");
    }
    input
}
