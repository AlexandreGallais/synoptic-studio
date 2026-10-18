//! `dependencies outdated` (`npm run deps:outdated`): prints each package, action, Node.js major and crate
//! behind its latest version at least 7 days old, and each locked crate under 7 days, and exits 1
//! when one is printed; exits 2, naming the source, when the npm registry, the crates.io index,
//! GitHub or Node.js's index does not answer. Run from the checkout's root.

use std::env;
use std::path::Path;

use deps::fetch::{self, Answer};
use deps::outdated::{self, Network};

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
    let report = outdated::outdated(&pins, &Commands, hooks::time::now_ms());

    for line in &report.lines {
        println!("{line}");
    }
    for failure in &report.failures {
        eprintln!("dependencies outdated: no answer from {failure}");
    }
    match report.exit_code() {
        0 => println!(
            "Every npm package, action, Node.js and crate on its latest version at least 7 days old"
        ),
        1 => eprintln!(
            "dependencies outdated: {} versions behind their latest version at least 7 days old (docs/tooling/versions-and-security.md)",
            report.lines.len()
        ),
        _ => {}
    }
    report.exit_code()
}

fn main() -> std::process::ExitCode {
    let command = env::args().nth(1).unwrap_or_default();

    tool_use::run("dependencies", &command, || {
        if command == "outdated" {
            check_outdated()
        } else {
            eprintln!("usage: dependencies outdated");
            2
        }
    })
}
