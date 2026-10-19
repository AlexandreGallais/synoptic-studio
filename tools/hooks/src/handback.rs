//! The hand-back check (EN-088): when a worker hands its report back, its story branch must be
//! pushed — so its pre-push `check:all` passed, since a push without it is refused (`guard-bash`) —
//! and the report must name its commits. A worker's transcript names the main checkout, never its
//! worktree (research note 0024): the worktree is its prompt's, the folder of the brief's
//! `cd "<worktree>" || exit 9` ([`crate::worktrees::prompt_worktree`], AUD-033).

use std::path::{Path, PathBuf};

use worktree::{base_branch, story_id, story_of_branch, story_of_file};

use crate::json::{Value, quote, write};

/// Agent types that only read: never checked.
const READ_ONLY_AGENTS: [&str; 9] = [
    "Explore",
    "Plan",
    "auditor",
    "verifier",
    "doc-editor",
    "math-expert",
    "functional-analyst",
    "claude-code-guide",
    "statusline-setup",
];

/// Characters of a hash shown.
const SHORT_HASH: usize = 7;

/// Unnamed commits listed in a refusal, the others counted: a story has a few, and a wrong base
/// once listed 934 of them.
pub const LISTED_COMMITS: usize = 10;

/// A commit of the story branch.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Commit {
    pub hash: String,
    pub subject: String,
}

/// A story branch as its worktree sees it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Branch {
    pub name: String,
    /// The worktree's head.
    pub head: String,
    /// The branch's head on `origin`, `None` when it was never pushed.
    pub remote: Option<String>,
    /// Its commits not on its feature branch.
    pub commits: Vec<Commit>,
}

/// What one listing of a worktree's references tells: its branch, its head, that branch's head on
/// `origin` and the branches there.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Refs {
    pub name: String,
    pub head: String,
    /// The branch's head on `origin`, `None` when it was never pushed.
    pub remote: Option<String>,
    /// The branches on `origin`, `origin/feature/f20-…`.
    pub remote_branches: Vec<String>,
}

/// Reads `git for-each-ref --format='%(HEAD) %(objectname) %(refname)' refs/heads
/// refs/remotes/origin`: one Git call in place of four, the hook's time being mostly Git's.
#[must_use]
pub fn read_refs(listing: &str) -> Option<Refs> {
    let rows: Vec<(bool, &str, &str)> = listing
        .lines()
        .filter_map(|line| {
            let (current, rest) = line.split_at_checked(2)?;
            let (hash, name) = rest.split_once(' ')?;

            Some((current == "* ", hash, name))
        })
        .collect();
    let (_, head, name) = rows.iter().find(|(current, _, _)| *current)?;
    let name = name.strip_prefix("refs/heads/")?;
    let remote_name = format!("refs/remotes/origin/{name}");
    let remote = rows
        .iter()
        .find(|(_, _, other)| *other == remote_name)
        .map(|(_, hash, _)| (*hash).to_owned());
    let remote_branches = rows
        .iter()
        .filter_map(|(_, _, other)| other.strip_prefix("refs/remotes/"))
        .map(str::to_owned)
        .collect();

    Some(Refs {
        name: name.to_owned(),
        head: (*head).to_owned(),
        remote,
        remote_branches,
    })
}

/// Whether the hand-back of this agent is checked: a subagent of a type that may write; never the
/// main thread (no agent id) nor an internal agent (an empty type).
#[must_use]
pub fn is_checked(agent_id: &str, agent_type: &str) -> bool {
    !agent_id.is_empty() && !agent_type.is_empty() && !READ_ONLY_AGENTS.contains(&agent_type)
}

/// A subagent's own transcript, in the `subagents/` folder beside the session's.
#[must_use]
pub fn agent_transcript(transcript_path: &str, agent_id: &str) -> PathBuf {
    Path::new(
        transcript_path
            .strip_suffix(".jsonl")
            .unwrap_or(transcript_path),
    )
    .join("subagents")
    .join(format!("agent-{agent_id}.jsonl"))
}

/// The story of a worktree: its branch's, `chore/en-088-…` → `EN-088`, else its folder's when that
/// is a story id, `None` otherwise: a folder may be named otherwise, `en-087-drop`.
#[must_use]
pub fn story_of(branch: &str, folder: &str) -> Option<String> {
    story_of_branch(branch).or_else(|| story_id(folder))
}

