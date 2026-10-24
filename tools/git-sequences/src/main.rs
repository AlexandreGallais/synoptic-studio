//! `git-sequences sync|commit|state` (EN-100, EN-119): one call for a Git sequence the agent ran
//! step by step, in the checkout it is started from. Each step's output is kept back and printed
//! only when the step fails, then the sequence stops, but the commit hooks' standard error is
//! printed when the commit succeeds too (AUD-035); on success one line says where the branch
//! stands; the Git hooks run as for any commit. A sync then rebuilds the release binaries of a
//! checkout that has a Cargo workspace (ADR-0078). `state` only reads: the branch, its base, the
//! status, the diff's stat and the commits ahead, without a fetch.

use std::env;
use std::fs;
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode, Stdio};

use git_sequences::{
    Branch, Request, Source, State, Target, USAGE, branch_of, changes_refusal, changes_step,
    commit_refusal, commit_steps, fetch_step, message_refusal, parse, state_base, state_report,
    state_steps, sync_steps, sync_target,
};
use stale_binaries::RELEASE_BUILD;

/// Runs one Git step, the text given on its standard input; its output, or Git's own message.
fn git(step: &[String], input: Option<&str>) -> Result<String, String> {
    git_with_errors(step, input).map(|(out, _)| out)
}

/// Runs one Git step as [`git`] does; on success, its output and its standard error, where the
/// hooks of a commit write (AUD-035, F3).
fn git_with_errors(step: &[String], input: Option<&str>) -> Result<(String, String), String> {
    let mut child = Command::new("git")
        .args(step)
        .stdin(if input.is_some() {
            Stdio::piped()
        } else {
            Stdio::null()
        })
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|error| format!("git: {error}"))?;

    if let (Some(text), Some(mut stdin)) = (input, child.stdin.take()) {
        stdin
            .write_all(text.as_bytes())
            .map_err(|error| format!("git {}: {error}", step.join(" ")))?;
    }

    let output = child
        .wait_with_output()
        .map_err(|error| format!("git: {error}"))?;
    // The leading spaces are kept: `status --short` and `diff --stat` open their lines with one.
    let out = String::from_utf8_lossy(&output.stdout)
        .trim_end()
        .to_owned();

    let err = String::from_utf8_lossy(&output.stderr).trim().to_owned();

    if output.status.success() {
        return Ok((out, err));
    }

    let message = [out, err]
        .into_iter()
        .filter(|text| !text.is_empty())
        .collect::<Vec<_>>()
        .join("\n");

    Err(format!("git {} failed:\n{message}", step.join(" ")))
}

/// Runs a Git step written as words.
fn git_words(words: &[&str]) -> Result<String, String> {
    let step: Vec<String> = words.iter().map(|&word| word.to_owned()).collect();

    git(&step, None)
}

/// Names of the story files of the checkout, none when it has no backlog.
fn story_files(top: &Path) -> Vec<String> {
    worktree::story_files(top).unwrap_or_default()
}

/// The message of a commit, from its file or the standard input.
fn read_message(source: &Source) -> Result<String, String> {
    let text = match source {
        Source::File(path) => {
            fs::read_to_string(path).map_err(|error| format!("message file {path}: {error}"))?
        }
        Source::Stdin => {
            let mut text = String::new();

            io::stdin()
                .read_to_string(&mut text)
                .map_err(|error| format!("message from stdin: {error}"))?;
            text
        }
    };

    message_refusal(&text).map_or(Ok(text), Err)
}

/// The checked-out branch's name, as typed by the agent.
fn branch_name(branch: &Branch) -> &str {
    match branch {
        Branch::Story(name) | Branch::Other(name) => name,
        Branch::Main => "main",
        Branch::Detached => "HEAD",
    }
}

fn sync(branch: &Branch, base: Option<&str>, top: &Path) -> Result<String, String> {
    // Refused before the fetch: nothing runs on `main` or a detached head.
    if matches!(branch, Branch::Main | Branch::Detached) {
        sync_target(branch, base, &[], &[])?;
    }

    git(&fetch_step(), None)?;

    let remotes: Vec<String> = git_words(&[
        "for-each-ref",
        "--format=%(refname:short)",
        "refs/remotes/origin/",
    ])?
    .lines()
    .map(str::to_owned)
    .collect();
    let target = sync_target(branch, base, &story_files(top), &remotes)?;

    if let Some(refusal) =
        changes_refusal(branch_name(branch), &target, &git(&changes_step(), None)?)
    {
        return Err(refusal);
    }

    for step in sync_steps(&target) {
        git(&step, None)?;
    }

    let name = branch_name(branch);
    let head = git_words(&["log", "-1", "--format=%h %s"])?;

    Ok(match target {
        Target::Rebase(base) => {
            let ahead = git_words(&["rev-list", "--count", &format!("{base}..HEAD")])?;

            format!("{name} rebased on {base}, {ahead} commit(s) ahead: {head}")
        }
        Target::FastForward => format!("{name} up to date with its upstream: {head}"),
    })
}

