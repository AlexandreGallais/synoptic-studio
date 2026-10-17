//! The tokens of transcripts, read incrementally (EN-092): each JSON Lines file is read from the
//! byte offset reached last time, its complete lines only, so that a refresh reads only what was
//! written since. A request written over several consecutive lines is counted once, by its message
//! id (`message.id`), with the `usage` of its last line: a subagent's lines carry a growing
//! `output_tokens`, its last line the final count (measured on 2026-10-17).

use hooks::json::{Value, parse};
use hooks::time::epoch_ms;

use crate::marker::whole;

/// Tokens by kind, as a request's `usage` gives them.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Tokens {
    /// `input_tokens`: read without the cache.
    pub input: i64,
    /// `cache_creation_input_tokens`: written to the cache.
    pub cache_write: i64,
    /// `cache_read_input_tokens`: read from the cache.
    pub cache_read: i64,
    /// `output_tokens`: written by the model.
    pub output: i64,
}

impl Tokens {
    /// Every kind together.
    #[must_use]
    pub fn total(&self) -> i64 {
        self.input + self.cache_write + self.cache_read + self.output
    }

    /// The sum of two tallies, kind by kind.
    #[must_use]
    pub fn plus(&self, other: &Tokens) -> Tokens {
        Tokens {
            input: self.input + other.input,
            cache_write: self.cache_write + other.cache_write,
            cache_read: self.cache_read + other.cache_read,
            output: self.output + other.output,
        }
    }

    /// The difference of two tallies, kind by kind.
    #[must_use]
    pub fn minus(&self, other: &Tokens) -> Tokens {
        Tokens {
            input: self.input - other.input,
            cache_write: self.cache_write - other.cache_write,
            cache_read: self.cache_read - other.cache_read,
            output: self.output - other.output,
        }
    }

    /// The tokens of a request's `usage`.
    #[must_use]
    pub fn of_usage(usage: &Value) -> Tokens {
        Tokens {
            input: whole(usage.get("input_tokens")),
            cache_write: whole(usage.get("cache_creation_input_tokens")),
            cache_read: whole(usage.get("cache_read_input_tokens")),
            output: whole(usage.get("output_tokens")),
        }
    }
}

/// What one transcript counted so far.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct FileTally {
    /// Bytes read: the end of the last complete line.
    pub offset: usize,
    /// Message id of the last request counted.
    pub last_id: String,
    /// Tokens of that request, as its latest line gives them.
    pub last: Tokens,
    /// Tokens of the requests counted.
    pub tokens: Tokens,
}

/// The time window counted, epoch milliseconds, both ends included.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Window {
    /// The run's start.
    pub from: i64,
    /// The run's end, or `i64::MAX` while it is open.
    pub to: i64,
}

/// The tally once the text written after `tally.offset` is read: its complete lines only, the
/// last one kept for the next reading when no line break ends it yet.
#[must_use]
pub fn read_chunk(tally: &FileTally, chunk: &str, window: Window) -> FileTally {
    let complete = chunk.rfind('\n').map_or(0, |at| at + 1);
    let mut next = FileTally {
        offset: tally.offset + complete,
        ..tally.clone()
    };

    // Only the model's answers carry a `usage`: the other lines are not parsed.
    for line in chunk[..complete]
        .lines()
        .filter(|line| line.contains("\"type\":\"assistant\""))
    {
        let Some(json) = parse(line) else { continue };

        if json.get("type").text() != "assistant" {
            continue;
        }

        let message = json.get("message");
        let id = message.get("id").text();
        let time = epoch_ms(json.get("timestamp").text());

        let usage = Tokens::of_usage(message.get("usage"));

        // A later line of the request counted last replaces its usage.
        if !id.is_empty() && id == next.last_id {
            next.tokens = next.tokens.minus(&next.last).plus(&usage);
            next.last = usage;
            continue;
        }

        if id.is_empty() || time < window.from || time > window.to {
            continue;
        }

        id.clone_into(&mut next.last_id);
        next.last = usage;
        next.tokens = next.tokens.plus(&usage);
    }

    next
}
