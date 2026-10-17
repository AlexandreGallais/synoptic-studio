//! Tests of the pages read through the library and of the `library-pages` hook (EN-101).

use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use hooks::json::{parse, quote};
use hooks::pages::{
    EMPTY_PAGE, Library, has_library_tools, library_file, library_query, library_state,
    page_of_command, refusal, refusal_without_library,
};

#[test]
fn a_curl_or_a_wget_printing_a_page_reads_it() {
    for command in [
        "curl https://www.sqlite.org/fts5.html",
        "curl -sSL 'https://code.claude.com/docs/en/hooks' | head -40",
        "cd /x && curl -s \"https://example.org/guide/\" > page.html",
        "LANG=C curl --compressed https://example.org/notes.md",
        "wget -qO- https://example.org/a",
        "/usr/bin/curl -L https://example.org/a?b=c#d",
    ] {
        assert!(page_of_command(command).is_some(), "{command}");
    }
    assert_eq!(
        page_of_command("curl -sSL 'https://code.claude.com/docs/en/hooks' | head -40").as_deref(),
        Some("https://code.claude.com/docs/en/hooks")
    );
}

#[test]
fn a_request_that_is_not_a_page_read_goes_through() {
    for command in [
        // A local server, the playground or the docs.
        "curl -s http://localhost:5173/?step=3",
        "curl http://127.0.0.1:8080/x",
        // Headers only, data sent, another method.
        "curl -I https://example.org/a",
        "curl -sI https://example.org/a",
        "curl -X POST https://example.org/a",
        "curl --data 'x=1' https://example.org/a",
        "curl -F f=@a https://example.org/a",
        // An interface for programs, not a page.
        "curl -s https://api.github.com/repos/a/b",
        "curl -s https://registry.example.org/api/v1/x",
        // A file downloaded, not a page.
        "curl -LO https://example.org/fonts/Inter.ttf",
        "wget https://example.org/a.pdf",
        // No address written, or another command naming one.
        "curl -s \"$URL\"",
        "echo https://example.org/a",
        "git log --grep curl",
        "wget --spider https://example.org/a",
    ] {
        assert_eq!(page_of_command(command), None, "{command}");
    }
}

#[test]
fn a_page_address_written_inside_a_here_document_is_data() {
    let command = "cat > note.md <<'EOF'\nA note.\ncurl https://example.org/a\nEOF\ngit status";

    assert_eq!(page_of_command(command), None);
    assert!(page_of_command("cat <<'EOF' > a\nx\nEOF\ncurl https://example.org/a").is_some());
    // `<<-` ends at its word indented by tabs.
    assert!(page_of_command("cat <<-EOF\n\tcurl https://example.org/a\n\tEOF").is_none());
}

#[test]
fn a_here_string_or_a_shift_opens_no_here_document() {
    // AUD-022: either once skipped every later line, a page read with them.
    for command in [
        "cat <<< x\ncurl https://example.org/a",
        "echo $((1<<2))\ncurl https://example.org/a",
        "echo $(( 1 << 2 ))\ncurl https://example.org/a",
        "echo $((x << n))\ncurl https://example.org/a",
    ] {
        assert_eq!(
            page_of_command(command).as_deref(),
            Some("https://example.org/a"),
            "{command}"
        );
    }
}

#[test]
fn a_quoted_address_is_kept_whole() {
    // A `&` or a `;` inside quotes cuts no command (AUD-022).
    assert_eq!(
        page_of_command("curl -s 'https://example.org/a?b=1&c=2;d' | head").as_deref(),
        Some("https://example.org/a?b=1&c=2;d")
    );
    assert_eq!(
        page_of_command("curl \"https://example.org/a?b=1&c=2\" && echo done").as_deref(),
        Some("https://example.org/a?b=1&c=2")
    );
    // Commands joined by `||` or `&&` are each read.
    assert!(page_of_command("false || curl https://example.org/a").is_some());
    assert!(page_of_command("echo x | curl https://example.org/a").is_some());
}

#[test]
fn a_script_run_an_archive_unpacked_or_a_file_saved_is_not_a_page_read() {
    for command in [
        // An installer run, an archive unpacked.
        "curl -fsSL https://sh.rustup.rs | sh -s -- -y",
        "curl -sSf https://example.org/install | bash",
        "curl -L https://example.org/release | tar -xz",
        // A link check: the status only, the page thrown away.
        "curl -s -o /dev/null -w '%{http_code}' https://example.org/a",
        "curl -so /dev/null https://example.org/a",
        // A file downloaded under a name of its own.
        "curl -L -o v1.tar.gz https://example.org/archive/v1",
        "curl --output data.bin https://example.org/x",
        "wget -O v1.zip https://example.org/archive/v1",
        // An interface for programs.
        "curl -s https://registry.npmjs.org/vitest",
        "curl -s https://crates.io/api/v1/crates/serde",
    ] {
        assert_eq!(page_of_command(command), None, "{command}");
    }
    // A page saved as a page is still read.
    for command in [
        "curl -o page.html https://example.org/a",
        "curl -sLo notes https://example.org/a",
        "curl -s https://example.org/a | sh-like-name",
    ] {
        assert!(page_of_command(command).is_some(), "{command}");
    }
}

