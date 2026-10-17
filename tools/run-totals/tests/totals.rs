//! Tests of a run's totals over its sessions' transcripts, in a temporary folder (EN-092).

use std::env;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};

use run_totals::marker::Run;
use run_totals::tally::{FileTally, Tokens};
use run_totals::totals::{Cache, parse_cache, projects_dir, render_cache, run_tokens, transcripts};

static NEXT: AtomicUsize = AtomicUsize::new(0);

fn folder() -> PathBuf {
    let path = env::temp_dir().join(format!(
        "run-totals-{}-{}",
        std::process::id(),
        NEXT.fetch_add(1, Ordering::SeqCst)
    ));

    let _ = fs::remove_dir_all(&path);
    fs::create_dir_all(path.join("s1").join("subagents")).expect("folder");
    path
}

fn request(id: &str, time: &str, output: i64) -> String {
    format!(
        "{{\"type\":\"assistant\",\"timestamp\":\"{time}\",\"message\":{{\"id\":\"{id}\",\"usage\":{{\"input_tokens\":1,\"cache_creation_input_tokens\":10,\"cache_read_input_tokens\":100,\"output_tokens\":{output}}}}}}}\n"
    )
}

fn append(path: &Path, text: &str) {
    OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .expect("transcript")
        .write_all(text.as_bytes())
        .expect("written");
}

fn open_run() -> Run {
    Run {
        start: 1,
        end: 0,
        sessions: vec!["s1".to_owned()],
    }
}

#[test]
fn lists_a_session_and_its_subagents_only() {
    let projects = folder();

    append(&projects.join("s1.jsonl"), "");
    append(&projects.join("s1/subagents/agent-b.jsonl"), "");
    append(&projects.join("s1/subagents/agent-a.jsonl"), "");
    append(&projects.join("s1/subagents/agent-a.meta.json"), "{}");
    append(&projects.join("other.jsonl"), "");

    assert_eq!(
        transcripts(&projects, &["s1".to_owned(), "missing".to_owned()]),
        vec![
            projects.join("s1.jsonl"),
            projects.join("s1/subagents/agent-a.jsonl"),
            projects.join("s1/subagents/agent-b.jsonl"),
        ]
    );
}

#[test]
fn f20_ac4_sums_the_session_and_its_subagents_and_reads_only_what_is_new() {
    let projects = folder();
    let main = projects.join("s1.jsonl");
    let agent = projects.join("s1/subagents/agent-a.jsonl");

    append(&main, &request("m1", "2026-10-17T10:00:00Z", 5));
    append(&agent, &request("a1", "2026-10-17T10:00:01Z", 7));

    // Each request: 1 + 10 + 100 + output; 116 + 118 = 234.
    let (tokens, cache) = run_tokens(&open_run(), &projects, &Cache::default());

    assert_eq!(tokens.total(), 234);
    assert_eq!(cache.files.len(), 2);

    // A compaction or a new message appends to the same file: only the new line is read, from the
    // offset the cache kept.
    append(&main, &request("m2", "2026-10-17T12:00:00Z", 20));

    let (more, next) = run_tokens(&open_run(), &projects, &cache);

    assert_eq!(more.total(), 234 + 131);
    assert_eq!(
        next.files[0].1.offset,
        usize::try_from(fs::metadata(&main).expect("main").len()).expect("size")
    );

    // A cache whose offset is past the end of a rewritten file reads it again from the start.
    let mut stale = next.clone();

    stale.files[1].1 = FileTally {
        offset: 1_000_000,
        last_id: String::new(),
        last: Tokens::default(),
        tokens: Tokens::default(),
    };
    assert_eq!(run_tokens(&open_run(), &projects, &stale).0.total(), 365);
}

#[test]
fn keeps_its_cache_for_one_run_only() {
    let cache = Cache {
        start: 42,
        end: 0,
        files: vec![(
            "/a \"b\".jsonl".to_owned(),
            FileTally {
                offset: 12,
                last_id: "m".to_owned(),
                last: Tokens {
                    input: 1,
                    cache_write: 0,
                    cache_read: 0,
                    output: 4,
                },
                tokens: Tokens {
                    input: 1,
                    cache_write: 2,
                    cache_read: 3,
                    output: 4,
                },
            },
        )],
    };
    let text = render_cache(&cache);

    let run = |start, end| Run {
        start,
        end,
        sessions: Vec::new(),
    };

    assert_eq!(parse_cache(&text, &run(42, 0)), cache);
    // Another run, or the same run once it ended: counted again from the start.
    assert_eq!(
        parse_cache(&text, &run(43, 0)),
        Cache {
            start: 43,
            end: 0,
            files: Vec::new()
        }
    );
    assert_eq!(parse_cache(&text, &run(42, 50)).files, Vec::new());
    assert_eq!(parse_cache("", &run(7, 0)).files, Vec::new());
}

#[test]
fn stops_counting_at_the_end_of_a_closed_run() {
    let projects = folder();

    append(
        &projects.join("s1.jsonl"),
        &request("m1", "2026-10-17T10:00:00Z", 5),
    );
    append(
        &projects.join("s1.jsonl"),
        &request("m2", "2099-01-01T00:00:00Z", 5),
    );

    // 2026-10-17T10:00:00Z = 1 792 231 200 000 ms; the 2099 request is after the end.
    let run = Run {
        start: 1,
        end: 1_792_231_200_000,
        sessions: vec!["s1".to_owned()],
    };

    assert_eq!(
        run_tokens(&run, &projects, &Cache::default()).0.total(),
        116
    );
}

#[test]
fn names_the_transcripts_folder_after_the_main_checkout() {
    // The Git directory's parent is the checkout; `/`, ` ` and `.` become `-`.
    assert_eq!(
        projects_dir(Path::new("/h"), Path::new("/Users/a b/svg.editor/.git")),
        PathBuf::from("/h/.claude/projects/-Users-a-b-svg-editor")
    );
}
