//! What the transcripts of Claude Code hold that serves, extracted into the records rather than
//! the transcripts copied (E49, the move into the records): each session's first and last
//! instants and its tokens, each Bash command of the main thread and of its agents, and the
//! Product Owner's own messages, verbatim, which the private records may hold. A transcript is a
//! file of JSON lines, `<session>.jsonl` in the project's folder of Claude Code, its agents' in
//! `<session>/subagents/`; one unchanged since it was read, by its size and its time, is not read
//! again.

use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::UNIX_EPOCH;

use rusqlite::{Connection, OptionalExtension, params};
use serde_json::Value as Json;

use crate::moved::Moved;

/// What a transcript gives a session's row.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Session {
    pub id: String,
    /// Its first and last instants, ISO 8601 as the transcript writes them.
    pub start: String,
    pub end: String,
    /// The tokens its answers read, cached ones included, and wrote, each answer counted once.
    pub input_tokens: i64,
    pub output_tokens: i64,
}

/// A Bash command of a transcript.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Command {
    /// The tool use's id.
    pub id: String,
    pub time: String,
    pub command: String,
    pub description: String,
}

/// A message the Product Owner typed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Message {
    /// The line's `uuid`.
    pub id: String,
    pub time: String,
    pub text: String,
}

/// What one transcript holds that serves.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Extract {
    pub session: Session,
    pub commands: Vec<Command>,
    pub messages: Vec<Message>,
}

/// A text field of a JSON object, empty when missing.
fn text_of<'a>(value: &'a Json, key: &str) -> &'a str {
    value.get(key).and_then(Json::as_str).unwrap_or_default()
}

/// A whole number field of a JSON object, 0 when missing.
fn number_of(value: &Json, key: &str) -> i64 {
    value.get(key).and_then(Json::as_i64).unwrap_or(0)
}

/// The text a person typed in a user line, `None` for a tool's result, a command's output or a
/// message Claude Code wrote: a line of a human's origin when it names one, never a side chain
/// nor a meta line, its content a text that opens with no tag.
#[must_use]
pub fn typed_text(line: &Json) -> Option<String> {
    let origin = line
        .get("origin")
        .and_then(|origin| origin.get("kind"))
        .and_then(Json::as_str);

    if line.get("isSidechain").and_then(Json::as_bool) == Some(true)
        || line.get("isMeta").and_then(Json::as_bool) == Some(true)
        || origin.is_some_and(|kind| kind != "human")
    {
        return None;
    }

    let content = line.get("message")?.get("content")?;
    let text = match content {
        Json::String(text) => text.clone(),
        Json::Array(parts) => {
            if parts
                .iter()
                .any(|part| text_of(part, "type") == "tool_result")
            {
                return None;
            }
            parts
                .iter()
                .filter(|part| text_of(part, "type") == "text")
                .map(|part| text_of(part, "text"))
                .collect::<Vec<_>>()
                .join("\n")
        }
        _ => return None,
    };
    let trimmed = text.trim();

    (!trimmed.is_empty() && !trimmed.starts_with('<')).then(|| trimmed.to_owned())
}

/// What a transcript's text holds that serves: its session's instants and tokens, its Bash
/// commands, and the messages typed; a line that is no JSON skipped.
#[must_use]
pub fn extract(text: &str) -> Extract {
    let mut extract = Extract::default();
    let mut answered: HashSet<String> = HashSet::new();

    for line in text.lines() {
        let Ok(value) = serde_json::from_str::<Json>(line) else {
            continue;
        };
        let time = text_of(&value, "timestamp");

        if extract.session.id.is_empty() {
            text_of(&value, "sessionId").clone_into(&mut extract.session.id);
        }
        if !time.is_empty() {
            if extract.session.start.is_empty() || time < extract.session.start.as_str() {
                time.clone_into(&mut extract.session.start);
            }
            if time > extract.session.end.as_str() {
                time.clone_into(&mut extract.session.end);
            }
        }
        match text_of(&value, "type") {
            "user" => {
                if let Some(text) = typed_text(&value) {
                    extract.messages.push(Message {
                        id: text_of(&value, "uuid").to_owned(),
                        time: time.to_owned(),
                        text,
                    });
                }
            }
            "assistant" => {
                let Some(message) = value.get("message") else {
                    continue;
                };

                // An answer comes in several lines, one per part, each with the same usage.
                if let Some(usage) = message.get("usage")
                    && answered.insert(text_of(message, "id").to_owned())
                {
                    extract.session.input_tokens += number_of(usage, "input_tokens")
                        + number_of(usage, "cache_creation_input_tokens")
                        + number_of(usage, "cache_read_input_tokens");
                    extract.session.output_tokens += number_of(usage, "output_tokens");
                }
                for part in message
                    .get("content")
                    .and_then(Json::as_array)
                    .into_iter()
                    .flatten()
                {
                    if text_of(part, "type") == "tool_use" && text_of(part, "name") == "Bash" {
                        let input = part.get("input").unwrap_or(&Json::Null);

                        extract.commands.push(Command {
                            id: text_of(part, "id").to_owned(),
                            time: time.to_owned(),
                            command: text_of(input, "command").to_owned(),
                            description: text_of(input, "description").to_owned(),
                        });
                    }
                }
            }
            _ => {}
        }
    }
    extract
}

