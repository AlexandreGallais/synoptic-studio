//! The run (EN-058): a copy of the repository as a temporary Git worktree outside it, holding the
//! committed and uncommitted changes of tracked files (`git stash create`) and the untracked files
//! Git does not ignore, a module not yet added included (REV-009); in it, each file's related
//! tests run once unchanged, then once per mutation; a file whose tests fail unchanged is skipped
//! with the command's last lines, its cause; the copy is removed at the end, even after an error.
//! Every Git and test command runs without the `GIT_*` variables, which a hook sets and which
//! would point it at another repository (EN-050).

use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;
use std::time::{Duration, Instant};

use git_repository::stdout_of;

use crate::mutations::{Kind, Language, mutations, selected};

/// The default test command, `{file}` standing for the mutated file.
pub const VITEST_RELATED: [&str; 5] = ["npx", "vitest", "related", "{file}", "--run"];

/// How many times the unchanged run a mutated run may last before it is stopped, plus the grace.
const SLOWER: u32 = 4;

/// Lines of the unchanged run's output a skipped file's reason quotes.
const CAUSE_LINES: usize = 5;

/// Time between two looks at a running test command.
const POLL: Duration = Duration::from_millis(50);

/// What to mutate and how to test it.
#[derive(Clone, Copy, Debug)]
pub struct Request<'a> {
    /// Folder the files are named from, inside the repository.
    pub dir: &'a Path,
    /// Files to mutate, from `dir`.
    pub files: &'a [String],
    /// Test command, `{file}` standing for the file's path from the repository's top, `{test}` for
    /// its test file beside it.
    pub command: &'a [String],
    /// Every mutation found, instead of the first of each kind.
    pub all: bool,
    /// Time a mutated run may last beyond `SLOWER` times the unchanged one.
    pub grace: Duration,
}

/// What became of a mutation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Verdict {
    /// A test failed.
    Caught,
    /// Every test passed.
    Survived,
    /// The tests ran too long and were stopped: a loop that no longer ends.
    TimedOut,
}

/// One mutation tried, or a file that could not be.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Outcome {
    /// A mutation and its verdict.
    Tried {
        /// The file, from the repository's top.
        file: String,
        /// Its line, from 1.
        line: usize,
        /// Its column, from 1.
        column: usize,
        /// What it changed.
        kind: Kind,
        /// The text replaced.
        original: String,
        /// The text put instead.
        replacement: String,
        /// Whether the tests caught it.
        verdict: Verdict,
    },
    /// A file left untried, and why.
    Skipped {
        /// The file as given.
        file: String,
        /// Why.
        reason: String,
    },
}

impl Outcome {
    /// One report line.
    #[must_use]
    pub fn line(&self) -> String {
        match self {
            Self::Tried {
                file,
                line,
                column,
                kind,
                original,
                replacement,
                verdict,
            } => {
                let verdict = match verdict {
                    Verdict::Caught => "caught",
                    Verdict::Survived => "SURVIVED",
                    Verdict::TimedOut => "caught (timed out)",
                };
                let short = |text: &str| text.split_whitespace().collect::<Vec<_>>().join(" ");

                format!(
                    "{verdict}  {file}:{line}:{column}  {kind}: {} → {}",
                    short(original),
                    short(replacement)
                )
            }
            Self::Skipped { file, reason } => format!("skipped  {file}: {reason}"),
        }
    }
}

/// Caught, surviving and skipped counts, as the report's last line.
#[must_use]
pub fn summary(outcomes: &[Outcome]) -> String {
    let count = |wanted: &dyn Fn(&Outcome) -> bool| outcomes.iter().filter(|o| wanted(o)).count();
    let caught = count(&|outcome| {
        matches!(
            outcome,
            Outcome::Tried {
                verdict: Verdict::Caught | Verdict::TimedOut,
                ..
            }
        )
    });
    let survived = count(&|outcome| {
        matches!(
            outcome,
            Outcome::Tried {
                verdict: Verdict::Survived,
                ..
            }
        )
    });
    let skipped = count(&|outcome| matches!(outcome, Outcome::Skipped { .. }));

    format!("{caught} caught, {survived} survived, {skipped} file(s) skipped")
}

/// A command without the `GIT_*` variables of the process running it.
fn isolated(program: impl AsRef<OsStr>, dir: &Path) -> Command {
    let mut command = Command::new(program);

    for (name, _) in std::env::vars_os() {
        if name.to_string_lossy().starts_with("GIT_") {
            command.env_remove(name);
        }
    }
    command.current_dir(dir);
    command
}

/// Output of a Git command run in `dir`, trimmed, or its error output as the reason.
fn git(dir: &Path, arguments: &[&str]) -> Result<String, String> {
    stdout_of(isolated("git", dir).args(arguments))
        .map(|output| output.trim().to_owned())
        .map_err(|failure| failure.to_string())
}

/// Links a folder at `link` to the folder `target`: a symbolic link (`std::os::unix::fs::symlink`).
// Unix-only: the standard library has no portable folder link; Windows gets its own call.
#[cfg(unix)]
fn link_folder(target: &Path, link: &Path) -> std::io::Result<()> {
    std::os::unix::fs::symlink(target, link)
}

