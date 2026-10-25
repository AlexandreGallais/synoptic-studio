//! The shell guard (ADR-0021), the `guard-bash` hook's judgement: the commands the project forbids
//! whatever the instructions say, read word by word ([`crate::forbidden`], AUD-023), the forms a
//! story's worktree refuses, and a script missing from the `package.json` a command runs against.
//! The worktree's forms read each segment word by word too (EN-119), the segments split by
//! [`separated`] outside quotes; the here-documents are read as the shell reads them
//! ([`crate::heredoc`], AUD-033), and a push naming no destination is judged in the folders it
//! may run in ([`crate::place`], AUD-033).
//!
//! After the brief's `cd "<worktree>" || exit <n>`, a chain of Git commands that only read goes
//! through ([`is_read_only_git`], EN-119), as does one Git command of any kind (EN-104), each alone
//! or piped into read-only filters ([`is_read_only_filter`]), with or without `2>&1`; a Git
//! command that writes, or any other command, chained with a Git command by `;`, `&&`, `||` or `&`
//! is refused (EN-104, decided 2026-10-18).

use git_repository::lexical_join;

use crate::forbidden::{
    Arguments, Forbidden, Invocation, abbreviates, first_forbidden, git_line, invocations,
    is_assignment, program_name,
};
use crate::place::BranchOf;
use crate::shell::{Simple, commands};
use crate::worktrees::worktree_root;

pub use crate::heredoc::{heredoc_bodies, without_heredocs};

/// Why a full Vitest run is denied.
pub const FULL_VITEST: &str = "A full Vitest run takes the heavy lock: run `npm run test`, or name the test files (ADR-0072).";

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

/// Why a push naming no destination is denied where the guard cannot read the branch it updates
/// (AUD-033).
pub const UNKNOWN_PUSH: &str = "A push naming no destination updates the current branch of the repository it runs against, which this command hides from the guard (a `cd` it cannot follow, a folder named by a variable, `env -C`, a `GIT_DIR` set apart, or a folder whose branch cannot be read): run `cd \"<folder>\" || exit 9`, then the push alone, or name its destination (`git push -u origin HEAD:<branch>`).";

/// Why taking the heavy lock by hand is denied (EN-119).
pub const HAND_LOCK: &str = "`.git/heavy.lock` is taken by the heavy commands themselves (EN-047): run `git push`, `npm run check:all`, `npm run test` or the other heavy command as it is, in the background when it may last; a lock taken by hand makes a `git push` inside it wait forever for its own pre-push.";

/// Why a Python script writing a file is denied in a worktree (EN-119).
pub const PYTHON_WRITE: &str = "A worktree's files are never written by a Python script (AGENTS.md, Always): edit a file with the Edit tool, several with `target/release/replace` (the records' tools), and write a new one with the Write tool.";

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
/// `&&`, `||`, `;`, `|`, `&` (a background job, not `&>` nor `>&`) or a line feed, outside quotes
/// as the shell reads them. A separator inside `'…'` or `"…"`, or escaped by a backslash outside
/// single quotes, is text: `grep -E 'a|b'` is one segment (EN-113). An unclosed quote runs to the
/// end.
#[must_use]
pub fn separated(command: &str) -> Vec<(&'static str, String)> {
    spans(command)
        .into_iter()
        .map(|(before, start, end)| {
            (
                before,
                command[start..end].trim_matches(is_space).to_owned(),
            )
        })
        .collect()
}