/// Milliseconds between two instants `YYYY-MM-DDTHH:MM:SS.sssZ`, 0 when either cannot be read.
#[must_use]
pub fn elapsed_ms(start: &str, end: &str) -> i64 {
    let instant = |text: &str| -> Option<i64> {
        let number = |from: usize, to: usize| text.get(from..to)?.parse::<i64>().ok();
        let days = calendar::days_from_civil(number(0, 4)?, number(5, 7)?, number(8, 10)?);
        let fraction = text.get(19..).unwrap_or_default();
        let milliseconds: String = fraction
            .trim_start_matches('.')
            .chars()
            .take_while(char::is_ascii_digit)
            .chain(std::iter::repeat('0'))
            .take(3)
            .collect();

        Some(
            days * 86_400_000
                + calendar::duration_ms(
                    number(11, 13)?,
                    number(14, 16)?,
                    number(17, 19)?,
                    milliseconds.parse().ok()?,
                ),
        )
    };

    match (instant(start), instant(end)) {
        (Some(start), Some(end)) => end - start,
        _ => 0,
    }
}

/// The transcripts of a project's folder of Claude Code: each session's file, then its agents'
/// files, by name.
#[must_use]
pub fn transcript_files(folder: &Path) -> Vec<(PathBuf, String)> {
    let sorted = |at: &Path| -> Vec<PathBuf> {
        let mut paths: Vec<PathBuf> = fs::read_dir(at)
            .map(|entries| entries.filter_map(Result::ok).map(|e| e.path()).collect())
            .unwrap_or_default();

        paths.sort();
        paths
    };
    let mut files = Vec::new();

    for path in sorted(folder) {
        let is_jsonl = path
            .extension()
            .is_some_and(|extension| extension == "jsonl");

        if path.is_file() && is_jsonl {
            files.push((path, "main".to_owned()));
        } else if path.is_dir() {
            for agent in sorted(&path.join("subagents")) {
                if agent
                    .extension()
                    .is_some_and(|extension| extension == "jsonl")
                {
                    let name = agent
                        .file_stem()
                        .map(|stem| stem.to_string_lossy().into_owned())
                        .unwrap_or_default();

                    files.push((agent, name));
                }
            }
        }
    }
    files
}

/// Whether a file is read already, its size and time unchanged; the two read.
fn is_unchanged(database: &Connection, path: &Path) -> rusqlite::Result<(bool, i64, i64)> {
    let metadata = fs::metadata(path).ok();
    let size = metadata
        .as_ref()
        .map_or(0, |data| i64::try_from(data.len()).unwrap_or(0));
    let modified = metadata
        .and_then(|data| data.modified().ok())
        .and_then(|time| time.duration_since(UNIX_EPOCH).ok())
        .map_or(0, |since| i64::try_from(since.as_millis()).unwrap_or(0));
    let held: Option<(i64, i64)> = database
        .query_row(
            "SELECT size, modified FROM transcript_files WHERE path = ?",
            [path.to_string_lossy()],
            |row| Ok((row.get(0)?, row.get(1)?)),
        )
        .optional()?;

    Ok((held == Some((size, modified)), size, modified))
}

/// Writes what a transcript holds: its session's row for a session's own file, its commands
/// under its agent's name, its messages.
fn write_extract(
    database: &Connection,
    extract: &Extract,
    agent: &str,
    session: &str,
) -> rusqlite::Result<()> {
    for command in &extract.commands {
        database.execute(
            "INSERT OR REPLACE INTO commands (id, session, agent, time, command, description)
            VALUES (?, ?, ?, ?, ?, ?)",
            params![
                command.id,
                session,
                agent,
                command.time,
                command.command,
                command.description
            ],
        )?;
    }
    if agent != "main" {
        return Ok(());
    }
    for message in &extract.messages {
        database.execute(
            "INSERT OR REPLACE INTO messages (id, session, time, text) VALUES (?, ?, ?, ?)",
            params![message.id, session, message.time, message.text],
        )?;
    }

    let row = &extract.session;

    database.execute(
        "INSERT OR REPLACE INTO sessions (id, start, end, wallMs, inputTokens, outputTokens,
          messages, commands)
        VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7,
          (SELECT count(*) FROM commands WHERE session = ?1))",
        params![
            session,
            row.start,
            row.end,
            elapsed_ms(&row.start, &row.end),
            row.input_tokens,
            row.output_tokens,
            i64::try_from(extract.messages.len()).unwrap_or(0)
        ],
    )?;
    Ok(())
}

/// Imports the transcripts of a project's folder of Claude Code, each file changed since it was
/// last read: the files read, and the commands and messages written.
///
/// # Errors
/// When SQLite fails.
pub fn import_transcripts(database: &Connection, folder: &Path) -> rusqlite::Result<Moved> {
    let mut moved = Moved::default();

    for (path, agent) in transcript_files(folder) {
        let (unchanged, size, modified) = is_unchanged(database, &path)?;

        if unchanged {
            continue;
        }

        let Ok(text) = fs::read_to_string(&path) else {
            continue;
        };
        let extract = extract(&text);
        // An agent's file names its session by its folder, `<session>/subagents/<agent>.jsonl`.
        let session = if agent == "main" {
            path.file_stem()
                .map(|stem| stem.to_string_lossy().into_owned())
                .unwrap_or_default()
        } else {
            path.parent()
                .and_then(Path::parent)
                .and_then(Path::file_name)
                .map(|name| name.to_string_lossy().into_owned())
                .unwrap_or_default()
        };

        write_extract(database, &extract, &agent, &session)?;
        database.execute(
            "INSERT OR REPLACE INTO transcript_files (path, size, modified) VALUES (?, ?, ?)",
            params![path.to_string_lossy(), size, modified],
        )?;
        moved.rows += extract.commands.len() + extract.messages.len();
        moved.documents += 1;
    }
    Ok(moved)
}
