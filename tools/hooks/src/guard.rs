//! The shell guard (ADR-0021), the `guard-bash` hook's judgement: the commands the project forbids
//! whatever the instructions say, the forms a story's worktree refuses, and a script missing from
//! the `package.json` a command runs against. Rewritten from `.claude/hooks/guard-bash.ts` (EN-104),
//! its regular expressions matched by hand, the standard library only: each matcher below names
//! the expression it replaces, backtracking included where it changes the answer.
//!
//! After the brief's `cd "<worktree>" || exit <n>`, one Git command goes through, alone or piped
//! into read-only filters (`head`, `tail`, `grep`, `wc`, `cut`, `sort`, `uniq`, `sed -n`, `awk`
//! without `system` nor a redirection), with or without `2>&1`; a second Git command, or any other
//! command chained after it by `;`, `&&` or `||`, is refused (EN-104, decided 2026-10-18).

use std::path::{Component, Path, PathBuf};

/// Why a full Vitest run is denied.
pub const FULL_VITEST: &str = "A full Vitest run takes the heavy lock: run `npm run test`, or name the test files (ADR-0063).";

/// Why `gh pr merge` is denied.
pub const MERGE: &str =
    "Story pull requests are merged by the Product Owner, never by the agent (AGENTS.md, Never).";

/// Why `--no-verify` is denied.
pub const NO_VERIFY: &str = "The Git hooks of .githooks/ (the staged files fixed and scanned, the commit message checked, check:all before a push) are never bypassed: fix the cause instead.";

/// Why switching the Git hooks off is denied.
pub const HOOKS_OFF: &str = "The Git hooks of .githooks/ (the staged files fixed and scanned, the commit message checked, check:all before a push) are never switched off: core.hooksPath stays as git-hooks install set it; fix the cause instead.";

/// Why a force push is denied.
pub const FORCE_PUSH: &str =
    "Force pushes use --force-with-lease, and only on the agent's own story branch.";

/// Why `golden:write` is denied in a worktree.
pub const GOLDEN_WRITE: &str = "golden:write writes .git/metrics/golden.json, shared by every worktree: only the main thread runs it, in the main checkout.";

/// Vitest's options that take a value as the next word, not a path of tests.
const VITEST_VALUED: [&str; 12] = [
    "--config",
    "--dir",
    "--environment",
    "--pool",
    "--project",
    "--reporter",
    "--root",
    "--shard",
    "--testNamePattern",
    "-c",
    "-r",
    "-t",
];

/// The folder of a story's worktree inside a path (EN-052).
const WORKTREES: &str = "/.claude/worktrees/";

/// The scripts npm runs without a `package.json` entry (`npm run env`).
const NPM_OWN_SCRIPTS: [&str; 1] = ["env"];

/// JavaScript's `\s`: Unicode white space without U+0085, with U+FEFF.
#[must_use]
pub fn is_space(character: char) -> bool {
    (character.is_whitespace() && character != '\u{85}') || character == '\u{feff}'
}

/// The characters `.` does not match in a JavaScript regular expression.
fn is_line_break(character: char) -> bool {
    matches!(character, '\n' | '\r' | '\u{2028}' | '\u{2029}')
}

/// JavaScript's `\w` without the `i` flag: an ASCII letter, digit or underscore.
fn is_word(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || byte == b'_'
}

/// `\b` after a word: the text ends at `at` or goes on with a character that is not `\w`.
fn ends_word(text: &str, at: usize) -> bool {
    text.as_bytes().get(at).is_none_or(|byte| !is_word(*byte))
}

/// The end of `\s*` from `at`.
fn after_spaces(text: &str, at: usize) -> usize {
    text[at..]
        .char_indices()
        .find(|(_, character)| !is_space(*character))
        .map_or(text.len(), |(index, _)| at + index)
}

/// The end of `\s+` from `at`, `None` when no space is there.
fn after_some_spaces(text: &str, at: usize) -> Option<usize> {
    let end = after_spaces(text, at);

    (end > at).then_some(end)
}

/// The end of `\S*` from `at`.
fn after_non_spaces(text: &str, at: usize) -> usize {
    text[at..]
        .char_indices()
        .find(|(_, character)| is_space(*character))
        .map_or(text.len(), |(index, _)| at + index)
}

