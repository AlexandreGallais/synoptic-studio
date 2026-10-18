//! The pre-commit hook's run (EN-108, ported from `lint-staged` 17.6.0, `REF-LINT-STAGED`): the
//! tasks run on the staged files, their fixes staged again, a partly staged file keeping its
//! unstaged lines, and the whole state kept in a backup stash until the run succeeds.
//!
//! The sequence is lint-staged's `runAll.js` and `gitWorkflow.js` (MIT, Copyright (c) 2016 Andrey
//! Okonetchnikov), with its defaults: a backup stash when the repository has a commit, the
//! unstaged changes of partly staged files hidden while the tasks run, the original state restored
//! when a task fails, an empty commit refused. The tasks run one pattern after the other rather
//! than at once, and each command's output is printed only when it fails, as lint-staged does.

use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::staged::{
    Program, Step, Task, changed_by_tasks, chunks, flatten, max_argument_length, partly_staged,
    patches_by_file, plan, staged_from_raw, status_entries, z_fields,
};

/// The options of every patch written or compared (lint-staged's `GIT_DIFF_ARGS`).
const DIFF: [&str; 8] = [
    "--binary",
    "--unified=0",
    "--no-color",
    "--no-ext-diff",
    "--src-prefix=a/",
    "--dst-prefix=b/",
    "--patch",
    "--submodule=short",
];

/// The options that apply the hidden changes back (`GIT_APPLY_ARGS`).
const APPLY: [&str; 4] = ["-v", "--whitespace=nowarn", "--recount", "--unidiff-zero"];

/// The files of an ongoing merge, which a reset clears and the run writes back.
const MERGE_FILES: [&str; 3] = ["MERGE_HEAD", "MERGE_MODE", "MERGE_MSG"];

/// The patch of the hidden unstaged changes, in the run's folder.
const PATCH: &str = "unstaged.patch";

/// What went wrong during a run (lint-staged's error symbols).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Failure {
    /// A Git command failed.
    Git,
    /// A task failed.
    Task,
    /// The hidden changes could not be applied back.
    RestoreUnstaged,
    /// The backup could not be applied.
    RestoreOriginal,
    /// The tasks undid every staged change.
    Empty,
}

/// The failures of a run.
#[derive(Clone, Debug, Default)]
struct Errors(Vec<Failure>);

impl Errors {
    fn add(&mut self, failure: Failure) {
        self.0.push(failure);
    }

    fn has(&self, failure: Failure) -> bool {
        self.0.contains(&failure)
    }
}

/// A repository's root and Git folder, where every command runs.
struct Repository {
    root: PathBuf,
    git_dir: PathBuf,
}

impl Repository {
    /// A Git command's output, or its error output as the reason.
    fn git(&self, arguments: &[&str]) -> Result<String, String> {
        self.git_with(arguments, None)
    }

    /// The same, with `GIT_INDEX_FILE` set when given.
    fn git_with(&self, arguments: &[&str], index: Option<&Path>) -> Result<String, String> {
        let mut command = Command::new("git");

        command.args(arguments).current_dir(&self.root);

        if let Some(file) = index {
            command.env("GIT_INDEX_FILE", file);
        }

        let output = command.output().map_err(|error| format!("git: {error}"))?;

        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).into_owned())
        } else {
            Err(format!(
                "git {}: {}",
                arguments.join(" "),
                String::from_utf8_lossy(&output.stderr).trim()
            ))
        }
    }

    /// The staged files, those of both sides only during a merge.
    fn staged_files(&self) -> Result<Vec<String>, String> {
        let raw = |range: &[&str]| -> Result<Vec<String>, String> {
            let mut arguments = vec!["diff"];

            arguments.extend_from_slice(range);
            arguments.extend(["--ita-visible-in-index", "--raw", "-z"]);
            Ok(staged_from_raw(&self.git(&arguments)?))
        };
        let staged = raw(&["--diff-filter=ACMR", "--staged"])?;

        if staged.is_empty() || !self.git_dir.join("MERGE_HEAD").is_file() {
            return Ok(staged);
        }

        let merged = raw(&["--staged", "MERGE_HEAD"])?;

        Ok(staged
            .into_iter()
            .filter(|file| merged.contains(file))
            .collect())
    }

    /// The files added with `--intent-to-add`, which the run refuses.
    fn intent_to_add(&self) -> Result<Vec<String>, String> {
        let visible = z_fields(&self.git(&[
            "diff",
            "--staged",
            "--ita-visible-in-index",
            "--name-only",
            "-z",
        ])?);
        let invisible = z_fields(&self.git(&[
            "diff",
            "--staged",
            "--ita-invisible-in-index",
            "--name-only",
            "-z",
        ])?);

        Ok(visible
            .into_iter()
            .filter(|file| !invisible.contains(file))
            .collect())
    }

    /// The work tree's diff against the index, by file.
    fn unstaged_patches(&self) -> Result<String, String> {
        let mut arguments = vec!["diff"];

        arguments.extend(DIFF);
        arguments.extend(["--raw", "-z", "--no-renames", "--no-textconv"]);
        self.git(&arguments)
    }
}

