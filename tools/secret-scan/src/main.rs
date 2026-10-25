//! `secret-scan [--staged | <files>…]` (EN-110): the secret scan, the `lint:secrets` check of `check-all` on every
//! tracked file without an argument, the pre-commit hook's on the staged files with `--staged` or
//! the paths it gives. It prints `secretlint` 13.0.7's report, every secret masked, and exits as
//! it does: 0 without a finding, 1 with one, 2 on a fatal error — Git failing, a file that cannot
//! be read, or paths given of which none exists. Called from the repository's root, by
//! `target/release/secret-scan` (ADR-0078).

use std::env;
use std::fs;
use std::io::ErrorKind;
use std::path::Path;
use std::thread;

use secret_scan::files;
use secret_scan::report::{self, Located};
use secret_scan::rules::Options;

/// What the scan reads.
enum Target {
    /// Every tracked file.
    Tracked,
    /// The staged files.
    Staged,
    /// The paths given.
    Paths(Vec<String>),
}

/// The target of the arguments.
fn target(arguments: Vec<String>) -> Result<Target, String> {
    match arguments.first().map(String::as_str) {
        None => Ok(Target::Tracked),
        Some("--staged") if arguments.len() == 1 => Ok(Target::Staged),
        Some(option) if option.starts_with("--") => Err(format!(
            "{option}: unknown, or not alone; usage: secret-scan [--staged | <files>…]"
        )),
        Some(_) => Ok(Target::Paths(arguments)),
    }
}

/// One file's findings, `None` when it does not exist.
fn scan_file(path: &str) -> Result<Option<Vec<Located>>, String> {
    if Path::new(path).is_dir() {
        return Ok(None);
    }
    match fs::read(path) {
        Ok(bytes) => Ok(Some(secret_scan::scan(path, &bytes, Options::default()))),
        Err(error) if error.kind() == ErrorKind::NotFound => Ok(None),
        Err(error) => Err(format!("{path}: {error}")),
    }
}

/// A path and its findings, `None` when it does not exist.
type Scanned = (String, Option<Vec<Located>>);

/// Every file's findings, scanned on the machine's cores, in the order of the paths.
fn scan_all(paths: &[String]) -> Result<Vec<Scanned>, String> {
    let workers = thread::available_parallelism().map_or(1, usize::from);
    let chunk = paths.len().div_ceil(workers).max(1);

    thread::scope(|scope| {
        let handles: Vec<_> = paths
            .chunks(chunk)
            .map(|part| {
                scope.spawn(move || {
                    part.iter()
                        .map(|path| scan_file(path).map(|found| (path.clone(), found)))
                        .collect::<Result<Vec<_>, String>>()
                })
            })
            .collect();
        let mut all = Vec::with_capacity(paths.len());

        for handle in handles {
            all.extend(
                handle
                    .join()
                    .map_err(|_| "a scan thread panicked".to_owned())??,
            );
        }
        Ok(all)
    })
}

/// The scan: the report printed, the exit code returned.
fn run(arguments: Vec<String>) -> Result<u8, String> {
    let target = target(arguments)?;
    let given = matches!(target, Target::Paths(_));
    let paths = match target {
        Target::Tracked => files::tracked()?,
        Target::Staged => files::staged()?,
        Target::Paths(paths) => paths,
    };
    let had_paths = !paths.is_empty();
    let results = scan_all(&files::to_scan(paths)?)?;

    if given && had_paths && results.iter().all(|(_, found)| found.is_none()) && !results.is_empty()
    {
        return Err("no file to scan: none of the paths given exists".to_owned());
    }
    let findings: Vec<(String, Vec<Located>)> = results
        .into_iter()
        .filter_map(|(path, found)| found.map(|found| (path, found)))
        .collect();
    let output = report::format(&findings);

    print!("{output}");
    Ok(u8::from(!output.is_empty()))
}

/// The usage, printed on `--help` or `-h` (REV-009).
const USAGE: &str = "usage: target/release/secret-scan [--staged | <files>…]";

fn main() -> std::process::ExitCode {
    if let Some(code) = tool_use::usage(&std::env::args().skip(1).collect::<Vec<_>>(), USAGE) {
        return code;
    }
    let arguments: Vec<String> = env::args().skip(1).collect();
    let subcommand = if arguments.first().map(String::as_str) == Some("--staged") {
        "--staged"
    } else {
        ""
    };

    tool_use::run("secret-scan", subcommand, || match run(arguments) {
        Ok(code) => code,
        Err(message) => {
            eprintln!("secret-scan: {message}");
            2
        }
    })
}