/// Links a folder at `link` to the folder `target`: a folder symbolic link on Windows
/// (`std::os::windows::fs::symlink_dir`), which needs the Developer Mode or the right to make one.
#[cfg(windows)]
fn link_folder(target: &Path, link: &Path) -> std::io::Result<()> {
    std::os::windows::fs::symlink_dir(target, link)
}

/// Puts a test command in a process group of its own, so that a time-out stops its children too.
fn in_own_group(command: &mut Command) {
    // Unix-only: process groups (`CommandExt::process_group`); elsewhere the command runs as is.
    #[cfg(unix)]
    std::os::unix::process::CommandExt::process_group(command, 0);
    #[cfg(not(unix))]
    let _ = command;
}

/// Stops a test command that ran over its limit, with its children where the system allows it.
fn stop_with_children(child: &mut std::process::Child) {
    // Unix-only: `kill` sends the signal to the whole group, which `Child::kill` cannot (`npx`
    // leaves Vitest's workers behind); elsewhere only the command itself is stopped. The group, or
    // the command, may already be gone; the caller's wait reaps it either way.
    #[cfg(unix)]
    {
        let _ = Command::new("kill")
            .args(group_kill_arguments(child.id()))
            .status();
        // The command itself too, should the group's signal not reach it.
        let _ = child.kill();
    }
    #[cfg(not(unix))]
    let _ = child.kill();
}

/// The arguments of `kill` that stop the process group `group` (POSIX `kill`): the group's
/// negative id after `--`, since Linux's `kill` reads an argument starting with `-` before it as a
/// signal (EN-127).
#[cfg_attr(
    all(not(unix), not(test)),
    expect(dead_code, reason = "only Unix stops a process group with kill")
)]
fn group_kill_arguments(group: u32) -> [String; 3] {
    ["-KILL".to_owned(), "--".to_owned(), format!("-{group}")]
}

/// The temporary copy, removed when dropped.
struct Copy {
    /// The repository's top.
    top: PathBuf,
    /// The copy's folder, outside the repository.
    dir: PathBuf,
}

