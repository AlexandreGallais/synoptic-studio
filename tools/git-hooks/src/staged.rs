//! What the pre-commit hook runs on which staged files, decided without Git (EN-108): the tasks of
//! `.lintstagedrc.json` as data, the patterns matched on base names, the commands cut in chunks
//! under the platform's argument length, and Git's `-z` outputs read.
//!
//! Ported from `lint-staged` 17.6.0 (MIT, Copyright (c) 2016 Andrey Okonetchnikov,
//! `REF-LINT-STAGED`): `matchFiles.js` (a pattern without `/` matches the base name, dot files
//! included), `getMaxArgLength.js`, `getSpawnedTasks.js` (a command over the limit is cut in two
//! halves until each fits), `parseGitZOutput.js`, `getStagedFiles.js` (submodules and symbolic
//! links left out), and `gitWorkflow.js` (the status entries of partly staged files, renames
//! flattened, and the diff of each file compared before and after the tasks).

use std::collections::BTreeMap;

/// How a task's program is found.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Program {
    /// A binary of the Rust workspace: `target/release/<binary>`, or `cargo run` when it is missing.
    Tool {
        /// The crate that builds it.
        krate: &'static str,
        /// The binary's name.
        binary: &'static str,
    },
    /// A program npm installs, `node_modules/.bin/<name>`.
    Package(&'static str),
    /// A program by its path or name, for a configuration read from a file (the tests').
    Path(String),
}

/// One command of a task, the matched files appended to its arguments.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Step {
    /// Its program.
    pub program: Program,
    /// Its arguments before the files.
    pub arguments: Vec<String>,
}

impl Step {
    /// The command as `.lintstagedrc.json` wrote it, to name it in a failure.
    #[must_use]
    pub fn title(&self) -> String {
        let name = match &self.program {
            Program::Tool { binary, .. } => (*binary).to_owned(),
            Program::Package(name) => (*name).to_owned(),
            Program::Path(path) => path.clone(),
        };

        std::iter::once(name)
            .chain(self.arguments.iter().cloned())
            .collect::<Vec<_>>()
            .join(" ")
    }
}

/// A pattern and the commands run in order on the staged files it matches.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Task {
    /// `*.ts`, `*.{json,yml}`, `*`.
    pub pattern: String,
    /// Each runs only when the one before it succeeded.
    pub steps: Vec<Step>,
}

/// oxlint's options at the commit, those of `npm run fix`.
const OXLINT: [&str; 5] = [
    "--type-check",
    "--fix",
    "--deny-warnings",
    "--report-unused-disable-directives-severity=error",
    "--no-error-on-unmatched-pattern",
];

/// The repository's formatter, writing.
fn format() -> Step {
    Step {
        program: Program::Tool {
            krate: "format",
            binary: "format",
        },
        arguments: vec!["--write".to_owned()],
    }
}

/// The project's linter, fixing.
fn lint() -> Step {
    Step {
        program: Program::Tool {
            krate: "lint",
            binary: "lint",
        },
        arguments: vec!["--fix".to_owned()],
    }
}

/// The tasks of the pre-commit hook, those `.lintstagedrc.json` held, in its order.
#[must_use]
pub fn repository_tasks() -> Vec<Task> {
    let oxlint = Step {
        program: Program::Package("oxlint"),
        arguments: OXLINT.iter().map(|&option| option.to_owned()).collect(),
    };
    let secret_scan = Step {
        program: Program::Tool {
            krate: "secret-scan",
            binary: "secret-scan",
        },
        arguments: Vec::new(),
    };

    vec![
        Task {
            pattern: "*.ts".to_owned(),
            steps: vec![format(), oxlint, lint()],
        },
        Task {
            pattern: "*.md".to_owned(),
            steps: vec![format(), lint()],
        },
        Task {
            pattern: "*.{cjs,css,html,json,scss,toml,yaml,yml}".to_owned(),
            steps: vec![format()],
        },
        Task {
            pattern: "*".to_owned(),
            steps: vec![secret_scan],
        },
    ]
}

