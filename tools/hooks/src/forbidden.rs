//! The commands the project forbids, read word by word (AUD-023): every simple command of a line,
//! its substitutions' and subshells' included ([`crate::shell`]), then the command each wrapper
//! runs (`env`, `command`, `sudo`, `heavy-lock`, `cargo run … --`, `bash -c`, `eval`, `xargs`,
//! `find -exec` and the like), each judged as Git or `gh` would read it: bundled short options
//! (`-uf`), a long option by any unique prefix (`--no-veri`, `REF-GITCLI`), configuration keys
//! without case (`core.hookspath`, `REF-GIT-HOOKS-PATH`), the destination of each refspec of a
//! push (`REF-GIT-PUSH`).
//!
//! The guard keeps an agent from a mistake, not an adversary from the repository: a program that
//! runs a command of its own (`node -e`, `python -c`), a script file, an alias written earlier by
//! `git config` are not read.

use crate::shell::{Simple, commands};

/// One command a line runs, a wrapper's looked through: the assignments before it, then its words.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Invocation {
    /// The `NAME=value` words before the program, an `env`'s and an `export`'s included.
    pub assignments: Vec<String>,
    /// The program and its arguments, unquoted.
    pub words: Vec<String>,
}

/// How deep wrappers and quoted commands are looked through.
const DEPTH: usize = 8;

/// The words that open a compound command or a pipeline, skipped before the program.
const RESERVED: [&str; 14] = [
    "!", "{", "}", "if", "then", "else", "elif", "fi", "do", "done", "while", "until", "time", "in",
];

/// The branches no agent pushes to (AGENTS.md, Never).
pub const PROTECTED_BRANCHES: [&str; 2] = ["main", "develop"];

/// Every command a line runs, each simple command's and the ones its wrappers run.
#[must_use]
pub fn invocations(line: &str) -> Vec<Invocation> {
    let mut found = Vec::new();

    for simple in commands(line) {
        unwrap(&simple.words, Vec::new(), 0, &mut found);
    }
    found
}

/// Whether a word is a shell assignment, `NAME=value`.
fn is_assignment(word: &str) -> bool {
    word.split_once('=').is_some_and(|(name, _)| {
        name.starts_with(|first: char| first.is_ascii_alphabetic() || first == '_')
            && name
                .bytes()
                .all(|byte| byte.is_ascii_alphanumeric() || byte == b'_')
    })
}

/// The program's name without its folder: `/usr/bin/git` is `git`.
#[must_use]
pub fn program_name(word: &str) -> &str {
    word.rsplit('/').next().unwrap_or(word)
}

/// The words after a wrapper's options: each word opening with `-` skipped, the next one too after
/// an option of `valued`, and the words up to `--` included.
fn after_options<'a>(words: &'a [String], valued: &[&str]) -> &'a [String] {
    let mut at = 0;

    while let Some(word) = words.get(at) {
        if word == "--" {
            return &words[at + 1..];
        }
        if !word.starts_with('-') || word == "-" {
            break;
        }
        at += if valued.contains(&word.as_str()) {
            2
        } else {
            1
        };
    }
    words.get(at..).unwrap_or_default()
}

/// The commands of a text a shell would run, `bash -c "…"` or `eval …`, each looked through.
fn unwrap_text(text: &str, assignments: &[String], depth: usize, found: &mut Vec<Invocation>) {
    for Simple { words, .. } in commands(text) {
        unwrap(&words, assignments.to_vec(), depth + 1, found);
    }
}

