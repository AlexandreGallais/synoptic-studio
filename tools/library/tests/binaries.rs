//! Tests of the server and the command line as binaries, each on a library file of its own in the
//! temporary folder, `HOME` pointed there too, so that the machine's library is never opened
//! (EN-106). `LIBRARY_PATH` is set as Apple's `python3` sets it, and never opened.

use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};

use library::chunks::page_chunks;
use library::store::{StoredPage, open_library, save_page};
use rusqlite::Connection;
use serde_json::{Value, json};

/// Markdown of the stored page.
const MARKDOWN: &str = "# Hooks\n\nIntro.\n\n## Input\n\nThe `duration_ms` field.";

/// A folder of its own holding a library with one page read just now, and a home folder.
struct Place {
    folder: PathBuf,
}

impl Place {
    fn new(name: &str) -> Self {
        let folder =
            std::env::temp_dir().join(format!("library-binaries-{name}-{}", std::process::id()));
        let _ = fs::remove_dir_all(&folder);

        fs::create_dir_all(folder.join("home")).unwrap();

        let database = open_library(&folder.join("library.sqlite")).unwrap();
        let page = StoredPage {
            fetched_at: hooks::time::now_ms(),
            markdown: MARKDOWN.to_owned(),
            title: "Hooks".to_owned(),
            url: "https://x.org/h.md".to_owned(),
            ..StoredPage::default()
        };

        save_page(&database, &page, &page_chunks(MARKDOWN)).unwrap();

        Self { folder }
    }

    fn library(&self) -> PathBuf {
        self.folder.join("library.sqlite")
    }

    fn toolchain_path(&self) -> PathBuf {
        self.folder.join("toolchain-lib")
    }

    /// A command of a binary with the library named by its variable, no `GIT_*` inherited.
    fn command(&self, binary: &str) -> Command {
        let mut command = Command::new(binary);

        for (name, _) in std::env::vars().filter(|(name, _)| name.starts_with("GIT_")) {
            command.env_remove(name);
        }

        command
            .env("AGENT_LIBRARY_FILE", self.library())
            .env("LIBRARY_PATH", self.toolchain_path())
            .env("HOME", self.folder.join("home"))
            .current_dir(&self.folder);

        command
    }

    fn uses(&self) -> Vec<(String, String, String)> {
        Connection::open(self.library())
            .unwrap()
            .prepare("SELECT project, tool, origin FROM project_uses ORDER BY rowid")
            .unwrap()
            .query_map([], |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)))
            .unwrap()
            .collect::<Result<_, _>>()
            .unwrap()
    }
}

impl Drop for Place {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.folder);
    }
}

/// The command line's output for some arguments.
fn cli(place: &Place, arguments: &[&str]) -> Output {
    place
        .command(env!("CARGO_BIN_EXE_library"))
        .args(arguments)
        .output()
        .unwrap()
}

fn stdout(output: &Output) -> String {
    String::from_utf8_lossy(&output.stdout).into_owned()
}

/// The server's answers to some messages, one JSON line each, its input closed after them.
fn server(place: &Place, project: &str, messages: &[Value]) -> Vec<Value> {
    let mut child = place
        .command(env!("CARGO_BIN_EXE_library-mcp"))
        .env("CLAUDE_PROJECT_DIR", project)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let input = messages
        .iter()
        .map(Value::to_string)
        .collect::<Vec<_>>()
        .join("\n")
        + "\n";

    child
        .stdin
        .take()
        .unwrap()
        .write_all(input.as_bytes())
        .unwrap();

    let output = child.wait_with_output().unwrap();

    String::from_utf8_lossy(&output.stdout)
        .lines()
        .map(|line| serde_json::from_str(line).unwrap())
        .collect()
}

#[test]
fn f21_ac3_the_server_on_rmcp_answers_initialize_the_tools_and_a_call_over_stdio() {
    let place = Place::new("server");
    let answers = server(
        &place,
        "/home/a/studio/.claude/worktrees/en-106",
        &[
            json!({ "jsonrpc": "2.0", "id": 0, "method": "initialize", "params": {
                "protocolVersion": "2025-06-18", "capabilities": {},
                "clientInfo": { "name": "test", "version": "1" } } }),
            json!({ "jsonrpc": "2.0", "method": "notifications/initialized" }),
            json!({ "jsonrpc": "2.0", "id": 1, "method": "tools/list", "params": {} }),
            json!({ "jsonrpc": "2.0", "id": 2, "method": "tools/call", "params": {
                "name": "library_read", "arguments": { "url": "https://x.org/h.md", "sections": [0] } } }),
            json!({ "jsonrpc": "2.0", "id": 3, "method": "tools/call", "params": {
                "name": "library_search", "arguments": { "query": " " } } }),
        ],
    );
    let by_id = |id: i64| answers.iter().find(|answer| answer["id"] == id).unwrap();

    assert_eq!(by_id(0)["result"]["serverInfo"]["name"], "library");
    assert!(by_id(0)["result"]["capabilities"]["tools"].is_object());
    assert_eq!(by_id(1)["result"]["tools"].as_array().unwrap().len(), 3);
    assert_eq!(
        by_id(2)["result"],
        json!({ "content": [{ "type": "text", "text": "# Hooks\n\nIntro." }], "isError": false })
    );
    assert_eq!(by_id(3)["result"]["isError"], true);
    // The project is named by the folder Claude Code gives, the worktree's project.
    assert_eq!(
        place.uses(),
        [("studio".to_owned(), "read".to_owned(), "library".to_owned())]
    );
    assert!(!place.toolchain_path().exists());
}