/// Whether the text at `at` is a space or its end: `(?:\s|$)`.
fn space_or_end(text: &str, at: usize) -> bool {
    text[at..].chars().next().is_none_or(is_space)
}

/// The positions after `.*\s` from `at`: each space reached without crossing a line break, the
/// position after it.
fn after_any_then_space(text: &str, at: usize) -> Vec<usize> {
    let mut ends = Vec::new();

    for (index, character) in text[at..].char_indices() {
        if is_space(character) {
            ends.push(at + index + character.len_utf8());
        }
        if is_line_break(character) {
            break;
        }
    }
    ends
}

/// The positions `.*` reaches from `at`: every one up to the first line break.
fn reached_by_any(text: &str, at: usize) -> Vec<usize> {
    let end = text[at..]
        .char_indices()
        .find(|(_, character)| is_line_break(*character))
        .map_or(text.len(), |(index, _)| at + index);

    (at..=end)
        .filter(|index| text.is_char_boundary(*index))
        .collect()
}

/// Whether the text starts with a word followed by `\b`.
fn starts_with_word(text: &str, at: usize, word: &str) -> bool {
    text[at..].starts_with(word) && ends_word(text, at + word.len())
}

/// The shell command without the bodies of its here-documents: their lines are data written to a
/// file or a pipe (a commit message, a test case), never run. Each `<<WORD … WORD` body is removed,
/// the line opening it kept, as `/(<<-?\s*(['"]?)(\w+)\2[^\n]*)\n[\s\S]*?\n\s*\3(?=\n|$)/gu`
/// replaced by its first group.
#[must_use]
pub fn without_heredocs(command: &str) -> String {
    let mut kept = String::with_capacity(command.len());
    let mut at = 0;

    while at < command.len() {
        if let Some((opening_end, end)) = heredoc_at(command, at) {
            kept.push_str(&command[at..opening_end]);
            at = end;
        } else {
            let character = command[at..].chars().next().unwrap_or_default();

            kept.push(character);
            at += character.len_utf8();
        }
    }
    kept
}

/// A here-document opening at `at`: the end of its opening line and the end of its body's last
/// word, `None` when none starts there.
fn heredoc_at(text: &str, at: usize) -> Option<(usize, usize)> {
    if !text[at..].starts_with("<<") {
        return None;
    }
    let mut after = at + 2;

    if text[after..].starts_with('-') {
        after += 1;
    }
    after = after_spaces(text, after);

    let quote = match text.as_bytes().get(after) {
        Some(byte @ (b'\'' | b'"')) => Some(*byte),
        _ => None,
    };
    let word_start = after + usize::from(quote.is_some());
    let longest = text.as_bytes()[word_start..]
        .iter()
        .take_while(|byte| is_word(**byte))
        .count();

    // `\w+` gives back characters when the rest fails: a shorter word is tried in turn.
    (1..=longest).rev().find_map(|length| {
        let word_end = word_start + length;
        let opened = match quote {
            Some(byte) => (text.as_bytes().get(word_end) == Some(&byte)).then_some(word_end + 1)?,
            None => word_end,
        };
        let opening_end = text[opened..].find('\n').map(|index| opened + index)?;

        body_end(text, opening_end, &text[word_start..word_end]).map(|end| (opening_end, end))
    })
}

/// The end of a here-document's body: the first `\n\s*WORD` after its opening line, followed by a
/// line feed or the end of the text.
fn body_end(text: &str, opening_end: usize, word: &str) -> Option<usize> {
    let mut search = opening_end + 1;

    loop {
        let line_feed = search + text.get(search..)?.find('\n')?;
        let start = after_spaces(text, line_feed + 1);
        let end = start + word.len();

        if text[start..].starts_with(word) && matches!(text.as_bytes().get(end), None | Some(b'\n'))
        {
            return Some(end);
        }
        search = line_feed + 1;
    }
}

/// The segments of a shell command, split at `&&`, `||`, `;`, a pipe and a line feed outside
/// quotes, each trimmed.
#[must_use]
pub fn segments(command: &str) -> Vec<String> {
    separated(command)
        .into_iter()
        .map(|(_, segment)| segment)
        .collect()
}

