//! `knowledge` (`target/release/knowledge`, E49, `docs/tooling/knowledge.md`): the project's
//! records in one SQLite file, `$AGENT_RECORDS_ROOT/<repo>.sqlite`, shared by every branch and
//! worktree:
//!   `target/release/knowledge init`                  opens the file, made or brought up to the
//!                                                    schema; its path
//!   `target/release/knowledge path`                  the file, which nothing opens
//!   `target/release/knowledge add <kind> <file|->`   a document, a draft of the current branch;
//!                                                    a journal row, an incident, an improvement
//!                                                    or a feedback goes by `sql` into its table
//!   `target/release/knowledge search <words>`        the best sections, BM25-ranked
//!   `target/release/knowledge show <id> [<heading>]` a document, or one of its sections
//!   `target/release/knowledge list`                  the documents, one line each
//!   `target/release/knowledge sync`                  the drafts of the branches merged into
//!                                                    `develop` marked merged
//!   `target/release/knowledge snapshot`              the day's copy into the synced folder
//!                                                    `AGENT_KNOWLEDGE_SNAPSHOT` names, seven kept
//!   `target/release/knowledge sql "<statement>"`     one statement: a reading's rows, or a write
//!                                                    (`--purpose` for a new table)
//!   `target/release/knowledge import-backlog <dir>`  the backlog's files into its tables
//!   `target/release/knowledge export-criteria <file>` the done features' criteria, for the CI
//!   `target/release/knowledge check-ids <paths>`     the ids cited the records lack
//!   `target/release/knowledge link-ids <paths>`      the links to backlog files written as ids
//!   `target/release/knowledge branch-state <branch> merged|dropped` a branch's drafts marked
//!   `target/release/knowledge import-<what> <folder|file>` what moved out of the Markdown and the
//!                                                    records folder: decisions, research,
//!                                                    derivations, sources, rules, logs, guide, memories,
//!                                                    metrics, library, archives, saves, transcripts
//!
//! `search`, `show` and `list` read the merged rows and the current branch's drafts, filtered by
//! `--kind`, `--epic`, `--feature`, `--status`, `--since`, `--branch` and `--state`; `--all` adds
//! the dropped rows. Every command but `link-ids` refuses a working file inside the synced folder.
//! An error is printed on the error output, exit 1.

use std::env;
use std::fs;
use std::io::{self, Read as _};
use std::path::Path;
use std::process::ExitCode;

use git_repository::{branch_of, checkout_of};
use knowledge::add::{Origin, add_document, is_kind, row_table};
use knowledge::backlog::{
    criteria_json, done_criteria, drop_branch, import_backlog, merge_branch, missing_ids,
};
use knowledge::links::with_ids;
use knowledge::moved::{
    Base, import_decisions, import_derivations, import_guide, import_logs, import_research,
    import_sources,
};
use knowledge::paths::text_files;
use knowledge::private_records::{
    import_archives, import_library_file, import_memories, import_metrics, import_saves,
};
use knowledge::project::records_file;
use knowledge::read::{
    Filters, hit_lines, list, listed_line, parse_filters, search, section_of, show,
};
use knowledge::records::git;
use knowledge::rules::import_rules;
use knowledge::snapshot::{NOT_SET, SNAPSHOT_VARIABLE, snapshot_folder, take_snapshot};
use knowledge::sql;
use knowledge::store::{SCHEMA_VERSION, open_store, schema_version, write, write_or_refuse};
use knowledge::sync::{TRUNK, mark_merged, merged_branches};
use knowledge::transcripts::import_transcripts;
use rusqlite::Connection;

/// The usage, printed for an unknown command or a wrong call.
const USAGE: &str = "Usage: target/release/knowledge init | path | add <kind> <file or -> | search <words> [<filters>] | show <id> [<heading>] [<filters>] | list [<filters>] | sync | snapshot | sql \"<statement>\" [--purpose \"<text>\"] | import-backlog <folder> | export-criteria <file or -> | check-ids <paths> | link-ids <paths> | branch-state <branch> merged|dropped | import-decisions, import-research, import-derivations, import-rules, import-logs, import-guide, import-memories, import-metrics, import-archives, import-transcripts <folder> | import-sources, import-library, import-saves <file>; the filters: --kind, --epic, --feature, --status, --since, --branch, --state <value>, --all";

/// The commands that take their own words, no filter.
const OWN_WORDS: [&str; 6] = [
    "sql",
    "import-backlog",
    "export-criteria",
    "check-ids",
    "link-ids",
    "branch-state",
];

