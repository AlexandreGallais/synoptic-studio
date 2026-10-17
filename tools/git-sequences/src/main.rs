//! `git-sequences sync|commit` (`npm run git:sync`, `npm run git:commit`, EN-100): one call for a
//! Git sequence the agent ran step by step, in the checkout it is started from. Each step's output
//! is kept back and printed only when the step fails, then the sequence stops; on success one line
//! says where the branch stands. `HUSKY` is removed from the steps' environment: the hooks run.

use std::env;
use std::fs;
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode, Stdio};

use git_sequences::{
    Branch, Request, Source, Target, branch_of, commit_refusal, commit_steps, fetch_step,
    message_refusal, parse, sync_steps, sync_target,
};

/// Runs one Git step, the text given on its standard input; its output, or Git's own message.
fn git(step: &[String], input: Option<&str>) -> Result<String, String> {
    let mut child = Command::new("git")
        .args(step)
        .env_remove("HUSKY")
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

fn run(argv: &[String]) -> Result<String, String> {
    let request = parse(argv)?;
    let top = PathBuf::from(git_words(&["rev-parse", "--show-toplevel"])?);
    let branch = branch_of(&git_words(&["rev-parse", "--abbrev-ref", "HEAD"])?);
    let place = top
        .file_name()
        .map_or_else(String::new, |name| name.to_string_lossy().into_owned());
    let result = match &request {
        Request::Sync { base } => sync(&branch, base.as_deref(), &top),
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

    match run(&argv) {
        Ok(line) => {
            println!("{line}");
            ExitCode::SUCCESS
        }
        Err(reason) => {
            eprintln!("git-sequences: {reason}");
            ExitCode::from(1)
        }
    }
}
