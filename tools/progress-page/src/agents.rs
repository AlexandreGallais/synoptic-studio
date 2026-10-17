//! The subagents of a run (EN-087): each one's transcript `agent-<id>.jsonl` and its
//! `agent-<id>.meta.json` under the session's `subagents/` folder (`REF-CC-HOOKS`, research note
//! 0024), and its tokens as `run-totals` counts them in `.git/metrics/run-cache.json` (EN-092).

use hooks::json::{Value, parse};
use hooks::time::epoch_ms;
use run_totals::tally::Tokens;
use run_totals::totals::Cache;

use crate::commits::story_of_reference;

/// A subagent idle for longer than this is taken as stopped, its end never written (a crash, a
/// session closed): 30 minutes.
pub const IDLE_MS: i64 = 30 * 60 * 1000;

/// A subagent of the run.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Agent {
    /// The id in its file name, `a1b26da4ea75d17df`.
    pub id: String,
    /// Its type, `story-worker`.
    pub kind: String,
    /// The description it was started with.
    pub description: String,
    /// The story its description names, if any.
    pub story: Option<String>,
    /// Its first line's time, epoch milliseconds.
    pub start: i64,
    /// Its transcript's last change, epoch milliseconds.
    pub last: i64,
    /// Whether its transcript does not show it done.
    pub running: bool,
    /// Its tokens in the run.
    pub tokens: Tokens,
}

/// The tokens the cache holds for the transcript named `name` (`agent-<id>.jsonl`), none when it
/// was not counted.
#[must_use]
pub fn tokens_of_file(cache: &Cache, name: &str) -> Tokens {
    cache
        .files
        .iter()
        .find(|(path, _)| path.rsplit(['/', '\\']).next() == Some(name))
        .map_or_else(Tokens::default, |(_, tally)| tally.tokens)
}

/// The first story id a description names, a word `EN-087` (capitals, a dash, digits).
#[must_use]
pub fn story_in(description: &str) -> Option<String> {
    description
        .split(|character: char| !(character.is_ascii_alphanumeric() || character == '-'))
        .find_map(story_of_reference)
}

/// The time of the first line that has one, in the start of a transcript; 0 when none.
#[must_use]
pub fn first_time(head: &str) -> i64 {
    head.lines()
        .filter_map(parse)
        .map(|line| epoch_ms(line.get("timestamp").text()))
        .find(|time| *time > 0)
        .unwrap_or(0)
}

/// Whether the end of a transcript shows its agent done: its last assistant line calls
/// `SubagentHandback` or ends the agent's turn. The lines after it (the hooks' attachments, the
/// call's result) do not count; a line cut by the read, or none, leaves it working.
#[must_use]
pub fn has_ended(tail: &str) -> bool {
    tail.lines()
        .rev()
        .filter_map(parse)
        .find(|line| line.get("type").text() == "assistant")
        .is_some_and(|line| {
            let message = line.get("message");

            message.get("stop_reason").text() == "end_turn"
                || message
                    .get("content")
                    .items()
                    .iter()
                    .any(|part| part.get("name").text() == "SubagentHandback")
        })
}

/// What a subagent's files give: `meta` its `.meta.json`, `head` and `tail` the start and the end
/// of its transcript, `last` the transcript's last change; its tokens are set by the caller.
#[must_use]
pub fn agent_of(id: &str, meta: &str, head: &str, tail: &str, last: i64) -> Agent {
    let meta = parse(meta).unwrap_or(Value::Null);
    let description = meta.get("description").text().to_owned();

    Agent {
        id: id.to_owned(),
        kind: meta.get("agentType").text().to_owned(),
        story: story_in(&description),
        description,
        start: first_time(head),
        last,
        running: !has_ended(tail),
        tokens: Tokens::default(),
    }
}

/// Whether `agent` is working at `now`: not done, and written within `IDLE_MS`.
#[must_use]
pub fn is_running(agent: &Agent, now: i64) -> bool {
    agent.running && now - agent.last <= IDLE_MS
}