/// How a step's program is started from the repository's root.
fn command_of(root: &Path, step: &Step) -> (String, Vec<String>) {
    let executable = |name: &str| {
        if cfg!(windows) {
            format!("{name}.exe")
        } else {
            name.to_owned()
        }
    };

    match &step.program {
        Program::Tool { krate, binary } => {
            let built = root.join("target/release").join(executable(binary));

            if built.is_file() {
                (built.to_string_lossy().into_owned(), step.arguments.clone())
            } else {
                let mut arguments: Vec<String> = ["run", "--release", "-q", "-p", krate, "--bin"]
                    .iter()
                    .map(|&word| word.to_owned())
                    .collect();

                arguments.push((*binary).to_owned());
                arguments.push("--".to_owned());
                arguments.extend(step.arguments.iter().cloned());
                ("cargo".to_owned(), arguments)
            }
        }
        Program::Package(name) => {
            let file = if cfg!(windows) {
                format!("{name}.cmd")
            } else {
                (*name).to_owned()
            };

            (
                root.join("node_modules/.bin")
                    .join(file)
                    .to_string_lossy()
                    .into_owned(),
                step.arguments.clone(),
            )
        }
        Program::Path(path) => (path.clone(), step.arguments.clone()),
    }
}

/// Runs one step on its files, in chunks; true when every chunk succeeds. A failure prints the
/// command and its output.
fn run_step(root: &Path, step: &Step, files: &[String]) -> bool {
    let (program, arguments) = command_of(root, step);
    let line = std::iter::once(program.clone())
        .chain(arguments.iter().cloned())
        .collect::<Vec<_>>()
        .join(" ");

    for chunk in chunks(&line, files, max_argument_length()) {
        let output = Command::new(&program)
            .args(&arguments)
            .args(&chunk)
            .current_dir(root)
            .output();

        match output {
            Ok(output) if output.status.success() => {}
            Ok(output) => {
                let code = output
                    .status
                    .code()
                    .map_or_else(|| "a signal".to_owned(), |code| code.to_string());

                eprintln!("✖ {} failed (exit {code}):", step.title());
                eprint!("{}", String::from_utf8_lossy(&output.stdout));
                eprint!("{}", String::from_utf8_lossy(&output.stderr));
                return false;
            }
            Err(error) => {
                eprintln!("✖ {} could not start: {error}", step.title());
                return false;
            }
        }
    }

    true
}

/// Runs each task's steps on its files, a task stopping at its first failing step; true when every
/// task succeeds.
fn run_tasks(root: &Path, planned: &[(&Task, Vec<String>)]) -> bool {
    let mut succeeded = true;

    for (task, files) in planned {
        let passed = task.steps.iter().all(|step| run_step(root, step, files));

        succeeded &= passed;
    }

    succeeded
}

/// A name for the run's backup and folder, unique enough on one machine.
fn run_id() -> String {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |elapsed| elapsed.subsec_nanos());

    format!("{:x}{nanos:08x}", std::process::id())
}

/// The run's state: the backup, the hidden files and the merge files kept.
struct Run<'a> {
    repository: &'a Repository,
    errors: Errors,
    backup: Option<String>,
    hidden: Vec<crate::staged::StatusEntry>,
    folder: PathBuf,
    merge: Vec<(PathBuf, Vec<u8>)>,
}