/// Tasks read from a file, one step a line: `<pattern> <program> [arguments…]`, separated by
/// spaces, the steps of a pattern in their order; blank lines and `#` lines skipped.
///
/// # Errors
///
/// A line without a program.
pub fn parse_tasks(text: &str) -> Result<Vec<Task>, String> {
    let mut tasks: Vec<Task> = Vec::new();

    for (index, line) in text.lines().enumerate() {
        let line = line.trim();

        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        let mut words = line.split_whitespace().map(str::to_owned);
        let pattern = words.next().unwrap_or_default();
        let program = words
            .next()
            .ok_or_else(|| format!("line {}: a pattern without a command", index + 1))?;
        let step = Step {
            program: Program::Path(program),
            arguments: words.collect(),
        };

        match tasks.iter_mut().find(|task| task.pattern == pattern) {
            Some(task) => task.steps.push(step),
            None => tasks.push(Task {
                pattern,
                steps: vec![step],
            }),
        }
    }

    Ok(tasks)
}

/// Whether a pattern matches a path: a pattern without `/` matches its base name (`matchBase`), dot
/// files included (`dot`). The patterns of the tasks are `*`, `*.<extension>` and
/// `*.{<extension>,…}`; any other pattern matches the path itself, as written.
#[must_use]
pub fn matches(pattern: &str, path: &str) -> bool {
    let name = if pattern.contains('/') {
        path
    } else {
        path.rsplit('/').next().unwrap_or(path)
    };

    if pattern == "*" {
        return true;
    }

    match pattern.strip_prefix("*.") {
        Some(rest) => match rest
            .strip_prefix('{')
            .and_then(|list| list.strip_suffix('}'))
        {
            Some(list) => list
                .split(',')
                .any(|extension| name.ends_with(&format!(".{extension}"))),
            None => name.ends_with(&format!(".{rest}")),
        },
        None => name == pattern,
    }
}

/// The staged files each task matches, in the order of the tasks; a task matching none is left
/// out.
#[must_use]
pub fn plan<'a>(tasks: &'a [Task], files: &[String]) -> Vec<(&'a Task, Vec<String>)> {
    tasks
        .iter()
        .map(|task| {
            let matched: Vec<String> = files
                .iter()
                .filter(|file| matches(&task.pattern, file))
                .cloned()
                .collect();

            (task, matched)
        })
        .filter(|(_, matched)| !matched.is_empty())
        .collect()
}

/// The longest command line a command may get on this platform (lint-staged's
/// `getMaxArgLength`).
#[must_use]
pub fn max_argument_length() -> usize {
    if cfg!(target_os = "macos") {
        262_144
    } else if cfg!(windows) {
        8_191
    } else {
        131_072
    }
}

/// The files cut into chunks, each fitting with the command in `max` characters: a list that does
/// not fit is cut in two halves, the first one the larger, until each fits or holds one file.
#[must_use]
pub fn chunks(command: &str, files: &[String], max: usize) -> Vec<Vec<String>> {
    let length = command.chars().count()
        + if files.is_empty() {
            0
        } else {
            1 + files.iter().map(|file| file.chars().count()).sum::<usize>() + files.len() - 1
        };

    if files.len() <= 1 || length <= max {
        return vec![files.to_vec()];
    }

    let middle = files.len().div_ceil(2);
    let mut cut = chunks(command, &files[..middle], max);

    cut.extend(chunks(command, &files[middle..], max));
    cut
}

/// The fields of a `-z` output, its last NUL removed (`parseGitZOutput`).
#[must_use]
pub fn z_fields(output: &str) -> Vec<String> {
    if output.is_empty() {
        return Vec::new();
    }

    output
        .strip_suffix('\0')
        .unwrap_or(output)
        .split('\0')
        .map(str::to_owned)
        .collect()
}

/// The staged files of `git diff --staged --diff-filter=ACMR --raw -z`, the new path of a rename
/// or a copy; a submodule (`160000`) or a symbolic link (`120000`) left out.
#[must_use]
pub fn staged_from_raw(output: &str) -> Vec<String> {
    let Some(raw) = output.strip_prefix(':') else {
        return Vec::new();
    };

    raw.split("\0:")
        .filter_map(|entry| {
            let fields = z_fields(entry);
            let (info, paths) = fields.split_first()?;
            let destination_mode = info.split(' ').nth(1)?;

            if destination_mode == "160000" || destination_mode == "120000" {
                return None;
            }

            paths.get(1).or_else(|| paths.first()).cloned()
        })
        .collect()
}