/// The segments of [`separated`] as byte ranges of the command, untrimmed, each with the
/// separator before it.
pub(crate) fn spans(command: &str) -> Vec<(&'static str, usize, usize)> {
    let mut pieces = Vec::new();
    let mut start = 0;
    let mut before = "";
    let bytes = command.as_bytes();
    let mut at = 0;
    let mut quote: Option<u8> = None;

    while at < bytes.len() {
        let byte = bytes[at];

        match (quote, byte) {
            // `$'…'`, marked `$`, closes at a quote its backslash does not escape (EN-119).
            (Some(b'\'' | b'$'), b'\'') | (Some(b'"'), b'"') => {
                quote = None;
                at += 1;
                continue;
            }
            (None, b'$') if bytes.get(at + 1) == Some(&b'\'') => {
                quote = Some(b'$');
                at += 2;
                continue;
            }
            (None | Some(b'"' | b'$'), b'\\') => {
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
        let after_redirection = at > 0 && matches!(bytes[at - 1], b'>' | b'<');
        let separator = match (byte, bytes.get(at + 1)) {
            (b'&', Some(b'&')) => "&&",
            (b'|', Some(b'|')) => "||",
            (b';', _) => ";",
            (b'|', _) => "|",
            (b'\n', _) => "\n",
            (b'&', next) if !after_redirection && next != Some(&b'>') => "&",
            _ => "",
        };

        if separator.is_empty() {
            at += 1;
        } else {
            pieces.push((before, start, at));
            at += separator.len();
            start = at;
            before = separator;
        }
    }
    pieces.push((before, start, command.len()));
    pieces
}

/// Whether a command is one Git command, then only read-only filters piped after it, its own
/// `2>&1` allowed: `git log --oneline | head -5`.
#[must_use]
pub fn is_git_read_through_filters(command: &str) -> bool {
    matches!(pipelines(command).as_deref(), Some([pipeline]) if pipeline.len() > 1 && is_git_pipeline(pipeline))
}

/// Whether a command is a chain of Git commands that only read, each alone or piped into read-only
/// filters, joined by `;`, `&&`, `||` or a line feed: `git status; git log -3 | head` (EN-119).
/// One Git command of any kind piped into filters is one too ([`is_git_read_through_filters`]).
#[must_use]
pub fn is_read_only_git_chain(command: &str) -> bool {
    let Some(chain) = pipelines(command) else {
        return false;
    };

    !chain.is_empty()
        && chain.iter().all(|pipeline| is_git_pipeline(pipeline))
        && (chain.len() == 1
            || chain.iter().all(|pipeline| {
                commands(&pipeline[0])
                    .first()
                    .is_some_and(|git| is_read_only_git(&git.words))
            }))
}

/// The pipelines of a command, its here-documents left out, each the segments piped one into the
/// next; `None` when a job is sent to the background (`&`), which is no chain.
fn pipelines(command: &str) -> Option<Vec<Vec<String>>> {
    let mut chain: Vec<Vec<String>> = Vec::new();

    for (before, segment) in separated(&without_heredocs(command)) {
        if segment.is_empty() {
            continue;
        }
        match (before, chain.last_mut()) {
            ("&", _) => return None,
            ("|", Some(pipeline)) => pipeline.push(segment),
            _ => chain.push(vec![segment]),
        }
    }
    Some(chain)
}

/// Whether a pipeline is one Git command, then only read-only filters, its redirections only
/// reading or duplicating ([`is_git_read_through_filters`]).
fn is_git_pipeline(pipeline: &[String]) -> bool {
    let Some((first, filters)) = pipeline.split_first() else {
        return false;
    };
    let git = commands(first);

    matches!(git.as_slice(), [simple] if simple.words.first().is_some_and(|word| word == "git")
        && only_reads_and_duplicates(simple))
        && filters.iter().all(|filter| is_read_only_filter(filter))
}

/// The options of `git branch` that only list.
const BRANCH_LISTING: [&str; 11] = [
    "--show-current",
    "--list",
    "-l",
    "-a",
    "--all",
    "-r",
    "--remotes",
    "-v",
    "-vv",
    "--verbose",
    "--no-color",
];

/// Git's options that write a file or run a program, read by any unique prefix (`REF-GITCLI`):
/// `--output=<file>` writes the output "to a specific file instead of stdout" (`REF-GIT-DIFF`,
/// `REF-GIT-LOG`), `--ext-diff` allows "an external diff helper to be executed", and `grep`'s
/// `-O` and `--open-files-in-pager` open the matching files in a pager (`REF-GIT-GREP`).
const GIT_WRITING_OPTIONS: [&str; 3] = ["--output", "--ext-diff", "--open-files-in-pager"];

/// Whether the words of a Git command only read the repository: no option before its subcommand,
/// the subcommand one of [`READ_ONLY_GIT`], no option of [`GIT_WRITING_OPTIONS`] nor `grep`'s
/// `-O`, and `branch` only listing (EN-119).
#[must_use]
pub fn is_read_only_git(words: &[String]) -> bool {
    let [git, subcommand, arguments @ ..] = words else {
        return false;
    };
    let writes = arguments.iter().any(|word| {
        let name = word.split('=').next().unwrap_or_default();

        GIT_WRITING_OPTIONS
            .iter()
            .any(|option| abbreviates(name, option))
            || (subcommand == "grep"
                && word.starts_with('-')
                && !word.starts_with("--")
                && word.contains('O'))
    });

    git == "git"
        && READ_ONLY_GIT.contains(&subcommand.as_str())
        && !writes
        && (subcommand != "branch"
            || arguments
                .iter()
                .all(|word| BRANCH_LISTING.contains(&word.as_str())))
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

/// The Git subcommands that only read the repository, which a chain after the brief's `cd` may
/// join (EN-119): the reads the transcripts chain most (`status`, `diff`, `show`, `log`, I93) and
/// the plumbing that prints a reference or an object. `diff` shows changes, `log` "shows the
/// commit logs" and `grep` looks for patterns in the tracked files (`REF-GIT-DIFF`, `REF-GIT-LOG`,
/// `REF-GIT-GREP`), none writing but by the options of [`GIT_WRITING_OPTIONS`].
pub const READ_ONLY_GIT: [&str; 15] = [
    "blame",
    "branch",
    "cat-file",
    "describe",
    "diff",
    "for-each-ref",
    "grep",
    "log",
    "ls-files",
    "ls-tree",
    "merge-base",
    "rev-list",
    "rev-parse",
    "show",
    "status",
];

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
        Forbidden::HandLock => HAND_LOCK,
        Forbidden::UnknownPlace => UNKNOWN_PUSH,
    }
}

/// The reason for denying a shell command, every branch read empty: see [`denied_reason_on`].
#[must_use]
pub fn denied_reason(command: &str) -> Option<&'static str> {
    denied_reason_on(command, "", &|_| Some(String::new()))
}

