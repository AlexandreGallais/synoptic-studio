//! Tests of the private records' importers (E49): the memories, the metrics, the library, the
//! archives, the saves and the transcripts, each imported from files of a folder of the system's
//! temporary folder, then imported again without a change. The real records folder is never
//! opened.

use std::fs;
use std::path::{Path, PathBuf};

use knowledge::moved::Moved;
use knowledge::private_records::{
    import_archives, import_library, import_memories, import_metrics, import_saves, last_save,
    memory_of,
};
use knowledge::read::{Filters, show};
use knowledge::store::{open_store, write};
use knowledge::transcripts::{elapsed_ms, extract, import_transcripts, typed_text};
use library::chunks::Chunk;
use library::store::{StoredPage, open_library, save_page};
use rusqlite::Connection;
use serde_json::json;

/// A folder of the temporary folder holding some files, and a store beside them.
fn place(name: &str, files: &[(&str, &str)]) -> (PathBuf, Connection) {
    let folder =
        std::env::temp_dir().join(format!("knowledge-private-{name}-{}", std::process::id()));
    let _ = fs::remove_dir_all(&folder);

    for (path, text) in files {
        let file = folder.join("files").join(path);

        fs::create_dir_all(file.parent().unwrap()).unwrap();
        fs::write(file, text).unwrap();
    }

    let database = open_store(&folder.join("records.sqlite")).unwrap();

    (folder, database)
}

/// The rows of a query, each its cells joined by tabs.
fn rows(database: &Connection, query: &str) -> Vec<String> {
    let mut statement = database.prepare(query).unwrap();
    let columns = statement.column_count();

    statement
        .query_map([], |row| {
            (0..columns)
                .map(|at| {
                    row.get::<_, rusqlite::types::Value>(at)
                        .map(|value| match value {
                            rusqlite::types::Value::Integer(number) => number.to_string(),
                            rusqlite::types::Value::Text(text) => text,
                            rusqlite::types::Value::Null => "null".to_owned(),
                            other => format!("{other:?}"),
                        })
                })
                .collect::<rusqlite::Result<Vec<String>>>()
                .map(|cells| cells.join("\t"))
        })
        .unwrap()
        .collect::<rusqlite::Result<_>>()
        .unwrap()
}

