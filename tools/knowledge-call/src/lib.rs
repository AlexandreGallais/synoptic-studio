//! The records read through the `knowledge` binary beside a tool's own (EN-154): the tools the
//! knowledge crate itself builds on — `worktree-new`, `git-sequences`, the hooks, `run-totals` —
//! cannot link it without a cycle, nor need SQLite in each hook; they run `knowledge sql` instead,
//! as `page` runs `knowledge show` (EN-152), and read its rows. The escapes of a cell are written
//! here, `knowledge sql` printing them, so that the two read a row alike.

use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

/// The `knowledge` binary beside a binary, `target/release/knowledge` beside
/// `target/release/worktree-new`.
#[must_use]
pub fn knowledge_beside(binary: &Path) -> PathBuf {
    binary.with_file_name(format!("knowledge{}", std::env::consts::EXE_SUFFIX))
}

/// A cell as `knowledge sql` prints it: a backslash, a tab and a line end escaped, so that a row
/// stays one line of cells joined by tabs.
#[must_use]
pub fn escaped(text: &str) -> String {
    text.replace('\\', "\\\\")
        .replace('\t', "\\t")
        .replace('\r', "\\r")
        .replace('\n', "\\n")
}

/// A cell printed by `knowledge sql` read back: the escapes of [`escaped`] undone.
#[must_use]
pub fn unescaped(cell: &str) -> String {
    let mut text = String::new();
    let mut characters = cell.chars();

    while let Some(character) = characters.next() {
        if character != '\\' {
            text.push(character);
            continue;
        }
        match characters.next() {
            Some('t') => text.push('\t'),
            Some('n') => text.push('\n'),
            Some('r') => text.push('\r'),
            Some(other) => text.push(other),
            None => text.push('\\'),
        }
    }
    text
}

/// The rows of `knowledge sql`'s output, its line of column names left out, each cell unescaped.
#[must_use]
pub fn rows_of(output: &str) -> Vec<Vec<String>> {
    output
        .lines()
        .skip(1)
        .map(|line| line.split('\t').map(unescaped).collect())
        .collect()
}

/// Whether a word may stand inside a statement between quotes: letters, digits, dashes, dots,
/// slashes and underscores, as an id or a branch is; nothing else is written into a statement.
#[must_use]
pub fn is_plain(word: &str) -> bool {
    !word.is_empty()
        && word
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'.' | b'/' | b'_'))
}

/// The rows a reading gives, run by a `knowledge` binary in a folder.
///
/// # Errors
/// When the binary cannot start or fails, with its error output: no records, a statement refused.
pub fn rows_by(knowledge: &Path, cwd: &Path, statement: &str) -> Result<Vec<Vec<String>>, String> {
    let output = Command::new(knowledge)
        .args(["sql", statement])
        .current_dir(cwd)
        .stdin(Stdio::null())
        .output()
        .map_err(|error| format!("{}: {error}", knowledge.display()))?;

    if output.status.success() {
        Ok(rows_of(&String::from_utf8_lossy(&output.stdout)))
    } else {
        Err(String::from_utf8_lossy(&output.stderr).trim().to_owned())
    }
}

/// The rows a reading gives, run by the `knowledge` binary beside the running one, in a folder.
///
/// # Errors
/// As [`rows_by`], or when the running binary's path is unknown.
pub fn rows(cwd: &Path, statement: &str) -> Result<Vec<Vec<String>>, String> {
    let binary = std::env::current_exe().map_err(|error| error.to_string())?;

    rows_by(&knowledge_beside(&binary), cwd, statement)
}