/// The reason for denying a shell command run from `cwd`: the first command it runs that is
/// forbidden, every simple command and the command each wrapper runs read word by word (AUD-023,
/// [`crate::forbidden::first_forbidden`]), then a full Vitest run, which takes the heavy lock
/// through `npm run test` (EN-047). `branch` gives the current branch of a repository, asked only
/// for a push naming no destination, in each folder that push may run in (AUD-033).
#[must_use]
pub fn denied_reason_on(command: &str, cwd: &str, branch: BranchOf) -> Option<&'static str> {
    let found = invocations(&without_heredocs(command));

    first_forbidden(command, &Some(vec![cwd.to_owned()]), branch)
        .map(forbidden_text)
        .or_else(|| {
            found
                .iter()
                .any(|invocation| is_full_vitest_run(&invocation.words.join(" ")))
                .then_some(FULL_VITEST)
        })
}

/// The reason for denying a shell command run from `cwd`, the `package.json` scripts aside: a
/// forbidden command ([`denied_reason_on`]), then the forms of the worktree it enters by a leading
/// `cd` ([`entered_worktree_reason`]), then those of a worktree a later `cd` enters.
#[must_use]
pub fn shell_reason(command: &str, cwd: &str, branch: BranchOf) -> Option<String> {
    let judged = entered(command, cwd);

    denied_reason_on(command, cwd, branch)
        .map(str::to_owned)
        .or_else(|| {
            entered_worktree_reason(&judged).or_else(|| {
                later_folders(command, &judged.folder)
                    .iter()
                    .find_map(|folder| worktree_reason(&judged.command, folder))
            })
        })
}