/// An entry of `git status -z`: its path, and the path it was renamed or copied from.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StatusEntry {
    /// Index and work tree status, two letters (`MM`).
    pub code: String,
    /// The path.
    pub path: String,
    /// The former path of a rename or a copy.
    pub from: Option<String>,
}

/// The entries of `git status -z --untracked-files=no` (porcelain v1): `XY <path>`, then the former
/// path in a field of its own for a rename or a copy.
#[must_use]
pub fn status_entries(output: &str) -> Vec<StatusEntry> {
    let fields = z_fields(output);
    let mut entries = Vec::new();
    let mut index = 0;

    while let Some(field) = fields.get(index) {
        index += 1;

        let (Some(code), Some(path)) = (field.get(..2), field.get(3..)) else {
            continue;
        };
        let renamed = code.contains('R') || code.contains('C');
        let from = if renamed {
            index += 1;
            fields.get(index - 1).cloned()
        } else {
            None
        };

        entries.push(StatusEntry {
            code: code.to_owned(),
            path: path.to_owned(),
            from,
        });
    }

    entries
}

/// The entries changed both in the index and in the work tree: partly staged.
#[must_use]
pub fn partly_staged(entries: &[StatusEntry]) -> Vec<StatusEntry> {
    entries
        .iter()
        .filter(|entry| {
            let mut letters = entry.code.chars();

            letters.next().is_some_and(|index| index != ' ')
                && letters.next().is_some_and(|tree| tree != ' ')
        })
        .cloned()
        .collect()
}

/// The paths of entries, a rename's former path first when asked (`processRenames`).
#[must_use]
pub fn flatten(entries: &[StatusEntry], with_former: bool) -> Vec<String> {
    entries
        .iter()
        .flat_map(|entry| {
            entry
                .from
                .iter()
                .filter(|_| with_former)
                .cloned()
                .chain(std::iter::once(entry.path.clone()))
        })
        .collect()
}

/// Each file's patch in `git diff --raw -z --patch` (`parseGitDiffRawWithPatch`): the raw part's
/// paths, in order, given to the patch's `diff --git` blocks, consecutive identical headers kept
/// with the same file.
#[must_use]
pub fn patches_by_file(output: &str) -> BTreeMap<String, String> {
    let mut result = BTreeMap::new();

    if output.is_empty() {
        return result;
    }

    let (raw, patch) = output.split_once("\0\0").unwrap_or((output, ""));
    let files: Vec<String> = z_fields(raw)
        .into_iter()
        .enumerate()
        .filter(|(index, _)| index % 2 == 1)
        .map(|(_, path)| path)
        .collect();
    let mut names = files.into_iter();
    let mut previous: Option<String> = None;
    let mut name: Option<String> = None;

    for block in blocks(patch) {
        let header = block.lines().next().unwrap_or_default().to_owned();

        if previous.as_deref() != Some(header.as_str()) {
            name = names.next();
            previous = Some(header);
        }

        if let Some(file) = &name {
            result
                .entry(file.clone())
                .and_modify(|text: &mut String| text.push_str(block))
                .or_insert_with(|| block.to_owned());
        }
    }

    result
}

/// The blocks of a patch after each line starting `diff --git `, the marker left out.
fn blocks(patch: &str) -> Vec<&str> {
    const MARKER: &str = "diff --git ";
    let mut starts: Vec<usize> = Vec::new();

    if patch.starts_with(MARKER) {
        starts.push(MARKER.len());
    }

    starts.extend(
        patch
            .match_indices(&format!("\n{MARKER}"))
            .map(|(at, _)| at + 1 + MARKER.len()),
    );

    starts
        .iter()
        .enumerate()
        .map(|(index, &start)| {
            let end = starts
                .get(index + 1)
                .map_or(patch.len(), |&next| next - MARKER.len());

            &patch[start..end]
        })
        .collect()
}

/// The files whose work-tree diff differs between before and after the tasks: those the tasks
/// changed, to stage again.
#[must_use]
pub fn changed_by_tasks(
    before: &BTreeMap<String, String>,
    after: &BTreeMap<String, String>,
) -> Vec<String> {
    let mut files: Vec<String> = before
        .keys()
        .chain(after.keys())
        .filter(|file| before.get(*file) != after.get(*file))
        .cloned()
        .collect();

    files.sort();
    files.dedup();
    files
}
