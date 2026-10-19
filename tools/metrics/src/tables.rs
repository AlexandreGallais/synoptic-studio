//! Statistics and Markdown tables of the metrics report (`docs/tooling/metrics.md`): commands
//! grouped by what they run, counted and timed.

use std::collections::HashMap;

use crate::format::format_duration;

/// A timed call, as the tables read it.
#[derive(Clone, Debug, PartialEq)]
pub struct TimedCall {
    /// How long it ran, milliseconds.
    pub duration_ms: f64,
    /// It did not fail.
    pub is_ok: bool,
    /// What it acted on.
    pub summary: String,
}

/// Statistics of one group of calls.
#[derive(Clone, Debug, PartialEq)]
pub struct CallStats {
    /// The group.
    pub key: String,
    /// Its calls.
    pub count: usize,
    /// Those that failed.
    pub failures: usize,
    /// The longest, milliseconds; 0 at least.
    pub max_ms: f64,
    /// The median, milliseconds.
    pub median_ms: f64,
    /// Their sum, milliseconds.
    pub total_ms: f64,
}

/// Groups of items by a key, in the order each key first came.
pub fn group_by<T, K: Fn(&T) -> String>(items: &[T], key_of: K) -> Vec<(String, Vec<&T>)> {
    let mut places: HashMap<String, usize> = HashMap::new();
    let mut groups: Vec<(String, Vec<&T>)> = Vec::new();

    for item in items {
        let key = key_of(item);

        if let Some(&at) = places.get(&key) {
            groups[at].1.push(item);
        } else {
            places.insert(key.clone(), groups.len());
            groups.push((key, vec![item]));
        }
    }
    groups
}

/// How many words name a command: `npm` takes three before `run` (`npm run check`) and two
/// otherwise (`npm install`), `gh` three (`gh pr checks`), `git`, `node` and `npx` two.
fn word_count(words: &[&str]) -> usize {
    match words.first().copied().unwrap_or_default() {
        "npm" if words.get(1) != Some(&"run") => 2,
        "gh" | "npm" => 3,
        "git" | "node" | "npx" => 2,
        _ => 1,
    }
}

/// Whether a word names part of a command: not a flag, a quoted text, a variable nor an
/// assignment.
fn is_command_word(word: &str) -> bool {
    word.chars()
        .next()
        .is_some_and(|first| !matches!(first, '-' | '"' | '\'' | '$'))
        && !word.contains('=')
}

/// What one segment of a shell command runs: its command words, empty for a directory change or
/// variables only.
fn segment_key(segment: &str) -> String {
    let words: Vec<&str> = segment
        .split(crate::transcript::is_js_space)
        .filter(|word| is_command_word(word))
        .collect();

    if words.first() == Some(&"cd") {
        return String::new();
    }

    words[..word_count(&words).min(words.len())].join(" ")
}

/// What a shell command runs, without its directory change, variables, flags and arguments: the
/// first segment between `&&`, `||`, `;` and line breaks that runs something, an `exit` after `||`
/// left out, since it runs only when the command before it fails: the brief's
/// `cd "<worktree>" || exit 9` names the command after it (EN-120).
#[must_use]
pub fn command_key(command: &str) -> String {
    command
        .replace("&&", "\n")
        .replace("||", "\n\u{0}")
        .replace(';', "\n")
        .split('\n')
        .filter_map(|segment| {
            let (is_after_or, text) = match segment.strip_prefix('\u{0}') {
                Some(rest) => (true, rest),
                None => (false, segment),
            };
            let key = segment_key(text.trim_matches(crate::transcript::is_js_space));

            (!(is_after_or && key == "exit")).then_some(key)
        })
        .find(|key| !key.is_empty())
        .unwrap_or_default()
}

/// The median of numbers in any order: the middle one, the mean of the two middle ones for an
/// even count, 0 for none.
#[must_use]
pub fn median(values: &[f64]) -> f64 {
    let mut sorted = values.to_vec();

    sorted.sort_by(f64::total_cmp);

    let middle = sorted.len() / 2;

    match sorted.len() {
        0 => 0.0,
        length if length % 2 == 1 => sorted[middle],
        _ => f64::midpoint(sorted[middle - 1], sorted[middle]),
    }
}

/// Statistics of calls grouped by a key, in the order each group first came.
pub fn call_stats<K: Fn(&TimedCall) -> String>(calls: &[TimedCall], key_of: K) -> Vec<CallStats> {
    group_by(calls, key_of)
        .into_iter()
        .map(|(key, group)| {
            let durations: Vec<f64> = group.iter().map(|call| call.duration_ms).collect();

            CallStats {
                key,
                count: group.len(),
                failures: group.iter().filter(|call| !call.is_ok).count(),
                max_ms: durations.iter().copied().fold(0.0, f64::max),
                median_ms: median(&durations),
                total_ms: durations.iter().sum(),
            }
        })
        .collect()
}

/// One row of a Markdown table, each `|` of a cell escaped.
fn table_row(cells: &[String]) -> String {
    format!(
        "| {} |",
        cells
            .iter()
            .map(|cell| cell.replace('|', "\\|"))
            .collect::<Vec<_>>()
            .join(" | ")
    )
}

/// A Markdown table with a header row.
#[must_use]
pub fn markdown_table(headers: &[&str], rows: &[Vec<String>]) -> String {
    let header: Vec<String> = headers.iter().map(|&cell| cell.to_owned()).collect();
    let rule: Vec<String> = headers.iter().map(|_| "---".to_owned()).collect();

    std::iter::once(table_row(&header))
        .chain(std::iter::once(table_row(&rule)))
        .chain(rows.iter().map(|row| table_row(row)))
        .collect::<Vec<_>>()
        .join("\n")
}

/// The table of call statistics: group, count, total, median, longest, failures.
#[must_use]
pub fn stats_table(stats: &[CallStats]) -> String {
    markdown_table(
        &["Command", "Count", "Total", "Median", "Longest", "Failed"],
        &stats
            .iter()
            .map(|entry| {
                vec![
                    format!("`{}`", entry.key),
                    entry.count.to_string(),
                    format_duration(entry.total_ms),
                    format_duration(entry.median_ms),
                    format_duration(entry.max_ms),
                    entry.failures.to_string(),
                ]
            })
            .collect::<Vec<_>>(),
    )
}

/// The first characters of a text as JavaScript's `slice(0, length)` counts them, in UTF-16 units:
/// a character cut in half is written as the replacement character, as Node.js writes a lone
/// surrogate.
#[must_use]
pub fn js_prefix(text: &str, length: usize) -> String {
    let mut units = 0;
    let mut prefix = String::new();

    for character in text.chars() {
        let size = character.len_utf16();

        if units + size > length {
            if units < length {
                prefix.push('\u{fffd}');
            }
            break;
        }
        units += size;
        prefix.push(character);
    }
    prefix
}
