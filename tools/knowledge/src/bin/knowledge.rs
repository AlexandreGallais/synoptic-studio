//! `knowledge` (`target/release/knowledge`, E49, `docs/tooling/knowledge.md`): the project's
//! records in one SQLite file, `$AGENT_RECORDS_ROOT/<repo>/records.sqlite`, shared by every branch
//! and worktree:
//!   `target/release/knowledge init`   opens the file, made or brought up to the schema; its path
//!   `target/release/knowledge path`   the file, which nothing opens
//!
//! An error is printed on the error output, exit 1.

use std::env;
use std::path::Path;
use std::process::ExitCode;

use knowledge::records::{DATABASE_FILE, ROOT_VARIABLE, main_and_origin, project_records};
use knowledge::store::{SCHEMA_VERSION, open_store, schema_version};

/// The usage, printed for an unknown command.
const USAGE: &str = "Usage: target/release/knowledge init | path | add <kind> <file or -> | search <words> [<filters>] | show <id> [<heading>] [<filters>] | list [<filters>] | sync; the filters: --kind, --epic, --feature, --status, --since, --branch, --state <value>, --all";

/// What a command prints, or the error it met.
fn output(command: &str) -> Result<String, String> {
    if !matches!(command, "init" | "path") {
        return Err(USAGE.to_owned());
    }

    let root = env::var(ROOT_VARIABLE).ok();
    let (main, origin) = main_and_origin(Path::new("."))?;
    let file = project_records(root.as_deref(), &main, &origin)?.join(DATABASE_FILE);

    if command == "path" {
        return Ok(file.display().to_string());
    }

    let database = open_store(&file)?;
    let version =
        schema_version(&database).map_err(|error| format!("{}: {error}", file.display()))?;
    let newer = if version > SCHEMA_VERSION {
        format!(", newer than this binary's {SCHEMA_VERSION}: read only")
    } else {
        String::new()
    };

    Ok(format!("{}, schema {version}{newer}", file.display()))
}

fn main() -> ExitCode {
    let arguments: Vec<String> = env::args().skip(1).collect();

    if let Some(code) = tool_use::usage(&arguments, USAGE) {
        return code;
    }

    let command = arguments.first().map_or("", String::as_str);

    tool_use::run("knowledge", command, || match output(command) {
        Ok(text) => {
            println!("{text}");
            0
        }
        Err(error) => {
            eprintln!("knowledge: {error}");
            1
        }
    })
}