/// Whether a command is called with the number of words it takes, and filters only where it reads.
fn is_called_right(command: &str, words: &[String], filters: &Filters) -> bool {
    let shaped = match command {
        "init" | "path" | "sync" | "list" | "snapshot" => words.is_empty(),
        "add" => words.len() == 2,
        "search" | "check-ids" | "link-ids" => !words.is_empty(),
        "show" => matches!(words.len(), 1 | 2),
        "import-backlog" | "export-criteria" => words.len() == 1,
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

/// Adds the file or the standard input as a draft of the branch.
fn add(database: &Connection, kind: &str, source: &str, branch: &str) -> Result<String, String> {
    if !is_kind(kind) {
        return Err(format!(
            "{kind}: a kind is one word of lower-case letters, as story or note"
        ));
    }
    if let Some(table) = row_table(kind) {
        return Err(format!(
            "{kind}: a row of the table {table}, with its columns, not a document: target/release/knowledge sql \"INSERT INTO {table} (…) VALUES (…)\""
        ));
    }
    if branch.is_empty() {
        return Err("add writes on a branch: check one out".to_owned());
    }

    let (text, path) = if source == "-" {
        let mut text = String::new();

        io::stdin()
            .read_to_string(&mut text)
            .map_err(|error| format!("the standard input: {error}"))?;
        (text, "")
    } else {
        let text = fs::read_to_string(source).map_err(|error| format!("{source}: {error}"))?;

        (text, source)
    };

    if text.trim().is_empty() {
        let name = if path.is_empty() {
            "the standard input"
        } else {
            path
        };

        return Err(format!("{name}: nothing to add"));
    }

    let today = calendar::today();
    let id = write(database, |transaction| {
        add_document(
            transaction,
            &text,
            Origin {
                kind,
                path,
                branch,
                today: &today,
            },
        )
    })?;

    Ok(format!("{id} added on {branch}, a draft\n"))
}

/// Shows a document, or its sections under a heading, as the branch reads it.
fn shown(
    database: &Connection,
    words: &[String],
    branch: &str,
    filters: &Filters,
) -> Result<String, String> {
    let id = words.first().map_or("", String::as_str);
    let reader = if branch.is_empty() {
        "a detached HEAD"
    } else {
        branch
    };
    let text = show(database, branch, id, filters)
        .map_err(|error| error.to_string())?
        .ok_or_else(|| format!("{id}: no document of it read on {reader}"))?;

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

/// Imports the backlog's folder.
fn imported(database: &Connection, folder: &str, branch: &str) -> Result<String, String> {
    let path = Path::new(folder);

    if !path.join("stories").is_dir() {
        return Err(format!("{folder}: no stories folder, not a backlog"));
    }

    let today = calendar::today();
    let done = write(database, |transaction| {
        import_backlog(transaction, path, branch, &today)
    })?;

    Ok(format!(
        "{} epics, {} features, {} stories, {} criteria and {} tasks imported\n",
        done.epics, done.features, done.stories, done.criteria, done.tasks
    ))
}

/// The commands that import what moved into the records, each given one folder or file.
const IMPORTS: [&str; 13] = [
    "import-rules",
    "import-decisions",
    "import-research",
    "import-derivations",
    "import-sources",
    "import-logs",
    "import-guide",
    "import-memories",
    "import-metrics",
    "import-library",
    "import-archives",
    "import-saves",
    "import-transcripts",
];

/// Runs an import of what moved into the records, from a folder or a file, in one write.
fn moved_in(
    database: &Connection,
    file: &Path,
    (command, source): (&str, &str),
    branch: &str,
) -> Result<String, String> {
    let path = Path::new(source);
    let today = calendar::today();
    let base = Base {
        branch,
        today: &today,
    };
    let is_file = matches!(
        command,
        "import-sources" | "import-library" | "import-saves"
    );

    if is_file && !path.is_file() || !is_file && !path.is_dir() {
        let what = if is_file { "file" } else { "folder" };

        return Err(format!("{source}: no such {what}"));
    }

    let read = |path: &Path| fs::read_to_string(path).map_err(|error| format!("{source}: {error}"));
    let moved = match command {
        "import-decisions" => write(database, |done| import_decisions(done, path, base))?,
        "import-research" => write(database, |done| import_research(done, path, base))?,
        "import-derivations" => write(database, |done| import_derivations(done, path, base))?,
        "import-sources" => {
            let text = read(path)?;

            write(database, |done| import_sources(done, source, &text, base))?
        }
        "import-logs" => write(database, |done| import_logs(done, path, base))?,
        "import-guide" => write(database, |done| import_guide(done, path, base))?,
        "import-rules" => write(database, |done| import_rules(done, path, base))?,
        "import-memories" => write(database, |done| import_memories(done, path, base))?,
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
            let text = read(path)?;

            write(database, |done| import_saves(done, &text))?
        }
        _ => write(database, |done| import_transcripts(done, path))?,
    };

    Ok(format!(
        "{} and {} imported from {source}\n",
        counted(moved.rows, "row", "rows"),
        counted(moved.documents, "document", "documents")
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

/// Writes the links and paths to backlog files as ids, in every text file under some paths.
fn linked(paths: &[String]) -> Result<String, String> {
    let mut changed = 0;

    for file in text_files(paths)? {
        let Ok(text) = fs::read_to_string(&file) else {
            continue;
        };
        let written = with_ids(&text);

        if written != text {
            fs::write(&file, written).map_err(|error| format!("{}: {error}", file.display()))?;
            changed += 1;
        }
    }
    Ok(format!(
        "{} written with ids\n",
        counted(changed, "file", "files")
    ))
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
    // A rewrite of files, which needs no records.
    if command == "link-ids" {
        return linked(&words);
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
    // prints it, empty on a detached head.
    let branch = checkout_of(cwd)
        .map(|checkout| branch_of(&checkout))
        .unwrap_or_default();
    let failed = |error: rusqlite::Error| format!("{}: {error}", file.display());

    match command {
        "add" => add(&database, &words[0], &words[1], &branch),
        "search" => Ok(lines(
            search(&database, &branch, &words.join(" "), &filters)
                .map_err(failed)?
                .iter()
                .map(hit_lines),
        )),
        "list" => Ok(lines(
            list(&database, &branch, &filters)
                .map_err(failed)?
                .iter()
                .map(listed_line),
        )),
        "show" => shown(&database, &words, &branch, &filters),
        "sql" => statement(&database, &words, &branch),
        "import-backlog" => imported(&database, &words[0], &branch),
        "export-criteria" => exported(&database, &words[0]),
        "check-ids" => checked(&database, &words),
        "branch-state" => branch_state(&database, &words),
        _ if IMPORTS.contains(&command) => {
            moved_in(&database, &file, (command, &words[0]), &branch)
        }
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
