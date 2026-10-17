//! `PreToolUse` hook on `Bash` (ADR-0021, EN-104): denies the shell commands the project forbids,
//! whatever the instructions say (`hooks::guard`). A command opening with `cd "<folder>"` is judged
//! in that folder: a story's worktree refuses a Git command chained with another, the brief's
//! `cd "<worktree>" || exit 9` aside, and a script missing from that folder's `package.json` is
//! refused with the rebase that brings it. Input: the hook JSON on stdin; output: a deny decision
//! on stdout, or nothing to let the call proceed. An input that is not JSON exits 1, a hook error
//! that lets the call through, as the TypeScript hook did.

use std::fs;
use std::io::{self, Read};
use std::path::Path;
use std::process::{Command, ExitCode};

use hooks::guard::{
    denied_reason, entered, later_folders, missing_script, missing_script_reason, scripts_of,
    worktree_reason,
};
use hooks::json;
use hooks::limit::deny_output;

fn main() -> ExitCode {
    tool_use::run("guard-bash", "", hook)
}

/// Reads the hook's input and prints its refusal, if any.
fn hook() -> u8 {
    let mut text = String::new();
    let input = io::stdin()
        .read_to_string(&mut text)
        .ok()
        .and_then(|_| json::parse(&text));
    let Some(input) = input else {
        eprintln!("guard-bash: the hook's input is not JSON");
        return 1;
    };
    let command = input.get("tool_input").get("command").text();
    let reason = denied_reason(command).map(str::to_owned).or_else(|| {
        let judged = entered(command, input.get("cwd").text());

        worktree_reason(&judged.command, &judged.folder)
            .or_else(|| {
                later_folders(command, &judged.folder)
                    .iter()
                    .find_map(|folder| worktree_reason(&judged.command, folder))
            })
            .or_else(|| missing_script_refusal(&judged.command, &judged.folder))
    });

    if let Some(reason) = reason {
        print!("{}", deny_output(&reason));
    }
    0
}

/// The refusal of a script missing from the `package.json` npm would read in a folder, the nearest
/// above it; nothing when no `package.json` is found or read.
fn missing_script_refusal(command: &str, folder: &str) -> Option<String> {
    let segments = hooks::guard::segments(&hooks::guard::without_heredocs(command));

    if !segments
        .iter()
        .any(|segment| hooks::guard::npm_script(segment).is_some())
    {
        return None;
    }

    let package = Path::new(folder)
        .ancestors()
        .map(|ancestor| ancestor.join("package.json"))
        .find(|candidate| candidate.is_file())?;
    let scripts = scripts_of(&fs::read_to_string(&package).ok()?)?;
    let script = missing_script(command, &scripts)?;
    let root = package.parent()?;
    let branch = git(root, &["branch", "--show-current"]);
    let upstream = git(
        root,
        &[
            "rev-parse",
            "--abbrev-ref",
            "--symbolic-full-name",
            "@{upstream}",
        ],
    );
    // A story branch pushed tracks its own copy: its base is then not known here.
    let base = if upstream.is_empty() || upstream == format!("origin/{branch}") {
        "origin/<base>".to_owned()
    } else {
        upstream
    };

    Some(missing_script_reason(&script, &branch, &base))
}

/// The trimmed output of a Git command run in a folder, empty when it fails.
fn git(folder: &Path, arguments: &[&str]) -> String {
    Command::new("git")
        .args(arguments)
        .current_dir(folder)
        .output()
        .ok()
        .filter(|output| output.status.success())
        .map(|output| String::from_utf8_lossy(&output.stdout).trim().to_owned())
        .unwrap_or_default()
}
