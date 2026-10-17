//! The hand-back check (EN-088): when a worker hands its report back, its story branch must be
//! pushed — so its pre-push `check:all` passed, since a push without it is refused (`guard-bash`) —
//! and the report must name its commits. A worker's transcript names the main checkout, never its
//! worktree (research note 0024): the worktree is the first one its transcript names, its prompt's.

use std::path::{Path, PathBuf};

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

/// Where the worktrees of the stories are, inside the project.
const WORKTREES: &str = ".claude/worktrees/";

/// Characters of a hash shown.
const SHORT_HASH: usize = 7;

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

/// The folder name of the first worktree a text names, `None` when it names none.
#[must_use]
pub fn worktree_id(text: &str) -> Option<&str> {
    let rest = &text[text.find(WORKTREES)? + WORKTREES.len()..];
    let end = rest
        .find(|character: char| !(character.is_ascii_alphanumeric() || character == '-'))
        .unwrap_or(rest.len());

    (end > 0).then(|| &rest[..end])
}

/// The story of a worktree: `en-088` → `EN-088`.
#[must_use]
pub fn story_id(worktree: &str) -> String {
    worktree.to_ascii_uppercase()
}

/// The feature of a story from its file's name, `E17-F20-EN-088-….md` → `f20`.
#[must_use]
pub fn feature_number(file_name: &str, story: &str) -> Option<String> {
    let (prefix, _) = file_name.split_once(&format!("-{story}-"))?;

    prefix
        .split('-')
        .find(|part| part.starts_with('F'))
        .map(str::to_ascii_lowercase)
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
        items.push(format!(
            "name each commit in the report: {}",
            unnamed.join("; ")
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
