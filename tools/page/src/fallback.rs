//! The fallback of `page <id>` for an id no file holds any more (EN-152, research note 0035, The
//! tool): the document of that id in the project's records, as `knowledge show <id>` prints it,
//! so that an id read in a commit or a test's title still opens once its file has moved into the
//! database. `knowledge` is run by its binary beside `page`'s own, as every agent tool sits in
//! `target/release/`.

use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

/// The `knowledge` binary beside a binary, `target/release/knowledge` beside `target/release/page`.
#[must_use]
pub fn knowledge_beside(binary: &Path) -> PathBuf {
    binary.with_file_name(format!("knowledge{}", std::env::consts::EXE_SUFFIX))
}

/// What `<knowledge> show <id>` prints, when it shows the document; `None` when it cannot start,
/// fails or shows none, its error output left out: the caller's own message says the id was not
/// found.
#[must_use]
pub fn shown_by(knowledge: &Path, id: &str) -> Option<String> {
    let output = Command::new(knowledge)
        .args(["show", id])
        .stdin(Stdio::null())
        .stderr(Stdio::null())
        .output()
        .ok()?;

    output
        .status
        .success()
        .then(|| String::from_utf8_lossy(&output.stdout).into_owned())
}