#[test]
fn reads_every_form_of_a_wget_or_a_curl_that_prints() {
    for command in [
        "curl -X GET https://example.org/a",
        "wget -O - https://example.org/a",
        "wget --output-document=- https://example.org/a",
    ] {
        assert!(page_of_command(command).is_some(), "{command}");
    }
    assert_eq!(page_of_command("curl http://[::1]:8080/"), None);
}

#[test]
fn the_query_quotes_the_address() {
    let query = library_query("https://x.org/it's");

    assert!(query.contains("'https://x.org/it''s'"), "{query}");
    assert!(query.contains("FROM pages"));
    assert!(query.contains("FROM uses"));
}

#[test]
fn the_library_state_is_read_from_the_query_output() {
    assert_eq!(
        library_state("1234|online\n"),
        Library::Stored { characters: 1234 }
    );
    assert_eq!(
        library_state("12|failed"),
        Library::Stored { characters: 12 }
    );
    // Stored with nothing in it: still stored.
    assert_eq!(library_state("0|"), Library::Stored { characters: 0 });
    assert_eq!(library_state("-1|failed"), Library::Failed);
    assert_eq!(library_state("-1|"), Library::Missing);
    assert_eq!(library_state(""), Library::Missing);
    assert_eq!(library_state("not a number|x"), Library::Missing);
}

#[test]
fn a_page_the_library_holds_or_has_not_read_is_refused_with_the_call_to_make() {
    let url = "https://www.sqlite.org/fts5.html";
    let stored = refusal(
        url,
        Library::Stored {
            characters: EMPTY_PAGE,
        },
    )
    .expect("refused");
    let missing = refusal(url, Library::Missing).expect("refused");

    assert!(stored.contains("library_get"), "{stored}");
    assert!(stored.contains(url), "{stored}");
    assert!(stored.contains("holds"), "{stored}");
    assert!(missing.contains("library_get"), "{missing}");
    assert!(missing.contains("empty"), "{missing}");
}

#[test]
fn a_page_the_library_returned_empty_may_be_fetched_otherwise() {
    let url = "https://example.org/app";

    // 299 characters: below the 300 of an empty page; a page refused by its site, not stored.
    assert_eq!(EMPTY_PAGE, 300);
    assert_eq!(
        refusal(
            url,
            Library::Stored {
                characters: EMPTY_PAGE - 1
            }
        ),
        None
    );
    assert_eq!(refusal(url, Library::Stored { characters: 0 }), None);
    assert_eq!(refusal(url, Library::Failed), None);
}

#[test]
fn the_library_file_is_the_one_the_library_scripts_open() {
    assert_eq!(
        library_file(Some("/l.sqlite".into()), Some("/home/a".into())),
        Some(PathBuf::from("/l.sqlite"))
    );
    assert_eq!(
        library_file(None, Some("/home/a".into())),
        Some(PathBuf::from(
            "/home/a/.local/share/agent-library/library.sqlite"
        ))
    );
    assert_eq!(library_file(None, None), None);
}

