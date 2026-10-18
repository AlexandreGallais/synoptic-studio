//! The shell guard (ADR-0021), the `guard-bash` hook's judgement: the commands the project forbids
//! whatever the instructions say, read word by word ([`crate::forbidden`], AUD-023), the forms a
//! story's worktree refuses, and a script missing from the `package.json` a command runs against.
//! The worktree's forms were rewritten from `.claude/hooks/guard-bash.ts` (EN-104), its regular
//! expressions matched by hand, the standard library only: each matcher below names the
//! expression it replaces, backtracking included where it changes the answer.
//!
//! After the brief's `cd "<worktree>" || exit <n>`, one Git command goes through, alone or piped
//! into read-only filters ([`is_read_only_filter`]), with or without `2>&1`; a second Git command,
//! or any other command chained after it by `;`, `&&` or `||`, is refused (EN-104, decided
//! 2026-10-18).

use git_repository::lexical_join;

use crate::forbidden::{Arguments, Forbidden, abbreviates, invocations, reason};
use crate::shell::{Simple, commands};

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

/// Why a push to `main` or `develop` is denied.
pub const PROTECTED_PUSH: &str = "main and develop are never pushed by the agent: a story's branch is pushed, and its pull request merges it (AGENTS.md, Never).";

/// Why writing the golden file is denied in a worktree.
pub const GOLDEN_WRITE: &str = "Vitest's mode golden-write writes .git/metrics/golden.json, shared by every worktree: only the main thread runs it, in the main checkout.";

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
    let mut pieces = Vec::new();
    let mut start = 0;
    let mut before = "";
    let bytes = command.as_bytes();
    let mut at = 0;
    let mut quote: Option<u8> = None;

    while at < bytes.len() {
        let byte = bytes[at];

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

/// Whether a command is one Git command, then only read-only filters piped after it, its own
/// `2>&1` allowed: `git log --oneline | head -5`.
#[must_use]
pub fn is_git_read_through_filters(command: &str) -> bool {
    let pieces = separated(&without_heredocs(command));
    let Some(((_, first), filters)) = pieces.split_first() else {
        return false;
    };

    let git = commands(first);

    runs_git_at_all(first)
        && git.len() == 1
        && git.iter().all(only_reads_and_duplicates)
        && !filters.is_empty()
        && filters
            .iter()
            .all(|(before, segment)| *before == "|" && is_read_only_filter(segment))
}

/// Whether a simple command holds no substitution and writes no file: its redirections only read
/// one (`<`) or duplicate a descriptor (`2>&1`).
fn only_reads_and_duplicates(simple: &Simple) -> bool {
    !simple.substituted
        && simple.redirections.iter().all(|redirection| {
            let operator = redirection.trim_start_matches(|digit: char| digit.is_ascii_digit());

            (operator.starts_with('<') && !operator.starts_with("<>"))
                || (operator.starts_with(">&")
                    && operator[2..]
                        .bytes()
                        .all(|byte| byte.is_ascii_digit() || byte == b'-'))
        })
}

/// The programs that only read what is piped into them, whatever their options, `grep`, `head`,
/// `tail`, `wc` and `cut` (EN-104, decided 2026-10-18); `sort`, `uniq`, `sed` and `awk` are read
/// further by [`is_read_only_filter`].
const READ_ONLY_FILTERS: [&str; 5] = ["head", "tail", "grep", "wc", "cut"];

/// Whether a segment only filters what is piped into it (AUD-023): one simple command, no
/// substitution, no redirection writing a file (`2>&1` allowed), and its program `head`, `tail`,
/// `grep`, `wc`, `cut`; `sort` without `-o` nor `--output` nor `--compress-program`, which runs a
/// program; `uniq` with one file at most, a second being its output; `sed -n` whose scripts only
/// choose lines and print them (no `s`, `w`, `e`, `r`), without `-i` nor any other option; `awk`
/// whose program has no `system`, `getline`, pipe nor redirection, nor `-f` file.
#[must_use]
pub fn is_read_only_filter(segment: &str) -> bool {
    let simples = commands(segment);
    let [simple] = simples.as_slice() else {
        return false;
    };
    let Some((program, arguments)) = simple.words.split_first() else {
        return false;
    };

    if !only_reads_and_duplicates(simple) {
        return false;
    }
    match program.as_str() {
        "sort" => {
            let read = Arguments::read(
                arguments,
                "ktSTo",
                &[
                    "--key",
                    "--field-separator",
                    "--buffer-size",
                    "--temporary-directory",
                    "--output",
                ],
            );

            !read.shorts.contains(&'o')
                && !read.has_long("--output")
                && !read.has_long("--compress-program")
        }
        "uniq" => {
            let read = Arguments::read(
                arguments,
                "fsw",
                &["--skip-fields", "--skip-chars", "--check-chars"],
            );

            read.operands.len() <= 1
        }
        "sed" => is_printing_sed(arguments),
        "awk" => {
            let read = Arguments::read(arguments, "vF", &[]);
            let program = read.operands.first().map_or("", String::as_str);

            !read.shorts.contains(&'f')
                && !read
                    .longs
                    .iter()
                    .any(|long| long.starts_with("--f") || long.starts_with("--e"))
                && !["system", "getline", "|", ">"]
                    .iter()
                    .any(|word| program.contains(word))
        }
        name => READ_ONLY_FILTERS.contains(&name),
    }
}

/// Whether a `sed` only prints lines: `-n` given, its options `-n`, `-E`, `-r`, `-u`, `-z`, `-s`
/// or `-e <script>` and their long names, each script made of addresses (numbers, `$`, `/regex/`,
/// `~`, `,`) and the commands `p`, `P`, `l`, `=`, `q`, `Q`, `d`, `D`, `n`, `N`, `!`, `{`, `}`.
fn is_printing_sed(arguments: &[String]) -> bool {
    let read = Arguments::read(arguments, "e", &["--expression"]);
    let known_long = [
        "--quiet",
        "--silent",
        "--regexp-extended",
        "--unbuffered",
        "--null-data",
        "--separate",
    ];
    let mut scripts: Vec<&str> = Vec::new();
    let mut at = 0;

    while let Some(word) = arguments.get(at) {
        at += 1;
        if word == "-e" || word == "--expression" {
            scripts.extend(arguments.get(at).map(String::as_str));
            at += 1;
        } else if let Some(script) = word.strip_prefix("--expression=") {
            scripts.push(script);
        }
    }
    if scripts.is_empty() {
        scripts.extend(read.operands.first().map(String::as_str));
    }
    read.shorts.contains(&'n')
        && read.shorts.iter().all(|letter| "nErzsue".contains(*letter))
        && read.longs.iter().all(|long| {
            known_long.iter().any(|option| abbreviates(long, option))
                || abbreviates(long, "--expression")
        })
        && !read
            .longs
            .iter()
            .any(|long| abbreviates(long, "--in-place"))
        && !scripts.is_empty()
        && scripts.iter().all(|script| is_printing_sed_script(script))
}

/// Whether a `sed` script only chooses lines and prints them: see [`is_printing_sed`].
fn is_printing_sed_script(script: &str) -> bool {
    let mut characters = script.chars();

    while let Some(character) = characters.next() {
        if character == '/' {
            let mut escaped = false;

            for inner in characters.by_ref() {
                if inner == '/' && !escaped {
                    break;
                }
                escaped = inner == '\\' && !escaped;
            }
        } else if !(character.is_ascii_digit() || " \t\n,$;~+pPl=qQdDnN!{}".contains(character)) {
            return false;
        }
    }
    true
}

/// `^git\s`: a segment that runs Git.
fn runs_git_at_all(segment: &str) -> bool {
    segment.starts_with("git") && segment[3..].starts_with(is_space)
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

/// The reason given for what a forbidden command does.
#[must_use]
pub fn forbidden_text(forbidden: Forbidden) -> &'static str {
    match forbidden {
        Forbidden::Merge => MERGE,
        Forbidden::NoVerify => NO_VERIFY,
        Forbidden::HooksOff => HOOKS_OFF,
        Forbidden::ForcePush => FORCE_PUSH,
        Forbidden::ProtectedPush => PROTECTED_PUSH,
    }
}

/// The reason for denying a shell command, the current branch unknown: see [`denied_reason_on`].
#[must_use]
pub fn denied_reason(command: &str) -> Option<&'static str> {
    denied_reason_on(command, &String::new)
}