/// The calls by which a Python program writes, moves or removes a file (EN-119), or runs a program
/// or a text that may (AUD-033); `shutil` by its functions that write, `.write(` read apart.
const PYTHON_WRITES: [&str; 25] = [
    "write_text(",
    "write_bytes(",
    ".touch(",
    ".rename(",
    "os.replace(",
    "os.remove(",
    ".unlink(",
    ".mkdir(",
    "os.makedirs(",
    "json.dump(",
    "shutil.copy",
    "shutil.move(",
    "shutil.rmtree(",
    "shutil.make_archive(",
    "shutil.unpack_archive(",
    "shutil.chown(",
    "inplace=",
    "os.system(",
    "os.popen(",
    "os.exec",
    "os.spawn",
    "os.posix_spawn",
    "pty.spawn(",
    "subprocess",
    "exec(",
];

/// The streams a `.write(` may print to without writing a file.
const PYTHON_STREAMS: [&str; 2] = ["sys.stdout", "sys.stderr"];

/// Whether a Python program's text writes a file (EN-119): a call of [`PYTHON_WRITES`]
/// (`write_text(`, `os.replace(`, `shutil.copy`, `fileinput`'s `inplace=`, `os.system(`,
/// `subprocess`, `exec(`…), a `.write(` or `.writelines(` on anything but `sys.stdout` or
/// `sys.stderr`, an `open(` given a quoted mode holding `w`, `a`, `x` or `+` (its first argument
/// for a method, `Path.open(mode)`, `REF-PYTHON-PATHLIB`), or an `os.open(` given flags other
/// than `O_RDONLY` (AUD-033). A script read from a file is not read.
#[must_use]
pub fn python_writes(code: &str) -> bool {
    let writes_to_a_file = [".write(", ".writelines("].iter().any(|call| {
        code.match_indices(call).any(|(at, _)| {
            !PYTHON_STREAMS
                .iter()
                .any(|stream| code[..at].ends_with(stream))
        })
    });

    PYTHON_WRITES.iter().any(|call| code.contains(call))
        || writes_to_a_file
        || code.match_indices("open(").any(|(at, call)| {
            let arguments = &code[at + call.len()..];
            let before = &code[..at];

            if before.ends_with("os.") {
                opens_flags_to_write(arguments)
            } else if before.ends_with('.') {
                opens_to_write(arguments, &[0, 1])
            } else {
                opens_to_write(arguments, &[1])
            }
        })
}

/// Whether the arguments of an `os.open(` give flags other than reading only: its second argument
/// is not `os.O_RDONLY`, `O_RDONLY` nor `0`.
fn opens_flags_to_write(arguments: &str) -> bool {
    top_level_arguments(arguments)
        .get(1)
        .is_some_and(|flags| !matches!(*flags, "os.O_RDONLY" | "O_RDONLY" | "0"))
}

/// Whether the arguments of an `open(` call, from after its parenthesis, give a mode that writes:
/// one of its arguments at `positions` or its `mode=`, a quoted string of `r`, `w`, `a`, `x`, `b`,
/// `t` and `+` holding one of `w`, `a`, `x`, `+`.
fn opens_to_write(arguments: &str, positions: &[usize]) -> bool {
    let parts = top_level_arguments(arguments);
    let positional = positions
        .iter()
        .filter_map(|position| parts.get(*position))
        .filter(|part| !part.contains('='))
        .copied();
    let keyword = parts
        .iter()
        .find_map(|part| part.strip_prefix("mode").map(str::trim_start))
        .and_then(|rest| rest.strip_prefix('='));

    positional.chain(keyword).any(|mode| {
        let mode = mode.trim();
        let text = ["'", "\""]
            .iter()
            .find_map(|quote| mode.strip_prefix(quote)?.strip_suffix(quote))
            .unwrap_or_default();

        text.chars().all(|letter| "rwaxbt+".contains(letter)) && text.contains(['w', 'a', 'x', '+'])
    })
}

