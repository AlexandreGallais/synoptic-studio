//! `knowledge` (`target/release/knowledge`, E49; its conventions: the records' `conventions`, area
//! `records`, and its row of `tools`): the project's
//! records in one SQLite file, `$AGENT_RECORDS_ROOT/<repo>.sqlite`, shared by every branch and
//! worktree, each record a row of its table, its text cut into an item table (schema 6):
//!   `target/release/knowledge init`                  opens the file, made or brought up to the
//!                                                    schema; its path
//!   `target/release/knowledge path`                  the file, which nothing opens
//!   `target/release/knowledge search <words>`        the best rows of the full-text index
//!                                                    `records_index`, BM25-ranked, with a snippet
//!   `target/release/knowledge show <id> [<heading>]` a record and its items, from whichever table
//!                                                    holds its id, or one of its parts
//!   `target/release/knowledge list`                  the records of every table, one line each
//!   `target/release/knowledge sql "<statement>"`     one statement: a reading's rows, or a write
//!                                                    (`--purpose` for a new table); every record
//!                                                    is written so
//!   `target/release/knowledge add <kind>`            refused, naming the table and the `sql` that
//!                                                    writes the kind's row
//!   `target/release/knowledge sync`                  the drafts of the branches merged into
//!                                                    `develop` marked merged
//!   `target/release/knowledge snapshot`              the day's copy into the synced folder
//!                                                    `AGENT_KNOWLEDGE_SNAPSHOT` names, seven kept
//!   `target/release/knowledge export-criteria <file>` the done features' criteria, for the CI
//!   `target/release/knowledge export-data [--check]` the other data files the CI reads, written
//!                                                    from the records, or checked against them
//!   `target/release/knowledge check-ids <paths>`     the ids cited the records lack
//!   `target/release/knowledge branch-state <branch> merged|dropped` a branch's drafts marked
//!   `target/release/knowledge import-<what> <folder|file>` the private records: memories,
//!                                                    metrics, library, archives, saves, transcripts
//!
//! `search`, `show` and `list` read every record but the dropped ones, which `--all` adds;
//! `search` and `list` filtered by `--kind`, `--epic`, `--feature`, `--status`, `--since`,
//! `--branch` and `--state`. Every command refuses a working file inside the synced folder. An
//! error is printed on the error output, exit 1.

use std::env;
use std::fs;
use std::path::Path;
use std::process::ExitCode;

use git_repository::{branch_of, checkout_of};
use knowledge::add::refusal;
use knowledge::backlog::{criteria_json, done_criteria, drop_branch, merge_branch, missing_ids};
use knowledge::data_files::{data_files, stale_data_files, stale_refusal};
use knowledge::paths::text_files;
use knowledge::private_records::{
    import_archives, import_library_file, import_memories, import_metrics, import_saves,
};
use knowledge::project::records_file;
use knowledge::read::{
    Filters, hit_lines, list, listed_line, parse_filters, search, section_of, show,
};
use knowledge::records::git;
use knowledge::snapshot::{NOT_SET, SNAPSHOT_VARIABLE, snapshot_folder, take_snapshot};
use knowledge::sql;
use knowledge::store::{SCHEMA_VERSION, open_store, schema_version, write, write_or_refuse};
use knowledge::sync::{TRUNK, mark_merged, merged_branches};
use knowledge::transcripts::import_transcripts;
use rusqlite::Connection;

/// The usage, printed for an unknown command or a wrong call.
const USAGE: &str = "Usage: target/release/knowledge init | path | search <words> [<filters>] | show <id> [<heading>] [--all] | list [<filters>] | sql \"<statement>\" [--purpose \"<text>\"] | add <kind> (refused: knowledge sql \"INSERT …\") | sync | snapshot | export-criteria <file or -> | export-data [--check] | check-ids <paths> | branch-state <branch> merged|dropped | import-memories, import-metrics, import-archives, import-transcripts <folder> | import-library, import-saves <file>; the filters: --kind, --epic, --feature, --status, --since, --branch, --state <value>, --all";

/// The commands that take their own words, no filter.
const OWN_WORDS: [&str; 6] = [
    "sql",
    "add",
    "export-criteria",
    "export-data",
    "check-ids",
    "branch-state",
];

/// Whether a command is called with the number of words it takes, and filters only where it reads.
fn is_called_right(command: &str, words: &[String], filters: &Filters) -> bool {
    let shaped = match command {
        "init" | "path" | "sync" | "list" | "snapshot" => words.is_empty(),
        "search" | "check-ids" | "add" => !words.is_empty(),
        "show" => matches!(words.len(), 1 | 2),
        "export-criteria" => words.len() == 1,
        "export-data" => words.is_empty() || words == ["--check"],
        _ if IMPORTS.contains(&command) => words.len() == 1,
        "branch-state" => words.len() == 2 && matches!(words[1].as_str(), "merged" | "dropped"),
        "sql" => matches!(words.len(), 1 | 3) && (words.len() == 1 || words[1] == "--purpose"),
        _ => false,
    };

    shaped && (matches!(command, "search" | "show" | "list") || *filters == Filters::default())
}