/// The segments of a shell command, each with the separator before it, empty for the first:
/// `&&`, `||`, `;`, `|` or a line feed, outside quotes as the shell reads them. A separator inside
/// `'…'` or `"…"`, or escaped by a backslash outside single quotes, is text: `grep -E 'a|b'` is one
/// segment (EN-113). An unclosed quote runs to the end.
#[must_use]
pub fn separated(command: &str) -> Vec<(&'static str, String)> {
    split_at_separators(command, true)
}

/// The segments of a shell command split at every separator, quoted ones too: what a quoted text
/// would run if a shell ran it (`sh -c "…; git push -f"`), which the guards of [`denied_reason`]
/// read beside [`segments`]; each without the quotes around it, as the quoted shell reads it.
#[must_use]
pub fn every_segment(command: &str) -> Vec<String> {
    split_at_separators(command, false)
        .into_iter()
        .map(|(_, segment)| {
            segment
                .trim_matches(|character: char| {
                    is_space(character) || matches!(character, '"' | '\'')
                })
                .to_owned()
        })
        .collect()
}

/// The segments of a command with the separator before each, quotes read or not.
fn split_at_separators(command: &str, quotes_read: bool) -> Vec<(&'static str, String)> {
    let mut pieces = Vec::new();
    let mut start = 0;
    let mut before = "";
    let bytes = command.as_bytes();
    let mut at = 0;
    let mut quote: Option<u8> = None;

    while at < bytes.len() {
        let byte = bytes[at];

        if quotes_read {
            match (quote, byte) {
                (Some(b'\''), b'\'') | (Some(b'"'), b'"') => {
                    quote = None;
                    at += 1;
                    continue;
                }
                (None | Some(b'"'), b'\\') => {
                    at += 2;
                    continue;
                }
                (None, b'\'' | b'"') => {
                    quote = Some(byte);
                    at += 1;
                    continue;
                }
                (Some(_), _) => {
                    at += 1;
                    continue;
                }
                (None, _) => {}
            }
        }
        let separator = match (byte, bytes.get(at + 1)) {
            (b'&', Some(b'&')) => "&&",
            (b'|', Some(b'|')) => "||",
            (b';', _) => ";",
            (b'|', _) => "|",
            (b'\n', _) => "\n",
            _ => "",
        };

        if separator.is_empty() {
            at += 1;
        } else {
            pieces.push((before, command[start..at].trim_matches(is_space).to_owned()));
            at += separator.len();
            start = at;
            before = separator;
        }
    }
    pieces.push((before, command[start..].trim_matches(is_space).to_owned()));
    pieces
}

/// The programs that only read what a Git command prints: a filter piped after it changes
/// nothing (EN-104, decided 2026-10-18).
const READ_ONLY_FILTERS: [&str; 7] = ["head", "tail", "grep", "wc", "cut", "sort", "uniq"];

/// Whether a segment only filters what is piped into it: `head`, `tail`, `grep`, `wc`, `cut`,
/// `sort`, `uniq`, `sed -n` without `-i`, `awk` without `system` nor a redirection; none writes a
/// file (`>`), `2>&1` aside.
#[must_use]
pub fn is_read_only_filter(segment: &str) -> bool {
    let text = segment.replace("2>&1", "");
    let mut words = text.split(is_space).filter(|word| !word.is_empty());
    let program = words.next().unwrap_or_default();
    let rest: Vec<&str> = words.collect();

    if text.contains('>') {
        return false;
    }
    match program {
        "sed" => rest.first() == Some(&"-n") && !rest.iter().any(|word| word.starts_with("-i")),
        "awk" => !text.contains("system") && !text.contains("getline"),
        _ => READ_ONLY_FILTERS.contains(&program),
    }
}

/// Whether a command is one Git command, then only read-only filters piped after it, its own
/// `2>&1` allowed: `git log --oneline | head -5`.
#[must_use]
pub fn is_git_read_through_filters(command: &str) -> bool {
    let pieces = separated(&without_heredocs(command));
    let Some(((_, first), filters)) = pieces.split_first() else {
        return false;
    };

    runs_git_at_all(first)
        && !first.replace("2>&1", "").contains('>')
        && !filters.is_empty()
        && filters
            .iter()
            .all(|(before, segment)| *before == "|" && is_read_only_filter(segment))
}

/// `^git\s`: a segment that runs Git.
fn runs_git_at_all(segment: &str) -> bool {
    segment.starts_with("git") && segment[3..].starts_with(is_space)
}

