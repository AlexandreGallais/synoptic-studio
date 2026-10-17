//! `format --write|--check <targets…>` (EN-073): the repository's one formatter, run from its root
//! by `npm run format`, `format:check`, `fix` and `lint-staged`. It runs oxfmt on the targets, then
//! writes every Markdown table of their `.md` files without padding (ADR-0068). `--check` changes
//! nothing: oxfmt checks the other files, and each Markdown file is formatted in a temporary copy,
//! its tables unpadded, then compared with the file.

use std::env;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};
use std::time::{SystemTime, UNIX_EPOCH};

use format::ignore::{is_ignored, patterns};
use format::table::unpad;
use format::targets::expand;

/// oxfmt's configuration, at the root.
const CONFIG: &str = ".oxfmtrc.json";

/// The shared editor settings, which oxfmt reads beside its configuration.
const EDITOR_CONFIG: &str = ".editorconfig";

/// oxfmt's command, installed by npm.
fn oxfmt() -> PathBuf {
    let name = if cfg!(windows) { "oxfmt.cmd" } else { "oxfmt" };

    Path::new("node_modules").join(".bin").join(name)
}

/// Runs oxfmt in a folder; true when it succeeds.
fn run_oxfmt(arguments: &[String], folder: &Path) -> io::Result<bool> {
    let program = env::current_dir()?.join(oxfmt());
    let status = Command::new(program)
        .arg("--no-error-on-unmatched-pattern")
        .args(arguments)
        .current_dir(folder)
        .status()?;

    Ok(status.success())
}

/// The root's files, sorted, for the target `*`.
fn root_files() -> io::Result<Vec<String>> {
    let mut files: Vec<String> = fs::read_dir(".")?
        .filter_map(Result::ok)
        .filter(|entry| fs::metadata(entry.path()).is_ok_and(|metadata| metadata.is_file()))
        .map(|entry| entry.file_name().to_string_lossy().into_owned())
        .collect();

    files.sort();

    Ok(files)
}

/// The Markdown files of the targets that oxfmt formats: tracked or new, not ignored by Git nor
/// by `ignorePatterns`.
fn markdown_files(targets: &[String], ignored: &[String]) -> io::Result<Vec<String>> {
    let output = Command::new("git")
        .args([
            "ls-files",
            "-z",
            "--cached",
            "--others",
            "--exclude-standard",
            "--",
        ])
        .args(targets)
        .output()?;

    if !output.status.success() {
        return Err(io::Error::other(format!(
            "git ls-files: {}",
            String::from_utf8_lossy(&output.stderr).trim()
        )));
    }

    let mut files: Vec<String> = String::from_utf8_lossy(&output.stdout)
        .split('\0')
        .filter(|path| {
            Path::new(path)
                .extension()
                .is_some_and(|extension| extension.eq_ignore_ascii_case("md"))
        })
        .filter(|path| !is_ignored(path, ignored) && Path::new(path).is_file())
        .map(str::to_owned)
        .collect();

    files.dedup();

    Ok(files)
}

/// Writes the tables of each file without padding; the number of files changed.
fn unpad_files(files: &[String]) -> io::Result<usize> {
    let mut changed = 0;

    for file in files {
        let text = fs::read_to_string(file)?;
        let written = unpad(&text);

        if written != text {
            fs::write(file, written)?;
            changed += 1;
        }
    }

    Ok(changed)
}

/// A new temporary folder of this run.
fn temporary_folder() -> io::Result<PathBuf> {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |elapsed| elapsed.as_nanos());
    let folder = env::temp_dir().join(format!("format-{}-{nanos}", std::process::id()));

    fs::create_dir_all(&folder)?;

    Ok(folder)
}

/// The Markdown files whose formatted, unpadded copy differs from them.
fn unformatted_markdown(files: &[String]) -> io::Result<Vec<String>> {
    let copy = temporary_folder()?;
    let root = env::current_dir()?;
    let result = (|| {
        let mut copies = Vec::with_capacity(files.len());

        for file in files {
            let target = copy.join(file);

            if let Some(parent) = target.parent() {
                fs::create_dir_all(parent)?;
            }

            fs::copy(file, &target)?;
            copies.push(target.to_string_lossy().into_owned());
        }

        if root.join(EDITOR_CONFIG).is_file() {
            fs::copy(root.join(EDITOR_CONFIG), copy.join(EDITOR_CONFIG))?;
        }

        let config = root.join(CONFIG).to_string_lossy().into_owned();
        let mut arguments = vec!["--write".to_owned(), "-c".to_owned(), config];

        arguments.extend(copies);

        if !run_oxfmt(&arguments, &copy)? {
            return Err(io::Error::other("oxfmt failed on the Markdown copies"));
        }

        let mut differing = Vec::new();

        for file in files {
            if unpad(&fs::read_to_string(copy.join(file))?) != fs::read_to_string(file)? {
                differing.push(file.clone());
            }
        }

        Ok(differing)
    })();

    fs::remove_dir_all(&copy)?;

    result
}

/// Formats the targets in place.
fn write(targets: &[String], markdown: &[String]) -> io::Result<bool> {
    let mut arguments = vec!["--write".to_owned()];

    arguments.extend_from_slice(targets);

    if !run_oxfmt(&arguments, Path::new("."))? {
        return Ok(false);
    }

    let changed = unpad_files(markdown)?;

    println!(
        "Tables without padding: {changed} of {} Markdown files changed.",
        markdown.len()
    );

    Ok(true)
}

/// Checks the targets without changing them.
fn check(targets: &[String], markdown: &[String]) -> io::Result<bool> {
    let mut arguments = vec!["--check".to_owned()];

    arguments.extend_from_slice(targets);
    arguments.push("!**/*.md".to_owned());

    let others = run_oxfmt(&arguments, Path::new("."))?;
    let differing = unformatted_markdown(markdown)?;

    for file in &differing {
        println!("{file}: not formatted (oxfmt, then tables without padding)");
    }

    println!(
        "Markdown: {} of {} files not formatted.",
        differing.len(),
        markdown.len()
    );

    Ok(others && differing.is_empty())
}

/// Reads the mode and the targets, then formats or checks.
fn run(arguments: &[String]) -> Result<bool, String> {
    let (mode, targets) = arguments
        .split_first()
        .filter(|(mode, targets)| (*mode == "--write" || *mode == "--check") && !targets.is_empty())
        .ok_or("usage: format --write|--check <targets…>")?;
    let config = fs::read_to_string(CONFIG).map_err(|error| format!("{CONFIG}: {error}"))?;
    let ignored = patterns(&config)?;
    let roots = root_files().map_err(|error| error.to_string())?;
    let expanded = expand(targets, &roots)?;
    let markdown = markdown_files(&expanded, &ignored).map_err(|error| error.to_string())?;

    if mode == "--write" {
        write(&expanded, &markdown)
    } else {
        check(&expanded, &markdown)
    }
    .map_err(|error| error.to_string())
}

fn main() -> ExitCode {
    let arguments: Vec<String> = env::args().skip(1).collect();
    let mode = arguments.first().map_or("", String::as_str);

    tool_use::run("format", mode, || match run(&arguments) {
        Ok(true) => 0,
        Ok(false) => 1,
        Err(message) => {
            eprintln!("format: {message}");

            2
        }
    })
}