/// The command of a simple command's words and those its wrapper runs, added to `found`.
fn unwrap(
    words: &[String],
    mut assignments: Vec<String>,
    depth: usize,
    found: &mut Vec<Invocation>,
) {
    if depth > DEPTH {
        return;
    }
    let mut at = 0;

    while let Some(word) = words.get(at) {
        if RESERVED.contains(&word.as_str()) {
            at += 1;
        } else if is_assignment(word) {
            assignments.push(word.clone());
            at += 1;
        } else {
            break;
        }
    }
    let Some(program) = words.get(at).map(|word| program_name(word)) else {
        if !assignments.is_empty() {
            found.push(Invocation {
                assignments,
                words: Vec::new(),
            });
        }
        return;
    };
    let rest = &words[at + 1..];

    found.push(Invocation {
        assignments: assignments.clone(),
        words: words[at..].to_vec(),
    });
    match program {
        "env" => unwrap_env(rest, assignments, depth, found),
        "export" | "declare" | "typeset" | "local" | "readonly" => found.push(Invocation {
            assignments: rest
                .iter()
                .filter(|word| is_assignment(word))
                .cloned()
                .collect(),
            words: Vec::new(),
        }),
        "bash" | "sh" | "zsh" | "dash" | "ksh" => {
            if let Some(text) = shell_text(rest) {
                unwrap_text(text, &assignments, depth, found);
            }
        }
        "eval" | "watch" => unwrap_text(
            &after_options(rest, &["-n", "--interval", "-d"]).join(" "),
            &assignments,
            depth,
            found,
        ),
        "find" => {
            for inner in find_commands(rest) {
                unwrap(inner, assignments.clone(), depth + 1, found);
            }
        }
        // `cargo run … -- <arguments>`: the arguments are a command for `heavy-lock`, its
        // `--lock <name>` first.
        "cargo" if rest.first().is_some_and(|word| word == "run") => {
            if let Some(dashes) = rest.iter().position(|word| word == "--") {
                let inner = after_options(&rest[dashes + 1..], &["--lock"]);

                unwrap(inner, assignments, depth + 1, found);
            }
        }
        _ => {
            if let Some(valued) = wrapper_options(program) {
                let inner = after_options(rest, valued);
                let inner = if program == "timeout" {
                    inner.get(1..).unwrap_or_default()
                } else {
                    inner
                };

                unwrap(inner, assignments, depth + 1, found);
            }
        }
    }
}

/// The options taking a value of a program that runs the command after its options, `None` for
/// any other program.
fn wrapper_options(program: &str) -> Option<&'static [&'static str]> {
    Some(match program {
        "command" | "builtin" | "exec" | "nohup" | "setsid" | "time" => &["-a"],
        "sudo" | "doas" => &[
            "-u", "-g", "-h", "-p", "-C", "-D", "-r", "-t", "-T", "-U", "-R",
        ],
        "nice" => &["-n", "--adjustment"],
        "timeout" => &["-s", "--signal", "-k", "--kill-after"],
        "stdbuf" => &["-i", "-o", "-e"],
        "ionice" => &["-c", "-n", "-p", "-P", "-u"],
        "caffeinate" => &["-t", "-w"],
        "xargs" => &["-I", "-L", "-n", "-P", "-s", "-d", "-E", "-a"],
        "heavy-lock" => &["--lock"],
        _ => return None,
    })
}

/// An `env`'s command: its options, `-u NAME` and `-C <folder>` with their value, its assignments
/// added to those before it, `-S <text>` read as a command line.
fn unwrap_env(
    words: &[String],
    mut assignments: Vec<String>,
    depth: usize,
    found: &mut Vec<Invocation>,
) {
    let mut at = 0;

    while let Some(word) = words.get(at) {
        let split = if word == "-S" || word == "--split-string" {
            Some((words.get(at + 1).map_or("", String::as_str), at + 2))
        } else {
            word.strip_prefix("--split-string=")
                .map(|text| (text, at + 1))
        };

        if let Some((text, after)) = split {
            let mut line = vec![text.to_owned()];

            line.extend(words.get(after..).unwrap_or_default().iter().cloned());
            unwrap_text(&line.join(" "), &assignments, depth, found);
            return;
        }
        if is_assignment(word) {
            assignments.push(word.clone());
            at += 1;
        } else if matches!(word.as_str(), "-u" | "--unset" | "-C" | "--chdir" | "-P") {
            at += 2;
        } else if word == "--" {
            at += 1;
            break;
        } else if word.starts_with('-') && word != "-" {
            at += 1;
        } else {
            break;
        }
    }
    unwrap(
        words.get(at..).unwrap_or_default(),
        assignments,
        depth + 1,
        found,
    );
}

/// The text a shell runs with `-c`: the first word after its options, when one of them holds `c`
/// (`-c`, `-lc`, `-ec`).
fn shell_text(words: &[String]) -> Option<&str> {
    let mut command = false;
    let mut at = 0;

    while let Some(word) = words.get(at) {
        at += 1;
        if matches!(word.as_str(), "-o" | "+o" | "-O" | "+O") {
            at += 1;
        } else if word.starts_with("--") {
        } else if let Some(letters) = word.strip_prefix(['-', '+']) {
            command |= letters.contains('c');
        } else {
            return command.then_some(word.as_str());
        }
    }
    None
}

