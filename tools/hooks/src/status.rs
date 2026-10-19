//! The status line of Claude Code (`docs/tooling/metrics.md`, `REF-CC-STATUSLINE`): where the work
//! stands (epic, feature, story, step); the current turn, from the person's prompt (active or
//! silent, time, exact tokens), the run's duration and tokens while a `/run` is open (EN-092), how
//! full the conversation memory is and the model; the tool call running, if any; one line per
//! background task. Rewritten from `.claude/hooks/status-line.ts`, `status-place.ts`, `status-run.ts`
//! and `status-usage.ts` (EN-104), the same text byte for byte.

use crate::json::{self, Value, number_text, quote};
use calendar::{MINUTE_MS, SECOND_MS};

use crate::record::{js_prefix, text_or};
use crate::tasks::{Kind, Task};
use crate::time::short_duration;
use crate::tokens::{Tally, whole};

/// Silence after which the session is shown as silent (2 minutes).
const IDLE_MS: i64 = 120_000;

/// A call running longer than this, milliseconds, is shown in red: look at what it does.
pub const LONG_CALL_MS: i64 = 300_000;

/// Characters of a running command shown.
const COMMAND_SHOWN: usize = 60;

/// ANSI colors of the status line.
const BOLD: &str = "\u{1b}[1m";
const CYAN: &str = "\u{1b}[36m";
const DIM: &str = "\u{1b}[2m";
const GREEN: &str = "\u{1b}[32m";
const MAGENTA: &str = "\u{1b}[35m";
const RED: &str = "\u{1b}[31m";
const RESET: &str = "\u{1b}[0m";
const YELLOW: &str = "\u{1b}[33m";

/// The story ids as they appear in branch names.
const STORY_PREFIXES: [&str; 9] = ["us", "en", "sp", "chk", "aud", "val", "nam", "rev", "ret"];

/// A duration to the second, so that the turn's clock moves at each refresh: `45 s`,
/// `12 min 05 s` or `2 h 05 min 09 s`.
#[must_use]
pub fn clock_duration(milliseconds: i64) -> String {
    let seconds = milliseconds.div_euclid(SECOND_MS);
    let minutes = milliseconds.div_euclid(MINUTE_MS);
    let seconds_text = format!("{:02} s", seconds % 60);

    if minutes < 1 {
        format!("{seconds} s")
    } else if minutes < 60 {
        format!("{minutes} min {seconds_text}")
    } else {
        format!("{} h {:02} min {seconds_text}", minutes / 60, minutes % 60)
    }
}

/// A whole number with its thousands separated by commas, as `toLocaleString("en-US")`.
#[must_use]
pub fn grouped(number: i64) -> String {
    let digits = number.unsigned_abs().to_string();
    let mut text = String::new();

    for (index, digit) in digits.chars().enumerate() {
        if index > 0 && (digits.len() - index).is_multiple_of(3) {
            text.push(',');
        }
        text.push(digit);
    }
    if number < 0 { format!("-{text}") } else { text }
}

/// Whether the person's turn has come: the agent ended its turn and waits for no background task.
#[must_use]
pub fn is_turn_done(tally: &Tally) -> bool {
    tally.is_waiting && tally.start != 0 && tally.pending.is_empty()
}

/// A number of things, the noun in the plural when needed: `1 agent`, `2 agents`.
fn counted(amount: usize, noun: &str) -> String {
    format!("{amount} {noun}{}", if amount == 1 { "" } else { "s" })
}

/// The background tasks in a few words, `2 agents, 1 command`, empty when nothing runs there.
#[must_use]
pub fn background_text(tally: &Tally) -> String {
    let of = |kind: Kind| {
        tally
            .pending
            .iter()
            .filter(|task| task.kind == kind)
            .count()
    };
    let parts: Vec<String> = [(of(Kind::Agent), "agent"), (of(Kind::Command), "command")]
        .into_iter()
        .filter(|(amount, _)| *amount > 0)
        .map(|(amount, noun)| counted(amount, noun))
        .collect();

    parts.join(", ")
}