/// The positions a Git subcommand may start at after `git` and its global options (`-C <path>`,
/// `-c <name>=<value>`, `--no-pager`): `^git(?:\s+(?:-[Cc]\s+(?:"[^"]*"|'[^']*'|\S+)|--[a-z-]+(?:=\S+)?))*\s+`,
/// every way it matches.
fn git_subcommands(segment: &str) -> Vec<usize> {
    if !segment.starts_with("git") {
        return Vec::new();
    }

    let mut starts = Vec::new();
    let mut frontier = vec![3];
    let mut seen = Vec::new();

    while let Some(position) = frontier.pop() {
        if seen.contains(&position) {
            continue;
        }
        seen.push(position);

        let Some(word) = after_some_spaces(segment, position) else {
            continue;
        };

        starts.push(word);
        frontier.extend(option_ends(segment, word));
    }
    starts
}

/// The ends of one global option of Git starting at `at`, every way it matches.
fn option_ends(segment: &str, at: usize) -> Vec<usize> {
    let rest = &segment[at..];
    let mut ends = Vec::new();

    if rest.starts_with("-C") || rest.starts_with("-c") {
        if let Some(value) = after_some_spaces(segment, at + 2) {
            for quote in ['"', '\''] {
                if segment[value..].starts_with(quote)
                    && let Some(close) = segment[value + 1..].find(quote)
                {
                    ends.push(value + 1 + close + 1);
                }
            }
            let end = after_non_spaces(segment, value);

            if end > value {
                ends.push(end);
            }
        }
    } else if let Some(name) = rest.strip_prefix("--") {
        let length = name
            .bytes()
            .take_while(|byte| byte.is_ascii_lowercase() || *byte == b'-')
            .count();
        let end = at + 2 + length;

        if length > 0 {
            let valued = segment[end..]
                .starts_with('=')
                .then(|| after_non_spaces(segment, end + 1))
                .filter(|value_end| *value_end > end + 1);

            ends.push(valued.unwrap_or(end));
        }
    }
    ends
}

/// `-[A-Za-z]*n[A-Za-z]*(?:\s|$)` at `at`: short options of which one is `-n`.
fn short_n(segment: &str, at: usize) -> bool {
    let Some(letters) = segment[at..].strip_prefix('-') else {
        return false;
    };
    let length = letters.bytes().take_while(u8::is_ascii_alphabetic).count();

    letters[..length].contains('n') && space_or_end(segment, at + 1 + length)
}

/// `--no-verify\b` at `at`.
fn no_verify(segment: &str, at: usize) -> bool {
    starts_with_word(segment, at, "--no-verify")
}

/// `(?:--force|-f|\+\S+)(?:\s|$)` at `at`.
fn forces(segment: &str, at: usize) -> bool {
    let rest = &segment[at..];

    (rest.starts_with("--force") && space_or_end(segment, at + 7))
        || (rest.starts_with("-f") && space_or_end(segment, at + 2))
        || (rest.starts_with('+') && after_non_spaces(segment, at + 1) > at + 1)
}

/// A commit or a push that skips the Git hooks: `commit\b.*\s(?:--no-verify\b|-…n…)`,
/// `push\b.*\s--no-verify\b`.
fn skips_hooks(segment: &str) -> bool {
    git_subcommands(segment).into_iter().any(|start| {
        let commit = starts_with_word(segment, start, "commit")
            && after_any_then_space(segment, start + 6)
                .into_iter()
                .any(|at| no_verify(segment, at) || short_n(segment, at));
        let push = starts_with_word(segment, start, "push")
            && after_any_then_space(segment, start + 4)
                .into_iter()
                .any(|at| no_verify(segment, at));

        commit || push
    })
}

/// A push forced without lease: `push\b(?=.*\s(?:--force|-f|\+\S+)(?:\s|$))`.
fn forces_push(segment: &str) -> bool {
    git_subcommands(segment).into_iter().any(|start| {
        starts_with_word(segment, start, "push")
            && after_any_then_space(segment, start + 4)
                .into_iter()
                .any(|at| forces(segment, at))
    })
}

