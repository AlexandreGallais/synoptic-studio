//! Where each part of a command line runs, for a push naming no destination, which updates the
//! current branch of the repository it runs against (AUD-033): the folders the `cd` before it may
//! have entered, then the Git command's own `-C`, `--git-dir` and `GIT_DIR` (`REF-GIT`). Only the
//! brief's `cd "<folder>" || exit <n>` at the top of the line enters a folder for certain; any
//! other `cd` adds its folder to those the line may be in, since it may fail or run in a subshell.
//! A folder the guard cannot know before the shell runs leaves the place unknown, and a push
//! naming no destination there is refused.

use git_repository::lexical_join;

use crate::forbidden::{Invocation, invocations, program_name};
use crate::guard::{cd_folder, is_exit, spans};

/// A repository a Git command may run against: the folder it runs in, and the Git folder a
/// `--git-dir` or a `GIT_DIR` names, `None` when Git finds it from the folder.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Repository {
    /// The folder the command runs in, absolute when the session's folder is.
    pub folder: String,
    /// The Git folder named, resolved against `folder`.
    pub git_dir: Option<String>,
}

/// The folders a part of a command line may run in, each one it may be; `None` when one of them
/// cannot be known before the shell runs.
pub type Folders = Option<Vec<String>>;

/// The current branch of a repository, `None` when it cannot be read.
pub type BranchOf<'a> = &'a dyn Fn(&Repository) -> Option<String>;

/// The folders of each top-level part of a command line, as [`walk`] finds them.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Walk {
    /// Each part's byte range in the line, and the folders it may run in.
    pub spans: Vec<(usize, usize, Folders)>,
    /// Every folder a part of the line may run in.
    pub every: Folders,
}

/// The words that open or close a compound command, after which a `cd` is no longer at the top
/// of the line.
const COMPOUND: [&str; 16] = [
    "if", "then", "else", "elif", "fi", "while", "until", "for", "do", "done", "case", "esac",
    "select", "function", "{", "}",
];

/// The folders of `first` and of `second` together, unknown when either is.
fn union(first: &Folders, second: &Folders) -> Folders {
    let (Some(first), Some(second)) = (first, second) else {
        return None;
    };
    let mut all = first.clone();

    for folder in second {
        if !all.contains(folder) {
            all.push(folder.clone());
        }
    }
    Some(all)
}

/// Whether a part of a line is a `cd` alone, `cd <folder>`: no bracket around it, one word after
/// it; its folder, and whether it is known before the shell runs (not a variable, a substitution,
/// `~` nor `-`).
fn lone_cd(segment: &str) -> Option<(String, bool)> {
    let trimmed = segment.trim();

    if !trimmed.starts_with("cd") || trimmed.ends_with(')') {
        return None;
    }
    cd_folder(trimmed).map(|(folder, known)| {
        let known = known && folder != "-";

        (folder, known)
    })
}

/// Whether a part of a line opens or closes a bracket or a compound command, outside quotes, so
/// that a `cd` after it may run in a subshell, a branch or a loop.
fn opens_structure(segment: &str) -> bool {
    crate::shell::commands(segment).iter().any(|simple| {
        simple
            .words
            .first()
            .is_some_and(|word| COMPOUND.contains(&word.as_str()))
    }) || {
        let mut quote = None;
        let mut escaped = false;

        segment.chars().any(|character| {
            let structural = quote.is_none() && !escaped && matches!(character, '(' | ')' | '`');

            match (quote, character) {
                _ if escaped => escaped = false,
                (Some('\''), '\'') | (Some('"'), '"') => quote = None,
                (None | Some('"'), '\\') => escaped = true,
                (None, '\'' | '"') => quote = Some(character),
                _ => {}
            }
            structural
        })
    }
}

/// Whether a command changes the folder or the repository of the commands after it in ways the
/// guard does not follow: a `cd`, `pushd` or `popd` that is no lone `cd`, an `env -C`, a
/// `sudo -D`, or a `GIT_DIR`, `GIT_WORK_TREE` or `GIT_COMMON_DIR` set for the rest of the line.
fn moves_unseen(invocation: &Invocation) -> bool {
    let words = &invocation.words;
    let Some(program) = words.first().map(|word| program_name(word)) else {
        return invocation.assignments.iter().any(|assignment| {
            ["GIT_DIR=", "GIT_WORK_TREE=", "GIT_COMMON_DIR="]
                .iter()
                .any(|name| assignment.starts_with(name))
        });
    };
    let has = |options: &[&str]| {
        words[1..].iter().any(|word| {
            options
                .iter()
                .any(|option| word == option || word.starts_with(&format!("{option}=")))
        })
    };

    match program {
        "cd" | "pushd" | "popd" => true,
        "env" => has(&["-C", "--chdir"]),
        "sudo" | "doas" => has(&["-D", "--chdir"]),
        _ => false,
    }
}