/// What the status line shows.
#[derive(Clone, Debug, PartialEq)]
pub struct Status {
    /// One line per background task: subagent or command, what it does now.
    pub background: Vec<String>,
    /// Share of the context window used, in percent.
    pub context_percent: f64,
    /// Time since the current turn started.
    pub duration_ms: i64,
    /// Name of the model.
    pub model: String,
    /// Epic, feature and story of the branch checked out.
    pub place: String,
    /// The open run's part, `run 2 h 05 · 48,123,456 tokens`, empty outside a run (EN-092).
    pub run: String,
    /// Whether the call running has run for more than 5 minutes: shown in red, to be checked.
    pub is_running_long: bool,
    /// Line of the tool call running, possibly empty.
    pub running: String,
    /// Time since the transcript last changed.
    pub silent_ms: i64,
    /// What the agent says it is doing, possibly empty.
    pub step: String,
    /// Exact tokens of the session.
    pub tally: Tally,
}

/// Activity, in four states: the agent works on the main thread (with or without tasks in the
/// background), or its turn is over and only background tasks run (the chat is free for the
/// person's messages); a working agent silent for a while is shown as such.
fn activity(silent_ms: i64, tally: &Tally) -> String {
    let background = background_text(tally);

    if !background.is_empty() && tally.is_waiting {
        return format!("{CYAN}◌ chat free · {background} in the background{RESET}");
    }

    let also = if background.is_empty() {
        String::new()
    } else {
        format!(" · {background} in the background")
    };

    if silent_ms < IDLE_MS {
        format!("{GREEN}● working{also}{RESET}")
    } else {
        format!(
            "{YELLOW}◌ silent for {}{also}{RESET}",
            short_duration(silent_ms)
        )
    }
}

/// The separator between the parts of a line.
fn separator() -> String {
    format!("{DIM} · {RESET}")
}

/// The turn's line: activity, time, tokens, the run, how full the conversation memory is, model.
fn session_line(status: &Status) -> String {
    let tally = &status.tally;
    let is_done = is_turn_done(tally);
    let mut parts = vec![
        if is_done {
            format!("{DIM}◌ your turn{RESET}")
        } else {
            activity(status.silent_ms, tally)
        },
        format!(
            "⏱ {} {}",
            if is_done { "last turn" } else { "turn" },
            clock_duration(status.duration_ms)
        ),
        format!(
            "{} tokens this turn ({} written)",
            grouped(tally.total),
            grouped(tally.output)
        ),
    ];

    if !status.run.is_empty() {
        parts.push(format!("{MAGENTA}{}{RESET}", status.run));
    }
    parts.push(format!(
        "conversation memory {} % full",
        number_text((status.context_percent + 0.5).floor())
    ));
    parts.push(format!("{DIM}{}{RESET}", status.model));
    parts.join(&separator())
}

/// The status line's text: place and step, session, the main thread's running call, then one line
/// per background task, with ANSI colors.
#[must_use]
pub fn status_text(status: &Status) -> String {
    let place = format!("{BOLD}{}{RESET}", status.place);
    // No step written: the place alone, never a word that reads as a state ("waiting", RET-003).
    let head = if status.step.is_empty() {
        place
    } else {
        format!("{place}{}{}", separator(), status.step)
    };
    let mut lines = vec![head, session_line(status)];

    if !status.running.is_empty() {
        let color = if status.is_running_long { RED } else { CYAN };

        lines.push(format!("{color}{}{RESET}", status.running));
    }
    lines.extend(
        status
            .background
            .iter()
            .map(|line| format!("{DIM}{line}{RESET}")),
    );
    lines.join("\n")
}

/// The session's state that each tool call's record copies (`now.json`, read by the
/// `log-tool-call` hook): written at each refresh, so that reading it costs the hook nothing.
#[must_use]
pub fn session_snapshot(status: &Status, now: i64) -> String {
    let of = |kind: Kind| {
        status
            .tally
            .pending
            .iter()
            .filter(|task| task.kind == kind)
            .count()
    };

    format!(
        "{{\"contextPercent\":{},\"isWaiting\":{},\"model\":{},\"pendingAgents\":{},\"pendingCommands\":{},\"time\":{now},\"turnOutputTokens\":{},\"turnStart\":{},\"turnTokens\":{}}}",
        number_text(status.context_percent),
        status.tally.is_waiting,
        quote(&status.model),
        of(Kind::Agent),
        of(Kind::Command),
        status.tally.output,
        status.tally.start,
        status.tally.total
    )
}

