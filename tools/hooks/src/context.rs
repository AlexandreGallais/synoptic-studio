//! What a subagent carries at its first request, read from its transcript (EN-065): the tokens of
//! that request, the instruction files loaded (`CLAUDE.md`, `AGENTS.md`, the memory index), and
//! the characters of the skill listing and of the task message.

use std::fmt::Write;

use crate::json::{self, Value};

/// What a subagent started with.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Start {
    /// Its type, `general-purpose` or an agent file's name.
    pub agent_type: String,
    /// The short description the main thread gave it.
    pub description: String,
    /// Tokens of its first request: input, cache writes and cache reads.
    pub tokens: u64,
    /// File names of the instructions loaded before it: `CLAUDE.md`, `AGENTS.md`, `MEMORY.md`.
    pub instructions: Vec<String>,
    /// Characters of the skill listing it received, 0 without one.
    pub listing: usize,
    /// Characters of its task message.
    pub task: usize,
}

/// Tokens of a request's `usage`: input, cache writes and cache reads.
fn request_tokens(usage: &Value) -> u64 {
    [
        "input_tokens",
        "cache_creation_input_tokens",
        "cache_read_input_tokens",
    ]
    .iter()
    .map(|key| match usage.get(key) {
        // Token counts are whole and far below 2^53.
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        Value::Number(count) => *count as u64,
        _ => 0,
    })
    .sum()
}

/// The last component of a path.
fn file_name(path: &str) -> String {
    path.rsplit('/').next().unwrap_or(path).to_owned()
}

/// What a subagent started with, from its transcript and the `meta.json` beside it; `None` when
/// the transcript holds no answer yet.
#[must_use]
pub fn first_request(transcript: &str, meta: &Value) -> Option<Start> {
    let mut start = Start {
        agent_type: meta.get("agentType").text().to_owned(),
        description: meta.get("description").text().to_owned(),
        tokens: 0,
        instructions: Vec::new(),
        listing: 0,
        task: 0,
    };

    for line in transcript.lines().filter_map(json::parse) {
        match line.get("type").text() {
            "assistant" => {
                start.tokens = request_tokens(line.get("message").get("usage"));
                return Some(start);
            }
            "user" if start.task == 0 => {
                start.task = line.get("message").get("content").text().chars().count();
            }
            "attachment" => {
                let attachment = line.get("attachment");

                match attachment.get("type").text() {
                    "instructions" => start.instructions.extend(
                        attachment
                            .get("files")
                            .items()
                            .iter()
                            .map(|file| file_name(file.get("path").text())),
                    ),
                    "skill_listing" => {
                        start.listing = attachment.get("content").text().chars().count();
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
    None
}

/// The median of whole numbers, the lower middle for an even count, 0 for none.
#[must_use]
pub fn median(values: &[u64]) -> u64 {
    let mut sorted = values.to_vec();

    sorted.sort_unstable();
    sorted
        .get(sorted.len().saturating_sub(1) / 2)
        .copied()
        .unwrap_or(0)
}

/// One row per subagent, then one per type with its median, as Markdown tables.
#[must_use]
pub fn tables(starts: &[Start]) -> String {
    let mut text = String::from(
        "| Agent type | Description | First request, tokens | Instructions | Skill listing, chars | Task, chars |\n| --- | --- | --- | --- | --- | --- |\n",
    );

    for start in starts {
        let _ = writeln!(
            text,
            "| {} | {} | {} | {} | {} | {} |",
            start.agent_type,
            start.description.replace('|', "\\|"),
            start.tokens,
            start.instructions.join(", "),
            start.listing,
            start.task
        );
    }

    let mut types: Vec<&str> = starts
        .iter()
        .map(|start| start.agent_type.as_str())
        .collect();

    types.sort_unstable();
    types.dedup();
    text.push_str(
        "\n| Agent type | Agents | First request, median tokens |\n| --- | --- | --- |\n",
    );
    for agent_type in types {
        let tokens: Vec<u64> = starts
            .iter()
            .filter(|start| start.agent_type == agent_type)
            .map(|start| start.tokens)
            .collect();

        let _ = writeln!(
            text,
            "| {agent_type} | {} | {} |",
            tokens.len(),
            median(&tokens)
        );
    }
    text
}
