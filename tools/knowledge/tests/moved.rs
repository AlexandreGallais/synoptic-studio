//! Tests of the move into the records (E49): the decisions, the research, the derivations, the
//! bibliography, the logs, the guide, the memories, the metrics, the library, the archives, the
//! saves and the transcripts, each imported from files of a folder of the system's temporary
//! folder, then imported again without a change. The real records folder is never opened.

use std::fs;
use std::path::{Path, PathBuf};

use knowledge::moved::{
    Base, Moved, bullets_of, decision_of, first_date, first_sentence, guide_id, import_decisions,
    import_derivations, import_guide, import_logs, import_research, import_sources, page_id,
    row_cells, source_of, status_word, tagged, title_of, used_by,
};
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

/// The branch and day the tests import on.
const BASE: Base<'static> = Base {
    branch: "chore/en-155-x",
    today: "2026-10-25",
};

/// A folder of the temporary folder holding some files, and a store beside them.
fn place(name: &str, files: &[(&str, &str)]) -> (PathBuf, Connection) {
    let folder =
        std::env::temp_dir().join(format!("knowledge-moved-{name}-{}", std::process::id()));
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

/// The text of a merged document as `show` prints it, on another branch.
fn shown(database: &Connection, id: &str) -> Option<String> {
    show(database, "develop", id, &Filters::default()).unwrap()
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
fn reads_the_parts_of_a_markdown_page() {
    assert_eq!(title_of("# ADR-0081 — The records\n"), "The records");
    assert_eq!(title_of("intro\n# Plain title\n"), "Plain title");
    assert_eq!(title_of("no heading"), "");
    // The first date standing alone, never a part of a longer number.
    assert_eq!(first_date("v12026-10-25 then 2026-10-24."), "2026-10-24");
    assert_eq!(first_date("2026-10-2"), "");
    assert_eq!(
        status_word("Accepted (Product Owner, 2026-10-18)."),
        "accepted"
    );
    assert_eq!(status_word("done (→ `notes/0004.md`)"), "done");
    assert_eq!(status_word(""), "");
    assert_eq!(
        row_cells("| `REF-A` | a \\| b | c |"),
        ["`REF-A`", "a \\| b", "c"]
    );
    assert_eq!(row_cells("not a row"), [] as [String; 0]);
    assert_eq!(
        first_sentence("\n1. **The records live in a folder**. Then more."),
        "The records live in a folder."
    );
    assert_eq!(
        first_sentence("- One rule, no period"),
        "One rule, no period"
    );
    assert_eq!(page_id("adr", "README.md"), "PAGE-adr");
    assert_eq!(
        page_id("guide", "_epic-report.md"),
        "PAGE-guide-epic-report"
    );
    assert_eq!(guide_id("e01-draw-shapes.md"), "GUIDE-e01-draw-shapes");
    assert_eq!(guide_id("README.md"), "PAGE-guide");
    assert_eq!(
        used_by("# x\n\nUsed by: EN-031 (`isBitSet`). Sources: `REF-A`.\n"),
        "EN-031 (`isBitSet`)"
    );
    assert_eq!(used_by("# x\n"), "");
    assert_eq!(
        tagged("`#run` EN-050: done"),
        ("run".to_owned(), "EN-050: done".to_owned())
    );
    assert_eq!(tagged("plain"), (String::new(), "plain".to_owned()));
}

#[test]
fn reads_a_decision_its_status_date_and_principle() {
    let text = "# ADR-0081 — The private records\n\n**Status**: Accepted (Product Owner, 2026-10-18, EN-114).\n\n## Context\n\nOn 2026-10-17 a push failed.\n\n## Decision\n\n1. **The records live in a folder**, a plain one. More.\n";
    let decision = decision_of("0081-the-private-records.md", text).unwrap();

    assert_eq!(decision.id, "ADR-0081");
    assert_eq!(decision.title, "The private records");
    assert_eq!(decision.status, "accepted");
    // The status line's date, not the context's earlier one.
    assert_eq!(decision.date, "2026-10-18");
    assert_eq!(
        decision.principle,
        "The records live in a folder, a plain one."
    );
    assert_eq!(decision_of("README.md", text), None);
    assert_eq!(decision_of("81-short.md", text), None);
}

#[test]
fn imports_the_decisions_with_their_index_as_a_page_twice_alike() {
    let adr = "# ADR-0001 — Scope\n\n**Status**: Accepted (2026-10-08).\n\n## Decision\n\nGeometry only. Nothing else.\n\nSee ADR-0002.\n";
    let (folder, database) = place(
        "decisions",
        &[
            ("adr/0001-scope.md", adr),
            ("adr/README.md", "# Architecture Decision Records\n"),
        ],
    );
    let path = folder.join("files/adr");
    let (first, again) = twice(&database, |done| import_decisions(done, &path, BASE));
    let table = rows(
        &database,
        "SELECT id, title, status, date, principle FROM decisions",
    );
    let links = rows(&database, "SELECT \"from\", \"to\" FROM links");
    let page = shown(&database, "PAGE-adr");
    let decision = shown(&database, "ADR-0001");

    let _ = fs::remove_dir_all(&folder);
    assert_eq!(
        first,
        Moved {
            rows: 1,
            documents: 2
        }
    );
    assert_eq!(again, first);
    assert_eq!(
        table,
        ["ADR-0001\tScope\taccepted\t2026-10-08\tGeometry only."]
    );
    assert_eq!(links, ["ADR-0001\tADR-0002"]);
    assert_eq!(page.as_deref(), Some("# Architecture Decision Records\n"));
    assert_eq!(decision.as_deref(), Some(adr));
}

#[test]
fn imports_the_research_notes_requests_and_pages() {
    let (folder, database) = place(
        "research",
        &[
            ("research/README.md", "# Research protocol\n"),
            (
                "research/notes/0035-e49.md",
                "# 0035 — E49: the knowledge database\n\nSpike SP-034, 2026-10-25.\n",
            ),
            (
                "research/requests/0004-polygons.md",
                "# Research request 0004 — Polygons\n\n**Status**: done (→ `notes/0004.md`) — on 2026-10-09\n",
            ),
            ("research/requests/_template.md", "# Request template\n"),
        ],
    );
    let path = folder.join("files/research");
    let (first, again) = twice(&database, |done| import_research(done, &path, BASE));
    let table = rows(
        &database,
        "SELECT id, kind, title, status, date FROM notes ORDER BY id",
    );

    let _ = fs::remove_dir_all(&folder);
    assert_eq!(
        first,
        Moved {
            rows: 4,
            documents: 4
        }
    );
    assert_eq!(again, first);
    assert_eq!(
        table,
        [
            "NOTE-0035\tnote\tE49: the knowledge database\t\t2026-10-25",
            "PAGE-research\tpage\tResearch protocol\t\t",
            "PAGE-research-requests-template\tpage\tRequest template\t\t",
            "REQUEST-0004\trequest\tPolygons\tdone\t2026-10-09",
        ]
    );
}

#[test]
fn imports_the_derivations_by_their_names() {
    let (folder, database) = place(
        "derivations",
        &[
            (
                "derivations/bit-of-integer.md",
                "# DERIV-bit-of-integer — A bit, by division\n\nUsed by: EN-031. Sources: `REF-A`.\n",
            ),
            ("derivations/README.md", "# Derivations\n"),
        ],
    );
    let path = folder.join("files/derivations");
    let (first, again) = twice(&database, |done| import_derivations(done, &path, BASE));
    let table = rows(&database, "SELECT id, title, used_by FROM derivations");
    let page = shown(&database, "PAGE-derivations");

    let _ = fs::remove_dir_all(&folder);
    assert_eq!(
        first,
        Moved {
            rows: 1,
            documents: 2
        }
    );
    assert_eq!(again, first);
    assert_eq!(table, ["DERIV-bit-of-integer\tA bit, by division\tEN-031"]);
    assert_eq!(page.as_deref(), Some("# Derivations\n"));
}

#[test]
fn reads_a_source_its_address_status_and_date() {
    let verified = source_of(
        "| `REF-CSS-BR` | W3C, CSS Backgrounds: \"Let f = min(Lᵢ/Sᵢ)\". <https://www.w3.org/TR/css-backgrounds-3/> ; history: <https://example.org> | radii | [verified] 2026-10-08 |",
        "Shapes",
    )
    .unwrap();
    let unverified = source_of(
        "| `REF-GG` | R. D. Miller, Graphics Gems, 1990 | fillet | [unverified] — not found online |",
        "Shapes",
    )
    .unwrap();

    assert_eq!(verified.id, "REF-CSS-BR");
    assert_eq!(verified.title, "W3C, CSS Backgrounds");
    assert_eq!(verified.url, "https://www.w3.org/TR/css-backgrounds-3/");
    assert_eq!(verified.used_for, "radii");
    assert_eq!(
        (verified.status.as_str(), verified.verified.as_str()),
        ("verified", "2026-10-08")
    );
    assert_eq!(verified.topic, "Shapes");
    assert_eq!(unverified.title, "R. D. Miller, Graphics Gems, 1990");
    assert_eq!(unverified.url, "");
    assert_eq!(
        (unverified.status.as_str(), unverified.verified.as_str()),
        ("unverified", "")
    );
    assert_eq!(
        source_of("| ID | Reference | Used for | Status |", ""),
        None
    );
    assert_eq!(source_of("| `REF-A` | short |", ""), None);
}

#[test]
fn imports_the_bibliography_each_source_a_document_shown_by_its_id() {
    let text = "# References\n\nStatus: verified or not.\n\n## Shapes\n\n| ID | Reference | Used for | Status |\n| --- | --- | --- | --- |\n| `REF-A` | Author, Title: \"quoted\". <https://a.example> | the radius | [verified] 2026-10-11 |\n\n## Git\n\n| ID | Reference | Used for | Status |\n| --- | --- | --- | --- |\n| `REF-B` | Git, branch | branches | [unverified] |\n";
    let (folder, database) = place("sources", &[]);
    let (first, again) = twice(&database, |done| {
        import_sources(done, "docs/references.md", text, BASE)
    });
    let table = rows(
        &database,
        "SELECT id, title, url, used_for, status, verified, topic FROM sources ORDER BY id",
    );
    let source = shown(&database, "REF-A");
    let page = shown(&database, "PAGE-references");

    let _ = fs::remove_dir_all(&folder);
    assert_eq!(
        first,
        Moved {
            rows: 2,
            documents: 3
        }
    );
    assert_eq!(again, first);
    assert_eq!(
        table,
        [
            "REF-A\tAuthor, Title\thttps://a.example\tthe radius\tverified\t2026-10-11\tShapes",
            "REF-B\tGit, branch\t\tbranches\tunverified\t\tGit",
        ]
    );
    assert_eq!(
        source.as_deref(),
        Some(
            "# REF-A — Author, Title\n\n- Address: https://a.example\n- Used for: the radius\n- Status: verified 2026-10-11\n- Topic: Shapes\n\nAuthor, Title: \"quoted\". <https://a.example>\n"
        )
    );
    assert_eq!(
        page.as_deref(),
        Some("# References\n\nStatus: verified or not.\n")
    );
}

#[test]
fn reads_the_bullets_of_a_log_under_its_headings_continuations_joined() {
    let bullets = bullets_of(
        "# Journal\n\n- above any session\n\n## 2026-10-25 — A run\n\n- `#run` one\n  continued\n- `#decision` two\n\nA paragraph.\n  not a continuation\n",
    );

    assert_eq!(bullets.len(), 2);
    assert_eq!(bullets[0].heading, "2026-10-25 — A run");
    assert_eq!(bullets[0].text, "`#run` one\ncontinued");
    assert_eq!(bullets[1].text, "`#decision` two");
}

#[test]
fn imports_the_logs_and_the_retrospectives_never_the_process_pages_that_stay() {
    let (folder, database) = place(
        "logs",
        &[
            (
                "process/journal.md",
                "# Journal\n\nIntro.\n\n## 2026-10-25 — A run\n\n- `#run` EN-151: the tables\n- untagged\n",
            ),
            (
                "process/improvements.md",
                "# Improvement log\n\n| Date | From | Item | Kind |\n| --- | --- | --- | --- |\n| 2026-10-13 | agent | A watchdog | tool |\n| 2026-10-14 | agent | a | b | process |\n",
            ),
            (
                "process/feedback.md",
                "# Feedback\n\n| Date | What happened | What the agent should have done | Now prevented by |\n| --- | --- | --- | --- |\n| 2026-10-24 | Estimates too long | measure | the review |\n",
            ),
            (
                "process/tool-incidents.md",
                "# Tool incidents\n\n## 2026-10-23 — `mutate`, a run stopped\n\n- Used: `npm run mutate`.\n- Failed: its worktree left.\n- Instead: removed by hand,\n  then quoted.\n",
            ),
            (
                "process/history.md",
                "# History before 0.1.0\n\n## 2026-10-08\n",
            ),
            ("process/agile.md", "# Agile, stays\n"),
            (
                "process/retrospectives/E01-retrospective.md",
                "# E01 retrospective\n\nText.\n",
            ),
            ("process/retrospectives/README.md", "# Retrospectives\n"),
        ],
    );
    let path = folder.join("files/process");
    let (first, again) = twice(&database, |done| import_logs(done, &path, BASE));
    let journal = rows(&database, "SELECT date, session, tag, text FROM journal");
    let improvements = rows(
        &database,
        "SELECT date, origin, item, kind FROM improvements",
    );
    let feedback = rows(
        &database,
        "SELECT date, happened, should_have, prevented_by FROM feedback",
    );
    let incidents = rows(
        &database,
        "SELECT date, tool, used, failed, instead FROM incidents",
    );
    let retrospectives = rows(&database, "SELECT id, epic, title FROM retrospectives");
    let documents = rows(&database, "SELECT id, kind FROM documents ORDER BY id");

    let _ = fs::remove_dir_all(&folder);
    // 2 journal rows, 2 improvements, 1 feedback, 1 incident and 1 retrospective; 7 documents.
    assert_eq!(
        first,
        Moved {
            rows: 7,
            documents: 7
        }
    );
    // The second run inserts nothing new in the journal (ignored) but replaces the rest.
    assert_eq!(
        again,
        Moved {
            rows: 5,
            documents: 7
        }
    );
    assert_eq!(
        journal,
        [
            "2026-10-25\t2026-10-25 — A run\trun\tEN-151: the tables",
            "2026-10-25\t2026-10-25 — A run\t\tuntagged",
        ]
    );
    // An item cut by an unescaped pipe is joined back.
    assert_eq!(
        improvements,
        [
            "2026-10-13\tagent\tA watchdog\ttool",
            "2026-10-14\tagent\ta | b\tprocess"
        ]
    );
    assert_eq!(
        feedback,
        ["2026-10-24\tEstimates too long\tmeasure\tthe review"]
    );
    assert_eq!(
        incidents,
        [
            "2026-10-23\t`mutate`, a run stopped\t`npm run mutate`.\tits worktree left.\tremoved by hand,\nthen quoted."
        ]
    );
    assert_eq!(retrospectives, ["RETRO-E01\tE01\tE01 retrospective"]);
    assert_eq!(
        documents,
        [
            "LOG-feedback\tlog",
            "LOG-history\tlog",
            "LOG-improvements\tlog",
            "LOG-journal\tlog",
            "LOG-tool-incidents\tlog",
            "PAGE-retrospectives\tpage",
            "RETRO-E01\tretrospective",
        ]
    );
}

#[test]
fn imports_the_guide_pages_as_documents() {
    let (folder, database) = place(
        "guide",
        &[
            ("guide/e01-draw-shapes.md", "# E01 — Draw shapes\n"),
            ("guide/README.md", "# Guide\n"),
            ("guide/_epic-report.md", "# Report\n"),
        ],
    );
    let path = folder.join("files/guide");
    let (first, again) = twice(&database, |done| import_guide(done, &path, BASE));
    let documents = rows(
        &database,
        "SELECT id, kind, title FROM documents ORDER BY id",
    );

    let _ = fs::remove_dir_all(&folder);
    assert_eq!(
        first,
        Moved {
            rows: 0,
            documents: 3
        }
    );
    assert_eq!(again, first);
    assert_eq!(
        documents,
        [
            "GUIDE-e01-draw-shapes\tguide\tE01 — Draw shapes",
            "PAGE-guide\tpage\tGuide",
            "PAGE-guide-epic-report\tpage\tReport",
        ]
    );
}

#[test]
fn imports_the_memories_as_private_rows_and_documents_the_files_kept() {
    let memory = "---\nname: feedback-privacy\ndescription: \"never: the workplace\"\nmetadata:\n  type: feedback\n  modified: 2026-10-16T16:21:55.099Z\n---\n\nNever write it.\n";
    let (folder, database) = place(
        "memories",
        &[
            ("memory/feedback-privacy.md", memory),
            ("memory/MEMORY.md", "- [Privacy](feedback-privacy.md)\n"),
        ],
    );
    let path = folder.join("files/memory");
    let (first, again) = twice(&database, |done| import_memories(done, &path, BASE));
    let table = rows(
        &database,
        "SELECT name, description, type, private, modified FROM memories ORDER BY name",
    );
    let private = rows(&database, "SELECT id, private FROM documents ORDER BY id");
    let kept = path.join("feedback-privacy.md").is_file();

    let _ = fs::remove_dir_all(&folder);
    assert_eq!(
        first,
        Moved {
            rows: 2,
            documents: 2
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
    assert_eq!(private, ["MEMORY-MEMORY\t1", "MEMORY-feedback-privacy\t1"]);
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