/// The arguments of a call, from after its opening parenthesis to the one closing it, split at the
/// commas outside brackets and quotes, each trimmed.
fn top_level_arguments(text: &str) -> Vec<&str> {
    let mut parts = Vec::new();
    let mut depth = 0_usize;
    let mut quote: Option<char> = None;
    let mut start = 0;

    for (index, character) in text.char_indices() {
        match (quote, character) {
            (Some(open), _) if character == open => quote = None,
            (None, '\'' | '"') => quote = Some(character),
            (None, '(' | '[' | '{') => depth += 1,
            (None, ')') if depth == 0 => {
                parts.push(text[start..index].trim());
                return parts;
            }
            (None, ')' | ']' | '}') => depth = depth.saturating_sub(1),
            (None, ',') if depth == 0 => {
                parts.push(text[start..index].trim());
                start = index + 1;
            }
            _ => {}
        }
    }
    parts.push(text[start..].trim());
    parts
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
/// except that after the brief's `cd "<worktree>" || exit <n>` a chain of Git commands that only
/// read goes through, each alone or piped into read-only filters (`git status; git log | head`,
/// EN-119), as does one Git command of any kind piped into filters (EN-104, decided 2026-10-18: a
/// refusal costs a call); a Git command that writes, or another command, chained with a Git
/// command is still refused.
#[must_use]
pub fn entered_worktree_reason(judged: &Entered) -> Option<String> {
    if judged.guarded && is_read_only_git_chain(&judged.command) {
        return None;
    }
    worktree_reason(&judged.command, &judged.folder)
}

/// The folder a command runs in: the one a leading `cd <folder>` enters, its words read as the
/// shell reads them (EN-119) and resolved against the session's, a folder named by a variable, a
/// substitution or a `~` left to the session's; and the part of the command judged there, without
/// the brief's `cd "<folder>" || exit <n>` and the `;`, `&&` or line feed after it (ADR-0072),
/// which is no Git command chained. The part keeps its here-documents, which the judges leave out.
#[must_use]
pub fn entered(command: &str, cwd: &str) -> Entered {
    let whole = Entered {
        folder: cwd.to_owned(),
        command: command.to_owned(),
        guarded: false,
    };
    let text = without_heredocs(command);
    let pieces = spans(&text);
    let Some((folder, known)) = pieces
        .first()
        .and_then(|(_, start, end)| cd_folder(&text[*start..*end]))
    else {
        return whole;
    };
    let resolved = if known {
        lexical_join(cwd, &folder)
    } else {
        cwd.to_owned()
    };
    let guarded = matches!(pieces.get(1), Some(("||", start, end)) if is_exit(&text[*start..*end]))
        && pieces
            .get(2)
            .is_none_or(|(before, _, _)| matches!(*before, ";" | "&&" | "\n"));
    // The `cd` and the `exit` hold no here-document: up to the rest, both texts are the same.
    let rest = pieces
        .get(2)
        .map_or_else(String::new, |(_, start, _)| command[*start..].to_owned());

    Entered {
        folder: resolved,
        command: if guarded { rest } else { whole.command },
        guarded,
    }
}

/// The folder a segment `cd <folder>` enters, its one word unquoted, and whether it is known before
/// the shell runs: not when a variable, a substitution or a `~` gives it. `None` for any other
/// segment.
pub(crate) fn cd_folder(segment: &str) -> Option<(String, bool)> {
    let simples = commands(segment);
    let [simple] = simples.as_slice() else {
        return None;
    };
    let [cd, folder] = simple.words.as_slice() else {
        return None;
    };
    let known = !simple.substituted && !folder.contains(['$', '`', '~']);

    (cd == "cd" && !folder.is_empty()).then(|| (folder.clone(), known))
}

/// Whether a segment is `exit` or `exit <n>`, alone.
pub(crate) fn is_exit(segment: &str) -> bool {
    matches!(commands(segment).as_slice(), [simple] if simple.redirections.is_empty()
        && matches!(simple.words.as_slice(), [exit] if exit == "exit")
            | matches!(simple.words.as_slice(), [exit, code] if exit == "exit" && !code.is_empty() && code.bytes().all(|byte| byte.is_ascii_digit())))
}

/// The folders a command enters by a `cd` after its first segment, each resolved against the one
/// before, starting from `folder`: a story's worktree entered there is judged as one too.
#[must_use]
pub fn later_folders(command: &str, folder: &str) -> Vec<String> {
    let mut current = folder.to_owned();
    let mut folders = Vec::new();

    for segment in segments(&without_heredocs(command)).iter().skip(1) {
        if let Some((named, true)) = cd_folder(segment) {
            current = lexical_join(&current, &named);
            folders.push(current.clone());
        }
    }
    folders
}

/// The reason for denying, in a worktree, a command Claude Code would refuse there, or one the
/// project keeps out of it, each segment read word by word (EN-119): Vitest's mode `golden-write`,
/// which writes a file every worktree shares; a Python script writing a file ([`PYTHON_WRITE`]);
/// a Git command chained with another, `cd` included (549 refusals in E15), the hint naming the
/// form that works. `None` outside a worktree or for a command that fits.
#[must_use]
pub fn worktree_reason(command: &str, folder: &str) -> Option<String> {
    let root = worktree_root(folder)?;
    let pieces: Vec<String> = segments(&without_heredocs(command))
        .into_iter()
        .filter(|segment| !segment.is_empty())
        .collect();
    let simples: Vec<Simple> = pieces
        .iter()
        .flat_map(|segment| commands(segment))
        .collect();

    if simples.iter().any(writes_golden) {
        return Some(GOLDEN_WRITE.to_owned());
    }
    let found = invocations(&without_heredocs(command));
    let python = found.iter().any(|invocation| {
        invocation
            .words
            .first()
            .is_some_and(|program| is_python(program))
    });

    if found
        .iter()
        .any(|invocation| python_writes_file(invocation, command))
        || (python && found.iter().any(tees_a_file))
    {
        return Some(PYTHON_WRITE.to_owned());
    }
    chained_git_hint(&pieces, root)
}

/// Whether a command is a `tee` writing a file: an operand other than `-`, `/dev/null`,
/// `/dev/stdout` and `/dev/stderr` (AUD-033).
fn tees_a_file(invocation: &Invocation) -> bool {
    invocation
        .words
        .split_first()
        .is_some_and(|(program, operands)| {
            program_name(program) == "tee"
                && operands.iter().any(|operand| {
                    !operand.starts_with('-')
                        && !matches!(
                            operand.as_str(),
                            "/dev/null" | "/dev/stdout" | "/dev/stderr"
                        )
                })
        })
}

/// Whether a simple command runs Vitest's mode `golden-write` (`--mode golden-write` or
/// `--mode=golden-write`), the command of the golden file since `golden:write` went (EN-115).
fn writes_golden(simple: &Simple) -> bool {
    let words = &simple.words;

    words.iter().any(|word| word == "--mode=golden-write")
        || words
            .windows(2)
            .any(|pair| pair[0] == "--mode" && pair[1] == "golden-write")
}

/// Where a Python program's text comes from (`REF-PYTHON-CMDLINE`).
enum PythonProgram<'a> {
    /// The text of `-c`, glued to it (`-ccode`, `-Ic`) or the next word.
    Inline(&'a str),
    /// Its standard input: no script, `-`, or `/dev/stdin`.
    Input,
    /// A script file or a module, which the guard does not read.
    Elsewhere,
}

/// The program a Python command line runs, its options read as Python reads them: short ones
/// bundled (`-Ic`), `-W` and `-X` taking a value, the first of `-c`, `-m`, `-` or a script ending
/// the options.
fn python_program(arguments: &[String]) -> PythonProgram<'_> {
    let mut at = 0;

    while let Some(word) = arguments.get(at) {
        at += 1;
        if word == "-" {
            return PythonProgram::Input;
        }
        if word.starts_with("--") {
            continue;
        }
        let Some(letters) = word.strip_prefix('-') else {
            return if matches!(word.as_str(), "/dev/stdin" | "/dev/fd/0") {
                PythonProgram::Input
            } else {
                PythonProgram::Elsewhere
            };
        };
        for (index, letter) in letters.char_indices() {
            let rest = &letters[index + letter.len_utf8()..];

            match letter {
                'c' if rest.is_empty() => {
                    return arguments
                        .get(at)
                        .map_or(PythonProgram::Input, |code| PythonProgram::Inline(code));
                }
                'c' => return PythonProgram::Inline(rest),
                'm' => return PythonProgram::Elsewhere,
                'W' | 'X' => {
                    at += usize::from(rest.is_empty());
                    break;
                }
                _ => {}
            }
        }
    }
    PythonProgram::Input
}

