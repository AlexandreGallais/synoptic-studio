//! The network, through the commands every machine of the project has rather than an HTTP crate
//! (research note 0025, The crates): `curl` for the registries and Node.js's index, every address
//! in one process with `--parallel`; `gh api` for GitHub, which brings the token of `gh auth` or
//! of `GH_TOKEN` and `GITHUB_TOKEN`.

use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::thread;
use std::time::{SystemTime, UNIX_EPOCH};

/// What one request answered: its body, or why there is none.
pub type Answer = Result<String, String>;

/// The `User-Agent` sent, as crates.io asks of every client (`REF-CRATES-IO-API`).
const USER_AGENT: &str = "synoptic-studio (https://github.com/AlexandreGallais/synoptic-studio)";

/// What `curl` writes after each transfer: its address, its HTTP status and its error.
const WRITE_OUT: &str = "%{url}\t%{http_code}\t%{errormsg}\n";

/// A folder of its own for one run's bodies, in the system's temporary folder.
fn scratch_folder() -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |elapsed| elapsed.as_nanos());

    std::env::temp_dir().join(format!("deps-{}-{nanos}", std::process::id()))
}

/// The answer to an address from `curl`'s write-out lines and the body it wrote: the body for a
/// status 200, the error or the status otherwise.
///
/// # Errors
///
/// The error `curl` wrote, the status other than 200, or `no answer` when the address has no
/// line.
pub fn answer_of(url: &str, write_out: &str, body: Option<String>) -> Answer {
    let line = write_out
        .lines()
        .filter_map(|line| line.split_once('\t'))
        .find(|(address, _)| *address == url)
        .map(|(_, rest)| rest.split_once('\t').unwrap_or((rest, "")));

    match line {
        Some(("200", _)) => body.ok_or_else(|| "no body written".to_owned()),
        Some(("000", error)) if !error.is_empty() => Err(error.to_owned()),
        Some((status, _)) => Err(format!("HTTP {status}")),
        None => Err("no answer".to_owned()),
    }
}

/// Every address, fetched by one `curl` process (`program`) in parallel; an answer per address,
/// in their order.
#[must_use]
pub fn urls(program: &str, addresses: &[String]) -> Vec<Answer> {
    if addresses.is_empty() {
        return Vec::new();
    }
    let folder = scratch_folder();

    if let Err(error) = fs::create_dir_all(&folder) {
        return addresses.iter().map(|_| Err(error.to_string())).collect();
    }
    let mut command = Command::new(program);

    command.args([
        "--silent",
        "--show-error",
        "--location",
        "--compressed",
        "--parallel",
        "--parallel-max",
        "32",
        "--retry",
        "2",
        "--max-time",
        "60",
        "--user-agent",
        USER_AGENT,
        "--write-out",
        WRITE_OUT,
    ]);
    for (index, address) in addresses.iter().enumerate() {
        command
            .arg(address)
            .arg("--output")
            .arg(folder.join(index.to_string()));
    }
    let answers = match command.output() {
        Ok(output) => {
            let write_out = String::from_utf8_lossy(&output.stdout);

            addresses
                .iter()
                .enumerate()
                .map(|(index, address)| {
                    let body = fs::read_to_string(folder.join(index.to_string())).ok();

                    answer_of(address, &write_out, body)
                })
                .collect()
        }
        Err(error) => addresses
            .iter()
            .map(|_| Err(format!("{program}: {error}")))
            .collect(),
    };
    let _ = fs::remove_dir_all(&folder);

    answers
}

/// Every path of GitHub's API, each by a `gh api` process (`program`) of its own, all at once; an
/// answer per path, in their order.
#[must_use]
pub fn github(program: &str, paths: &[String]) -> Vec<Answer> {
    thread::scope(|scope| {
        let handles: Vec<_> = paths
            .iter()
            .map(|path| {
                scope.spawn(
                    move || match Command::new(program).args(["api", path]).output() {
                        Ok(output) if output.status.success() => {
                            Ok(String::from_utf8_lossy(&output.stdout).into_owned())
                        }
                        Ok(output) => Err(String::from_utf8_lossy(&output.stderr)
                            .lines()
                            .next()
                            .unwrap_or("no answer")
                            .to_owned()),
                        Err(error) => Err(format!("{program}: {error}")),
                    },
                )
            })
            .collect();

        handles
            .into_iter()
            .map(|handle| {
                handle
                    .join()
                    .unwrap_or_else(|_| Err("no answer".to_owned()))
            })
            .collect()
    })
}

/// The answers of addresses fetched once each, by address.
#[must_use]
pub fn by_address(addresses: &[String], answers: Vec<Answer>) -> HashMap<String, Answer> {
    addresses.iter().cloned().zip(answers).collect()
}
