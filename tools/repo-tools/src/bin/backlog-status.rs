//! `backlog-status <ID> <status>` (`target/release/backlog-status`, RET-002, EN-103, EN-154), from
//! any folder of the repository: the item's row in the records, `epics`, `features` or `stories` by
//! the form of its id, in one `BEGIN IMMEDIATE`; every branch and worktree reads it at once.

use std::env;
use std::process::ExitCode;

use knowledge::backlog::set_status;
use knowledge::project::open_records;
use knowledge::store::write_or_refuse;
use repo_tools::backlog_status::status_refusal;

/// The usage, printed on `--help` or `-h` (REV-009).
const USAGE: &str = "usage: target/release/backlog-status <ID> <status>";

/// Sets the status; the line to print, or why it failed.
fn change(id: &str, status: &str) -> Result<String, String> {
    if let Some(refusal) = status_refusal(status) {
        return Err(refusal);
    }

    let cwd = env::current_dir().map_err(|error| error.to_string())?;
    let database = open_records(&cwd)?;
    let found = write_or_refuse(&database, |transaction| set_status(transaction, id, status))?;

    if found {
        Ok(format!("{id} {status} in the records"))
    } else {
        Err(format!("{USAGE}; no item {id} in the records"))
    }
}

fn work() -> u8 {
    let arguments: Vec<String> = env::args().skip(1).collect();

    let [id, status] = arguments.as_slice() else {
        eprintln!("{USAGE}");
        return 1;
    };

    match change(id, status) {
        Ok(line) => {
            println!("{line}");
            0
        }
        Err(error) => {
            eprintln!("backlog-status: {error}");
            1
        }
    }
}

fn main() -> ExitCode {
    if let Some(code) = tool_use::usage(&std::env::args().skip(1).collect::<Vec<_>>(), USAGE) {
        return code;
    }
    tool_use::run("backlog-status", "", work)
}