/// `core.hooksPath` set for a Git command: `^git\s.*(?:-c\s+|config\b.*\s)core\.hooksPath\b`,
/// unless the `config` command only reads it ([`reads_config`]).
fn sets_hooks_path(segment: &str) -> bool {
    let Some(space) = segment
        .strip_prefix("git")
        .and_then(|rest| rest.chars().next())
        .filter(|character| is_space(*character))
    else {
        return false;
    };
    let hooks_path = |at: usize| starts_with_word(segment, at, HOOKS_PATH);

    reached_by_any(segment, 3 + space.len_utf8())
        .into_iter()
        .any(|at| {
            let by_option = segment[at..].starts_with("-c")
                && after_some_spaces(segment, at + 2).is_some_and(hooks_path);
            let by_config = starts_with_word(segment, at, "config")
                && after_any_then_space(segment, at + 6)
                    .into_iter()
                    .any(|key| hooks_path(key) && !reads_config(segment, at + 6, key));

            by_option || by_config
        })
}

/// The key the Git hooks' folder is set by.
const HOOKS_PATH: &str = "core.hooksPath";

/// The options of `git config` that only say where or how to read (EN-113).
const CONFIG_READ_OPTIONS: [&str; 15] = [
    "--get",
    "--get-all",
    "--get-regexp",
    "get",
    "--all",
    "--local",
    "--global",
    "--system",
    "--worktree",
    "--show-origin",
    "--show-scope",
    "--null",
    "-z",
    "--includes",
    "--no-includes",
];

/// Whether a `git config` whose options run from `from` to the key at `key` only reads the key:
/// every option one of [`CONFIG_READ_OPTIONS`] or a `--type=`, `--default=` or `--file=`, and
/// either a getter among them (`--get`, `get`) or no value after the key, a redirection aside
/// (`git config core.hooksPath 2>&1`). `--unset`, `--add`, `set` or a value change it (EN-113).
fn reads_config(segment: &str, from: usize, key: usize) -> bool {
    let options: Vec<&str> = segment[from..key]
        .split(is_space)
        .filter(|word| !word.is_empty())
        .collect();
    let read_only = options.iter().all(|option| {
        CONFIG_READ_OPTIONS.contains(option)
            || ["--type=", "--default=", "--file="]
                .iter()
                .any(|prefix| option.starts_with(prefix))
    });
    let getter = options
        .iter()
        .any(|option| matches!(*option, "--get" | "--get-all" | "--get-regexp" | "get"));
    let no_value = segment[key + HOOKS_PATH.len()..]
        .split(is_space)
        .filter(|word| !word.is_empty())
        .all(|word| {
            word.trim_start_matches(|digit: char| digit.is_ascii_digit())
                .starts_with('>')
        });

    read_only && (getter || no_value)
}

/// `^gh\s+pr\s+merge\b`.
fn merges(segment: &str) -> bool {
    segment
        .strip_prefix("gh")
        .and_then(|_| after_some_spaces(segment, 2))
        .filter(|at| segment[*at..].starts_with("pr"))
        .and_then(|at| after_some_spaces(segment, at + 2))
        .is_some_and(|at| starts_with_word(segment, at, "merge"))
}

/// Whether a segment runs Vitest on every test, outside the heavy lock (EN-047): no test path, or
/// only the current folder, whatever its options (`--project x`, `--reporter dot`). Vitest is run
/// by `npx` or from `node_modules/.bin`:
/// `^(?:npx\s+|(?:\.\/)?node_modules\/\.bin\/)vitest(?<rest>(?:\s.*)?)$`.
#[must_use]
pub fn is_full_vitest_run(segment: &str) -> bool {
    let after = if let Some(rest) = segment.strip_prefix("npx") {
        after_some_spaces(segment, 3).filter(|_| !rest.is_empty())
    } else {
        ["./node_modules/.bin/", "node_modules/.bin/"]
            .into_iter()
            .find(|prefix| segment.starts_with(prefix))
            .map(str::len)
    };
    let Some(rest) = after.and_then(|at| segment[at..].strip_prefix("vitest")) else {
        return false;
    };
    let mut characters = rest.chars();

    match characters.next() {
        None => {}
        Some(first) if is_space(first) && !characters.as_str().contains(is_line_break) => {}
        Some(_) => return false,
    }

    let words: Vec<&str> = rest
        .split(is_space)
        .filter(|word| !word.is_empty() && *word != "run")
        .collect();

    words.iter().enumerate().all(|(index, word)| {
        let is_value = index > 0 && VITEST_VALUED.contains(&words[index - 1]);

        word.starts_with('-') || is_value || matches!(*word, "." | "./")
    })
}

