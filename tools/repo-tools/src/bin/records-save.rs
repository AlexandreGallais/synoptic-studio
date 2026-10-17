//! `records-save` (`npm run records:save`, ADR-0041, ADR-0044, EN-103), at the end of every session
//! and at every retrospective: the metrics logs, this project's transcripts and the source library
//! copied into the private records repository, committed, then pushed when it has a remote. A
//! missing source is skipped; the library is copied consistently by SQLite's `VACUUM INTO`, even
//! while the library server holds it open.

use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode, Stdio};
use std::time::{SystemTime, UNIX_EPOCH};

use hooks::pages::library_file;
use repo_tools::records_save::{
    Kind, RecordCopy, copy_file, copy_folder, instant, record_copies, records_folder,
};

/// Runs Git in a folder, its standard error shown; its trimmed output.
fn git(cwd: &Path, words: &[&str]) -> Result<String, String> {
    let output = Command::new("git")
        .args(words)
        .current_dir(cwd)
        .stdin(Stdio::null())
        .stderr(Stdio::inherit())
        .output()
        .map_err(|error| format!("git: {error}"))?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).trim().to_owned())
    } else {
        Err(format!(
            "git {} failed in {}",
            words.join(" "),
            cwd.display()
        ))
    }
}

/// A SQLite database copied consistently by `VACUUM INTO`, the source opened read-only.
fn vacuum_into(from: &Path, to: &Path) -> Result<(), String> {
    let _ = fs::remove_file(to);

    let target = to.to_string_lossy().replace('\'', "''");
    let output = Command::new("sqlite3")
        .arg("-readonly")
        .arg(from)
        .arg(format!("VACUUM INTO '{target}'"))
        .stdin(Stdio::null())
        .output()
        .map_err(|error| format!("sqlite3: {error}"))?;

    if output.status.success() {
        Ok(())
    } else {
        Err(format!(
            "sqlite3 {}: {}",
            from.display(),
            String::from_utf8_lossy(&output.stderr).trim()
        ))
    }
}

/// Copies one record; false when its source is missing.
fn save(copy: &RecordCopy, folder: &Path) -> Result<bool, String> {
    if !copy.from.exists() {
        return Ok(false);
    }

    let to = folder.join(copy.to);
    let parent = to.parent().unwrap_or(folder);
    let failed = |error: std::io::Error| format!("{}: {error}", copy.from.display());

    fs::create_dir_all(parent).map_err(failed)?;
    match copy.kind {
        Kind::Database => vacuum_into(&copy.from, &to)?,
        Kind::Folder => copy_folder(&copy.from, &to).map_err(failed)?,
        Kind::File => copy_file(&copy.from, &to).map_err(failed)?,
    }
    Ok(true)
}

fn run() -> Result<(), String> {
    let common = git(
        Path::new("."),
        &["rev-parse", "--path-format=absolute", "--git-common-dir"],
    )?;
    let main = Path::new(&common)
        .parent()
        .map_or_else(PathBuf::new, Path::to_path_buf);
    let folder = records_folder(env::var("RECORDS_PATH").ok().as_deref(), &main);

    if !folder.join(".git").exists() {
        return Err(format!(
            "No records repository in {}: create it (git init) or set RECORDS_PATH.",
            folder.display()
        ));
    }

    let home = env::var("HOME").unwrap_or_default();
    let library =
        library_file(env::var("LIBRARY_PATH").ok(), Some(home.clone())).unwrap_or_default();

    for copy in record_copies(&main, Path::new(&home), &library) {
        let saved = save(&copy, &folder)?;

        println!("{} {}", if saved { "saved" } else { "missing" }, copy.to);
    }

    git(&folder, &["add", "--all"])?;
    if git(&folder, &["status", "--porcelain"])?.is_empty() {
        println!("Nothing changed since the last save.");
    } else {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default();
        let message = format!(
            "Save records {}",
            instant(now.as_secs(), now.subsec_nanos())
        );

        git(&folder, &["commit", "--quiet", "--message", &message])?;
        println!("Committed in {}.", folder.display());
    }

    if !git(&folder, &["remote"])?.is_empty() {
        git(&folder, &["push", "--quiet"])?;
        println!("Pushed.");
    }
    Ok(())
}

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("records-save: {error}");
            ExitCode::from(1)
        }
    }
}
