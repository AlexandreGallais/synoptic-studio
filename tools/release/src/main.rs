//! `release <notes|check|publish>` from the repository's root (ADR-0086):
//!
//! - `notes [<base>]` prints the draft of the features' text from the commits `base` lacks,
//!   `origin/main` by default, then, on the error output, the commits behind each line; it writes
//!   no file and makes no branch;
//! - `check <file>` checks a release text, the body of a pull request into `main`, up to its
//!   first line `---`;
//! - `publish`, by the button of `release.yml`, refuses a ref other than the branch `main`
//!   (`GITHUB_REF`, a tag named `main` refused), computes the version from the last tag,
//!   makes the GitHub release on the head with the bodies of the pull requests merged since and
//!   the test report, and does nothing without a commit since that tag.

use std::fs;
use std::path::Path;
use std::process::{Command, ExitCode};

use release::body::release_text;
use release::check::check;
use release::feature_text::feature_text;
use release::publish::{Published, publish};

/// The base `release notes` drafts from when none is given: what `main` already holds.
const DEFAULT_BASE: &str = "origin/main";

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

/// `release notes`: its exit code.
fn print_notes(args: &[String]) -> u8 {
    let base = match args {
        [] => DEFAULT_BASE,
        [base] => base.as_str(),
        _ => {
            eprintln!("usage: target/release/release notes [<base>]");
            return 2;
        }
    };
    let mut runner = run;

    match feature_text(Path::new("."), &mut runner, base) {
        Ok(drafted) => {
            print!("{}", drafted.text);
            eprint!("{}", drafted.hints);
            eprintln!(
                "{} line(s) drafted from the commits {base} lacks. Rewrite each `[draft]` line in \
                 the imperative mood (the records' conventions, area release), then run `release check` on the \
                 body.",
                drafted.lines
            );
            0
        }
        Err(message) => {
            eprintln!("release notes: {message}");
            1
        }
    }
}

/// `release check`: its exit code.
fn check_text(args: &[String]) -> u8 {
    let [file] = args else {
        eprintln!("usage: target/release/release check <file>");
        return 2;
    };
    let body = match fs::read_to_string(file) {
        Ok(body) => body,
        Err(error) => {
            eprintln!("release check: {file}: {error}");
            return 1;
        }
    };
    let findings = check(&release_text(&body));

    for finding in &findings {
        eprintln!("{file}:{}: {}", finding.line, finding.message);
    }
    if findings.is_empty() {
        println!("{file}: the release text follows the rules release check reads");
        0
    } else {
        1
    }
}

/// `release publish`: its exit code.
fn publish_release() -> u8 {
    let folder = std::env::temp_dir().join(format!("release-publish-{}", std::process::id()));
    let mut runner = run;
    // The fully-formed ref the run is on, which GitHub sets in every job (`REF-GH-VARIABLES`):
    // `refs/heads/main` for the branch, `refs/tags/main` for a tag of that name.
    let reference = std::env::var("GITHUB_REF").unwrap_or_default();
    let published = publish(&mut runner, &reference, &folder);
    let _ = fs::remove_dir_all(&folder);

    match published {
        Ok(Published::Released(tag)) => {
            println!(
                "released {tag}, its notes the bodies of the pull requests merged since the last \
                 tag, the test report attached"
            );
            0
        }
        Ok(Published::Nothing(tag)) => {
            println!("no commit since {tag}: nothing to release");
            0
        }
        Err(message) => {
            eprintln!("release publish: {message}");
            1
        }
    }
}

/// The usage, printed on `--help` or `-h` (REV-009).
const USAGE: &str = "usage: target/release/release <notes [<base>] | check <file> | publish>";

fn main() -> ExitCode {
    if let Some(code) = tool_use::usage(&std::env::args().skip(1).collect::<Vec<_>>(), USAGE) {
        return code;
    }
    let args: Vec<String> = std::env::args().skip(1).collect();
    let subcommand = args.first().cloned().unwrap_or_default();
    let rest = args.get(1..).unwrap_or_default().to_vec();

    tool_use::run("release", &subcommand, || match subcommand.as_str() {
        "notes" => print_notes(&rest),
        "check" => check_text(&rest),
        "publish" => publish_release(),
        _ => {
            eprintln!("usage: target/release/release <notes [<base>] | check <file> | publish>");
            2
        }
    })
}
