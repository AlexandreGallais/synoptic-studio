//! What the transcripts held when last read (EN-107), kept in `.git/metrics/transcripts-cache.jsonl`
//! so that the next report reads only what was written since, as `run-totals` keeps its counts
//! (EN-092): one line per transcript, its offset and its events. A transcript shorter than its
//! offset was rewritten, and is read again from its start; a cache of another version is dropped.

use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{Read, Seek, SeekFrom};
use std::path::{Path, PathBuf};

use hooks::json::quote;
use run_totals::tally::Tokens;
use serde_json::Value;

use crate::json::Json;
use crate::transcript::{Answer, Request, ToolUse, Transcript, read_chunk};

/// The cache's first line: a cache written by another version of this reader is not read.
const HEADER: &str = "{\"version\":1}";

/// The cache's file name, in `.git/metrics/`.
pub const CACHE_FILE: &str = "transcripts-cache.jsonl";

/// A request written as an array: id, model, branch, session, time, then its tokens by kind.
fn request_json(request: &Request) -> String {
    format!(
        "[{},{},{},{},{},{},{},{},{}]",
        quote(&request.id),
        quote(&request.model),
        quote(&request.branch),
        quote(&request.session),
        request.time,
        request.tokens.input,
        request.tokens.cache_write,
        request.tokens.cache_read,
        request.tokens.output
    )
}

/// A tool call written as an array: id, tool, summary, branch, session, start.
fn use_json(asked: &ToolUse) -> String {
    format!(
        "[{},{},{},{},{},{}]",
        quote(&asked.id),
        quote(&asked.tool),
        quote(&asked.summary),
        quote(&asked.branch),
        quote(&asked.session),
        asked.start
    )
}

/// A transcript's line of the cache.
#[must_use]
pub fn render_entry(path: &str, transcript: &Transcript) -> String {
    let list = |items: Vec<String>| items.join(",");

    format!(
        "{{\"path\":{},\"offset\":{},\"isWaiting\":{},\"turnStart\":{},\"requests\":[{}],\"uses\":[{}],\"answers\":[{}]}}",
        quote(path),
        transcript.offset,
        transcript.is_waiting,
        transcript.turn_start,
        list(transcript.requests.iter().map(request_json).collect()),
        list(transcript.uses.iter().map(use_json).collect()),
        list(
            transcript
                .answers
                .iter()
                .map(|(id, answer)| format!("[{},{},{}]", quote(id), answer.end, answer.is_ok))
                .collect()
        ),
    )
}

/// The text of an item of an array, empty when missing.
fn text_at(items: &[Value], at: usize) -> String {
    items.get(at).map_or("", Json::text).to_owned()
}

/// The whole number of an item of an array, 0 when missing.
fn whole_at(items: &[Value], at: usize) -> i64 {
    items.get(at).map_or(0, Json::whole)
}

/// A transcript read back from its line of the cache, with its path.
#[must_use]
pub fn parse_entry(line: &str) -> Option<(String, Transcript)> {
    let json: Value = serde_json::from_str(line).ok()?;
    let requests = json
        .at("requests")
        .items()
        .iter()
        .map(|item| {
            let fields = item.items();

            Request {
                id: text_at(fields, 0),
                model: text_at(fields, 1),
                branch: text_at(fields, 2),
                session: text_at(fields, 3),
                time: whole_at(fields, 4),
                tokens: Tokens {
                    input: whole_at(fields, 5),
                    cache_write: whole_at(fields, 6),
                    cache_read: whole_at(fields, 7),
                    output: whole_at(fields, 8),
                },
            }
        })
        .collect();
    let uses = json
        .at("uses")
        .items()
        .iter()
        .map(|item| {
            let fields = item.items();

            ToolUse {
                id: text_at(fields, 0),
                tool: text_at(fields, 1),
                summary: text_at(fields, 2),
                branch: text_at(fields, 3),
                session: text_at(fields, 4),
                start: whole_at(fields, 5),
            }
        })
        .collect();
    let answers = json
        .at("answers")
        .items()
        .iter()
        .map(|item| {
            let fields = item.items();

            (
                text_at(fields, 0),
                Answer {
                    end: whole_at(fields, 1),
                    is_ok: fields.get(2) == Some(&Value::Bool(true)),
                },
            )
        })
        .collect();

    Some((
        json.at("path").text().to_owned(),
        Transcript {
            offset: usize::try_from(json.at("offset").whole()).unwrap_or(0),
            requests,
            uses,
            answers,
            is_waiting: *json.at("isWaiting") != Value::Bool(false),
            turn_start: json.at("turnStart").whole(),
        },
    ))
}