/// A guard: whether a segment is forbidden, and why.
type Guard = (fn(&str) -> bool, &'static str);

/// The reason for denying a shell command, if one of its segments is forbidden: the first guard
/// matching, then a full Vitest run, which takes the heavy lock through `npm run test` (EN-047).
/// The segments are read both as the shell splits them and split inside quotes too, so that a
/// command quoted for `sh -c` is judged as well (EN-113).
#[must_use]
pub fn denied_reason(command: &str) -> Option<&'static str> {
    let text = without_heredocs(command);
    let mut pieces = segments(&text);

    pieces.extend(every_segment(&text));
    let guards: [Guard; 4] = [
        (merges, MERGE),
        (skips_hooks, NO_VERIFY),
        (sets_hooks_path, HOOKS_OFF),
        (forces_push, FORCE_PUSH),
    ];

    guards
        .iter()
        .find(|(matches, _)| pieces.iter().any(|segment| matches(segment)))
        .map(|(_, reason)| *reason)
        .or_else(|| {
            pieces
                .iter()
                .any(|segment| is_full_vitest_run(segment))
                .then_some(FULL_VITEST)
        })
}

/// The worktree a folder is inside (EN-052), its root:
/// `^(?<root>.*\/\.claude\/worktrees\/[^/]+)(?:\/|$)`, the last such folder of the path.
#[must_use]
pub fn worktree_root(folder: &str) -> Option<&str> {
    let before_break = folder.find(is_line_break).unwrap_or(folder.len());

    folder
        .match_indices(WORKTREES)
        .filter(|(index, _)| *index <= before_break)
        .map(|(index, _)| index + WORKTREES.len())
        .filter_map(|start| {
            let end = folder[start..]
                .find('/')
                .map_or(folder.len(), |at| start + at);

            (end > start).then(|| &folder[..end])
        })
        .last()
}

/// A command as the guard judges it: the folder it runs in and the part judged there.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entered {
    /// The folder a leading `cd` enters, else the session's.
    pub folder: String,
    /// The command without its leading `cd "<folder>" || exit <n>`, else the whole command.
    pub command: String,
    /// Whether the command opened with the brief's `cd "<folder>" || exit <n>`.
    pub guarded: bool,
}

/// The reason for denying a command in the folder it entered: as [`worktree_reason`] gives it,
/// except that after the brief's `cd "<worktree>" || exit <n>` one Git command piped into
/// read-only filters goes through (`git log | head`, EN-104, decided 2026-10-18: a refusal costs a
/// call); a second Git command or another command chained by `;`, `&&` or `||` is still refused.
#[must_use]
pub fn entered_worktree_reason(judged: &Entered) -> Option<String> {
    if judged.guarded && is_git_read_through_filters(&judged.command) {
        return None;
    }
    worktree_reason(&judged.command, &judged.folder)
}

/// The folder a command runs in: the one a leading `cd "<folder>"` enters, resolved against the
/// session's, a folder named by a variable or a `~` left to the session's; and the part of the
/// command judged there, without the brief's `cd "<folder>" || exit <n>` (ADR-0072), which is no
/// Git command chained. The part keeps its here-documents, which the judges leave out.
#[must_use]
pub fn entered(command: &str, cwd: &str) -> Entered {
    let text = command.to_owned();
    let whole = Entered {
        folder: cwd.to_owned(),
        command: text.clone(),
        guarded: false,
    };
    let start = after_spaces(&text, 0);
    let Some(target) = text[start..]
        .strip_prefix("cd")
        .and_then(|_| after_some_spaces(&text, start + 2))
    else {
        return whole;
    };
    let Some((folder, after_folder)) = cd_target(&text, target) else {
        return whole;
    };
    let resolved = if folder.contains(['$', '`', '~']) {
        cwd.to_owned()
    } else {
        lexical_join(cwd, folder)
    };
    let (rest, guarded) = after_exit_guard(&text, after_folder);

    Entered {
        folder: resolved,
        command: if guarded { rest } else { text },
        guarded,
    }
}