impl Copy {
    /// A worktree of the repository at `top` holding its tracked files as they are now, and its
    /// untracked files that Git does not ignore.
    fn new(top: &Path) -> Result<Self, String> {
        static NEXT: AtomicUsize = AtomicUsize::new(0);
        let dir = std::env::temp_dir().join(format!(
            "mutate-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        let stash = git(top, &["stash", "create"])?;
        let snapshot = if stash.is_empty() { "HEAD" } else { &stash };

        git(
            top,
            &[
                "worktree",
                "add",
                "--quiet",
                "--detach",
                &dir.to_string_lossy(),
                snapshot,
            ],
        )?;
        let copy = Self {
            top: top.to_owned(),
            dir,
        };
        let untracked = git(top, &["ls-files", "--others", "--exclude-standard", "-z"])?;

        for file in untracked.split('\0').filter(|file| !file.is_empty()) {
            let target = copy.dir.join(file);

            if let Some(folder) = target.parent() {
                fs::create_dir_all(folder).map_err(|error| format!("{file}: {error}"))?;
            }
            fs::copy(top.join(file), &target).map_err(|error| format!("{file}: {error}"))?;
        }
        let modules = top.join("node_modules");

        if let Ok(modules) = modules.canonicalize() {
            link_folder(&modules, &copy.dir.join("node_modules"))
                .map_err(|error| format!("node_modules: {error}"))?;
        }

        Ok(copy)
    }
}

impl Drop for Copy {
    fn drop(&mut self) {
        // A copy already gone is as good as removed; the prune forgets it in either case.
        let _ = git(
            &self.top,
            &["worktree", "remove", "--force", &self.dir.to_string_lossy()],
        );
        let _ = fs::remove_dir_all(&self.dir);
        let _ = git(&self.top, &["worktree", "prune"]);
    }
}

/// How a test command ended.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Ended {
    Passed,
    Failed,
    TimedOut,
}

/// The test file beside a source file: `src/a/b.test.ts` for `src/a/b.ts`.
#[must_use]
pub fn test_file(file: &str) -> String {
    let path = Path::new(file);
    let stem = path.file_stem().unwrap_or_default().to_string_lossy();
    let name = path.extension().map_or_else(
        || format!("{stem}.test"),
        |extension| format!("{stem}.test.{}", extension.to_string_lossy()),
    );

    path.with_file_name(name).to_string_lossy().into_owned()
}

/// A command argument with `{file}` and `{test}` replaced by the file and its test file.
fn expanded(argument: &str, file: &str) -> String {
    argument
        .replace("{file}", file)
        .replace("{test}", &test_file(file))
}

/// Runs the test command for a file in the copy, stopped with its children after `limit`; its
/// standard output and error go to `log` when one is given, else nowhere.
fn test(
    dir: &Path,
    command: &[String],
    file: &str,
    limit: Option<Duration>,
    log: Option<&Path>,
) -> Result<Ended, String> {
    let [program, arguments @ ..] = command else {
        return Err("no test command".to_owned());
    };
    let mut command = isolated(program, dir);
    let (out, err) = match log {
        Some(log) => {
            let file = fs::File::create(log).map_err(|error| error.to_string())?;
            let copy = file.try_clone().map_err(|error| error.to_string())?;
            (Stdio::from(file), Stdio::from(copy))
        }
        None => (Stdio::null(), Stdio::null()),
    };

    command
        .args(arguments.iter().map(|argument| expanded(argument, file)))
        .stdin(Stdio::null())
        .stdout(out)
        .stderr(err);
    in_own_group(&mut command);
    let mut child = command
        .spawn()
        .map_err(|error| format!("{program}: {error}"))?;
    let start = Instant::now();

    loop {
        if let Some(status) = child.try_wait().map_err(|error| error.to_string())? {
            return Ok(if status.success() {
                Ended::Passed
            } else {
                Ended::Failed
            });
        }
        if limit.is_some_and(|limit| start.elapsed() > limit) {
            stop_with_children(&mut child);
            let _ = child.wait();
            return Ok(Ended::TimedOut);
        }
        thread::sleep(POLL);
    }
}

/// The last non-blank lines of a command's output, each indented under the reason quoting them.
fn last_lines(output: &str) -> String {
    let lines: Vec<&str> = output
        .lines()
        .map(str::trim_end)
        .filter(|line| !line.trim().is_empty())
        .collect();

    lines[lines.len().saturating_sub(CAUSE_LINES)..]
        .iter()
        .fold(String::new(), |mut quoted, line| {
            quoted.push_str("\n    ");
            quoted.push_str(line);
            quoted
        })
}

/// A file's path from the repository's top.
fn from_top(top: &Path, dir: &Path, file: &str) -> Result<String, String> {
    let path = dir
        .join(file)
        .canonicalize()
        .map_err(|error| format!("{error}"))?;

    path.strip_prefix(top)
        .map(|relative| relative.to_string_lossy().into_owned())
        .map_err(|_| "outside the repository".to_owned())
}

/// Tries each mutation of one file in the copy, then puts the file back.
fn mutate_file(
    copy: &Path,
    request: &Request,
    file: &str,
    report: &mut dyn FnMut(&Outcome),
) -> Result<Vec<Outcome>, String> {
    let path = copy.join(file);
    let source =
        fs::read_to_string(&path).map_err(|_| "not a text file of the working tree".to_owned())?;
    let start = Instant::now();

    let log = copy.with_extension("log");
    let unchanged = test(copy, request.command, file, None, Some(&log));
    let output = fs::read_to_string(&log).unwrap_or_default();
    // A log already gone is as good as removed.
    let _ = fs::remove_file(&log);

    if unchanged? != Ended::Passed {
        let cause = last_lines(&output);
        let said = if cause.is_empty() {
            " and printed nothing".to_owned()
        } else {
            format!("; the command's last lines:{cause}")
        };

        return Err(format!(
            "its related tests fail, or none was found, before any mutation{said}"
        ));
    }

    let limit = start.elapsed() * SLOWER + request.grace;
    let mut outcomes = Vec::new();

    for mutation in selected(mutations(&source, Language::of(file)), request.all) {
        fs::write(&path, mutation.apply(&source)).map_err(|error| error.to_string())?;
        let ended = test(copy, request.command, file, Some(limit), None);

        fs::write(&path, &source).map_err(|error| error.to_string())?;
        let (line, column) = mutation.position(&source);
        let outcome = Outcome::Tried {
            file: file.to_owned(),
            line,
            column,
            kind: mutation.kind,
            original: source[mutation.start..mutation.end].to_owned(),
            replacement: mutation.replacement,
            verdict: match ended? {
                Ended::Passed => Verdict::Survived,
                Ended::Failed => Verdict::Caught,
                Ended::TimedOut => Verdict::TimedOut,
            },
        };

        report(&outcome);
        outcomes.push(outcome);
    }

    Ok(outcomes)
}

/// Runs the mutations of each file in a temporary copy of the repository, reporting each outcome as
/// it comes; the copy is removed before returning.
///
/// # Errors
///
/// When the folder is not in a Git repository, or the copy cannot be made.
pub fn run(request: &Request, report: &mut dyn FnMut(&Outcome)) -> Result<Vec<Outcome>, String> {
    let top = PathBuf::from(git(request.dir, &["rev-parse", "--show-toplevel"])?)
        .canonicalize()
        .map_err(|error| error.to_string())?;
    let copy = Copy::new(&top)?;
    let mut outcomes = Vec::new();

    for file in request.files {
        let tried = from_top(&top, request.dir, file)
            .and_then(|relative| mutate_file(&copy.dir, request, &relative, report));

        match tried {
            Ok(found) => outcomes.extend(found),
            Err(reason) => {
                let skipped = Outcome::Skipped {
                    file: file.clone(),
                    reason,
                };

                report(&skipped);
                outcomes.push(skipped);
            }
        }
    }

    Ok(outcomes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn f26_ac1_names_the_group_after_the_end_of_kills_options() {
        assert_eq!(group_kill_arguments(4242), ["-KILL", "--", "-4242"]);
    }
}
