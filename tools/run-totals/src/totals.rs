//! A run's totals from its sessions' transcripts (EN-092, research note 0024): each session's
//! `<projects>/<session>.jsonl` and its subagents' `<projects>/<session>/subagents/*.jsonl`, read
//! from the offsets of a cache (`.git/metrics/run-cache.json`) that belongs to one run's start and
//! end: a run that ends is counted once more from the start, so that nothing after its end stays.

use std::fs::{self, File};
use std::io::{Read, Seek, SeekFrom};
use std::path::{Path, PathBuf};

use hooks::json::{parse, quote};

use crate::marker::{Run, whole};
use crate::tally::{FileTally, Tokens, Window, read_chunk};

/// What the cache holds: the run's start and end it belongs to, and each transcript's tally.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Cache {
    /// Start of the run counted, epoch milliseconds.
    pub start: i64,
    /// End of the run counted, 0 while it was open.
    pub end: i64,
    /// Each transcript read, with its tally.
    pub files: Vec<(String, FileTally)>,
}

/// Reads a cache, empty when the text is not one or belongs to another run, or to the same run
/// before it ended.
#[must_use]
pub fn parse_cache(text: &str, run: &Run) -> Cache {
    let empty = Cache {
        start: run.start,
        end: run.end,
        files: Vec::new(),
    };
    let Some(json) = parse(text) else {
        return empty;
    };

    if whole(json.get("start")) != run.start || whole(json.get("end")) != run.end {
        return empty;
    }

    let files = json
        .get("files")
        .items()
        .iter()
        .map(|file| {
            (
                file.get("path").text().to_owned(),
                FileTally {
                    offset: usize::try_from(whole(file.get("offset"))).unwrap_or(0),
                    last_id: file.get("lastId").text().to_owned(),
                    last: Tokens::of_usage(file.get("last")),
                    tokens: Tokens {
                        input: whole(file.get("input")),
                        cache_write: whole(file.get("cacheWrite")),
                        cache_read: whole(file.get("cacheRead")),
                        output: whole(file.get("output")),
                    },
                },
            )
        })
        .collect();

    Cache { files, ..empty }
}

/// Writes a cache as JSON, one line.
#[must_use]
pub fn render_cache(cache: &Cache) -> String {
    let files: Vec<String> = cache
        .files
        .iter()
        .map(|(path, tally)| {
            format!(
                "{{\"path\":{},\"offset\":{},\"lastId\":{},\"last\":{},\"input\":{},\"cacheWrite\":{},\"cacheRead\":{},\"output\":{}}}",
                quote(path),
                tally.offset,
                quote(&tally.last_id),
                usage_json(&tally.last),
                tally.tokens.input,
                tally.tokens.cache_write,
                tally.tokens.cache_read,
                tally.tokens.output
            )
        })
        .collect();

    format!(
        "{{\"start\":{},\"end\":{},\"files\":[{}]}}\n",
        cache.start,
        cache.end,
        files.join(",")
    )
}

/// A request's tokens written as its `usage`.
fn usage_json(tokens: &Tokens) -> String {
    format!(
        "{{\"input_tokens\":{},\"cache_creation_input_tokens\":{},\"cache_read_input_tokens\":{},\"output_tokens\":{}}}",
        tokens.input, tokens.cache_write, tokens.cache_read, tokens.output
    )
}

/// The transcripts of a run's sessions: each session's own, then its subagents', sorted.
#[must_use]
pub fn transcripts(projects: &Path, sessions: &[String]) -> Vec<PathBuf> {
    sessions
        .iter()
        .flat_map(|session| {
            let main = projects.join(format!("{session}.jsonl"));
            let mut subagents: Vec<PathBuf> =
                fs::read_dir(projects.join(session).join("subagents"))
                    .map(|entries| {
                        entries
                            .filter_map(Result::ok)
                            .map(|entry| entry.path())
                            .filter(|path| path.extension().is_some_and(|ext| ext == "jsonl"))
                            .collect()
                    })
                    .unwrap_or_default();

            subagents.sort();
            std::iter::once(main)
                .filter(|path| path.is_file())
                .chain(subagents)
        })
        .collect()
}

/// A transcript's tally once what was written after its offset is read; read again from the start
/// when the file is shorter than the offset (rewritten).
fn read_file(path: &Path, tally: &FileTally, window: Window) -> FileTally {
    let Ok(mut file) = File::open(path) else {
        return tally.clone();
    };
    let length = file
        .metadata()
        .map_or(0, |data| usize::try_from(data.len()).unwrap_or(0));
    let from = if length < tally.offset {
        FileTally::default()
    } else {
        tally.clone()
    };
    let mut bytes = Vec::new();

    if file
        .seek(SeekFrom::Start(u64::try_from(from.offset).unwrap_or(0)))
        .and_then(|_| file.read_to_end(&mut bytes))
        .is_err()
    {
        return tally.clone();
    }

    read_chunk(&from, &String::from_utf8_lossy(&bytes), window)
}

/// The run's tokens, every transcript summed, and the cache to keep for the next count.
#[must_use]
pub fn run_tokens(run: &Run, projects: &Path, cache: &Cache) -> (Tokens, Cache) {
    let window = Window {
        from: run.start,
        to: if run.is_open() { i64::MAX } else { run.end },
    };
    let files: Vec<(String, FileTally)> = transcripts(projects, &run.sessions)
        .iter()
        .map(|path| {
            let key = path.to_string_lossy().into_owned();
            let known = cache
                .files
                .iter()
                .find(|(name, _)| *name == key)
                .map(|(_, tally)| tally.clone())
                .unwrap_or_default();

            (key, read_file(path, &known, window))
        })
        .collect();
    let tokens = files
        .iter()
        .fold(Tokens::default(), |sum, (_, tally)| sum.plus(&tally.tokens));

    (
        tokens,
        Cache {
            start: run.start,
            end: run.end,
            files,
        },
    )
}

/// Claude Code's transcripts folder of a repository: `<home>/.claude/projects/` and the path of
/// its main checkout, the folder holding `common` (the Git directory every worktree shares), every
/// character other than an ASCII letter or digit written `-`.
#[must_use]
pub fn projects_dir(home: &Path, common: &Path) -> PathBuf {
    let name: String = common
        .parent()
        .unwrap_or(common)
        .to_string_lossy()
        .chars()
        .map(|character| {
            if character.is_ascii_alphanumeric() {
                character
            } else {
                '-'
            }
        })
        .collect();

    home.join(".claude").join("projects").join(name)
}

/// The run's tokens, `metrics/run-cache.json` read before and written after when it changed.
#[must_use]
pub fn count_run(run: &Run, metrics: &Path, projects: &Path) -> Tokens {
    let cache_path = metrics.join("run-cache.json");
    let cache = parse_cache(&fs::read_to_string(&cache_path).unwrap_or_default(), run);
    let (tokens, next) = run_tokens(run, projects, &cache);

    if next != cache {
        let _ = fs::write(&cache_path, render_cache(&next));
    }

    tokens
}