/// The folders a command enters by a `cd` after its first segment, each resolved against the one
/// before, starting from `folder`: a story's worktree entered there is judged as one too.
#[must_use]
pub fn later_folders(command: &str, folder: &str) -> Vec<String> {
    let mut current = folder.to_owned();
    let mut folders = Vec::new();

    for segment in segments(&without_heredocs(command)).iter().skip(1) {
        let Some(target) = segment
            .strip_prefix("cd")
            .and_then(|_| after_some_spaces(segment, 2))
        else {
            continue;
        };
        if let Some((named, _)) = cd_target(segment, target)
            && !named.contains(['$', '`', '~'])
        {
            current = lexical_join(&current, named);
            folders.push(current.clone());
        }
    }
    folders
}

/// The folder a `cd` names at `at`, quoted or not, and the position after it; `None` when the
/// argument is not a single word followed by a space, a separator or the end.
fn cd_target(text: &str, at: usize) -> Option<(&str, usize)> {
    let rest = &text[at..];
    let (folder, end) = if let quote @ ('"' | '\'') = rest.chars().next()? {
        let close = rest[1..].find(quote)?;

        (&rest[1..=close], at + close + 2)
    } else {
        let length = rest
            .find(|character: char| is_space(character) || matches!(character, ';' | '&' | '|'))
            .unwrap_or(rest.len());

        (&rest[..length], at + length)
    };
    let next = text[end..].chars().next();

    (!folder.is_empty()
        && next.is_none_or(|character| is_space(character) || matches!(character, ';' | '&' | '|')))
    .then_some((folder, end))
}

/// The command after `|| exit <n>` and the separator that follows it, when the `cd` is guarded so;
/// else the text unchanged and false.
fn after_exit_guard(text: &str, at: usize) -> (String, bool) {
    let not_guarded = (text.to_owned(), false);
    let start = after_spaces(text, at);
    let Some(exit) = text[start..]
        .strip_prefix("||")
        .map(|_| after_spaces(text, start + 2))
        .filter(|exit| starts_with_word(text, *exit, "exit"))
    else {
        return not_guarded;
    };
    let mut end = exit + 4;
    let code = after_spaces_on_line(text, end);
    let digits = text.as_bytes()[code..]
        .iter()
        .take_while(|byte| byte.is_ascii_digit())
        .count();

    if digits > 0 {
        end = code + digits;
    }
    let separator = after_spaces_on_line(text, end);
    let rest = &text[separator..];
    let width = if rest.starts_with("&&") {
        2
    } else if rest.starts_with([';', '\n']) {
        1
    } else if rest.is_empty() {
        0
    } else {
        return not_guarded;
    };

    (text[separator + width..].to_owned(), true)
}

/// The end of the spaces and tabs from `at`, a line feed left in place.
fn after_spaces_on_line(text: &str, at: usize) -> usize {
    at + text[at..]
        .bytes()
        .take_while(|byte| matches!(byte, b' ' | b'\t'))
        .count()
}

/// A folder joined to the session's and normalized without reading the disk: `.` dropped, `..`
/// taking off the folder before it.
#[must_use]
pub fn lexical_join(base: &str, folder: &str) -> String {
    let mut path = PathBuf::new();

    for component in Path::new(base).join(folder).components() {
        match component {
            Component::CurDir => {}
            Component::ParentDir => {
                path.pop();
            }
            other => path.push(other),
        }
    }
    path.to_string_lossy().into_owned()
}

/// The reason for denying, in a worktree, a command Claude Code would refuse there: a Git command
/// chained with another, `cd` included (549 refusals in E15), the hint naming the form that works;
/// or `golden:write`, which writes a file every worktree shares. `None` outside a worktree or for
/// a command that fits.
#[must_use]
pub fn worktree_reason(command: &str, folder: &str) -> Option<String> {
    let root = worktree_root(folder)?;
    let pieces: Vec<String> = segments(&without_heredocs(command))
        .into_iter()
        .filter(|segment| !segment.is_empty())
        .collect();

    if pieces.iter().any(|segment| writes_golden(segment)) {
        return Some(GOLDEN_WRITE.to_owned());
    }
    chained_git_hint(&pieces, root)
}

