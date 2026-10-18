//! `check-rule-docs <message file>` (EN-059, F18.AC16): run by the `commit-msg` hook from the
//! repository's root. For each story the commit closes (`Closes: <ID>`), it gathers the files of the
//! story's commits (`Refs:` or `Closes:` naming it or its tasks) and the files staged; when they
//! change a rule in `docs/domain/` without the Q row, the VAL's decisions to confirm and the guide
//! page, it refuses the commit and names what is missing, unless the message holds the line
//! `Rule-docs: none affected, <why>`.

use std::env;
use std::fs;
use std::io;
use std::process::{Command, ExitCode};

use rule_docs::check::{closed_stories, docs_of, missing, none_affected, refusal, story_files};

/// The folder of the story files.
const STORIES: &str = "docs/backlog/stories";

/// Output of a Git command.
fn git(arguments: &[&str]) -> io::Result<String> {
    let output = Command::new("git").args(arguments).output()?;

    if !output.status.success() {
        return Err(io::Error::other(format!(
            "git {}: {}",
            arguments.join(" "),
            String::from_utf8_lossy(&output.stderr).trim()
        )));
    }

    Ok(String::from_utf8_lossy(&output.stdout).into_owned())
}

/// The files of a story's commits already made, none on a repository without a commit.
fn committed_files(story: &str) -> io::Result<Vec<String>> {
    if git(&["rev-parse", "--verify", "--quiet", "HEAD"]).is_err() {
        return Ok(Vec::new());
    }

    let log = git(&[
        "log",
        "--no-merges",
        "--fixed-strings",
        &format!("--grep={story}"),
        "--format=%x1e%B%x1f",
        "--name-only",
        "HEAD",
    ])?;

    Ok(story_files(&log, story))
}

/// The refusal of one story, empty when it lacks nothing or has no story file.
fn check_story(story: &str, staged: &[String]) -> io::Result<String> {
    let docs = fs::read_dir(STORIES)?
        .filter_map(Result::ok)
        .find_map(|entry| docs_of(&entry.file_name().to_string_lossy(), story));
    let Some(docs) = docs else {
        return Ok(String::new());
    };
    let mut files = committed_files(story)?;

    files.extend_from_slice(staged);
    Ok(refusal(story, &missing(&docs, &files)))
}

/// The refusals of a commit message, empty when it passes.
fn check(message: &str) -> io::Result<String> {
    let stories = closed_stories(message);

    if stories.is_empty() || none_affected(message) {
        return Ok(String::new());
    }

    let staged: Vec<String> = git(&["diff", "--cached", "--name-only"])?
        .lines()
        .map(str::to_owned)
        .collect();
    let mut refusals = String::new();

    for story in &stories {
        refusals.push_str(&check_story(story, &staged)?);
    }

    Ok(refusals)
}

fn main() -> ExitCode {
    tool_use::run("check-rule-docs", "", check_message)
}

/// Checks the message file named; the exit code.
fn check_message() -> u8 {
    let Some(path) = env::args().nth(1) else {
        eprintln!("usage: check-rule-docs <message file>");
        return 2;
    };

    match fs::read_to_string(&path).and_then(|message| check(&message)) {
        Ok(refusals) if refusals.is_empty() => 0,
        Ok(refusals) => {
            eprint!("{refusals}");
            1
        }
        Err(error) => {
            eprintln!("check-rule-docs: {error}");
            1
        }
    }
}
