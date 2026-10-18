//! The release binaries older than their sources (EN-104, ADR-0078). The hooks, the status line,
//! the Git hooks and the agent call `target/release/<binary>` by its path, not through `cargo run`,
//! which would rebuild it: a binary built before its sources changed runs the old code. A binary is
//! stale when one of the files Cargo built it from — listed by Cargo in its dep-info,
//! `target/release/<binary>.d` — was written after it, or when it is missing. Read from the files
//! alone, without Cargo, so that a hook can afford it; a file Cargo did not read, such as another
//! binary's, never makes it stale.
//!
//! The Rust inputs ([`RUST_INPUTS`], EN-130) live here too: `check:all` skips its Rust lanes on a
//! change that touches none, and the `post-merge` and `post-rewrite` hooks rebuild the binaries
//! after a pull that touches one (EN-131).

use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

/// The package's name, from a manifest's text: `name = "…"` under `[package]`.
#[must_use]
pub fn package_name(manifest: &str) -> String {
    let mut section = "";

    for line in manifest.lines().map(str::trim) {
        if line.starts_with('[') {
            section = line;
        } else if section == "[package]"
            && let Some(("name", value)) =
                line.split_once('=').map(|(key, value)| (key.trim(), value))
        {
            return value.split('"').nth(1).unwrap_or_default().to_owned();
        }
    }
    String::new()
}

/// The binaries of the workspace's crates, `tools/*`: `src/main.rs` named after its package, each
/// `src/bin/<name>.rs`; sorted by crate, then by name.
#[must_use]
pub fn binaries(root: &Path) -> Vec<String> {
    let mut folders: Vec<PathBuf> = fs::read_dir(root.join("tools"))
        .into_iter()
        .flatten()
        .flatten()
        .map(|entry| entry.path())
        .collect();

    folders.sort();
    folders
        .iter()
        .flat_map(|folder| {
            let mut names: Vec<String> = fs::read_dir(folder.join("src").join("bin"))
                .into_iter()
                .flatten()
                .flatten()
                .map(|entry| entry.path())
                .filter(|path| path.extension().is_some_and(|extension| extension == "rs"))
                .filter_map(|path| Some(path.file_stem()?.to_string_lossy().into_owned()))
                .collect();

            if folder.join("src").join("main.rs").is_file() {
                names.extend(
                    fs::read_to_string(folder.join("Cargo.toml"))
                        .map(|manifest| package_name(&manifest))
                        .ok()
                        .filter(|name| !name.is_empty()),
                );
            }
            names.sort();
            names
        })
        .collect()
}

/// The files of a dep-info text, `<target>: <file> <file> …`, a space inside a path written `\ `.
#[must_use]
pub fn dep_info_files(text: &str) -> Vec<PathBuf> {
    let line = text.lines().next().unwrap_or_default();
    let mut words = Vec::new();
    let mut word = String::new();
    let mut characters = line.chars().peekable();

    while let Some(character) = characters.next() {
        match character {
            '\\' if characters.peek() == Some(&' ') => {
                word.push(' ');
                characters.next();
            }
            ' ' => words.push(std::mem::take(&mut word)),
            other => word.push(other),
        }
    }
    words.push(word);

    // The first word is the target, ended by its colon.
    words
        .into_iter()
        .skip_while(|word| !word.ends_with(':'))
        .skip(1)
        .filter(|word| !word.is_empty())
        .map(PathBuf::from)
        .collect()
}

/// When a file was last written, `None` when it is missing.
fn written(path: &Path) -> Option<SystemTime> {
    fs::metadata(path)
        .and_then(|metadata| metadata.modified())
        .ok()
}

/// Why a binary is stale, given when it was built and its newest source with its time; `None`
/// when it is newer than every source.
#[must_use]
pub fn verdict(
    binary: &str,
    built: Option<SystemTime>,
    newest: Option<(&str, SystemTime)>,
) -> Option<String> {
    match (built, newest) {
        (None, _) => Some(format!("target/release/{binary} is missing")),
        (Some(built), Some((source, time))) if time > built => {
            Some(format!("target/release/{binary} is older than {source}"))
        }
        _ => None,
    }
}

/// The stale release binaries of a checkout, one line each, the newest source that makes each one
/// stale named relative to the checkout.
#[must_use]
pub fn stale_binaries(root: &Path) -> Vec<String> {
    let release = root.join("target").join("release");

    binaries(root)
        .iter()
        .filter_map(|binary| {
            let file = release.join(format!("{binary}{}", std::env::consts::EXE_SUFFIX));
            let sources = fs::read_to_string(release.join(format!("{binary}.d")))
                .map(|text| dep_info_files(&text))
                .unwrap_or_default();
            let newest = sources
                .iter()
                .filter_map(|path| written(path).map(|time| (path, time)))
                .max_by_key(|(_, time)| *time)
                .map(|(path, time)| {
                    let shown = path.strip_prefix(root).unwrap_or(path);

                    (shown.to_string_lossy().into_owned(), time)
                });

            verdict(
                binary,
                written(&file),
                newest.as_ref().map(|(path, time)| (path.as_str(), *time)),
            )
        })
        .collect()
}

/// What the Rust lanes read: Cargo's inputs — the crates, a new one by its folder alone, the
/// workspace's manifest and lock, the toolchain, Cargo's configuration and cargo-deny's — and the
/// files of the repository the crates' tests read outside `tools/`, each named beside it.
pub const RUST_INPUTS: &[&str] = &[
    "tools/",
    "Cargo.toml",
    "Cargo.lock",
    "rust-toolchain.toml",
    ".cargo/",
    "deny.toml",
    // The lanes' scripts, read against `LANES` (`repo-tools`, `[F19.AC9]`, `[F24.AC1]`).
    "package.json",
    // The oxfmt the formatter's tests run, and its settings (`format`).
    "package-lock.json",
    ".oxfmtrc.json",
    // The CI's matrix and the checks `main` requires, read against `LANES` (`[F24.AC1]`).
    ".github/workflows/",
    "docs/tooling/ci.md",
    // The folders past the signal, read against the lint's settings (`lint`).
    "docs/conventions/files.md",
    // The hooks' settings, read against their binaries (`hooks`).
    ".claude/settings.json",
    // The playground's checklists, copied by `val-prepare`'s tests.
    "playground/checklists/",
    // The test files, read for the binaries they start (`repo-tools`, `[F19.AC9]`).
    "*.test.ts",
];

/// Whether changed files touch one of the inputs: a file ending with its suffix (`*.test.ts`), in
/// its folder (`docs/`), or the file itself.
#[must_use]
pub fn touches(paths: &[String], inputs: &[&str]) -> bool {
    paths.iter().any(|path| {
        inputs.iter().any(|input| {
            if let Some(suffix) = input.strip_prefix('*') {
                path.ends_with(suffix)
            } else if input.ends_with('/') {
                path.starts_with(input)
            } else {
                path == input
            }
        })
    })
}

/// Whether changed files touch an input of the Rust lanes or of the release binaries
/// ([`RUST_INPUTS`]).
#[must_use]
pub fn touches_rust(paths: &[String]) -> bool {
    touches(paths, RUST_INPUTS)
}
