//! The limit of subagents (ADR-0072): a fourth one at once is refused, naming those running.

use crate::json::quote;
use crate::tasks::{Kind, Task, agent_names};
use crate::time::short_duration;

/// At most this many subagents at once.
pub const AGENT_LIMIT: usize = 3;

/// The subagents running before a call: those started earlier, the call itself and any started
/// after it in the same message left out.
#[must_use]
pub fn running_before<'a>(pending: &'a [Task], id: &str) -> Vec<&'a Task> {
    let agents = pending.iter().filter(|task| task.kind == Kind::Agent);

    match pending.iter().position(|task| task.id == id) {
        Some(index) => agents
            .filter(|task| pending[..index].contains(task))
            .collect(),
        None => agents.collect(),
    }
}

/// Why a new subagent is refused, `None` when fewer than `limit` run.
#[must_use]
pub fn refusal(running: &[&Task], limit: usize, now: i64) -> Option<String> {
    if running.len() < limit {
        return None;
    }

    let names: Vec<String> = running
        .iter()
        .map(|task| format!("{} ({})", task.label, short_duration(now - task.start)))
        .collect();

    Some(format!(
        "At most {limit} subagents at once (ADR-0072): {} running — {}. Wait for one to end, then start or resume this one.",
        running.len(),
        names.join("; ")
    ))
}

/// The hook's answer refusing a tool call (Claude Code's `PreToolUse` decision).
#[must_use]
pub fn deny_output(reason: &str) -> String {
    format!(
        "{{\"hookSpecificOutput\":{{\"hookEventName\":\"PreToolUse\",\"permissionDecision\":\"deny\",\"permissionDecisionReason\":{}}}}}",
        quote(reason)
    )
}

/// Whether a `SendMessage` goes to the main conversation or to one of the subagents running, read
/// from the transcript's lines: queued for a subagent that runs, it starts none (ADR-0072, EN-063).
#[must_use]
pub fn reaches_running(to: &str, running: &[&Task], lines: &str) -> bool {
    let ids: Vec<&str> = running.iter().map(|task| task.id.as_str()).collect();

    to == "main" || agent_names(lines, &ids).iter().any(|name| name == to)
}