/// The words of a simple command from its program on, the assignments before it skipped.
fn program_words(simple: &Simple) -> &[String] {
    let skipped = simple
        .words
        .iter()
        .take_while(|word| is_assignment(word))
        .count();

    &simple.words[skipped..]
}

/// Whether a program is Python: `python`, `python3`, `python3.12`, by any path.
fn is_python(word: &str) -> bool {
    program_name(word)
        .strip_prefix("python")
        .is_some_and(|version| {
            version
                .bytes()
                .all(|byte| byte.is_ascii_digit() || byte == b'.')
        })
}

/// Whether a command runs Python writing a file, a wrapper's looked through (AUD-033): its output
/// redirected to a file other than `/dev/null`, or its program text writing one
/// ([`python_writes`]), given by `-c`, or by a here-string or a here-document of `command` when it
/// reads its program from its input.
fn python_writes_file(invocation: &Invocation, command: &str) -> bool {
    let Some((program, arguments)) = invocation.words.split_first() else {
        return false;
    };

    if !is_python(program) {
        return false;
    }
    let redirects_output = invocation.redirections.iter().any(|redirection| {
        let operator = redirection.trim_start_matches(|digit: char| digit.is_ascii_digit());
        let target = operator.trim_start_matches(['&', '>', '|']);

        operator.trim_start_matches('&').starts_with('>')
            && !operator.starts_with(">&")
            && target != "/dev/null"
    });
    let here_strings = invocation
        .redirections
        .iter()
        .filter_map(|redirection| redirection.strip_prefix("<<<"));

    redirects_output
        || match python_program(arguments) {
            PythonProgram::Inline(code) => python_writes(code),
            PythonProgram::Input => heredoc_bodies(command)
                .into_iter()
                .chain(here_strings)
                .any(python_writes),
            PythonProgram::Elsewhere => false,
        }
}