/// The commands a `find` runs: the words after each `-exec`, `-execdir`, `-ok` or `-okdir`, up to
/// its `;` or `+`.
fn find_commands(words: &[String]) -> Vec<&[String]> {
    let mut found = Vec::new();
    let mut at = 0;

    while at < words.len() {
        if matches!(words[at].as_str(), "-exec" | "-execdir" | "-ok" | "-okdir") {
            let start = at + 1;
            let end = words[start..]
                .iter()
                .position(|word| word == ";" || word == "+")
                .map_or(words.len(), |length| start + length);

            found.push(&words[start..end]);
            at = end;
        }
        at += 1;
    }
    found
}

/// The options, shorts and longs, and the other words of a command's arguments, read as Git's
/// parser reads them (`REF-GITCLI`): `-abc` is `-a -b -c`; after a short option taking a value, the
/// rest of its word, or else the next word, is that value; a long option taking one takes the next
/// word unless written `--name=value`; `--` ends the options.
#[derive(Debug, Default)]
pub struct Arguments {
    /// The short options' letters, in order.
    pub shorts: Vec<char>,
    /// The long options as written, without their `=value`.
    pub longs: Vec<String>,
    /// The words that are no option nor an option's value.
    pub operands: Vec<String>,
}

impl Arguments {
    /// Reads the arguments, `short_valued` the letters and `long_valued` the long options taking a
    /// value, each by any unique prefix.
    #[must_use]
    pub fn read(words: &[String], short_valued: &str, long_valued: &[&str]) -> Self {
        let mut read = Self::default();
        let mut at = 0;

        while let Some(word) = words.get(at) {
            at += 1;
            if word == "--" {
                read.operands.extend(words[at..].iter().cloned());
                break;
            }
            if let Some(long) = word.strip_prefix("--") {
                let name = long.split('=').next().unwrap_or_default();
                let written = format!("--{name}");

                if !long.contains('=')
                    && long_valued
                        .iter()
                        .any(|option| abbreviates(&written, option))
                {
                    at += 1;
                }
                read.longs.push(written);
            } else if let Some(letters) =
                word.strip_prefix('-').filter(|letters| !letters.is_empty())
            {
                for (index, letter) in letters.char_indices() {
                    read.shorts.push(letter);
                    if short_valued.contains(letter) {
                        at += usize::from(index + letter.len_utf8() == letters.len());
                        break;
                    }
                }
            } else {
                read.operands.push(word.clone());
            }
        }
        read
    }

    /// Whether a long option is given, spelled out or by a prefix of it.
    #[must_use]
    pub fn has_long(&self, option: &str) -> bool {
        self.longs
            .iter()
            .any(|written| abbreviates(written, option))
    }
}

/// Whether a long option written stands for `option`: the same, or a prefix of it longer than
/// `--`. A prefix several options share makes Git stop with an error, so it is read as each.
#[must_use]
pub fn abbreviates(written: &str, option: &str) -> bool {
    written.len() > 2 && option.starts_with(written)
}

/// Whether a configuration key is `core.hooksPath`, its section and name without case.
#[must_use]
pub fn is_hooks_path(key: &str) -> bool {
    key.trim().eq_ignore_ascii_case("core.hooksPath")
}

/// Whether assignments set `core.hooksPath` for the Git they reach: `GIT_CONFIG_KEY_<n>` naming
/// it, or `GIT_CONFIG_PARAMETERS` holding it (`REF-GIT-HOOKS-PATH`, ENVIRONMENT).
#[must_use]
pub fn sets_hooks_path_by_environment(assignments: &[String]) -> bool {
    assignments.iter().any(|assignment| {
        let (name, value) = assignment.split_once('=').unwrap_or_default();
        let numbered = name.strip_prefix("GIT_CONFIG_KEY_").is_some_and(|index| {
            !index.is_empty() && index.bytes().all(|byte| byte.is_ascii_digit())
        });

        (numbered && is_hooks_path(value))
            || (name == "GIT_CONFIG_PARAMETERS"
                && value.to_ascii_lowercase().contains("core.hookspath"))
    })
}

/// What a Git command line asks: the settings of its `-c` and `--config-env`, its subcommand and
/// the words after it, an alias given by `-c alias.<name>=…` expanded once.
#[derive(Debug, Default)]
pub struct GitLine {
    /// The `<key>=<value>` of each `-c` and `--config-env`.
    pub settings: Vec<String>,
    /// The subcommand, empty when none.
    pub subcommand: String,
    /// The words after the subcommand.
    pub arguments: Vec<String>,
    /// The command line a `!` alias runs, empty when none.
    pub shell_alias: String,
}