/// The lines of some texts, each ended by a line feed; nothing for none.
fn lines(texts: impl Iterator<Item = String>) -> String {
    texts.map(|text| text + "\n").collect()
}

/// A count of things, `1 row`, `2 rows`, given the word for one and for several.
fn counted(count: usize, one: &str, several: &str) -> String {
    if count == 1 {
        format!("1 {one}")
    } else {
        format!("{count} {several}")
    }
}

/// What `init` prints: the file and its schema, a newer one read only.
fn init(file: &Path) -> Result<String, String> {
    let database = open_store(file)?;
    let version =
        schema_version(&database).map_err(|error| format!("{}: {error}", file.display()))?;
    let newer = if version > SCHEMA_VERSION {
        format!(", newer than this binary's {SCHEMA_VERSION}: read only")
    } else {
        String::new()
    };

    Ok(format!("{}, schema {version}{newer}\n", file.display()))
}

/// Shows a record, or its parts under a heading.
fn shown(database: &Connection, words: &[String], filters: &Filters) -> Result<String, String> {
    let id = words.first().map_or("", String::as_str);
    let text = show(database, id, filters)
        .map_err(|error| error.to_string())?
        .ok_or_else(|| format!("{id}: no record of it in the records"))?;

    match words.get(1) {
        None => Ok(text),
        Some(heading) => section_of(id, &text, heading)
            .ok_or_else(|| format!("{id}: no heading {heading} in it")),
    }
}

/// Marks merged the drafts of the branches a merge commit brought into `develop`, and `develop`'s.
fn synced(database: &Connection, cwd: &Path) -> Result<String, String> {
    let listed = git(
        cwd,
        &[
            "branch",
            "--merged",
            TRUNK,
            "--format=%(refname:short) %(objectname)",
        ],
    )?;
    let parents = git(cwd, &["log", "--merges", "--format=%P", TRUNK])?;
    let branches = merged_branches(&listed, &parents);
    let marked = write(database, |transaction| mark_merged(transaction, &branches))?;

    Ok(format!(
        "{} marked merged, of the {} merged into develop\n",
        counted(marked, "row", "rows"),
        counted(branches.len(), "branch", "branches")
    ))
}

/// Runs one statement: a reading prints its rows, a write runs in `BEGIN IMMEDIATE`.
fn statement(database: &Connection, words: &[String], branch: &str) -> Result<String, String> {
    let text = &words[0];
    let purpose = words.get(2).map(String::as_str);

    if sql::is_reading(database, text)? {
        if purpose.is_some() {
            return Err("--purpose names a table a statement creates".to_owned());
        }
        return sql::read(database, text);
    }
    write_or_refuse(database, |transaction| {
        sql::write(transaction, text, purpose, branch)
    })
}

/// The commands that import the private records, each given one folder or file.
const IMPORTS: [&str; 6] = [
    "import-memories",
    "import-metrics",
    "import-library",
    "import-archives",
    "import-saves",
    "import-transcripts",
];

/// Runs an import of the private records, from a folder or a file, in one write.
fn moved_in(
    database: &Connection,
    file: &Path,
    (command, source): (&str, &str),
) -> Result<String, String> {
    let path = Path::new(source);
    let is_file = matches!(command, "import-library" | "import-saves");

    if is_file && !path.is_file() || !is_file && !path.is_dir() {
        let what = if is_file { "file" } else { "folder" };

        return Err(format!("{source}: no such {what}"));
    }

    let moved = match command {
        "import-memories" => write(database, |done| import_memories(done, path))?,
        "import-metrics" => write(database, |done| import_metrics(done, path))?,
        "import-library" => {
            let same = fs::canonicalize(path).ok() == fs::canonicalize(file).ok();

            if same {
                return Err(format!(
                    "{source}: the records themselves, nothing to import"
                ));
            }

            import_library_file(database, path)?
        }
        "import-archives" => write_or_refuse(database, |done| import_archives(done, path))?,
        "import-saves" => {
            let text = fs::read_to_string(path).map_err(|error| format!("{source}: {error}"))?;

            write(database, |done| import_saves(done, &text))?
        }
        _ => write(database, |done| import_transcripts(done, path))?,
    };

    Ok(format!(
        "{} imported from {source}\n",
        counted(moved.rows, "row", "rows")
    ))
}

/// Writes the criteria of the done features to a file, or prints them for `-`.
fn exported(database: &Connection, target: &str) -> Result<String, String> {
    let criteria = done_criteria(database).map_err(|error| error.to_string())?;
    let json = criteria_json(&criteria);

    if target == "-" {
        return Ok(json);
    }
    fs::write(target, json).map_err(|error| format!("{target}: {error}"))?;
    Ok(format!(
        "{} written: {}\n",
        target,
        counted(criteria.len(), "criterion", "criteria")
    ))
}

