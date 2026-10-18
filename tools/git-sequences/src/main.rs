//! `git-sequences sync|commit` (EN-100): one call for a Git sequence the agent ran step by step,
//! in the checkout it is started from. Each step's output is kept back and printed only when the
//! step fails, then the sequence stops; on success one line
//! says where the branch stands; the Git hooks run as for any commit. A sync then rebuilds the
//! release binaries of a checkout that has a Cargo workspace (ADR-0078).

use std::env;
use std::fs;
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode, Stdio};

use git_sequences::{
    Branch, Request, Source, Target, branch_of, changes_refusal, changes_step, commit_refusal,
    commit_steps, fetch_step, message_refusal, parse, sync_steps, sync_target,
};

/// Runs one Git step, the text given on its standard input; its output, or Git's own message.
fn git(step: &[String], input: Option<&str>) -> Result<String, String> {
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
    let out = String::from_utf8_lossy(&output.stdout).trim().to_owned();

    if output.status.success() {
        return Ok(out);
    }

    let err = String::from_utf8_lossy(&output.stderr).trim().to_owned();
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

fn sync(branch: &Branch, base: Option<&str>) -> Result<String, String> {
    // Decided before the fetch: nothing runs on `main` or a detached head.
    let target = sync_target(branch, base)?;

    git(&fetch_step(), None)?;

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

    for step in commit_steps(paths, push) {
        let input = (step.first().map(String::as_str) == Some("commit")).then_some(text.as_str());

        git(&step, input)?;
    }

    let head = git_words(&["log", "-1", "--format=%h %s"])?;
    let pushed = if push == git_sequences::Push::No {
        ""
    } else {
        ", pushed"
    };

    Ok(format!("{} on {}{pushed}", head, branch_name(branch)))
}

/// Rebuilds the checkout's release binaries after a sync brought new sources: its hooks, its Git
/// hooks and the agent call them by their path (ADR-0078). A checkout without a Cargo workspace
/// has none; a failed build stops with Cargo's message.
fn rebuild(top: &Path) -> Result<bool, String> {
    if !top.join("Cargo.toml").is_file() {
        return Ok(false);
    }

    let output = Command::new("cargo")
        .args(["build", "--release", "--workspace", "--locked", "--quiet"])
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
        Request::Sync { base } => sync(&branch, base.as_deref()).and_then(|line| {
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
    }?;

    Ok(format!("{place}: {result}"))
}

fn main() -> ExitCode {
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