/// The program a segment runs, its assignments skipped, and the subcommand of a Git one: `None`
/// for a segment that runs no Git command.
fn git_subcommand(segment: &str) -> Option<String> {
    commands(segment).iter().find_map(|simple| {
        let words = program_words(simple);

        (words.first().map(|word| program_name(word)) == Some("git"))
            .then(|| git_line(words).subcommand)
    })
}

/// The hint for a Git command chained with another in a worktree: `cd` into the worktree first,
/// since a subagent's calls start in the main checkout, then the Git command alone (AUD-020); and
/// the Rust command running the sequence in one call (EN-100), by its binary's path (ADR-0078).
/// `None` when no Git command is chained.
fn chained_git_hint(pieces: &[String], root: &str) -> Option<String> {
    let git_command = pieces
        .iter()
        .find(|segment| git_subcommand(segment).is_some())?;

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
    let subcommands: Vec<String> = pieces
        .iter()
        .filter_map(|segment| git_subcommand(segment))
        .collect();
    let hints: Vec<&str> = sequences
        .iter()
        .filter(|(names, _)| {
            subcommands
                .iter()
                .any(|subcommand| names.contains(&subcommand.as_str()))
        })
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
        "`npm run {script}`: no script `{script}` in the package.json of branch `{branch}`. A tool of the agent is called by its binary, `target/release/<binary>`, as the records' tools lists them (EN-115). A branch made before a script gets it by a rebase on its base: run `git fetch origin`, then `git rebase {base}`, one call each."
    )
}
