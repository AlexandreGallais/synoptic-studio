//! `PreToolUse` hook on `WebFetch` and `Bash` (EN-101): a page is read through the library. A
//! `WebFetch`, or a `curl` or `wget` printing a web page, is refused with the `library_get` call
//! to make instead, unless the library returned that page empty, read from its file by the
//! `sqlite3` command. Unreadable input lets the call through; an unreadable library refuses, since
//! `library_get` still works.

use std::env;
use std::io::{self, Read};
use std::process::Command;

use hooks::json;
use hooks::limit::deny_output;
use hooks::pages::{Library, library_file, library_query, library_state, page_of_command, refusal};

/// What the library holds of a page, read by `sqlite3` without writing the file.
fn library(url: &str) -> Library {
    let Some(file) = library_file(env::var("LIBRARY_PATH").ok(), env::var("HOME").ok()) else {
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

fn main() {
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

    if let Some(reason) = page.and_then(|url| refusal(&url, library(&url))) {
        print!("{}", deny_output(&reason));
    }
}