/// The story id a branch names, `\b(us|en|…)-\d{3}\b`, the first one.
fn story_of(branch: &str) -> Option<&str> {
    let bytes = branch.as_bytes();
    let is_word = |at: usize| {
        bytes
            .get(at)
            .is_some_and(|byte| byte.is_ascii_alphanumeric() || *byte == b'_')
    };

    (0..bytes.len()).find_map(|start| {
        if start > 0 && is_word(start - 1) {
            return None;
        }
        STORY_PREFIXES.iter().find_map(|prefix| {
            let digits = start + prefix.len() + 1;
            let end = digits + 3;
            let matches = branch[start..].starts_with(prefix)
                && bytes.get(digits - 1) == Some(&b'-')
                && bytes
                    .get(digits..end)
                    .is_some_and(|run| run.iter().all(u8::is_ascii_digit))
                && !is_word(end);

            matches.then(|| &branch[start..end])
        })
    })
}

/// The backlog item a branch works on: `US-018`, `F04`, or empty.
fn item_of(branch: &str) -> String {
    let feature = branch.strip_prefix("feature/").and_then(|rest| {
        let bytes = rest.as_bytes();

        (bytes.len() >= 4
            && bytes[0] == b'f'
            && bytes[1..3].iter().all(u8::is_ascii_digit)
            && bytes[3] == b'-')
            .then(|| &rest[..3])
    });

    story_of(branch)
        .or(feature)
        .unwrap_or_default()
        .to_uppercase()
}

/// `^E\d{2}-(?:F\d{2}-)?(?:[A-Z]+-\d{3}-)?(.+)\.md$`, every way it matches in the order the
/// expression tries them: the epic, the feature when there is one, the slug.
fn backlog_name(file: &str) -> Option<(&str, Option<&str>, &str)> {
    let bytes = file.as_bytes();
    let two_digits = |at: usize| {
        bytes
            .get(at..at + 2)
            .is_some_and(|run| run.iter().all(u8::is_ascii_digit))
    };

    if !(bytes.first() == Some(&b'E') && two_digits(1) && bytes.get(3) == Some(&b'-')) {
        return None;
    }
    let feature =
        (bytes.get(4) == Some(&b'F') && two_digits(5) && bytes.get(7) == Some(&b'-')).then_some(8);
    let story_after = |at: usize| {
        let capitals = bytes[at..]
            .iter()
            .take_while(|byte| byte.is_ascii_uppercase())
            .count();
        let digits = at + capitals + 1;

        (capitals > 0
            && bytes.get(digits - 1) == Some(&b'-')
            && bytes
                .get(digits..digits + 3)
                .is_some_and(|run| run.iter().all(u8::is_ascii_digit))
            && bytes.get(digits + 3) == Some(&b'-'))
        .then_some(digits + 4)
    };
    let slug = |at: usize| {
        let rest = file.get(at..)?.strip_suffix(".md")?;

        (!rest.is_empty() && !rest.contains(['\n', '\r', '\u{2028}', '\u{2029}'])).then_some(rest)
    };
    let mut tries = Vec::new();

    if let Some(after) = feature {
        tries.extend(story_after(after).map(|end| (true, end)));
        tries.push((true, after));
    }
    tries.extend(story_after(4).map(|end| (false, end)));
    tries.push((false, 4));

    tries.into_iter().find_map(|(with_feature, at)| {
        slug(at).map(|found| (&file[..3], with_feature.then(|| &file[4..7]), found))
    })
}

/// Epic, feature and story of a branch, from the names of the backlog files:
/// `E01 › F04 › US-018 turn a shape`, the branch itself when it names no item.
#[must_use]
pub fn place_of(branch: &str, files: &[String]) -> String {
    let id = item_of(branch);
    let file = files
        .iter()
        .find(|name| !id.is_empty() && name.contains(&format!("-{id}-")))
        .map_or("", String::as_str);
    let Some((epic, feature, slug)) = backlog_name(file) else {
        return branch.to_owned();
    };
    let item = format!("{id} {}", slug.replace('-', " "));
    let mut parts = vec![epic];

    parts.extend(feature);
    if item != format!("{id} ") {
        parts.push(&item);
    }
    parts.join(" › ")
}

/// A tool call started and not finished yet, as the `track-tool-call` hook writes it.
#[derive(Clone, Debug, PartialEq)]
pub struct RunningCall {
    /// Who runs it: `main`, or the type of the subagent.
    pub agent: String,
    /// The subagent's id, empty for the main agent and in a file written before EN-120.
    pub agent_id: String,
    pub description: String,
    /// The session's id, empty in a file written before EN-120.
    pub session: String,
    /// When it started, epoch milliseconds, a whole number as the hook writes it.
    pub start: i64,
    pub summary: String,
    pub tool: String,
}

