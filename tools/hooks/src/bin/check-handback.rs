//! `PreToolUse` hook on `SubagentHandback` (EN-088): when a worker hands back its report, checks
//! that its story branch is pushed and that the report names its commits. Missing, the hand-back is
//! refused once, the reason the worker's next instruction; the next one goes through with what is
//! missing written above the report. Complete, a line above the report names the story, its branch
//! and `story:finish` for the main thread. The main thread, a read-only agent and an agent without
//! a worktree are left alone; any failure lets the call through: the hook never blocks by its own
//! fault.

use std::fs;
use std::io::{self, Read};
use std::path::Path;
use std::process::Command;

use hooks::handback::{
    Branch, Commit, agent_transcript, allow_output, earlier_handbacks, feature_number, finish_line,
    is_checked, missing, refusal, story_id, worktree_id,
};
use hooks::json::{self, Value};
use hooks::limit::deny_output;

/// Git's output in a folder, trimmed; `None` when it fails.
fn git(folder: &Path, arguments: &[&str]) -> Option<String> {
    let output = Command::new("git")
        .args(arguments)
        .current_dir(folder)
        .output()
        .ok()?;

    output
        .status
        .success()
        .then(|| String::from_utf8_lossy(&output.stdout).trim().to_owned())
}

/// The feature branch on `origin` of a story, from its file's name in the worktree.
fn feature_branch(worktree: &Path, story: &str) -> Option<String> {
    let number = fs::read_dir(worktree.join("docs/backlog/stories"))
        .ok()?
        .filter_map(Result::ok)
        .find_map(|entry| feature_number(&entry.file_name().to_string_lossy(), story))?;
    let pattern = format!("refs/remotes/origin/feature/{number}-*");
    let refs = git(
        worktree,
        &["for-each-ref", "--format=%(refname:lstrip=3)", &pattern],
    )?;

    refs.lines().next().map(str::to_owned)
}

/// The story branch as its worktree sees it, with its commits not on the feature branch.
fn branch(worktree: &Path, feature: Option<&str>) -> Option<Branch> {
    let name = git(worktree, &["branch", "--show-current"]).filter(|name| !name.is_empty())?;
    let head = git(worktree, &["rev-parse", "HEAD"])?;
    let remote = git(
        worktree,
        &[
            "rev-parse",
            "--verify",
            "-q",
            &format!("refs/remotes/origin/{name}"),
        ],
    );
    let range = feature.map_or("HEAD".to_owned(), |feature| {
        format!("origin/{feature}..HEAD")
    });
    let commits = git(worktree, &["log", "--format=%H %s", &range])?
        .lines()
        .filter_map(|line| line.split_once(' '))
        .map(|(hash, subject)| Commit {
            hash: hash.into(),
            subject: subject.into(),
        })
        .collect();

    Some(Branch {
        name,
        head,
        remote,
        commits,
    })
}

/// The hook's answer, `None` when it lets the hand-back through untouched.
fn answer(input: &Value) -> Option<String> {
    let agent_id = input.get("agent_id").text();

    if !is_checked(agent_id, input.get("agent_type").text()) {
        return None;
    }

    let lines = fs::read(agent_transcript(
        input.get("transcript_path").text(),
        agent_id,
    ))
    .ok()?;
    let lines = String::from_utf8_lossy(&lines);
    let id = worktree_id(&lines)?;
    let project =
        std::env::var("CLAUDE_PROJECT_DIR").unwrap_or_else(|_| input.get("cwd").text().to_owned());
    let worktree = Path::new(&project).join(".claude/worktrees").join(id);

    if !worktree.is_dir() {
        return None;
    }

    let story = story_id(id);
    let feature = feature_branch(&worktree, &story);
    let branch = branch(&worktree, feature.as_deref())?;
    let tool_input = input.get("tool_input");
    let items = missing(&branch, tool_input.get("message").text());

    if !items.is_empty() && earlier_handbacks(&lines, input.get("tool_use_id").text()) == 0 {
        return Some(deny_output(&refusal(&story, &items)));
    }

    let head = branch.head.get(..7).unwrap_or(&branch.head);
    let line = finish_line(&story, &branch.name, head, feature.as_deref(), &items);

    Some(allow_output(tool_input, &line))
}

fn main() {
    let mut text = String::new();

    if io::stdin().read_to_string(&mut text).is_err() {
        return;
    }

    if let Some(output) = json::parse(&text).as_ref().and_then(answer) {
        print!("{output}");
    }
}
