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

/// One segment of a command line: whether `||` opened it, and its words, each quoted text kept
/// whole with its quotes.
type Segment = (bool, Vec<String>);

/// The segments of a command line, split at `&&`, `||`, `;` and line breaks outside quotes, into
/// words split at spaces outside quotes; a backslash keeps the next character (REV-009: a `;` of
/// a quoted message named `the` and `to`).
fn segments(command: &str) -> Vec<Segment> {
    let mut found: Vec<Segment> = vec![(false, Vec::new())];
    let mut word = String::new();
    let mut quote: Option<char> = None;
    let mut characters = command.chars().peekable();
    let end_word = |found: &mut Vec<Segment>, word: &mut String| {
        if !word.is_empty()
            && let Some((_, words)) = found.last_mut()
        {
            words.push(std::mem::take(word));
        }
    };

    while let Some(character) = characters.next() {
        match (quote, character) {
            (Some(open), _) if character == open => {
                word.push(character);
                quote = None;
            }
            (None, '"' | '\'') => {
                word.push(character);
                quote = Some(character);
            }
            (_, '\\') => {
                word.push(character);
                word.extend(characters.next());
            }

            (None, '\n' | ';') => {
                end_word(&mut found, &mut word);
                found.push((false, Vec::new()));
            }
            (None, '&' | '|') if characters.peek() == Some(&character) => {
                characters.next();
                end_word(&mut found, &mut word);
                found.push((character == '|', Vec::new()));
            }
            (None, space) if crate::transcript::is_js_space(space) => {
                end_word(&mut found, &mut word);
            }
            // Inside quotes every character is the word's, a separator included.
            _ => word.push(character),
        }
    }
    end_word(&mut found, &mut word);
    found
}

/// What one segment runs: its command words, empty for a directory change, a comment or
/// variables only; a negation `!` and the assignments before the program left out, and after
/// `||` an `exit` and its code, which run only when the command before fails: the words after
/// them are the next command, of a summary whose line break became a space (REV-009).
fn segment_key(is_after_or: bool, words: &[String]) -> String {
    let mut words: Vec<&str> = words
        .iter()
        .map(String::as_str)
        .skip_while(|word| *word == "!" || (word.contains('=') && !word.starts_with('-')))
        .collect();

    if is_after_or && words.first() == Some(&"exit") {
        let code = words
            .get(1)
            .is_some_and(|code| code.bytes().all(|byte| byte.is_ascii_digit()));
        words.drain(..if code { 2 } else { 1 });
        return segment_key(
            false,
            &words
                .iter()
                .map(|&word| word.to_owned())
                .collect::<Vec<_>>(),
        );
    }
    if words
        .first()
        .is_none_or(|first| *first == "cd" || first.starts_with('#'))
    {
        return String::new();
    }

    let words: Vec<&str> = words
        .into_iter()
        .filter(|word| is_command_word(word))
        .collect();

    words[..word_count(&words).min(words.len())].join(" ")
}

/// What a shell command runs, without its directory change, variables, flags and arguments: the
/// first segment between `&&`, `||`, `;` and line breaks that runs something, an `exit` after `||`
/// left out, since it runs only when the command before it fails: the brief's
/// `cd "<worktree>" || exit 9` names the command after it (EN-120), on the same line too.
#[must_use]
pub fn command_key(command: &str) -> String {
    segments(command)
        .iter()
        .map(|(is_after_or, words)| segment_key(*is_after_or, words))
        .find(|key| !key.is_empty())
        .unwrap_or_default()
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
                median_ms: tool_use::median(&durations).unwrap_or(0.0),
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
