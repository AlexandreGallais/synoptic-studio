//! `release <prepare|check|notes|publish>` from the repository's root (EN-112, ADR-0079):
//!
//! - `prepare [--date YYYY-MM-DD]` makes the branch `release/x.y.z` at the head of a clean
//!   checkout and writes the version and the entry's draft, then lists the commits behind each line;
//! - `check` checks the latest entry of `CHANGELOG.md` and the versions of the manifests;
//! - `notes [x.y.z]` prints a version's entry without its heading, `package.json`'s by default;
//! - `publish`, on `main` after a release's merge, makes its GitHub release with the entry and the
//!   test report, and writes `released` and `tag` into `$GITHUB_OUTPUT` when the workflow sets it.

use std::fs::{self, OpenOptions};
use std::io::Write as _;
use std::path::Path;
use std::process::{Command, ExitCode};
use std::time::{SystemTime, UNIX_EPOCH};

use release::changelog::notes;
use release::check::check;
use release::manifest::{lock_versions, repository, version};
use release::prepare::prepare;
use release::publish::{Published, publish};

/// A command run in the repository's root, for its output.
fn run(program: &str, args: &[&str]) -> Result<String, String> {
    let output = Command::new(program)
        .args(args)
        .output()
        .map_err(|error| format!("{program}: {error}"))?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).into_owned())
    } else {
        Err(format!(
            "{program} {}: {}",
            args.join(" "),
            String::from_utf8_lossy(&output.stderr).trim()
        ))
    }
}

/// Today's date in UTC, `YYYY-MM-DD`.
fn today() -> String {
    let seconds = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |elapsed| elapsed.as_secs());
    let (year, month, day) =
        tool_use::civil_from_days(i64::try_from(seconds / 86_400).unwrap_or(0));

    format!("{year:04}-{month:02}-{day:02}")
}

/// A file of the root, empty when it cannot be read.
fn read(file: &str) -> String {
    fs::read_to_string(file).unwrap_or_default()
}

/// `release prepare`: its exit code.
fn prepare_release(args: &[String]) -> u8 {
    let date = match args {
        [] => today(),
        [flag, date] if flag == "--date" => date.clone(),
        _ => {
            eprintln!("usage: release prepare [--date YYYY-MM-DD]");
            return 2;
        }
    };
    let mut runner = run;

    match run("git", &["status", "--porcelain", "--untracked-files=no"]) {
        Ok(status) if status.trim().is_empty() => {}
        Ok(_) => {
            eprintln!("release prepare: the checkout has uncommitted changes");
            return 1;
        }
        Err(error) => {
            eprintln!("release prepare: {error}");
            return 1;
        }
    }

    let prepared = match prepare(Path::new("."), &mut runner, &date) {
        Ok(prepared) => prepared,
        Err(message) => {
            eprintln!("release prepare: {message}");
            return 1;
        }
    };
    let branch = format!("release/{}", prepared.version);

    if let Err(error) = run("git", &["switch", "-c", &branch]) {
        eprintln!("release prepare: {error}");
        return 1;
    }
    for (file, text) in [
        ("package.json", &prepared.package),
        ("package-lock.json", &prepared.lock),
        ("CHANGELOG.md", &prepared.changelog),
    ] {
        if let Err(error) = fs::write(file, text) {
            eprintln!("release prepare: {file}: {error}");
            return 1;
        }
    }

    print!("{}", prepared.hints);
    println!(
        "{} → {} on {branch}: {} line(s) drafted in CHANGELOG.md. Rewrite each `[draft]` line in \
         the imperative mood (docs/tooling/releases.md), run `release check`, then commit \
         `chore: release {}`.",
        prepared.previous, prepared.version, prepared.lines, prepared.version
    );
    0
}

/// `release check`: its exit code.
fn check_release() -> u8 {
    let package = read("package.json");
    let lock = read("package-lock.json");
    let changelog = read("CHANGELOG.md");
    let Some(address) = repository(&package) else {
        eprintln!("release check: package.json names no repository url");
        return 1;
    };
    let written = version(&package).unwrap_or_default();
    let mut failed = false;

    for finding in check(&changelog, &address, written) {
        eprintln!("CHANGELOG.md:{}: {}", finding.line, finding.message);
        failed = true;
    }
    let (top, root) = lock_versions(&lock);

    if top != Some(written) || root != Some(written) {
        eprintln!("package-lock.json: its two versions of the package must be {written}");
        failed = true;
    }
    if failed {
        1
    } else {
        println!("CHANGELOG.md: the entry of {written} follows the rules release check reads");
        0
    }
}

/// `release notes`: its exit code.
fn print_notes(args: &[String]) -> u8 {
    let package = read("package.json");
    let wanted = args
        .first()
        .cloned()
        .unwrap_or_else(|| version(&package).unwrap_or_default().to_owned());

    if let Some(text) = notes(&read("CHANGELOG.md"), &wanted) {
        print!("{text}");
        0
    } else {
        eprintln!("release notes: CHANGELOG.md holds no entry of {wanted}");
        1
    }
}

/// Appends the outputs of the release to the file the workflow names, when it names one.
fn write_outputs(released: bool, tag: &str) -> Result<(), String> {
    let Some(file) = std::env::var_os("GITHUB_OUTPUT") else {
        return Ok(());
    };

    OpenOptions::new()
        .create(true)
        .append(true)
        .open(&file)
        .and_then(|mut output| writeln!(output, "released={released}\ntag={tag}"))
        .map_err(|error| format!("GITHUB_OUTPUT: {error}"))
}

/// `release publish`: its exit code.
fn publish_release() -> u8 {
    let folder = std::env::temp_dir().join(format!("release-publish-{}", std::process::id()));
    let mut runner = run;
    let published = publish(Path::new("."), &mut runner, &folder);
    let _ = fs::remove_dir_all(&folder);

    let (released, tag) = match published {
        Ok(Published::Released(tag)) => {
            println!(
                "released {tag}, its notes the entry of CHANGELOG.md, the test report attached"
            );
            (true, tag)
        }
        Ok(Published::Already(tag)) => {
            println!("{tag} is already released: nothing to do");
            (false, tag)
        }
        Err(message) => {
            eprintln!("release publish: {message}");
            return 1;
        }
    };

    match write_outputs(released, &tag) {
        Ok(()) => 0,
        Err(message) => {
            eprintln!("release publish: {message}");
            1
        }
    }
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let subcommand = args.first().cloned().unwrap_or_default();
    let rest = args.get(1..).unwrap_or_default().to_vec();

    tool_use::run("release", &subcommand, || match subcommand.as_str() {
        "prepare" => prepare_release(&rest),
        "check" => check_release(),
        "notes" => print_notes(&rest),
        "publish" => publish_release(),
        _ => {
            eprintln!(
                "usage: release <prepare [--date YYYY-MM-DD] | check | notes [x.y.z] | publish>"
            );
            2
        }
    })
}
