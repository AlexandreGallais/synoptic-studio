//! Git and the hook's input, as the ported hooks read them (EN-104); a checkout's folders are
//! `git_repository`'s (NAM-002).

use std::io::{self, Read};
use std::path::Path;
use std::process::Command;

use git_repository::stdout_of;

use crate::json::{self, Value};

/// Git's output in a folder, trimmed; `None` when it cannot run or fails, its message dropped: a
/// hook that exits 0 shows Claude no error output. The one Git call of the hooks (AUD-023).
#[must_use]
pub fn git(folder: &Path, arguments: &[&str]) -> Option<String> {
    stdout_of(Command::new("git").args(arguments).current_dir(folder))
        .ok()
        .map(|output| output.trim().to_owned())
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