/// The branch a story's commits are counted from: its feature branch on `origin`, or
/// `origin/develop` for an epic's own story; `None` when its file or that branch is not found,
/// never the whole history.
#[must_use]
pub fn base_of(story: &str, story_files: &[String], remote_branches: &[String]) -> Option<String> {
    let story = story_files
        .iter()
        .find_map(|file| story_of_file(story, file))?;

    base_branch(story.feature.as_deref(), remote_branches).ok()
}

/// Hand-backs the subagent called before this one, read from its transcript's lines.
#[must_use]
pub fn earlier_handbacks(lines: &str, tool_use_id: &str) -> usize {
    lines
        .lines()
        // Only the lines that may hold one are parsed: a transcript runs to megabytes.
        .filter(|line| line.contains("SubagentHandback"))
        .filter_map(crate::json::parse)
        .flat_map(|line| line.get("message").get("content").items().to_vec())
        .filter(|block| {
            block.get("type").text() == "tool_use"
                && block.get("name").text() == "SubagentHandback"
                && block.get("id").text() != tool_use_id
        })
        .count()
}

/// The first characters of a hash.
fn short(hash: &str) -> &str {
    hash.get(..SHORT_HASH).unwrap_or(hash)
}

/// What the hand-back lacks: the push, the commits the report does not name.
#[must_use]
pub fn missing(branch: &Branch, report: &str) -> Vec<String> {
    let mut items = Vec::new();

    match &branch.remote {
        None => items.push(format!("push `{}`: origin has no such branch", branch.name)),
        Some(remote) if *remote != branch.head => items.push(format!(
            "push `{}`: origin at {}, the worktree at {}",
            branch.name,
            short(remote),
            short(&branch.head)
        )),
        Some(_) => {}
    }

    let unnamed: Vec<String> = branch
        .commits
        .iter()
        .filter(|commit| !report.contains(short(&commit.hash)) && !report.contains(&commit.subject))
        .map(|commit| format!("{} {}", short(&commit.hash), commit.subject))
        .collect();

    if !unnamed.is_empty() {
        let more = unnamed.len().saturating_sub(LISTED_COMMITS);
        let rest = if more == 0 {
            String::new()
        } else {
            format!("; and {more} more")
        };

        items.push(format!(
            "name each commit in the report: {}{rest}",
            unnamed
                .iter()
                .take(LISTED_COMMITS)
                .cloned()
                .collect::<Vec<_>>()
                .join("; ")
        ));
    }
    items
}

/// Why the hand-back is refused, the worker's next instruction.
#[must_use]
pub fn refusal(story: &str, items: &[String]) -> String {
    format!(
        "Hand-back of {story} refused once (check-handback, EN-088): {}. Do it, or hand back again saying why not.",
        items.join("; ")
    )
}

/// The line put above the report for the main thread: the next step, or what is still missing.
#[must_use]
pub fn finish_line(
    story: &str,
    branch: &str,
    head: &str,
    feature: Option<&str>,
    items: &[String],
) -> String {
    if !items.is_empty() {
        return format!(
            "check-handback: {story} on `{branch}` handed back with: {}.",
            items.join("; ")
        );
    }

    let next = feature.map_or("npm run story:finish".to_owned(), |feature| {
        format!("npm run story:finish -- {feature}")
    });

    format!("check-handback: {story} on `{branch}` pushed at {head}; next: `{next}`.")
}

/// The hook's answer letting the hand-back through with the line above its report.
#[must_use]
pub fn allow_output(tool_input: &Value, line: &str) -> String {
    let message = format!("{line}\n\n{}", tool_input.get("message").text());
    let mut members = match tool_input {
        Value::Object(members) => members.clone(),
        _ => Vec::new(),
    };

    members.retain(|(name, _)| name != "message");
    members.insert(0, ("message".to_owned(), Value::String(message)));
    format!(
        "{{\"hookSpecificOutput\":{{\"hookEventName\":\"PreToolUse\",\"permissionDecision\":\"allow\",\"permissionDecisionReason\":{},\"updatedInput\":{}}}}}",
        quote("check-handback (EN-088)"),
        write(&Value::Object(members))
    )
}
