//! `PreToolUse` hook on `WebFetch` and `Bash` (EN-101): a page is read through the library. A
//! `WebFetch`, or a `curl` or `wget` printing a web page, is refused with the `library_get` call
//! to make instead, unless the library returned that page empty, read from its file by the
//! `sqlite3` command. An agent whose file gives it no library tools is told to name the page to
//! its caller instead. Unreadable input lets the call through; an unreadable library refuses,
//! since `library_get` still works.

use std::env;
use std::fs;
use std::io::{self, Read};
use std::path::Path;
use std::process::Command;

use hooks::json;
use hooks::limit::deny_output;
use hooks::pages::{
    Library, has_library_tools, library_file_of_process, library_query, library_state,
    page_of_command, refusal, refusal_without_library,
};

/// What the library holds of a page, read by `sqlite3` without writing the file.
fn library(url: &str) -> Library {
    let Some(file) = library_file_of_process() else {
        return Library::Missing;
    };

    Command::new("sqlite3")
        .arg("-readonly")
        .arg(file)
        .arg(library_query(url))
        .output()
        .ok()
        .filter(|output| output.status.success())
        .map_or(Library::Missing, |output| {
            library_state(&String::from_utf8_lossy(&output.stdout))
        })
}

fn main() -> std::process::ExitCode {
    tool_use::run("library-pages", "", || {
        hook();
        0
    })
}

/// Reads the hook's input and prints its refusal, if any.
fn hook() {
    let mut text = String::new();

    if io::stdin().read_to_string(&mut text).is_err() {
        return;
    }

    let Some(input) = json::parse(&text) else {
        return;
    };
    let tool_input = input.get("tool_input");
    let page = match input.get("tool_name").text() {
        "WebFetch" => Some(tool_input.get("url").text().to_owned()).filter(|url| !url.is_empty()),
        "Bash" => page_of_command(tool_input.get("command").text()),
        _ => None,
    };

    let Some(url) = page else {
        return;
    };

    if let Some(reason) = refusal(&url, library(&url)) {
        let reason = if has_library(&input) {
            reason
        } else {
            refusal_without_library(&url)
        };

        print!("{}", deny_output(&reason));
    }
}

/// Whether the agent calling has the library's tools: the main thread and a built-in type have
/// them; a type of `.claude/agents/` has them when its file names them.
fn has_library(input: &json::Value) -> bool {
    let agent_type = input.get("agent_type").text();
    let project =
        env::var("CLAUDE_PROJECT_DIR").unwrap_or_else(|_| input.get("cwd").text().to_owned());

    agent_type.is_empty()
        || fs::read_to_string(
            Path::new(&project)
                .join(".claude/agents")
                .join(format!("{agent_type}.md")),
        )
        .map_or(true, |file| has_library_tools(&file))
}
