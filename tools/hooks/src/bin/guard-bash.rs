//! `PreToolUse` hook on `Bash` (ADR-0021, EN-104): denies the shell commands the project forbids,
//! whatever the instructions say (`hooks::guard`), a push naming no destination judged by the
//! current branch of the folder it runs in. A command opening with `cd "<folder>"` is judged
//! in that folder: a story's worktree refuses a Git command chained with another, the brief's
//! `cd "<worktree>" || exit 9` followed by Git reads aside (EN-119), and a script missing from that
//! folder's `package.json` is refused with the rebase that brings it. Run on `Write`, `Edit` and
//! `NotebookEdit` too, it refuses a worker's write outside the worktree its prompt names
//! (`hooks::written`, EN-119). Input: the hook JSON on stdin; output: a deny decision on stdout,
//! or nothing to let the call proceed. An input that is not JSON exits 1, a hook error, which
//! blocks the call since its handler sets `onFailure` to `block` (AUD-023).

use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Read};
use std::path::Path;
use std::process::ExitCode;

use hooks::git::git;
use hooks::guard::{missing_script, missing_script_reason, scripts_of, shell_reason};
use hooks::handback::{agent_transcript, is_checked, worktree_id};
use hooks::json::{self, Value};
use hooks::limit::deny_output;
use hooks::written::misplaced_write;

/// The lines of a worker's transcript read for its worktree: its prompt opens it.
const PROMPT_LINES: usize = 20;

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
    let reason = match input.get("tool_name").text() {
        "Write" | "Edit" | "NotebookEdit" => write_refusal(&input),
        _ => shell_refusal(&input),
    };

    if let Some(reason) = reason {
        print!("{}", deny_output(&reason));
    }
    0
}

/// The refusal of a shell command, judged in the folder it runs in.
fn shell_refusal(input: &Value) -> Option<String> {
    let command = input.get("tool_input").get("command").text();
    let judged = hooks::guard::entered(command, input.get("cwd").text());
    // A push naming no destination updates the current branch of the folder it runs in.
    let branch =
        || git(Path::new(&judged.folder), &["branch", "--show-current"]).unwrap_or_default();

    shell_reason(command, input.get("cwd").text(), &branch)
        .or_else(|| missing_script_refusal(&judged.command, &judged.folder))
}

/// The refusal of a worker's write outside the worktree its prompt names; nothing for the main
/// thread, an agent that only reads, a prompt naming no worktree, or one no longer there.
fn write_refusal(input: &Value) -> Option<String> {
    let agent_id = input.get("agent_id").text();

    if !is_checked(agent_id, input.get("agent_type").text()) {
        return None;
    }
    let tool_input = input.get("tool_input");
    let file = [tool_input.get("file_path"), tool_input.get("notebook_path")]
        .into_iter()
        .map(Value::text)
        .find(|path| !path.is_empty())?;
    let transcript = File::open(agent_transcript(
        input.get("transcript_path").text(),
        agent_id,
    ))
    .ok()?;
    let worktree = BufReader::new(transcript)
        .lines()
        .take(PROMPT_LINES)
        .map_while(Result::ok)
        .find_map(|line| worktree_id(&line).map(str::to_owned))?;
    let project =
        std::env::var("CLAUDE_PROJECT_DIR").unwrap_or_else(|_| input.get("cwd").text().to_owned());

    if !Path::new(&project)
        .join(".claude/worktrees")
        .join(&worktree)
        .is_dir()
    {
        return None;
    }
    misplaced_write(file, &project, &worktree)
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
    let branch = git(root, &["branch", "--show-current"]).unwrap_or_default();
    let upstream = git(
        root,
        &[
            "rev-parse",
            "--abbrev-ref",
            "--symbolic-full-name",
            "@{upstream}",
        ],
    )
    .unwrap_or_default();
    // A story branch pushed tracks its own copy: its base is then not known here.
    let base = if upstream.is_empty() || upstream == format!("origin/{branch}") {
        "origin/<base>".to_owned()
    } else {
        upstream
    };

    Some(missing_script_reason(&script, &branch, &base))
}
