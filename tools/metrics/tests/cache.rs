//! Tests of the transcripts' cache (EN-107): a transcript read once, then from where it was left.

use std::env;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::PathBuf;

use metrics::cache::{parse_cache, parse_entry, read_transcripts, render_entry};
use metrics::transcript::{Transcript, read_chunk};

/// A request line of id `id` with `output` tokens written, at 10:00 on 2026-10-12.
fn line(id: &str, output: i64) -> String {
    format!(
        "{{\"type\":\"assistant\",\"sessionId\":\"s\",\"gitBranch\":\"feat/en-107-x\",\"timestamp\":\"2026-10-12T10:00:00Z\",\"message\":{{\"id\":\"{id}\",\"model\":\"claude-opus-5-5\",\"usage\":{{\"output_tokens\":{output}}},\"content\":[{{\"type\":\"tool_use\",\"id\":\"t-{id}\",\"name\":\"Bash\",\"input\":{{\"command\":\"echo \\\"a\\\"\"}}}}]}}}}\n"
    )
}

/// A later line of request `id`, its usage only.
fn usage(id: &str, output: i64) -> String {
    format!(
        "{{\"type\":\"assistant\",\"timestamp\":\"2026-10-12T10:00:01Z\",\"message\":{{\"id\":\"{id}\",\"usage\":{{\"output_tokens\":{output}}}}}}}\n"
    )
}

/// A result line for the call of request `id`.
fn result(id: &str) -> String {
    format!(
        "{{\"type\":\"user\",\"timestamp\":\"2026-10-12T10:00:02Z\",\"message\":{{\"content\":[{{\"type\":\"tool_result\",\"tool_use_id\":\"t-{id}\",\"is_error\":true}}]}}}}\n"
    )
}

/// A folder of its own for a test.
fn folder(name: &str) -> PathBuf {
    let path = env::temp_dir().join(format!("metrics-cache-{}-{name}", std::process::id()));

    let _ = fs::remove_dir_all(&path);
    fs::create_dir_all(&path).expect("folder");
    path
}

#[test]
fn writes_a_transcript_and_reads_it_back_the_same() {
    let text = format!("{}{}", line("m1", 5), result("m1"));
    let transcript = read_chunk(Transcript::default(), &text);
    let entry = render_entry("/a b/\"c\".jsonl", &transcript);

    assert_eq!(
        parse_entry(&entry),
        Some(("/a b/\"c\".jsonl".to_owned(), transcript))
    );
    assert_eq!(parse_entry("not json"), None);
}

#[test]
fn drops_a_cache_of_another_version() {
    let entry = render_entry("x", &Transcript::default());

    assert_eq!(
        parse_cache(&format!("{{\"version\":1}}\n{entry}\n")).len(),
        1
    );
    assert!(parse_cache(&format!("{{\"version\":0}}\n{entry}\n")).is_empty());
    assert!(parse_cache("").is_empty());
}

#[test]
fn reads_a_transcript_from_where_the_cache_left_it() {
    let dir = folder("incremental");
    let file = dir.join("s.jsonl");
    let cache = dir.join("cache.jsonl");

    fs::write(&file, line("m1", 5)).expect("transcript");
    let first = read_transcripts(std::slice::from_ref(&file), &cache);

    assert_eq!(first[0].events().requests.len(), 1);
    assert!(
        fs::read_to_string(&cache)
            .expect("cache written")
            .starts_with("{\"version\":1}\n")
    );

    // The request goes on (its last line counts 9 written), its call is answered, a second one is
    // asked: read from the offset, with what the cache held.
    let mut append = OpenOptions::new().append(true).open(&file).expect("open");

    append
        .write_all(format!("{}{}{}", usage("m1", 9), result("m1"), line("m2", 1)).as_bytes())
        .expect("append");
    let second = read_transcripts(std::slice::from_ref(&file), &cache);
    let events = second[0].events();
    let whole = read_chunk(
        Transcript::default(),
        &fs::read_to_string(&file).expect("text"),
    );

    assert_eq!(second[0], whole);
    assert_eq!(events.requests.len(), 2);
    assert_eq!(events.requests[0].tokens.output, 9);
    assert_eq!(events.calls.len(), 1);
    assert!(!events.calls[0].answer.is_ok);

    // Rewritten shorter: read again from its start.
    fs::write(&file, line("m3", 2)).expect("rewritten");
    let third = read_transcripts(std::slice::from_ref(&file), &cache);

    assert_eq!(third[0].events().requests[0].id, "m3");

    // A transcript gone leaves the cache at its next writing; one missing reads as empty.
    let missing = dir.join("gone.jsonl");
    let none = read_transcripts(std::slice::from_ref(&missing), &cache);

    assert_eq!(none[0], Transcript::default());
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn reads_the_same_without_a_cache_it_can_write() {
    let dir = folder("read-only");
    let file = dir.join("s.jsonl");

    fs::write(&file, line("m1", 5)).expect("transcript");
    // The Git folder above the cache's does not exist: nothing written, the transcript read all
    // the same.
    let read = read_transcripts(
        std::slice::from_ref(&file),
        &dir.join("no-git").join("metrics").join("cache.jsonl"),
    );

    assert_eq!(read[0].events().requests.len(), 1);
    assert!(!dir.join("no-git").exists());

    // A Git folder without its `metrics` folder yet: made, then the cache written in it.
    fs::create_dir_all(dir.join("git")).expect("git folder");
    let cache = dir.join("git").join("metrics").join("cache.jsonl");
    let _ = read_transcripts(std::slice::from_ref(&file), &cache);

    assert!(cache.is_file());
    let _ = fs::remove_dir_all(&dir);
}