/// Runs an import twice, each in its write: the two results.
fn twice(
    database: &Connection,
    import: impl Fn(&rusqlite::Transaction<'_>) -> rusqlite::Result<Moved>,
) -> (Moved, Moved) {
    (
        write(database, |done| import(done)).unwrap(),
        write(database, |done| import(done)).unwrap(),
    )
}

#[test]
fn imports_the_memories_as_private_rows_alone_indexed_once_the_files_kept() {
    let memory = "---\nname: feedback-privacy\ndescription: \"never: the workplace\"\nmetadata:\n  type: feedback\n  modified: 2026-10-16T16:21:55.099Z\n---\n\nNever write it.\n";
    let (folder, database) = place(
        "memories",
        &[
            ("memory/feedback-privacy.md", memory),
            ("memory/MEMORY.md", "- [Privacy](feedback-privacy.md)\n"),
        ],
    );
    let path = folder.join("files/memory");
    let (first, again) = twice(&database, |done| import_memories(done, &path));
    let table = rows(
        &database,
        "SELECT name, description, type, private, modified FROM memories ORDER BY name",
    );
    let documents = rows(&database, "SELECT count(*) FROM documents");
    // Imported twice, each memory indexed once: the second import updates its row in place.
    let indexed = rows(
        &database,
        "SELECT id FROM records_index WHERE kind = 'memories' ORDER BY id",
    );
    let shown = show(&database, "MEMORY-feedback-privacy", &Filters::default()).unwrap();
    let kept = path.join("feedback-privacy.md").is_file();

    let _ = fs::remove_dir_all(&folder);
    assert_eq!(
        first,
        Moved {
            rows: 2,
            documents: 0
        }
    );
    assert_eq!(again, first);
    assert_eq!(
        table,
        [
            "MEMORY\t\t\t1\t",
            "feedback-privacy\tnever: the workplace\tfeedback\t1\t2026-10-16T16:21:55.099Z",
        ]
    );
    assert_eq!(documents, ["0"]);
    assert_eq!(indexed, ["MEMORY-MEMORY", "MEMORY-feedback-privacy"]);
    assert!(
        shown.as_deref().is_some_and(|text| text
            .starts_with("# MEMORY-feedback-privacy — never: the workplace\n")
            && text.contains("Never write it.")),
        "{shown:?}"
    );
    assert!(kept);
    assert_eq!(memory_of("x.md", "no front matter").name, "x");
}

#[test]
fn imports_the_metrics_once_each_line_its_fields_under_their_names() {
    let calls = "{\"toolUseId\":\"t1\",\"time\":\"2026-10-25T10:00:00Z\",\"tool\":\"Bash\",\"isOk\":true,\"durationMs\":40,\"extra\":1}\n{\"session\":\"s\",\"time\":\"2026-10-12T21:37:53Z\",\"tool\":\"Read\"}\nnot json\n[1]\n";
    let turns = "{\"session\":\"s\",\"start\":1,\"wallMs\":10}\n{\"session\":\"s\",\"start\":1,\"wallMs\":20}\n";
    let uses = "{\"tool\":\"page\",\"subcommand\":\"\",\"worktree\":\"en-1\",\"start\":\"2026-10-25T11:57:40Z\",\"durationMs\":3,\"exitCode\":0}\n";
    let (folder, database) = place(
        "metrics",
        &[
            ("metrics/tool-calls.jsonl", calls),
            ("metrics/turns.jsonl", turns),
            ("metrics/tool-uses.jsonl", uses),
        ],
    );
    let path = folder.join("files/metrics");
    let (first, again) = twice(&database, |done| import_metrics(done, &path));
    let table = rows(
        &database,
        "SELECT toolUseId, tool, isOk, durationMs FROM calls ORDER BY time",
    );
    let turns = rows(&database, "SELECT session, start, wallMs FROM turns");
    let uses = rows(
        &database,
        "SELECT tool, worktree, durationMs FROM tool_uses",
    );
    let missing = write(&database, |done| import_metrics(done, &folder)).unwrap();

    let _ = fs::remove_dir_all(&folder);
    // 2 calls, 2 turns' lines (the second replacing the first), 1 use; again only the turns.
    assert_eq!(
        first,
        Moved {
            rows: 5,
            documents: 0
        }
    );
    assert_eq!(
        again,
        Moved {
            rows: 2,
            documents: 0
        }
    );
    // A call without its id takes `<session>-<time>`.
    assert_eq!(
        table,
        [
            "s-2026-10-12T21:37:53Z\tRead\tnull\tnull",
            "t1\tBash\t1\t40"
        ]
    );
    assert_eq!(turns, ["s\t1\t20"]);
    assert_eq!(uses, ["page\ten-1\t3"]);
    assert_eq!(missing, Moved::default());
}

#[test]
fn merges_a_library_into_the_records_its_pages_replaced_and_its_uses_kept_once() {
    let (folder, database) = place("library", &[]);
    let source = open_library(&folder.join("library.sqlite")).unwrap();

    save_page(
        &source,
        &StoredPage {
            url: "https://a.example".to_owned(),
            title: "A".to_owned(),
            fetched_at: 1,
            modified_at: 1,
            etag: String::new(),
            last_modified: String::new(),
            markdown: "# A\n\ntext one\n\n## B\n\ntext two\n".to_owned(),
        },
        &[
            Chunk {
                heading: "A".to_owned(),
                index: 0,
                text: "# A\n\ntext one\n".to_owned(),
            },
            Chunk {
                heading: "A › B".to_owned(),
                index: 1,
                text: "## B\n\ntext two\n".to_owned(),
            },
        ],
    )
    .unwrap();
    source
        .execute(
            "INSERT INTO project_uses VALUES (1, 'library_get', 'https://a.example', '', 'agent', 'p')",
            [],
        )
        .unwrap();

    let (first, again) = twice(&database, |done| import_library(done, &source));
    // An older copy of the page, read at 0, never replaces the one read at 1.
    source
        .execute("UPDATE pages SET fetched_at = 0, title = 'Old'", [])
        .unwrap();
    let older = write(&database, |done| import_library(done, &source)).unwrap();
    let pages = rows(&database, "SELECT url, title FROM pages");
    let chunks = rows(&database, "SELECT count(*) FROM chunks");
    let uses = rows(&database, "SELECT count(*) FROM project_uses");
    let found = library::store::search_library(&database, "two", &8)
        .unwrap()
        .len();

    let _ = fs::remove_dir_all(&folder);
    assert_eq!(
        first,
        Moved {
            rows: 1,
            documents: 0
        }
    );
    assert_eq!(again, first);
    assert_eq!(older, Moved::default());
    assert_eq!(pages, ["https://a.example\tA"]);
    assert_eq!(chunks, ["2"]);
    assert_eq!(uses, ["1"]);
    // The library's own search reads the records.
    assert_eq!(found, 1);
}

#[test]
fn keeps_the_archives_as_blobs_and_the_saves_as_rows() {
    let (folder, database) = place(
        "archives",
        &[
            ("archives/f10.bundle", "bundle bytes"),
            ("last-save.txt", "2026-10-25T11:32:57.324658Z\n"),
        ],
    );
    let path = folder.join("files/archives");
    let first = write(&database, |done| {
        import_archives(done, &path).map_err(|error| {
            rusqlite::Error::SqliteFailure(rusqlite::ffi::Error::new(1), Some(error))
        })
    })
    .unwrap();
    let blob = rows(&database, "SELECT name, size, length(bytes) FROM archives");
    let text = fs::read_to_string(folder.join("files/last-save.txt")).unwrap();
    let saves = twice(&database, |done| import_saves(done, &text));
    let last = last_save(&database).unwrap();
    let missing = write(&database, |done| {
        Ok(import_archives(done, &folder.join("nowhere")).is_err())
    })
    .unwrap();

    let _ = fs::remove_dir_all(&folder);
    assert_eq!(
        first,
        Moved {
            rows: 1,
            documents: 0
        }
    );
    assert_eq!(blob, ["f10.bundle\t12\t12"]);
    assert_eq!(
        saves,
        (
            Moved {
                rows: 1,
                documents: 0
            },
            Moved {
                rows: 0,
                documents: 0
            }
        )
    );
    assert_eq!(last.as_deref(), Some("2026-10-25T11:32:57.324658Z"));
    assert!(missing);
}

#[test]
fn reads_the_messages_a_person_typed_never_a_tool_result_nor_a_tag() {
    let typed = json!({"type": "user", "message": {"content": "Lance la suite"}, "origin": {"kind": "human"}});
    let parts = json!({"type": "user", "message": {"content": [{"type": "text", "text": "Deux"}]}});
    let result =
        json!({"type": "user", "message": {"content": [{"type": "tool_result", "content": "x"}]}});
    let command =
        json!({"type": "user", "message": {"content": "<command-name>/x</command-name>"}});
    let meta = json!({"type": "user", "isMeta": true, "message": {"content": "meta"}});
    let side = json!({"type": "user", "isSidechain": true, "message": {"content": "agent"}});
    let task = json!({"type": "user", "origin": {"kind": "task"}, "message": {"content": "x"}});

    assert_eq!(typed_text(&typed).as_deref(), Some("Lance la suite"));
    assert_eq!(typed_text(&parts).as_deref(), Some("Deux"));
    for other in [result, command, meta, side, task] {
        assert_eq!(typed_text(&other), None, "{other}");
    }
}

#[test]
fn measures_the_time_between_two_instants() {
    // 14:27:42.480 to 14:28:43.000: 60.52 s.
    assert_eq!(
        elapsed_ms("2026-10-22T14:27:42.480Z", "2026-10-22T14:28:43Z"),
        60_520
    );
    // Across midnight: 23:59:59.900 to 00:00:00.100 the next day, 200 ms.
    assert_eq!(
        elapsed_ms("2026-10-22T23:59:59.900Z", "2026-10-23T00:00:00.1Z"),
        200
    );
    assert_eq!(elapsed_ms("", "2026-10-22T14:28:43Z"), 0);
}

/// A transcript of one session: a message typed, an answer in two lines with the same usage, a
/// Bash command, a tool's result.
fn transcript() -> String {
    [
        json!({"type": "user", "sessionId": "s1", "uuid": "u1", "timestamp": "2026-10-22T14:00:00.000Z", "message": {"content": "Bonjour"}}),
        json!({"type": "assistant", "timestamp": "2026-10-22T14:00:05.000Z", "message": {"id": "m1", "usage": {"input_tokens": 10, "cache_read_input_tokens": 90, "cache_creation_input_tokens": 0, "output_tokens": 5}, "content": [{"type": "text", "text": "hi"}]}}),
        json!({"type": "assistant", "timestamp": "2026-10-22T14:00:06.000Z", "message": {"id": "m1", "usage": {"input_tokens": 10, "cache_read_input_tokens": 90, "cache_creation_input_tokens": 0, "output_tokens": 5}, "content": [{"type": "tool_use", "id": "toolu_1", "name": "Bash", "input": {"command": "git status", "description": "Show status"}}]}}),
        json!({"type": "user", "uuid": "u2", "timestamp": "2026-10-22T14:01:00.000Z", "message": {"content": [{"type": "tool_result", "content": "clean"}]}}),
    ]
    .iter()
    .map(|line| line.to_string() + "\n")
    .collect()
}

#[test]
fn extracts_a_sessions_times_tokens_commands_and_messages() {
    let extracted = extract(&format!("{}not json\n", transcript()));

    assert_eq!(extracted.session.id, "s1");
    assert_eq!(extracted.session.start, "2026-10-22T14:00:00.000Z");
    assert_eq!(extracted.session.end, "2026-10-22T14:01:00.000Z");
    // One answer in two lines: its usage counted once, 10 + 90 + 0 read, 5 written.
    assert_eq!(extracted.session.input_tokens, 100);
    assert_eq!(extracted.session.output_tokens, 5);
    assert_eq!(extracted.commands.len(), 1);
    assert_eq!(extracted.commands[0].command, "git status");
    assert_eq!(extracted.messages.len(), 1);
    assert_eq!(extracted.messages[0].text, "Bonjour");
}

#[test]
fn imports_the_transcripts_an_unchanged_one_not_read_again() {
    let agent = format!(
        "{}\n",
        json!({"type": "assistant", "timestamp": "2026-10-22T14:00:30.000Z", "message": {"id": "a1", "content": [{"type": "tool_use", "id": "toolu_2", "name": "Bash", "input": {"command": "ls"}}]}})
    );
    let main = transcript();
    let (folder, database) = place(
        "transcripts",
        &[
            ("project/s1.jsonl", main.as_str()),
            ("project/s1/subagents/agent-a.jsonl", agent.as_str()),
            ("project/notes.txt", "not a transcript"),
        ],
    );
    let path: &Path = &folder.join("files/project");
    let (first, again) = twice(&database, |done| import_transcripts(done, path));
    let sessions = rows(
        &database,
        "SELECT id, start, end, wallMs, inputTokens, outputTokens, messages, commands FROM sessions",
    );
    let commands = rows(
        &database,
        "SELECT id, session, agent, command FROM commands ORDER BY id",
    );
    let messages = rows(&database, "SELECT id, session, text FROM messages");

    let _ = fs::remove_dir_all(&folder);
    // The agent's file, then the session's: 1 command, then 1 command and 1 message.
    assert_eq!(
        first,
        Moved {
            rows: 3,
            documents: 2
        }
    );
    assert_eq!(again, Moved::default());
    assert_eq!(
        sessions,
        ["s1\t2026-10-22T14:00:00.000Z\t2026-10-22T14:01:00.000Z\t60000\t100\t5\t1\t2"]
    );
    assert_eq!(
        commands,
        ["toolu_1\ts1\tmain\tgit status", "toolu_2\ts1\tagent-a\tls"]
    );
    assert_eq!(messages, ["u1\ts1\tBonjour"]);
}