/// The folders each top-level part of `text`, its here-documents left out, may run in, from the
/// folders `start` the line starts in: after a lone `cd` at the top of the line followed by
/// `|| exit [n]`, its folder alone; after any other lone `cd`, the folders before it and its
/// own; after a command that moves where the guard cannot follow, none known.
#[must_use]
pub fn walk(text: &str, start: &Folders) -> Walk {
    let pieces = spans(text);
    let found = invocations(text);
    let mut current = start.clone();
    let mut every = start.clone();
    let mut nested = false;
    let mut out = Vec::new();

    for (index, (before, from, to)) in pieces.iter().enumerate() {
        let segment = &text[*from..*to];

        if let Some((folder, known)) = lone_cd(segment) {
            out.push((*from, *to, current.clone()));
            let entered = if !known {
                None
            } else if folder.starts_with('/') {
                Some(vec![lexical_join("/", &folder)])
            } else {
                current.as_ref().map(|folders| {
                    folders
                        .iter()
                        .map(|base| lexical_join(base, &folder))
                        .collect()
                })
            };
            let exits = matches!(pieces.get(index + 1), Some(("||", exit_from, exit_to))
                if is_exit(&text[*exit_from..*exit_to]) && !text[*exit_from..*exit_to].contains(')'))
                && pieces
                    .get(index + 2)
                    .is_none_or(|(after, _, _)| matches!(*after, ";" | "&&" | "\n"));
            let certain = exits && !nested && matches!(*before, "" | ";" | "&&" | "\n");

            current = if certain {
                entered
            } else {
                union(&current, &entered)
            };
            every = union(&every, &current);
            continue;
        }
        nested |= opens_structure(segment);
        if found
            .iter()
            .filter(|invocation| (*from..*to).contains(&invocation.at))
            .any(moves_unseen)
        {
            current = None;
            every = None;
        }
        out.push((*from, *to, current.clone()));
    }
    Walk { spans: out, every }
}

/// The folders the part of a line holding the byte `at` may run in.
#[must_use]
pub fn folders_at(walk: &Walk, at: usize) -> Folders {
    walk.spans
        .iter()
        .find(|(start, end, _)| (*start..*end).contains(&at))
        .map_or_else(|| walk.every.clone(), |(_, _, folders)| folders.clone())
}

/// Git's options before its subcommand that take the next word, other than `-C`, `--git-dir` and
/// `--work-tree`.
const GIT_VALUED: [&str; 6] = [
    "-c",
    "--config-env",
    "--namespace",
    "--super-prefix",
    "--attr-source",
    "--list-cmds",
];

/// Whether a path written in a command is known before the shell runs: not empty (a substitution's
/// place), nor holding a variable, a substitution, a `~` or `xargs`'s `{}`.
fn is_known(path: &str) -> bool {
    !path.is_empty() && !path.contains(['$', '`', '~']) && !path.contains("{}")
}

/// The repositories a Git command may run against: each folder, moved by its `-C` options in
/// turn, its Git folder named by `--git-dir` or else by a `GIT_DIR` of `assignments`, resolved
/// against the folder moved to; `None` when one cannot be known: a folder unknown, a path that is
/// no known one, or a working tree named apart (`--work-tree`, `GIT_WORK_TREE`).
#[must_use]
pub fn repositories(
    words: &[String],
    assignments: &[String],
    folders: &Folders,
) -> Option<Vec<Repository>> {
    let mut moves: Vec<&str> = Vec::new();
    let mut git_dir: Option<&str> = None;
    let mut at = 1;

    while let Some(word) = words.get(at).filter(|word| word.starts_with('-')) {
        at += 1;
        if word == "-C" {
            moves.push(words.get(at)?);
            at += 1;
        } else if word == "--git-dir" {
            git_dir = Some(words.get(at)?);
            at += 1;
        } else if let Some(value) = word.strip_prefix("--git-dir=") {
            git_dir = Some(value);
        } else if word == "--work-tree" || word.starts_with("--work-tree=") {
            return None;
        } else if GIT_VALUED.contains(&word.as_str()) {
            at += 1;
        }
    }
    let assigned = |name: &str| {
        assignments
            .iter()
            .rev()
            .find_map(|assignment| assignment.strip_prefix(&format!("{name}=")))
    };

    if assigned("GIT_WORK_TREE").is_some() {
        return None;
    }
    let git_dir = git_dir.or_else(|| assigned("GIT_DIR"));

    if !moves
        .iter()
        .chain(git_dir.iter())
        .all(|path| is_known(path))
    {
        return None;
    }
    folders.as_ref().map(|folders| {
        folders
            .iter()
            .map(|folder| {
                let folder = moves
                    .iter()
                    .fold(folder.clone(), |base, path| lexical_join(&base, path));

                Repository {
                    git_dir: git_dir.map(|path| lexical_join(&folder, path)),
                    folder,
                }
            })
            .collect()
    })
}