impl Run<'_> {
    /// Saves the merge files, writes the patch of the partly staged files' unstaged changes, and
    /// stores the backup stash (`prepare`).
    fn prepare(&mut self, with_backup: bool, id: &str) {
        let result = (|| -> Result<(), String> {
            if with_backup {
                self.merge = MERGE_FILES
                    .iter()
                    .map(|name| self.repository.git_dir.join(name))
                    .filter_map(|path| fs::read(&path).ok().map(|bytes| (path, bytes)))
                    .collect();
            }

            let entries =
                status_entries(
                    &self
                        .repository
                        .git(&["status", "-z", "--untracked-files=no"])?,
                );

            self.hidden = partly_staged(&entries);

            if !self.hidden.is_empty() {
                fs::create_dir_all(&self.folder).map_err(|error| error.to_string())?;

                let patch = self.folder.join(PATCH).to_string_lossy().into_owned();
                let files = flatten(&self.hidden, true);
                let mut arguments = vec!["diff"];

                arguments.extend(DIFF);
                arguments.extend(["--output", &patch, "--"]);
                arguments.extend(files.iter().map(String::as_str));
                self.repository.git(&arguments)?;
            }

            if with_backup {
                let hash = self.repository.git(&["stash", "create"])?.trim().to_owned();
                let message = format!("git-hooks automatic backup ({id})");

                self.repository.git(&[
                    "stash",
                    "store",
                    "--quiet",
                    "--message",
                    &message,
                    &hash,
                ])?;
                self.backup = Some(hash);
            }

            Ok(())
        })();

        if let Err(reason) = result {
            self.errors.add(Failure::Git);
            eprintln!("✖ Failed to back up the original state: {reason}");
        }
    }

    /// Removes the unstaged changes of the partly staged files from the work tree.
    fn hide(&mut self) {
        let files = flatten(&self.hidden, false);
        let mut arguments = vec!["restore", "--worktree", "--"];

        arguments.extend(files.iter().map(String::as_str));

        if let Err(reason) = self.repository.git(&arguments) {
            self.errors.add(Failure::Git);
            eprintln!("✖ Failed to hide the unstaged changes: {reason}");
        }
    }

    /// Stages again the files the tasks changed, then refuses a commit left empty (`updateIndex`).
    fn update_index(&mut self, before: &str) {
        let result = (|| -> Result<(), String> {
            let after = self.repository.unstaged_patches()?;
            let changed = changed_by_tasks(&patches_by_file(before), &patches_by_file(&after));
            // A commit of named paths runs on a temporary index: the default one gets the fixes too.
            let active = env::var_os("GIT_INDEX_FILE").map(|file| self.repository.root.join(file));
            let default_lock = match &active {
                Some(file) if file.to_string_lossy().ends_with(".lock") => Some(
                    self.repository.root.join(
                        self.repository
                            .git(&[
                                "rev-parse",
                                "--path-format=absolute",
                                "--git-path",
                                "index.lock",
                            ])?
                            .trim(),
                    ),
                ),
                _ => None,
            };

            for chunk in chunks("git add --", &changed, max_argument_length()) {
                let mut arguments = vec!["add", "--"];

                arguments.extend(chunk.iter().map(String::as_str));
                self.repository.git(&arguments)?;

                if let Some(lock) = default_lock
                    .as_deref()
                    .filter(|lock| active.as_deref() != Some(lock))
                {
                    self.repository.git_with(&arguments, Some(lock))?;
                }
            }

            let staged = z_fields(&self.repository.git(&[
                "diff",
                "--diff-filter=ACMR",
                "--staged",
                "--name-only",
                "-z",
            ])?);

            if staged.is_empty() {
                self.errors.add(Failure::Empty);
                eprintln!("✖ Prevented an empty git commit: the tasks undid every staged change.");
            }

            Ok(())
        })();

        if let Err(reason) = result {
            self.errors.add(Failure::Git);
            eprintln!("✖ Failed to stage the changes of the tasks: {reason}");
        }
    }

    /// Applies the hidden changes back, by a three-way merge when they conflict with the fixes.
    fn restore_unstaged(&mut self) {
        let patch = self.folder.join(PATCH).to_string_lossy().into_owned();
        let apply = |three_way: bool| {
            let mut arguments = vec!["apply"];

            arguments.extend(APPLY);

            if three_way {
                arguments.push("--3way");
            }

            arguments.push(&patch);
            self.repository.git(&arguments)
        };

        if apply(false).or_else(|_| apply(true)).is_ok() {
            let _ = fs::remove_dir_all(&self.folder);
        } else {
            self.errors.add(Failure::Git);
            self.errors.add(Failure::RestoreUnstaged);
            eprintln!("✖ Failed to restore the unstaged changes: they are kept in {patch}");
        }
    }

    /// Puts back the state before the run from the backup stash (`restoreOriginalState`).
    fn restore_original(&mut self, hash: &str) {
        let result = (|| -> Result<(), String> {
            self.repository.git(&["reset", "--hard", "HEAD"])?;
            self.repository
                .git(&["stash", "apply", "--quiet", "--index", hash])?;

            for (path, bytes) in &self.merge {
                fs::write(path, bytes).map_err(|error| error.to_string())?;
            }

            let _ = fs::remove_dir_all(&self.folder);
            Ok(())
        })();

        match result {
            Ok(()) => eprintln!("✖ Reverted to the original state because of errors."),
            Err(reason) => {
                self.errors.add(Failure::Git);
                self.errors.add(Failure::RestoreOriginal);
                eprintln!("✖ Failed to revert to the original state: {reason}");
            }
        }
    }

    /// Drops the backup stash.
    fn cleanup(&mut self, hash: &str) {
        let result = self
            .repository
            .git(&["stash", "list", "--format=%H"])
            .and_then(
                |list| match list.lines().position(|line| line.trim() == hash) {
                    Some(index) => self
                        .repository
                        .git(&["stash", "drop", "--quiet", &index.to_string()])
                        .map(|_| ()),
                    None => Err("the automatic backup is missing".to_owned()),
                },
            );

        if let Err(reason) = result {
            self.errors.add(Failure::Git);
            eprintln!("✖ Failed to clean up: {reason}");
        }
    }
}