/// The reason for denying a shell command: the first command it runs that is forbidden, every
/// simple command and the command each wrapper runs read word by word (AUD-023,
/// [`crate::forbidden`]), then a full Vitest run, which takes the heavy lock through `npm run test`
/// (EN-047). `branch` gives the current branch, asked only for a push naming no destination.
#[must_use]
pub fn denied_reason_on(command: &str, branch: &dyn Fn() -> String) -> Option<&'static str> {
    let found = invocations(&without_heredocs(command));

    found
        .iter()
        .find_map(|invocation| reason(invocation, branch))
        .map(forbidden_text)
        .or_else(|| {
            found
                .iter()
                .any(|invocation| is_full_vitest_run(&invocation.words.join(" ")))
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

/// The reason for denying, in a worktree, a command Claude Code would refuse there: a Git command
/// chained with another, `cd` included (549 refusals in E15), the hint naming the form that works;
/// or Vitest's mode `golden-write`, which writes a file every worktree shares. `None` outside a
/// worktree or for a command that fits.
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

/// Whether a segment runs Vitest's mode `golden-write` (`--mode golden-write` or
/// `--mode=golden-write`), the command of the golden file since `golden:write` went (EN-115).
fn writes_golden(segment: &str) -> bool {
    let words: Vec<&str> = segment
        .split(is_space)
        .filter(|word| !word.is_empty())
        .collect();

    words.contains(&"--mode=golden-write")
        || words
            .windows(2)
            .any(|pair| pair == ["--mode", "golden-write"])
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
/// message; a branch made before the script lacks it until it is rebased on its base, and a tool
/// whose script went (EN-115) is called by its binary.
#[must_use]
pub fn missing_script_reason(script: &str, branch: &str, base: &str) -> String {
    format!(
        "`npm run {script}`: no script `{script}` in the package.json of branch `{branch}`. A tool of the agent is called by its binary, `target/release/<binary>`, as docs/tooling/agent-tools.md lists them (EN-115). A branch made before a script gets it by a rebase on its base: run `git fetch origin`, then `git rebase {base}`, one call each."
    )
}