/// The transcripts of a cache's text, by path; none when the text is not a cache of this version.
#[must_use]
pub fn parse_cache(text: &str) -> HashMap<String, Transcript> {
    let mut lines = text.lines();

    if lines.next() != Some(HEADER) {
        return HashMap::new();
    }

    lines.filter_map(parse_entry).collect()
}

/// A transcript once what was written after its offset is read; read again from its start when
/// the file is shorter than the offset; unchanged when the file cannot be read.
#[must_use]
pub fn read_file(path: &Path, known: Transcript) -> Transcript {
    let Ok(mut file) = File::open(path) else {
        return known;
    };
    let length = file
        .metadata()
        .map_or(0, |data| usize::try_from(data.len()).unwrap_or(0));
    let from = if length < known.offset {
        Transcript::default()
    } else {
        known
    };

    if length == from.offset {
        return from;
    }

    let mut bytes = Vec::new();

    if file
        .seek(SeekFrom::Start(u64::try_from(from.offset).unwrap_or(0)))
        .and_then(|_| file.read_to_end(&mut bytes))
        .is_err()
    {
        return from;
    }

    match std::str::from_utf8(&bytes) {
        Ok(text) => read_chunk(from, text),
        // A character cut at the end, still being written: its complete lines are read alike.
        Err(_) => read_chunk(from, &String::from_utf8_lossy(&bytes)),
    }
}

/// The transcripts of `paths`, each read from where the cache file says it was left, and the cache
/// written again when one moved on: its other transcripts kept while their file exists. A cache
/// that cannot be written leaves the result as it is.
#[must_use]
pub fn read_transcripts(paths: &[PathBuf], cache_file: &Path) -> Vec<Transcript> {
    let mut cache = parse_cache(&fs::read_to_string(cache_file).unwrap_or_default());
    let mut is_changed = false;
    let transcripts: Vec<Transcript> = paths
        .iter()
        .map(|path| {
            let key = path.to_string_lossy().into_owned();
            let known = cache.remove(&key).unwrap_or_default();
            let offset = known.offset;
            let read = read_file(path, known);

            is_changed |= read.offset != offset;
            cache.insert(key, read.clone());
            read
        })
        .collect();

    if is_changed {
        let mut entries: Vec<(&String, &Transcript)> = cache
            .iter()
            .filter(|(path, _)| Path::new(path).is_file())
            .collect();

        entries.sort_by(|left, right| left.0.cmp(right.0));

        let text: String = std::iter::once(HEADER.to_owned())
            .chain(
                entries
                    .iter()
                    .map(|(path, transcript)| render_entry(path, transcript)),
            )
            .map(|line| line + "\n")
            .collect();
        let temporary = cache_file.with_extension(format!("tmp-{}", std::process::id()));

        // The `metrics` folder is made inside an existing Git folder, never the Git folder itself.
        if let Some(folder) = cache_file
            .parent()
            .filter(|folder| folder.parent().is_some_and(Path::is_dir))
        {
            let _ = fs::create_dir_all(folder);
        }

        // Written aside then renamed, so that a report reading it at the same time sees a whole
        // cache; a cache not written is read again from the transcripts next time.
        if fs::write(&temporary, text).is_ok() && fs::rename(&temporary, cache_file).is_err() {
            let _ = fs::remove_file(&temporary);
        }
    }

    transcripts
}