/// Git's options before its subcommand that take the next word as their value.
const GIT_VALUED: [&str; 7] = [
    "-C",
    "--git-dir",
    "--work-tree",
    "--namespace",
    "--super-prefix",
    "--attr-source",
    "--list-cmds",
];

/// Reads a Git command line, `words` from `git` on.
#[must_use]
pub fn git_line(words: &[String]) -> GitLine {
    let mut line = GitLine::default();
    let mut at = 1;

    while let Some(word) = words.get(at).filter(|word| word.starts_with('-')) {
        at += 1;
        if word == "-c" || word == "--config-env" {
            line.settings.extend(words.get(at).cloned());
            at += 1;
        } else if let Some(setting) = word
            .strip_prefix("--config-env=")
            .or_else(|| word.strip_prefix("-c"))
        {
            line.settings.push(setting.to_owned());
        } else if GIT_VALUED.contains(&word.as_str()) {
            at += 1;
        }
    }
    let Some(subcommand) = words.get(at) else {
        return line;
    };
    let mut arguments: Vec<String> = words[at + 1..].to_vec();
    let alias = line.settings.iter().find_map(|setting| {
        let (key, value) = setting.split_once('=')?;

        key.get(..6)
            .filter(|section| section.eq_ignore_ascii_case("alias."))
            .filter(|_| key[6..].eq_ignore_ascii_case(subcommand))
            .map(|_| value.to_owned())
    });

    line.subcommand.clone_from(subcommand);
    if let Some(alias) = alias {
        if let Some(shell) = alias.strip_prefix('!') {
            shell.clone_into(&mut line.shell_alias);
        } else {
            let mut expanded: Vec<String> = commands(&alias)
                .into_iter()
                .next()
                .map(|simple| simple.words)
                .unwrap_or_default();

            if !expanded.is_empty() {
                line.subcommand = expanded.remove(0);
                expanded.append(&mut arguments);
                arguments = expanded;
            }
        }
    }
    line.arguments = arguments;
    line
}

/// Why a Git command is forbidden: it skips the hooks, switches them off, forces a push or pushes
/// to a protected branch; `branch` gives the current branch, which a push without a destination
/// updates.
#[must_use]
pub fn git_reason(words: &[String], branch: &dyn Fn() -> String) -> Option<Forbidden> {
    let line = git_line(words);

    if line
        .settings
        .iter()
        .any(|setting| is_hooks_path(setting.split('=').next().unwrap_or_default()))
    {
        return Some(Forbidden::HooksOff);
    }
    if !line.shell_alias.is_empty() {
        return invocations(&line.shell_alias)
            .iter()
            .find_map(|invocation| reason(invocation, branch));
    }
    match line.subcommand.as_str() {
        "commit" => {
            let read = Arguments::read(
                &line.arguments,
                "mFCct",
                &[
                    "--message",
                    "--file",
                    "--reuse-message",
                    "--reedit-message",
                    "--fixup",
                    "--squash",
                    "--author",
                    "--date",
                    "--template",
                    "--cleanup",
                    "--trailer",
                    "--pathspec-from-file",
                ],
            );

            (read.shorts.contains(&'n') || read.has_long("--no-verify"))
                .then_some(Forbidden::NoVerify)
        }
        "merge" | "am" | "rebase" => Arguments::read(
            &line.arguments,
            "mFsXx",
            &[
                "--message",
                "--file",
                "--strategy",
                "--strategy-option",
                "--onto",
                "--exec",
            ],
        )
        .has_long("--no-verify")
        .then_some(Forbidden::NoVerify),
        "push" => push_reason(&line.arguments, branch),
        "config" => config_sets_hooks_path(&line.arguments).then_some(Forbidden::HooksOff),
        _ => None,
    }
}