/// `^npm\s+run\s+golden:write\b`.
fn writes_golden(segment: &str) -> bool {
    segment
        .strip_prefix("npm")
        .and_then(|_| after_some_spaces(segment, 3))
        .filter(|at| segment[*at..].starts_with("run"))
        .and_then(|at| after_some_spaces(segment, at + 3))
        .is_some_and(|at| starts_with_word(segment, at, "golden:write"))
}

/// `^git\s+(?:<one of the subcommands>)\b`.
fn runs_git(segment: &str, subcommands: &[&str]) -> bool {
    segment
        .strip_prefix("git")
        .and_then(|_| after_some_spaces(segment, 3))
        .is_some_and(|at| {
            subcommands
                .iter()
                .any(|subcommand| starts_with_word(segment, at, subcommand))
        })
}

/// The hint for a Git command chained with another in a worktree: `cd` into the worktree first,
/// since a subagent's calls start in the main checkout, then the Git command alone (AUD-020); and
/// the Rust command running the sequence in one call (EN-100), by its binary's path (ADR-0078).
/// `None` when no Git command is chained.
fn chained_git_hint(pieces: &[String], root: &str) -> Option<String> {
    let git_command = pieces
        .iter()
        .find(|segment| segment.starts_with("git") && segment[3..].starts_with(is_space))?;

    if pieces.len() <= 1 {
        return None;
    }

    let sequences: [(&[&str], &str); 2] = [
        (
            &["add", "commit", "push"],
            "`target/release/git-sequences commit -F <message file> [--push] <paths>` stages, commits and pushes",
        ),
        (
            &["fetch", "rebase", "pull"],
            "`target/release/git-sequences sync` fetches and rebases the story on its feature branch",
        ),
    ];
    let hints: Vec<&str> = sequences
        .iter()
        .filter(|(subcommands, _)| pieces.iter().any(|segment| runs_git(segment, subcommands)))
        .map(|(_, hint)| *hint)
        .collect();
    let one_call = if hints.is_empty() {
        String::new()
    } else {
        format!(" Or in one call: {}.", hints.join("; "))
    };

    Some(format!(
        "A worktree refuses a chained Git command: run `cd \"{root}\" || exit 9` then `{git_command}` alone.{one_call}"
    ))
}

/// The script a segment runs with `npm run` or `npm run-script`, npm's own options before or after
/// `run` skipped (`npm run -s page`); `None` for another command or `npm run` alone, which lists
/// the scripts.
#[must_use]
pub fn npm_script(segment: &str) -> Option<&str> {
    let mut words = segment.split(is_space).filter(|word| !word.is_empty());

    if words.next()? != "npm" {
        return None;
    }
    words
        .by_ref()
        .find(|word| !word.starts_with('-'))
        .filter(|word| matches!(*word, "run" | "run-script"))?;
    // A name with quotes or brackets is a piece of code or text quoted around `npm run`, not a
    // script the shell runs.
    words
        .find(|word| !word.starts_with('-'))
        .filter(|script| {
            script.bytes().all(|byte| {
                byte.is_ascii_alphanumeric() || matches!(byte, b':' | b'.' | b'_' | b'-')
            })
        })
        .filter(|script| !NPM_OWN_SCRIPTS.contains(script))
}

/// The first script a command runs that is missing from the scripts of its `package.json`.
#[must_use]
pub fn missing_script(command: &str, scripts: &[String]) -> Option<String> {
    segments(command)
        .iter()
        .filter_map(|segment| npm_script(segment))
        .find(|script| !scripts.iter().any(|name| name == script))
        .map(str::to_owned)
}

/// The names of the scripts of a `package.json`'s text, none when it is not JSON.
#[must_use]
pub fn scripts_of(package: &str) -> Option<Vec<String>> {
    match crate::json::parse(package)?.get("scripts") {
        crate::json::Value::Object(members) => {
            Some(members.iter().map(|(name, _)| name.clone()).collect())
        }
        _ => Some(Vec::new()),
    }
}

/// Why a script missing from the branch's `package.json` is refused: `npm run -s` hides npm's own
/// message, and a branch made before the script lacks it until it is rebased on its base.
#[must_use]
pub fn missing_script_reason(script: &str, branch: &str, base: &str) -> String {
    format!(
        "`npm run {script}`: no script `{script}` in the package.json of branch `{branch}`. A branch made before the script gets it by a rebase on its base: run `git fetch origin`, then `git rebase {base}`, one call each."
    )
}