fn commit(
    branch: &Branch,
    message: &Source,
    paths: &[String],
    push: git_sequences::Push,
) -> Result<String, String> {
    if let Some(reason) = commit_refusal(branch, push) {
        return Err(reason);
    }

    let text = read_message(message)?;
    let paths = paths_to_add(paths)?;

    for step in commit_steps(&paths, push) {
        let is_commit = step.first().map(String::as_str) == Some("commit");
        let (_, errors) = git_with_errors(&step, is_commit.then_some(text.as_str()))?;

        // What the commit's hooks wrote, the notes of `move-rows --notes` among them, is shown
        // though the commit succeeded; a push's, `check:all`'s report, only when it fails.
        if is_commit && !errors.is_empty() {
            eprintln!("{errors}");
        }
    }

    let head = git_words(&["log", "-1", "--format=%h %s"])?;
    let pushed = if push == git_sequences::Push::No {
        ""
    } else {
        ", pushed"
    };

    Ok(format!("{} on {}{pushed}", head, branch_name(branch)))
}

/// The paths named, but those gone from the working tree whose deletion is staged already, which
/// `git add` refuses as matching no file (`git rm -r <folder>` before the commit, REV-009): the
/// commit takes their staged deletion. A path neither there nor deleted is kept, for Git to name.
fn paths_to_add(paths: &[String]) -> Result<Vec<String>, String> {
    let mut kept = Vec::new();

    for path in paths {
        let deleted = !Path::new(path).exists()
            && !git_words(&[
                "diff",
                "--cached",
                "--name-only",
                "--diff-filter=D",
                "--",
                path,
            ])?
            .is_empty();

        if !deleted {
            kept.push(path.clone());
        }
    }
    Ok(kept)
}

/// Rebuilds the checkout's release binaries after a sync brought new sources: its hooks, its Git
/// hooks and the agent call them by their path (ADR-0078). A checkout without a Cargo workspace
/// has none; a failed build stops with Cargo's message.
fn rebuild(top: &Path) -> Result<bool, String> {
    if !top.join("Cargo.toml").is_file() {
        return Ok(false);
    }

    let output = Command::new("cargo")
        .args(RELEASE_BUILD)
        .current_dir(top)
        .stdin(Stdio::null())
        .output()
        .map_err(|error| format!("cargo: {error}"))?;

    if output.status.success() {
        Ok(true)
    } else {
        Err(format!(
            "synced, but the rebuild of target/release failed (cargo build --release --workspace --locked):\n{}",
            String::from_utf8_lossy(&output.stderr).trim()
        ))
    }
}

fn run(argv: &[String]) -> Result<String, String> {
    let request = parse(argv)?;
    let top = PathBuf::from(git_words(&["rev-parse", "--show-toplevel"])?);
    let branch = branch_of(&git_words(&["rev-parse", "--abbrev-ref", "HEAD"])?);
    let place = top
        .file_name()
        .map_or_else(String::new, |name| name.to_string_lossy().into_owned());
    let result = match &request {
        Request::Sync { base } => sync(&branch, base.as_deref(), &top).and_then(|line| {
            Ok(if rebuild(&top)? {
                format!("{line}; release binaries rebuilt")
            } else {
                line
            })
        }),
        Request::Commit {
            message,
            paths,
            push,
        } => commit(&branch, message, paths, *push),
        Request::State => return state(&branch, &top, place),
    }?;

    Ok(format!("{place}: {result}"))
}

/// The state of the checkout, read without a fetch: the remote branches as last fetched, the
/// upstream when the branch has one.
fn state(branch: &Branch, top: &Path, place: String) -> Result<String, String> {
    let remotes: Vec<String> = git_words(&[
        "for-each-ref",
        "--format=%(refname:short)",
        "refs/remotes/origin/",
    ])?
    .lines()
    .map(str::to_owned)
    .collect();
    let upstream = git_words(&["rev-parse", "--abbrev-ref", "@{upstream}"]).ok();
    let base = state_base(branch, &story_files(top), &remotes, upstream.as_deref());
    let mut parts = Vec::new();

    for step in state_steps(base.as_deref()) {
        parts.push(git(&step, None)?);
    }
    let mut parts = parts.into_iter();
    let mut next = || parts.next().unwrap_or_default();

    Ok(state_report(&State {
        place,
        branch: branch_name(branch).to_owned(),
        base,
        status: next(),
        diff: next(),
        commits: next(),
    }))
}

fn main() -> ExitCode {
    if let Some(code) = tool_use::usage(&std::env::args().skip(1).collect::<Vec<_>>(), USAGE) {
        return code;
    }
    let argv: Vec<String> = env::args().skip(1).collect();
    let subcommand = argv.first().map_or("", String::as_str);

    tool_use::run("git-sequences", subcommand, || match run(&argv) {
        Ok(line) => {
            println!("{line}");
            0
        }
        Err(reason) => {
            eprintln!("git-sequences: {reason}");
            1
        }
    })
}