/// Output of the hook for one tool call, run against a library file.
fn hook(tool: &str, input: &str, library: &Path) -> String {
    let line = format!(r#"{{"tool_name":{},"tool_input":{input}}}"#, quote(tool));
    let mut child = Command::new(env!("CARGO_BIN_EXE_library-pages"))
        .env("LIBRARY_PATH", library)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("hook starts");

    child
        .stdin
        .take()
        .expect("stdin")
        .write_all(line.as_bytes())
        .expect("input");
    let output = child.wait_with_output().expect("hook ends");

    assert!(output.status.success());
    String::from_utf8(output.stdout).expect("UTF-8")
}

/// A library file holding one full page and one empty page.
fn library(folder: &Path) -> PathBuf {
    let file = folder.join("library.sqlite");
    let full = "x".repeat(EMPTY_PAGE);
    let script = format!(
        "CREATE TABLE pages (url TEXT PRIMARY KEY, title TEXT, fetched_at INTEGER, modified_at INTEGER, etag TEXT, last_modified TEXT, markdown TEXT);
        CREATE TABLE uses (time INTEGER, tool TEXT, url TEXT, query TEXT, origin TEXT);
        INSERT INTO pages VALUES ('https://x.org/full', 'Full', 1, 0, '', '', '{full}');
        INSERT INTO pages VALUES ('https://x.org/app', 'App', 1, 0, '', '', 'Loading');
        INSERT INTO uses VALUES (1, 'get', 'https://x.org/refused', '', 'failed');"
    );
    let status = Command::new("sqlite3")
        .arg(&file)
        .arg(script)
        .status()
        .expect("sqlite3 runs");

    assert!(status.success());
    file
}

#[test]
fn the_hook_sends_web_fetch_and_curl_to_the_library_unless_the_page_came_back_empty() {
    let folder = std::env::temp_dir().join(format!("library-pages-{}", std::process::id()));

    fs::create_dir_all(&folder).expect("folder");
    let file = library(&folder);
    let fetch = |url: &str| hook("WebFetch", &format!(r#"{{"url":{}}}"#, quote(url)), &file);
    let full = fetch("https://x.org/full");
    let missing = fetch("https://x.org/new");
    let curl = hook(
        "Bash",
        r#"{"command":"curl -s https://x.org/full | head"}"#,
        &file,
    );
    let empty = fetch("https://x.org/app");
    let refused = fetch("https://x.org/refused");
    let local = hook(
        "Bash",
        r#"{"command":"curl http://localhost:5173/"}"#,
        &file,
    );
    let other = hook("Read", r#"{"file_path":"/a"}"#, &file);
    let absent = hook(
        "WebFetch",
        r#"{"url":"https://x.org/full"}"#,
        &folder.join("none.sqlite"),
    );

    fs::remove_dir_all(&folder).expect("cleaned");

    for output in [&full, &missing, &curl, &absent] {
        let decision = parse(output).expect("JSON");

        assert_eq!(
            decision
                .get("hookSpecificOutput")
                .get("permissionDecision")
                .text(),
            "deny",
            "{output}"
        );
    }
    assert!(full.contains("holds"), "{full}");
    assert!(curl.contains("holds"), "{curl}");
    assert!(!missing.contains("holds"), "{missing}");
    assert_eq!(
        [empty, refused, local, other],
        ["", "", "", ""].map(String::from)
    );
}

#[test]
fn the_hook_lets_the_call_through_when_it_cannot_read_its_input() {
    let mut child = Command::new(env!("CARGO_BIN_EXE_library-pages"))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("hook starts");

    child
        .stdin
        .take()
        .expect("stdin")
        .write_all(b"not JSON")
        .expect("input");
    let output = child.wait_with_output().expect("hook ends");

    assert!(output.status.success());
    assert_eq!(String::from_utf8_lossy(&output.stdout), "");
}

#[test]
fn an_agent_has_the_library_when_its_file_names_it_or_lists_no_tools() {
    let with = "---\nname: auditor\ntools: Read, Bash, mcp__library__library_get\n---\n# Auditor";
    let without = "---\nname: verifier\ntools: Read, Grep, Glob, Bash\n---\n# Verifier";
    let every = "---\nname: any\n---\ntools: library_get named in the body only";

    assert!(has_library_tools(with));
    assert!(!has_library_tools(without));
    assert!(has_library_tools(every));
    assert!(refusal_without_library("https://x.org/a").contains("name \"https://x.org/a\""));
}

/// Output of the hook for a `curl` of a stored page by an agent of this type, its file in
/// `project`.
fn hook_as(agent_type: &str, project: &Path, library: &Path) -> String {
    let line = format!(
        r#"{{"tool_name":"Bash","tool_input":{{"command":"curl https://x.org/full"}},"agent_type":{}}}"#,
        quote(agent_type)
    );
    let mut child = Command::new(env!("CARGO_BIN_EXE_library-pages"))
        .env("LIBRARY_PATH", library)
        .env("CLAUDE_PROJECT_DIR", project)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("hook starts");

    child
        .stdin
        .take()
        .expect("stdin")
        .write_all(line.as_bytes())
        .expect("input");
    String::from_utf8(child.wait_with_output().expect("hook ends").stdout).expect("UTF-8")
}

#[test]
fn the_hook_tells_an_agent_without_the_library_to_hand_the_page_to_its_caller() {
    let folder = std::env::temp_dir().join(format!("library-pages-agents-{}", std::process::id()));
    let agents = folder.join(".claude/agents");

    fs::create_dir_all(&agents).expect("folders");
    fs::write(
        agents.join("verifier.md"),
        "---\nname: verifier\ntools: Read, Bash\n---\n",
    )
    .expect("agent");
    fs::write(
        agents.join("researcher.md"),
        "---\nname: researcher\ntools: Bash, mcp__library__library_get\n---\n",
    )
    .expect("agent");
    let file = library(&folder);
    let verifier = hook_as("verifier", &folder, &file);
    let researcher = hook_as("researcher", &folder, &file);
    let built_in = hook_as("general-purpose", &folder, &file);

    fs::remove_dir_all(&folder).expect("cleaned");
    // Refused all three: the guard stays; only the instruction differs.
    assert!(verifier.contains("no library tools"), "{verifier}");
    assert!(verifier.contains("\"deny\""), "{verifier}");
    assert!(researcher.contains("holds"), "{researcher}");
    assert!(built_in.contains("holds"), "{built_in}");
}