#[test]
fn f21_ac3_each_command_prints_as_the_typescript_command_line_did() {
    let place = Place::new("cli");
    let search = cli(&place, &["search", "duration"]);
    let get = cli(&place, &["get", "https://x.org/h.md", "--refresh-not"]);
    let read = cli(&place, &["read", "https://x.org/h.md", "1"]);
    let list = cli(&place, &["list"]);
    let stats = cli(&place, &["stats"]);
    let since = cli(&place, &["stats", "2999-01-01"]);

    assert_eq!(
        stdout(&search),
        "https://x.org/h.md §1 — Hooks › Hooks › Input\n  ## Input\n\nThe `[duration]_ms` field.\n"
    );
    assert!(stdout(&get).starts_with("# Hooks\nhttps://x.org/h.md — read "));
    assert_eq!(stdout(&read), "## Input\n\nThe `duration_ms` field.\n");
    assert!(stdout(&list).ends_with("  https://x.org/h.md  Hooks\n"));
    // Two pages consulted (get, read), both from the library; the search found a section.
    assert!(stdout(&stats).starts_with("2 pages consulted, 100 % served by the library"));
    assert!(stdout(&stats).contains("| `duration` | 1 | 1 |"));
    assert!(stdout(&since).starts_with("0 pages consulted, 0 % served"));
    assert!(!place.toolchain_path().exists());
}

#[test]
fn an_unknown_command_or_a_date_temporal_refuses_prints_on_the_error_output_and_fails() {
    let place = Place::new("errors");

    for arguments in [
        &["nope"][..],
        &[][..],
        &["stats", "2026-13-01"],
        &["search"],
    ] {
        let output = cli(&place, arguments);

        assert!(!output.status.success(), "{arguments:?}");
        assert_eq!(stdout(&output), "", "{arguments:?}");
        assert!(!output.stderr.is_empty(), "{arguments:?}");
    }
}

#[test]
fn without_its_variable_the_library_is_in_the_home_data_folder_never_at_library_path() {
    let place = Place::new("home");
    let output = place
        .command(env!("CARGO_BIN_EXE_library"))
        .env_remove("AGENT_LIBRARY_FILE")
        .arg("list")
        .output()
        .unwrap();

    assert!(output.status.success());
    assert!(
        Path::new(&place.folder)
            .join("home/.local/share/agent-library/library.sqlite")
            .exists()
    );
    assert!(!place.toolchain_path().exists());
}

#[test]
fn without_its_variable_nor_a_home_neither_binary_starts() {
    let place = Place::new("nowhere");

    for binary in [
        env!("CARGO_BIN_EXE_library"),
        env!("CARGO_BIN_EXE_library-mcp"),
    ] {
        let output = place
            .command(binary)
            .env_remove("AGENT_LIBRARY_FILE")
            .env_remove("HOME")
            .env_remove("USERPROFILE")
            .arg("list")
            .stdin(Stdio::null())
            .output()
            .unwrap();

        assert!(!output.status.success(), "{binary}");
    }
}

#[test]
fn a_library_that_cannot_be_opened_stops_both_binaries() {
    let place = Place::new("unopenable");
    // A folder where the file should be: SQLite cannot open it.
    let folder = place.folder.join("a-folder");

    fs::create_dir_all(&folder).unwrap();

    for binary in [
        env!("CARGO_BIN_EXE_library"),
        env!("CARGO_BIN_EXE_library-mcp"),
    ] {
        let output = place
            .command(binary)
            .env("AGENT_LIBRARY_FILE", &folder)
            .arg("list")
            .stdin(Stdio::null())
            .output()
            .unwrap();

        assert!(!output.status.success(), "{binary}");
    }
}

#[test]
fn the_path_command_prints_the_library_file_without_creating_it() {
    let place = Place::new("path");
    let file = place.folder.join("not-yet.sqlite");
    let output = place
        .command(env!("CARGO_BIN_EXE_library"))
        .env("AGENT_LIBRARY_FILE", &file)
        .arg("path")
        .output()
        .unwrap();

    assert_eq!(stdout(&output), format!("{}\n", file.display()));
    assert!(!file.exists());
}