/// A running call read back from its file: empty or 0 where the file lacks a field.
#[must_use]
pub fn read_running_call(record: &Value) -> RunningCall {
    RunningCall {
        agent: text_or(record, "agent", "main").to_owned(),
        agent_id: String::new(),
        description: record.get("description").text().to_owned(),
        session: String::new(),
        start: whole(record, "start"),
        summary: record.get("summary").text().to_owned(),
        tool: record.get("tool").text().to_owned(),
    }
}

/// The files of the calls left by subagents that ended (EN-120): a subagent's call of this session,
/// or of a file that names none, started before every subagent still running started, so that no
/// subagent running can have made it; the main agent's calls are never among them.
#[must_use]
pub fn ended_calls(calls: &[(String, RunningCall)], tasks: &[Task], session: &str) -> Vec<String> {
    let _ = (calls, tasks, session);
    Vec::new()
}

/// A call in a few words, its tool and its description or the start of its command:
/// `Bash · Run the checks`.
fn call_text(call: &RunningCall) -> String {
    let what = if call.description.is_empty() {
        js_prefix(&call.summary, COMMAND_SHOWN)
    } else {
        &call.description
    };

    format!("{} · {what}", call.tool)
}

/// The line of the oldest call still running, `▶ Bash · Run the checks — 12 s`, empty when
/// nothing runs.
#[must_use]
pub fn running_line(calls: &[RunningCall], now: i64, duration: fn(i64) -> String) -> String {
    let oldest = calls.iter().reduce(|oldest, call| {
        if call.start < oldest.start {
            call
        } else {
            oldest
        }
    });

    oldest.map_or_else(String::new, |call| {
        format!("▶ {} — {}", call_text(call), duration(now - call.start))
    })
}

/// One line per background task: what it is, for how long it has run, and, for a subagent, the
/// call it is making now (the latest one started by a subagent of its type); a message whose
/// result is still to come is left out.
#[must_use]
pub fn background_lines(
    tasks: &[Task],
    calls: &[RunningCall],
    now: i64,
    duration: fn(i64) -> String,
) -> Vec<String> {
    tasks
        .iter()
        .filter(|task| task.kind != Kind::Message)
        .map(|task| {
            let kind = task.label.split(':').next().unwrap_or_default();
            let current = calls
                .iter()
                .filter(|call| task.kind == Kind::Agent && call.agent == kind)
                .reduce(|latest, call| {
                    if call.start > latest.start {
                        call
                    } else {
                        latest
                    }
                });
            let doing =
                current.map_or_else(String::new, |call| format!(" · now {}", call_text(call)));

            format!(
                "↳ {} {} — {}{doing}",
                task.kind.name(),
                task.label,
                duration(now - task.start)
            )
        })
        .collect()
}

/// Whether the run's marker holds a run still open (`end` 0, a numeric `start`), as `run-totals
/// start` writes it.
#[must_use]
pub fn is_run_open(marker: &str) -> bool {
    json::parse(marker).is_some_and(|run| {
        run.get("end") == &Value::Number(0.0) && matches!(run.get("start"), Value::Number(_))
    })
}

/// One window of `rate_limits`, `{"resetsAt":…,"usedPercent":…}`, `None` when Claude Code did not
/// send it whole.
fn usage_window(limits: &Value, key: &str) -> Option<String> {
    let window = limits.get(key);

    match (window.get("used_percentage"), window.get("resets_at")) {
        // Seconds since the epoch, a whole number.
        (Value::Number(used), Value::Number(_)) => Some(format!(
            "{{\"resetsAt\":\"{}\",\"usedPercent\":{}}}",
            calendar::temporal_instant_ms(whole(window, "resets_at") * SECOND_MS),
            number_text(*used)
        )),
        _ => None,
    }
}

/// The subscription's usage that a run reads to stop in time (`.git/metrics/usage.json`, EN-092):
/// the five-hour and weekly windows of `rate_limits`, sent to Pro and Max subscribers after the
/// session's first response; `None` when neither was sent, so that the last file is kept.
#[must_use]
pub fn usage_snapshot(input: &Value, now: i64) -> Option<String> {
    let limits = input.get("rate_limits");
    let windows: Vec<String> = [("fiveHour", "five_hour"), ("sevenDay", "seven_day")]
        .into_iter()
        .filter_map(|(name, key)| {
            usage_window(limits, key).map(|window| format!("\"{name}\":{window}"))
        })
        .collect();

    (!windows.is_empty()).then(|| format!("{{{},\"time\":{now}}}", windows.join(",")))
}
