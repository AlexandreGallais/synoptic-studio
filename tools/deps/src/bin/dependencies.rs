//! `dependencies outdated` (`npm run deps:outdated`): prints each package, action, Node.js major,
//! crate and cargo tool behind its latest version at least 7 days old, each locked crate under 7
//! days, and each cargo tool written at two versions or without its row, and exits 1 when one is
//! printed; exits 2, naming the source, when the npm registry, the crates.io index,
//! GitHub or Node.js's index does not answer. `dependencies unused` (`npm run deps:unused`): prints
//! the unused files, exports and dev packages, the unlisted packages and unresolved imports, and
//! exits 1 when it prints one. Run from the checkout's root.

use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

use deps::fetch::{self, Answer};
use deps::outdated::{self, Network};
use deps::unused;
use git_repository::stdout_of;

/// The network of a real run: `curl` and `gh`.
struct Commands;

impl Network for Commands {
    fn urls(&self, addresses: &[String]) -> Vec<Answer> {
        fetch::urls("curl", addresses)
    }

    fn github(&self, paths: &[String]) -> Vec<Answer> {
        fetch::github("gh", paths)
    }
}

/// `deps outdated`: the report, its exit code.
fn check_outdated() -> u8 {
    let pins = outdated::read_pins(Path::new("."));
    let report = outdated::outdated(&pins, &Commands, calendar::now_ms());

    for line in &report.lines {
        println!("{line}");
    }
    for failure in &report.failures {
        eprintln!("dependencies outdated: no answer from {failure}");
    }
    match report.exit_code() {
        0 => println!(
            "Every npm package, action, Node.js, crate and cargo tool on its latest version at least 7 days old"
        ),
        1 => eprintln!(
            "dependencies outdated: {} findings, versions behind their latest version at least 7 days old or cargo tools whose places disagree (docs/tooling/versions-and-security.md)",
            report.lines.len()
        ),
        _ => {}
    }
    report.exit_code()
}

/// `dependencies unused`: the findings, exit 1 when there is one, 2 when Git cannot list the files.
fn check_unused() -> u8 {
    let root = PathBuf::from(".");
    let Ok(listed) = stdout_of(Command::new("git").args(unused::ls_files_arguments())) else {
        eprintln!("dependencies unused: git ls-files failed: run it from a checkout's root");
        return 2;
    };
    let tree = unused::Tree {
        files: unused::project_files(&root, &listed),
        root,
    };
    let findings = unused::check(&tree);

    print!("{}", findings.report());
    u8::from(!findings.is_empty())
}

/// The usage, printed on `--help` or `-h` (REV-009).
const USAGE: &str = "usage: target/release/dependencies outdated|unused";

fn main() -> std::process::ExitCode {
    if let Some(code) = tool_use::usage(&std::env::args().skip(1).collect::<Vec<_>>(), USAGE) {
        return code;
    }
    let command = env::args().nth(1).unwrap_or_default();

    tool_use::run("dependencies", &command, || match command.as_str() {
        "outdated" => check_outdated(),
        "unused" => check_unused(),
        _ => {
            eprintln!("usage: target/release/dependencies outdated|unused");
            2
        }
    })
}