/// Why a push is forbidden: forced without lease (`-f`, `--force`, `--mirror`, a `+` refspec), its
/// hooks skipped, or one of [`PROTECTED_BRANCHES`] updated: named as a refspec's destination
/// (`HEAD:develop`, `refs/heads/main`, `:develop`), matched by a pattern or by `:`, every branch
/// pushed (`--all`, `--branches`), or the current branch when no refspec or `HEAD` names it.
fn push_reason(arguments: &[String], branch: &dyn Fn() -> String) -> Option<Forbidden> {
    let read = Arguments::read(
        arguments,
        "o",
        &["--repo", "--receive-pack", "--exec", "--push-option"],
    );
    let refspecs = read.operands.get(1..).unwrap_or_default();

    if read.shorts.contains(&'f')
        || read.has_long("--force")
        || read.has_long("--mirror")
        || read.operands.iter().any(|operand| operand.starts_with('+'))
    {
        return Some(Forbidden::ForcePush);
    }
    if read.has_long("--no-verify") {
        return Some(Forbidden::NoVerify);
    }
    let protected = |name: &str| {
        let name = name.strip_prefix("refs/heads/").unwrap_or(name);

        name.contains('*') || PROTECTED_BRANCHES.contains(&name)
    };
    let every_branch = read.has_long("--all") || read.has_long("--branches");
    let named = refspecs
        .iter()
        .filter(|refspec| !refspec.starts_with('^') && *refspec != "tag")
        .any(|refspec| match refspec.rsplit_once(':') {
            Some((_, "")) => true,
            Some((_, destination)) => protected(destination),
            None if *refspec == "HEAD" || *refspec == "@" => protected(&branch()),
            None => protected(refspec),
        });
    let current = refspecs.is_empty() && !every_branch && protected(&branch());

    (every_branch || named || current).then_some(Forbidden::ProtectedPush)
}

/// Whether a `git config` changes `core.hooksPath`: sets it (a value after the key, `set`,
/// `--add`, `--replace-all`), unsets it (`unset`, `--unset`, `--unset-all`), edits the file it is
/// in, or renames or removes the section `core`; reading it, alone or with `--get`, changes
/// nothing (EN-113).
fn config_sets_hooks_path(arguments: &[String]) -> bool {
    let read = Arguments::read(
        arguments,
        "f",
        &[
            "--file",
            "--blob",
            "--type",
            "--default",
            "--comment",
            "--value",
            "--url",
        ],
    );
    let (action, rest) = match read.operands.split_first() {
        Some((first, rest))
            if matches!(
                first.as_str(),
                "get" | "set" | "unset" | "list" | "edit" | "rename-section" | "remove-section"
            ) =>
        {
            (first.as_str(), rest)
        }
        _ => ("", read.operands.as_slice()),
    };
    let section_changed = action.ends_with("-section")
        || read.has_long("--rename-section")
        || read.has_long("--remove-section");

    if action == "edit" || read.has_long("--edit") || read.shorts.contains(&'e') {
        return true;
    }
    if section_changed {
        return rest
            .first()
            .is_some_and(|section| section.eq_ignore_ascii_case("core"));
    }
    let Some((_, values)) = rest.split_first().filter(|(key, _)| is_hooks_path(key)) else {
        return false;
    };
    let writes = ["--unset", "--unset-all", "--add", "--replace-all"]
        .iter()
        .any(|option| read.has_long(option));
    let getter = ["--get", "--get-all", "--get-regexp", "--get-urlmatch"]
        .iter()
        .any(|option| read.has_long(option));

    matches!(action, "set" | "unset")
        || writes
        || (action.is_empty() && !getter && !values.is_empty())
}

/// What a forbidden command does.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Forbidden {
    /// `gh pr merge`, or the API's merge.
    Merge,
    /// A commit, push or merge skipping the Git hooks.
    NoVerify,
    /// `core.hooksPath` changed.
    HooksOff,
    /// A push forced without lease.
    ForcePush,
    /// A push to `main` or `develop`.
    ProtectedPush,
}

/// Whether a `gh` command merges a pull request: `gh pr merge`, `pr`'s `-R <repository>` before
/// it allowed, or the REST or GraphQL merge through `gh api`.
fn gh_merges(words: &[String]) -> bool {
    match words.get(1).map(String::as_str) {
        Some("pr") => {
            let mut at = 2;

            while let Some(word) = words.get(at) {
                if word == "-R" || word == "--repo" {
                    at += 2;
                } else if word.starts_with('-') {
                    at += 1;
                } else {
                    return word == "merge";
                }
            }
            false
        }
        Some("api") => words[2..].iter().any(|word| {
            let path = word.trim_end_matches('/');

            (path.ends_with("/merge") && path.contains("/pulls/"))
                || word.contains("mergePullRequest")
        }),
        _ => false,
    }
}

/// Why one command is forbidden, if it is.
#[must_use]
pub fn reason(invocation: &Invocation, branch: &dyn Fn() -> String) -> Option<Forbidden> {
    if sets_hooks_path_by_environment(&invocation.assignments) {
        return Some(Forbidden::HooksOff);
    }
    match invocation.words.first().map(|word| program_name(word)) {
        Some("git") => git_reason(&invocation.words, branch),
        Some("gh") => gh_merges(&invocation.words).then_some(Forbidden::Merge),
        _ => None,
    }
}