/// Writes the data files the CI reads from the records, in the checkout's root; with `--check`,
/// names those that differ from the records instead, an error when one does.
fn exported_data(database: &Connection, root: &Path, words: &[String]) -> Result<String, String> {
    if words.is_empty() {
        let files = data_files(database)?;

        for (path, text) in &files {
            fs::write(root.join(path), text).map_err(|error| format!("{path}: {error}"))?;
        }
        return Ok(lines(
            files.into_iter().map(|(path, _)| format!("{path} written")),
        ));
    }

    let stale = stale_data_files(database, root)?;

    stale_refusal(&stale).map_or_else(
        || Ok("every data file the CI reads agrees with the records\n".to_owned()),
        Err,
    )
}

/// The ids cited under some paths that the records lack, one line each; an error when there are.
fn checked(database: &Connection, paths: &[String]) -> Result<String, String> {
    let files = text_files(paths)?;
    let mut missing = Vec::new();

    for file in &files {
        let Ok(text) = fs::read_to_string(file) else {
            continue;
        };

        for (line, id) in missing_ids(database, &text).map_err(|error| error.to_string())? {
            missing.push(format!(
                "{}:{line}: {id} is not in the records",
                file.display()
            ));
        }
    }
    if missing.is_empty() {
        Ok(format!(
            "every id cited is in the records, {} read\n",
            counted(files.len(), "file", "files")
        ))
    } else {
        Err(missing.join("\n"))
    }
}

/// Marks a branch's drafts merged or dropped.
fn branch_state(database: &Connection, words: &[String]) -> Result<String, String> {
    let (branch, state) = (&words[0], words[1].as_str());
    let marked = write(database, |transaction| {
        if state == "merged" {
            merge_branch(transaction, branch)
        } else {
            drop_branch(transaction, branch)
        }
    })?;

    Ok(format!(
        "{} of {branch} marked {state}\n",
        counted(marked, "row", "rows")
    ))
}

/// What a command prints, or the error it met.
fn output(arguments: &[String]) -> Result<String, String> {
    let (command, words) = arguments
        .split_first()
        .map_or(("", &[][..]), |(command, words)| (command.as_str(), words));
    let (words, filters) = if OWN_WORDS.contains(&command) || IMPORTS.contains(&command) {
        (words.to_vec(), Filters::default())
    } else {
        parse_filters(words)?
    };

    if !is_called_right(command, &words, &filters) {
        return Err(USAGE.to_owned());
    }
    // No record is a document any more: refused before the records are opened.
    if command == "add" {
        return Err(refusal(&words[0]));
    }

    let setting = env::var(SNAPSHOT_VARIABLE).ok();
    // Without the synced folder the snapshot copies nothing and does not fail, whatever the rest.
    let snapshot_into = if command == "snapshot" {
        match snapshot_folder(setting.as_deref())? {
            None => return Ok(format!("{NOT_SET}\n")),
            folder => folder,
        }
    } else {
        None
    };
    let cwd = Path::new(".");
    let file = records_file(cwd)?;

    if let Some(folder) = snapshot_into {
        return take_snapshot(&file, &folder, &calendar::today()).map(|line| line + "\n");
    }
    match command {
        "path" => return Ok(format!("{}\n", file.display())),
        "init" => return init(&file),
        _ => {}
    }

    let database = open_store(&file)?;
    // The branch checked out, read from the checkout's `HEAD` as `git branch --show-current`
    // prints it, empty on a detached head: a row `sql` writes without a branch takes it.
    let branch = checkout_of(cwd)
        .map(|checkout| branch_of(&checkout))
        .unwrap_or_default();
    let failed = |error: rusqlite::Error| format!("{}: {error}", file.display());

    match command {
        "search" => Ok(lines(
            search(&database, &words.join(" "), &filters)
                .map_err(failed)?
                .iter()
                .map(hit_lines),
        )),
        "list" => Ok(lines(
            list(&database, &filters)
                .map_err(failed)?
                .iter()
                .map(listed_line),
        )),
        "show" => shown(&database, &words, &filters),
        "sql" => statement(&database, &words, &branch),
        "export-criteria" => exported(&database, &words[0]),
        "export-data" => exported_data(&database, cwd, &words),
        "check-ids" => checked(&database, &words),
        "branch-state" => branch_state(&database, &words),
        _ if IMPORTS.contains(&command) => moved_in(&database, &file, (command, &words[0])),
        _ => synced(&database, cwd),
    }
}

fn main() -> ExitCode {
    let arguments: Vec<String> = env::args().skip(1).collect();

    if let Some(code) = tool_use::usage(&arguments, USAGE) {
        return code;
    }

    let command = arguments.first().map_or("", String::as_str);

    tool_use::run("knowledge", command, || match output(&arguments) {
        Ok(text) => {
            print!("{text}");
            0
        }
        Err(error) => {
            eprintln!("knowledge: {error}");
            1
        }
    })
}