/// Runs the tasks on the staged files of the repository holding `folder`; the exit code, 0 when
/// every task and every Git step succeeded.
#[must_use]
pub fn pre_commit(folder: &Path, tasks: &[Task]) -> u8 {
    match run(folder, tasks) {
        Ok(code) => code,
        Err(reason) => {
            eprintln!("✖ git-hooks pre-commit: {reason}");
            1
        }
    }
}

/// The run, the reasons it cannot start as errors.
fn run(folder: &Path, tasks: &[Task]) -> Result<u8, String> {
    let probe = Repository {
        root: folder.to_path_buf(),
        git_dir: folder.to_path_buf(),
    };
    let root = PathBuf::from(probe.git(&["rev-parse", "--show-toplevel"])?.trim());
    let git_dir = PathBuf::from(probe.git(&["rev-parse", "--absolute-git-dir"])?.trim());
    let repository = Repository { root, git_dir };
    let with_backup = repository.git(&["log", "-1"]).is_ok();
    let staged = repository.staged_files()?;

    if staged.is_empty() {
        return Ok(0);
    }

    let intent = repository.intent_to_add()?;

    if !intent.is_empty() {
        eprintln!("✖ Refused: files staged with --intent-to-add:");
        for file in &intent {
            eprintln!("- {file}");
        }
        return Ok(1);
    }

    let planned = plan(tasks, &staged);

    if planned.is_empty() {
        return Ok(0);
    }

    let id = run_id();
    let mut state = Run {
        repository: &repository,
        errors: Errors::default(),
        backup: None,
        hidden: Vec::new(),
        folder: repository.git_dir.join("git-hooks").join(&id),
        merge: Vec::new(),
    };

    state.prepare(with_backup, &id);

    if !state.hidden.is_empty() && !state.errors.has(Failure::Git) {
        state.hide();
    }

    let mut before = None;

    if !state.errors.has(Failure::Git) {
        match repository.unstaged_patches() {
            Ok(diff) => {
                before = Some(diff);
                if !run_tasks(&repository.root, &planned) {
                    state.errors.add(Failure::Task);
                }
            }
            Err(reason) => {
                state.errors.add(Failure::Git);
                eprintln!("✖ Failed to read the unstaged changes: {reason}");
            }
        }
    }

    // With a backup, a failure leaves the index alone: the backup puts everything back.
    let skip_index =
        with_backup && (state.errors.has(Failure::Git) || state.errors.has(Failure::Task));

    if !skip_index {
        state.update_index(before.as_deref().unwrap_or_default());
    }

    if !state.hidden.is_empty()
        && !state.errors.has(Failure::Git)
        && !state.errors.has(Failure::Task)
    {
        state.restore_unstaged();
    }

    if let Some(hash) = state.backup.clone() {
        let revert = state.errors.has(Failure::Task) || state.errors.has(Failure::RestoreUnstaged);
        let unknown_git_error =
            state.errors.has(Failure::Git) && !state.errors.has(Failure::RestoreUnstaged);

        if revert && !unknown_git_error {
            state.restore_original(&hash);
        }

        if !unknown_git_error && !state.errors.has(Failure::RestoreOriginal) {
            state.cleanup(&hash);
        }
    }

    let failed = !state.errors.0.is_empty();

    Ok(u8::from(failed))
}
